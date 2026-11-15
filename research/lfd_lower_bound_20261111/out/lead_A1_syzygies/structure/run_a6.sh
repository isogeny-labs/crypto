#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
for c in "15 1 8" "15 6 8" "17 1 8" "17 8 8" "19 4 6"; do set -- $c; python3 a6scan.py $1 $2 $3 >> a6scan.jsonl 2>> a6scan.log; done
