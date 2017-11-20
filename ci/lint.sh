#!/bin/bash

if [ $TARGET = thumbv6m-none-eabi ]; then
  pushd stm32f0_hal
    cargo clean
    cargo fmt -- --write-mode=diff
  popd
fi
