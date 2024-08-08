#!/bin/bash

rustup target add x86_64-apple-darwin

cargo build --target x86_64-apple-darwin

BINNAME="target/$1/release/peers_updater"
chmod og+x "$BINNAME"
zip -9 -j "target/$1/release/$1.zip" "$BINNAME"
