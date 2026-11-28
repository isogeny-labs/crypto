#!/usr/bin/env bash
# Thin launch of the unchanged frozen native parent runner.
set -euo pipefail
BUNDLE=$1
OUT=$2
test ! -e "$OUT"
mkdir "$OUT"
export EPHEMERAL_OUT="$OUT"
ulimit -v 4194304
bash "$BUNDLE/run.sh"
