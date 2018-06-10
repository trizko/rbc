#!/bin/bash
set -e

./target/debug/coinr | nc localhost 18444 | hexdump -C
