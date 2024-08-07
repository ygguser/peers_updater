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

#DEBIAN_FRONTEND='noninteractive' sudo apt-get -y -o Dpkg::Options::='--force-confdef' -o Dpkg::Options::='--force-confold' upgrade && sudo apt-get -y install zip upx musl-tools clang gcc g++ zlib1g-dev libmpc-dev libmpfr-dev libc++-dev libgmp-dev

rustup target add x86_64-apple-darwin

#curl -sSL https://cmake.org/files/v3.14/cmake-3.14.5-Linux-x86_64.tar.gz | sudo tar -xzC /opt
#export PATH=/opt/cmake-3.14.5-Linux-x86_64/bin:$PATH

#sed -i -e 's|-march=native||g' build_clang.sh wrapper/build.sh

#wget -O Xcode_14.2.xip https://drive.google.com/uc?export=download&id=1NWh_pv9M0Ey336ODU1JGkSuKxz2DWBxK
#curl --cookie-jar cookie_file 'https://download.developer.apple.com/Developer_Tools/Xcode_14.2/Xcode_14.2.xip'
#curl --cookie cookie_file --remote-name 'https://download.developer.apple.com/Developer_Tools/Xcode_14.2/Xcode_14.2.xip'

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
