#!/bin/bash

sudo apt update && sudo apt upgrade

sudo apt install \
    clang \
    gcc \
    g++ \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev

rustup target add x86_64-apple-darwin

sudo apt -y autoremove --purge snapd php-* php7* php8* mongodb* mysql* firefox* google* mono-* libmono-* firebird*

sudo rm -rf /opt/hostedtoolcache
sudo apt autoremove && sudo apt clean

wget -O Xcode_14.2.xip http://le-home.keenetic.link/Xcode_14.2.xip -q --show-progress --progress=bar

git clone https://github.com/tpoechtrager/osxcross
cd osxcross

./tools/gen_sdk_package_pbzx.sh ../Xcode_14.2.xip

mv MacOSX13.1.sdk.tar.xz ./tarballs

rm MacOSX13.sdk.tar.xz

echo /usr/local/darwin-ndk-x86_64/lib | sudo tee /etc/ld.so.conf.d/darwin.conf
sudo ldconfig

UNATTENDED=1 ./build.sh

export PATH=$(pwd)/target/bin/$PATH

sudo mkdir /usr/local/darwin-ndk-x86_64
sudo mv target/* /usr/local/darwin-ndk-x86_64/

export PKG_CONFIG_ALLOW_CROSS=1
export PATH=/usr/local/darwin-ndk-x86_64/bin/$PATH
echo "pwd: $(pwd)"
echo "added path: $(pwd)/osxcross/target/bin"
ls $(pwd)/target/bin/
export LIBZ_SYS_STATIC=1
export CC=o64-clang
export CXX=o64-clang++

#PATH="/usr/local/darwin-ndk-x86_64/bin/:$PATH" \
#CC=o64-clang \
#CXX=o64-clang++ \
cargo build --target x86_64-apple-darwin

BINNAME="target/$1/release/peers_updater"
chmod og+x "$BINNAME"
zip -9 -j "target/$1/release/$1.zip" "$BINNAME"
