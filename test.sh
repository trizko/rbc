#!/bin/bash
set -e

./target/debug/rbc | nc localhost 18444 | hexdump -C
