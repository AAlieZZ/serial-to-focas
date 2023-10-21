#!/bin/bash
if ! cargo --version
then curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && cargo install cross --git https://github.com/cross-rs/cross
fi
rm -rf Focas Focas.zip
cargo clean
~/.cargo/bin/cross build --target i686-unknown-linux-gnu --release
mkdir Focas
mv target/i686-unknown-linux-gnu/release/serial-to-focas Focas
cp -r lib Focas
cp -r src Focas
cp LICENSE Focas/src
cp run.sh Focas
cp temp.txt Focas
cp Cargo.toml Focas
cp build.rs Focas
cp build.sh Focas
7z a Focas.zip Focas
