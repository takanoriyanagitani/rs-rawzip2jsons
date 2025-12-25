#!/bin/bash

izip=./sample.d/input.zip

genzip(){
	echo creating the sample input zip...
	mkdir -p ./sample.d

	jq -c -n '{"helo":"wrld0"}' > ./sample.d/hw1.json
	jq -c -n '{"helo":"wrld1"}' > ./sample.d/hw2.json

	find \
		./sample.d \
		-type f \
		-name '*.json' |
		sort |
		zip \
			-0 \
			-@ \
			-T \
			-v \
			-o \
			./sample.d/input.zip

	echo
}

test -f "${izip}" || genzip

echo listing the entries of the zip...
unzip -lv "${izip}"
echo

echo showing the jsons of the zip...
unzip -p "${izip}" | jq -c
echo

echo showing the jsons processed by the wasm module...
cat "${izip}" |
	wazero \
		run \
		-timeout 10s \
		./target/wasm32-wasip1/release-wasi/rawzip2jsons.wasm \
		-- \
		--max-zip-bytes 1048576 |
	jq -c
