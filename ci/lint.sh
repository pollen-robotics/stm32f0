#!/bin/bash

if [ $TARGET = thumbv6m-none-eabi ]; then
  pushd stm32f0_hal
    cargo clean
    xargo build --features "clippy" --target $TARGET
    cargo fmt --all -- --write-mode=diff
  popd
fi
