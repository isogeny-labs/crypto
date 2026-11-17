#!/usr/bin/env bash
# usage: replay.sh <dir> [xor]
cd "$1"; shift
if [ -n "${1:-}" ]; then export L2SEED_XOR="$1"; fi
./run.sh quick > replay_stdout.txt 2>&1
