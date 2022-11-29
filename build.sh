#!/bin/bash

# Download the cross-compilation toolchain
CROSS_DIR=/usr/local/lib/gcc_cross/gcc-arm-10.3-2021.07-aarch64-arm-none-linux-gnueabihf

PKG_CONFIG_SYSROOT_DIR=$CROSS_DIR
PATH=$PATH:$CROSS_DIR/bin
cargo build --target=armv7-unknown-linux-gnueabihf

