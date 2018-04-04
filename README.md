# Rust API for the STM32F0 micro controller

[![Build Status](https://travis-ci.org/pollen-robotics/stm32f0.svg?branch=master)](https://travis-ci.org/pollen-robotics/stm32f0)

The API is composed of two main crates:

* the stm32f0x2 is the raw API as generated via [svd2rust](https://github.com/japaric/svd2rust)
* the stm32f0_hal is a higher level API with GPIO, LED, UART, etc support

## Installation

First you need to setup your environment to cross compile it for armv6. You can find good instructions for Win/Mac/Linux [here](https://japaric.github.io/discovery/03-setup/README.html).

Then you can build both workspaces directly from the root using ```xargo build --target thumbv6m-none-eabi```.

## Usage

You can find examples in the [stm32f0_hal/examples](./stm32f0_hal/examples) folder. They can be built directly.

For instance, to build the *blinky* example: ```xargo build --target thumbv6m-none-eabi --example blinky```.

You can generate the doc via ```xargo doc``` for a complete API.

## Troubleshoot

### Xargo version

If you encounter an error such as: ```error: no matching version = 0.0.0 found for package compiler_builtins (required by sysroot)```

Make sure you downgrade your xargo version to 0.3.8: ```cargo install xargo --vers 0.3.8 -f```
