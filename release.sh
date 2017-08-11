#!/usr/bin/env bash

set -e

export PATH=${HOME}/.cargo/bin:${PATH}

if [ -z "${TARGET}" ]; then
    echo "WARNING: no explicit target set, using host target"
    export TARGET=`rustup show | awk 'match($0, /Default host: ([0-9a-zA-Z\_]).+/) { ver = substr($3, RSTART, RLENGTH); print ver;}'`
fi

TOOLCHAIN=$(<rust-toolchain)

echo "Building for ${TOOLCHAIN}"

rustup override set $(<rust-toolchain)

echo "Building for ${TARGET}"


rm -rf dist-${TARGET}
mkdir -p dist-${TARGET}/lib
mkdir -p dist-${TARGET}/include
mkdir -p dist-${TARGET}/bin

yarn install

npm run-script build

RUST_BACKTRACE=1 cargo build --release -p infinciad --target ${TARGET} > /dev/null

cp -a target/${TARGET}/release/infinciad dist-${TARGET}/bin/
