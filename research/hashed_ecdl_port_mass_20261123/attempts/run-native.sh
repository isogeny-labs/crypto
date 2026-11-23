#!/usr/bin/env bash
set -euo pipefail
R=/tmp/ecdlp-port-mass-20261123
export PORT_MASS_ROWS="$R/native-run/rows.jsonl"
test ! -e "$PORT_MASS_ROWS"
"$R/qualified/library-tests" --list > "$R/native-run/test-list.txt"
timeout --signal=TERM --kill-after=5 120 "$R/qualified/library-tests" --test-threads=1 --nocapture
