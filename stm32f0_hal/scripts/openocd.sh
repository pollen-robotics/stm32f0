#!/usr/bin/env bash

openocd \
    -f interface/stlink-v2.cfg \
    -f board/stm32f0discovery.cfg
