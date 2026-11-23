#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
ulimit -v 8388608
ulimit -s 16384
timeout --signal=TERM --kill-after=5s 295s python3 replay.py
