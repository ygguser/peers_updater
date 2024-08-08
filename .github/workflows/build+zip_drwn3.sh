#!/bin/bash

rustup target add x86_64-apple-darwin

cargo build --release --target x86_64-apple-darwin

BINNAME="build/release/peers_updater"
chmod og+x "$BINNAME"
zip -9 -j "build/release/x86_64-apple-darwin.zip" "$BINNAME"
