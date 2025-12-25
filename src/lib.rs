use std::io;

use io::BufWriter;
use io::Write;

use io::Read;

use rawzip::ZipArchive;
use rawzip::ZipArchiveEntryWayfinder;
use rawzip::ZipSliceArchive;
use rawzip::ZipSliceEntries;
use rawzip::ZipSliceEntry;

pub struct Archive<'a>(pub ZipSliceArchive<&'a [u8]>);

impl<'a> Archive<'a> {
    pub fn way2entry(
        &self,
        finder: ZipArchiveEntryWayfinder,
    ) -> Result<ZipSliceEntry<'_>, rawzip::Error> {
        self.0.get_entry(finder)
    }

    pub fn way2entry_raw(&self, finder: ZipArchiveEntryWayfinder) -> Result<&[u8], rawzip::Error> {
        self.way2entry(finder).map(|ent| ent.data())
    }

    pub fn entries(&self) -> ZipSliceEntries<'_> {
        self.0.entries()
    }

    pub fn ways(&self) -> impl Iterator<Item = Result<ZipArchiveEntryWayfinder, rawzip::Error>> {
        self.entries().map(|rslt| rslt.map(|hdr| hdr.wayfinder()))
    }
}

pub fn slice2archive(s: &[u8]) -> Result<Archive<'_>, rawzip::Error> {
    ZipArchive::from_slice(s).map(Archive)
}

pub fn json2writer<W>(jbytes: &[u8], newline: &[u8], mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    wtr.write_all(jbytes)?;
    wtr.write_all(newline)?;
    Ok(())
}

pub fn slice2zip2jsons2writer<W>(zip: &[u8], newline: &[u8], mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let arc: Archive = slice2archive(zip).map_err(io::Error::other)?;
    let ways = arc.ways();
    for rway in ways {
        let way: ZipArchiveEntryWayfinder = rway.map_err(io::Error::other)?;
        let json_bytes: &[u8] = arc.way2entry_raw(way).map_err(io::Error::other)?;
        json2writer(json_bytes, newline, &mut wtr)?;
    }
    wtr.flush()
}

pub fn stdin2zip2jsons2stdout(newline: &[u8], max_zip_size: u64) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let mut taken = il.take(max_zip_size);
    let mut buf: Vec<u8> = vec![];
    taken.read_to_end(&mut buf)?;

    let o = io::stdout();
    let mut ol = o.lock();
    let bw = BufWriter::new(&mut ol);
    slice2zip2jsons2writer(&buf, newline, bw)?;
    ol.flush()
}
