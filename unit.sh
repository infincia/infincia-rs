#!/usr/bin/env bash

set -e

export PATH=${HOME}/.cargo/bin:${PATH}

if [ -z "${TARGET}" ]; then
    export TARGET=`rustup show | awk 'match($0, /Default host: ([0-9a-zA-Z\_]).+/) { ver = substr($3, RSTART, RLENGTH); print ver;}'`
fi

rustup override set $(<rust-toolchain)


echo "Testing for $TARGET"


RUST_BACKTRACE=1 cargo test --release -p libinfincia --target $TARGET
