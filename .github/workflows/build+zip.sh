#!/bin/bash

DIR=$(git rev-parse --show-toplevel)
cd "$DIR"

cross +nightly build --release --target="$1"

if [[ "$1" == *"windows"* ]]; then
    BINNAME="target/$1/release/peers_updater.exe"
    zip -9 -j "target/$1/release/no_upx_$1.zip" "$BINNAME"
else
    BINNAME="target/$1/release/peers_updater"
    chmod og+x "$BINNAME"
fi

upx --ultra-brute "$BINNAME"
zip -9 -j "target/$1/release/$1.zip" "$BINNAME"
