use std::process;

use clap::Parser;

use rs_rawzip2jsons::stdin2zip2jsons2stdout;

const MAX_ZIP_BYTES_DEFAULT: u64 = 1 << 20;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value_t = MAX_ZIP_BYTES_DEFAULT)]
    max_zip_bytes: u64,

    #[arg(long)]
    append_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    let newline: &[u8] = if cli.append_newline { b"\n" } else { b"" };
    if let Err(e) = stdin2zip2jsons2stdout(newline, cli.max_zip_bytes) {
        eprintln!("failed to process zip from stdin: {e}");
        process::exit(1);
    }
}
