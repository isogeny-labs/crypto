#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
ulimit -v 2097152
ulimit -c 0
timeout --signal=TERM --kill-after=5s 115s python3 runner.py
