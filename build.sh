#!/bin/bash
rm -rf Focas Focas.zip
cargo clean
cargo build --target i686-unknown-linux-gnu --release
mkdir Focas
mv target/i686-unknown-linux-gnu/release/serial-to-focas Focas
cp -r lib Focas
cp run.sh Focas
cp temp.txt Focas
7z a Focas.zip Focas
