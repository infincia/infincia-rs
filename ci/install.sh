#!/usr/bin/env bash

# `install` phase: install stuff needed for the `script` phase

set -ex

export PATH=${HOME}/.cargo/bin:${PATH}

. $(dirname $0)/utils.sh

install_rustup() {
    echo "Using Rust $(<rust-toolchain)"
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=$(<rust-toolchain)
    rustc -V
    cargo -V
}

main() {
    install_rustup
}

main
