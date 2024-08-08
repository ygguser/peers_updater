#!/bin/bash

rustup target add x86_64-apple-darwin

cargo build --release --target x86_64-apple-darwin

BINNAME="target/release/peers_updater"
ls target
ls target/release
ls release
chmod og+x "$BINNAME"
zip -9 -j "target/release/x86_64-apple-darwin.zip" "$BINNAME"
