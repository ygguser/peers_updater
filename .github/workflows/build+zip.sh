#!/bin/bash

DIR=$(git rev-parse --show-toplevel)
cd "$DIR"

#temporary (cross bug)
touch ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/Cargo.lock
touch ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/Cargo.lock

cross +nightly build --locked --release --target="$1"

if [[ "$1" == *"windows"* ]]; then
    BINNAME="target/$1/release/peers_updater.exe"
else
    BINNAME="target/$1/release/peers_updater"
    chmod og+x "$BINNAME"
    upx --ultra-brute "$BINNAME"
fi

zip -9 -j "target/$1/release/$1.zip" "$BINNAME"
