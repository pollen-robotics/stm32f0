#!/bin/bash

if [ $TARGET = x86_64-unknown-linux-gnu ]; then
  which xargo || (cd .. && cargo install xargo -f --vers 0.3.8)
fi
