#!/bin/bash

apt install \
    clang \
    gcc \
    g++ \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev

rustup target add x86_64-apple-darwin

git clone https://github.com/tpoechtrager/osxcross
cd osxcross

./tools/gen_sdk_package_pbzx.sh /home/ttys3/Downloads/Compressed/Xcode_14.2.xip

mv MacOSX13.1.sdk.tar.xz ./tarballs

rm MacOSX13.sdk.tar.xz

echo /usr/local/darwin-ndk-x86_64/lib | sudo tee /etc/ld.so.conf.d/darwin.conf
sudo ldconfig

UNATTENDED=1 ./build.sh

mkdir /usr/local/darwin-ndk-x86_64
mv target/* /usr/local/darwin-ndk-x86_64/

PATH="/usr/local/darwin-ndk-x86_64/bin/:$PATH" \
CC=o64-clang \
CXX=o64-clang++ \
cargo build --target x86_64-apple-darwin

BINNAME="target/$1/release/peers_updater"
chmod og+x "$BINNAME"
zip -9 -j "target/$1/release/$1.zip" "$BINNAME"
