#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
ulimit -v 8388608
ulimit -s 10240
timeout --signal=TERM --kill-after=5s 90s python3 runner.py "$1"
