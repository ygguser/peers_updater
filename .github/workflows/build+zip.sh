#!/bin/bash

DIR=$(git rev-parse --show-toplevel)
cd "$DIR"

cross +nightly build --locked --release --target="$1"

if [[ "$1" == *"windows"* ]]; then
    BINNAME="target/$1/release/peers_updater.exe"
else
    BINNAME="target/$1/release/peers_updater"
    chmod og+x "$BINNAME"
    upx --ultra-brute "$BINNAME"
fi

zip -9 -j "target/$1/release/$1.zip" "$BINNAME"
