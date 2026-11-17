#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
ulimit -v 2097152
timeout --signal=TERM --kill-after=5s 115s python3 run_tests.py
