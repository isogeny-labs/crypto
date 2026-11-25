#!/bin/bash
set -euo pipefail
test "$#" -eq 1
origin=$(cd "$(dirname "$0")" && pwd)
mkdir "$1"
mkdir "$1/relations"
cp "$origin/fixtures/actual.json" "$1/relations/result.json"
cp -r "$origin/fixtures/construction" "$1/relations/input"
printf '%s\n' 'baseline fixture only; no mathematical replay' > "$1/BASELINE_STUB.txt"
