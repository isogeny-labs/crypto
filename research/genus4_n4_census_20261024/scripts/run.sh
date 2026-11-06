#!/bin/sh
# usage: run.sh <item> <PART> <args...>   (run from the repository root)
# Builds the tool, creates results/<item>-run-<UTC>/, writes env.txt, runs.
set -eu
ROOT=$(pwd)
STUDY=research/genus4_n4_census_20261024
ITEM=$1; shift
UTC=$(date -u +%Y%m%dT%H%MZ)
OUT=$STUDY/results/$ITEM-run-$UTC
mkdir -p "$OUT"
BIN=${G4_BIN:-/tmp/claude-0/g4/g4}
rustc --edition 2021 -O examples/genus4_n4_census.rs -o "$BIN"
{
  echo "run: $ITEM $UTC"
  echo "git HEAD: $(git rev-parse HEAD)"
  echo "tool: examples/genus4_n4_census.rs sha256 $(sha256sum examples/genus4_n4_census.rs | cut -d' ' -f1)"
  echo "protocol: $STUDY/PROTOCOL.md sha256 $(sha256sum $STUDY/PROTOCOL.md | cut -d' ' -f1)"
  echo "command: $BIN $OUT $*"
  echo "rustc: $(rustc --version)"
  echo "host: $(uname -sr) $(uname -m), $(nproc) cores, $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | sed 's/^ //')"
  echo "threads: 1"
} > "$OUT/env.txt"
START=$(date +%s)
"$BIN" "$OUT" "$@" 2> "$OUT/tool_log.txt" || echo "NONZERO EXIT" >> "$OUT/env.txt"
echo "wall seconds: $(( $(date +%s) - START ))" >> "$OUT/env.txt"
echo "$OUT"
