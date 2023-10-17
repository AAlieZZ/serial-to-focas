#!/bin/bash
rm -rf Focas Focas.zip
cargo clean
cargo build --target i686-unknown-linux-gnu --release
mkdir Focas
mv target/i686-unknown-linux-gnu/release/serial-to-focas Focas
cp -r lib Focas
cp -r src Focas
cp LICENSE Focas/src
cp run.sh Focas
cp temp.txt Focas
cp Cargo.toml Focas
cp build.rs Focas
7z a Focas.zip Focas
