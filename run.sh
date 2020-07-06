#!/bin/bash

function install_bootimage() {
    cargo install bootimage
}

# xbuild cross-compiles into new target
function install_xbuild () {
    cargo install cargo-xbuild
}

rustup override add nightly
install_bootimage
install_xbuild
cargo xrun


