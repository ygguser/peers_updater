#!/bin/bash

rustup target add x86_64-apple-darwin

cargo build --relese --target x86_64-apple-darwin

BINNAME="target/release/peers_updater"
chmod og+x "$BINNAME"
zip -9 -j "target/release/x86_64-apple-darwin.zip" "$BINNAME"
