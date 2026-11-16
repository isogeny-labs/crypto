#!/usr/bin/env bash
set -euo pipefail
cd /var/tmp/pencil-solve4-stage-20261103-v1
exec timeout --signal=TERM --kill-after=3s 120s python3 runner.py "$1"
