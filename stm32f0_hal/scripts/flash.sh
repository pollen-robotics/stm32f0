#!/usr/bin/env bash

EXAMPLE=$1

cargo build --example $EXAMPLE &&

arm-none-eabi-gdb ../target/thumbv6m-none-eabi/debug/examples/$EXAMPLE \
    -ex "target remote :3333" \
    -ex "load" \
    -ex "continue"
