#!/bin/bash

git clone https://github.com/tpoechtrager/osxcross
cd osxcross

#https://github.com/phracker/MacOSX-SDKs/releases #versions of MacOSX sdk
wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz
#wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.11.sdk.tar.xz
#wget -nc https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX10.10.sdk.tar.xz
#mv MacOSX10.11.sdk.tar.xz tarballs/
mv MacOSX10.10.sdk.tar.xz tarballs/

#curl -sSL https://cmake.org/files/v3.14/cmake-3.14.5-Linux-x86_64.tar.gz | sudo tar -xzC /opt
#export PATH=/opt/cmake-3.14.5-Linux-x86_64/bin:$PATH

#sed -i -e 's|-march=native||g' build_clang.sh wrapper/build.sh

UNATTENDED=yes OSX_VERSION_MIN=10.7 sudo ./build.sh

sudo mkdir -p /usr/local/osx-ndk-x86
sudo mv target/* /usr/local/osx-ndk-x86

cd ..

#MACOS_TARGET="x86_64-apple-darwin"
MACOS_TARGET="$1"

echo "Building target for platform ${MACOS_TARGET}"
echo

# Add osxcross toolchain to path
#export PATH="$(pwd)/osxcross/target/bin:$PATH"
export PATH=/usr/local/osx-ndk-x86/bin:$PATH
export PKG_CONFIG_ALLOW_CROSS=1

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

#if [[ "$1" == *"x86_64h"* ]]; then
#    rustup +nightly component add rust-src
#    #export RUSTFLAGS="-Zbuild-std,panic_abort"
#    #export CARGO_TARGET_X86_64H_APPLE_DARWIN_LINKER=x86_64-apple-darwin14-clang
#    RUSTFLAGS="-Zbuild-std,panic_abort" cargo +nightly build --config "target.x86_64h-apple-darwin.ar = 'x86_64-apple-darwin14-ar'" --config "target.x86_64h-apple-darwin.linker = 'x86_64-apple-darwin14-clang'" -Z build-std,panic_abort -Z build-std-features=panic_immediate_abort -Z build-std=core --target x86_64h-apple-darwin
#fi

echo /usr/local/darwin-ndk-x86_64/lib | sudo tee /etc/ld.so.conf.d/darwin.conf
sudo ldconfig

#cargo build --config "target.${MACOS_TARGET}.ar = 'x86_64-apple-darwin14-ar'" --config "target.${MACOS_TARGET}.linker = 'x86_64-apple-darwin14-clang'" --config "profile.release.strip = false" --release --target "${MACOS_TARGET}"
cargo build --config "target.${MACOS_TARGET}.linker = 'x86_64-apple-darwin14-clang'" --config "profile.release.strip = false" --release --target "${MACOS_TARGET}"
#cargo build --config "target.${MACOS_TARGET}.ar = 'x86_64-apple-darwin15-ar'" --config "target.${MACOS_TARGET}.linker = 'x86_64-apple-darwin15-gcc'" --config "profile.release.strip = false" --release --target "${MACOS_TARGET}"

BINNAME="target/$1/release/peers_updater"
chmod og+x "$BINNAME"
#upx --ultra-brute "$BINNAME"

zip -9 -j "target/$1/release/$1.zip" "$BINNAME"
