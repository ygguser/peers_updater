#!/bin/bash

git clone https://github.com/tpoechtrager/osxcross
cd osxcross
wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz
mv MacOSX10.10.sdk.tar.xz tarballs/
UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh

#MACOS_TARGET="x86_64-apple-darwin"
MACOS_TARGET="$1"

echo "Building target for platform ${MACOS_TARGET}"
echo

# Add osxcross toolchain to path
export PATH="$(pwd)/osxcross/target/bin:$PATH"

echo "pwd: $(pwd)"
echo "added path: $(pwd)/osxcross/target/bin"

# Make libz-sys (git2-rs -> libgit2-sys -> libz-sys) build as a statically linked lib
# This prevents the host zlib from being linked
export LIBZ_SYS_STATIC=1

# Use Clang for C/C++ builds
export CC=o64-clang
export CXX=o64-clang++

DIR=$(git rev-parse --show-toplevel)
cd "$DIR"

echo "curr dir: $(pwd)"

#cross +nightly build --release --target="$1"
cargo build --release --target "${MACOS_TARGET}"

BINNAME="target/$1/release/peers_updater"
chmod og+x "$BINNAME"
upx --ultra-brute "$BINNAME"

zip -9 -j "target/$1/release/$1.zip" "$BINNAME"
