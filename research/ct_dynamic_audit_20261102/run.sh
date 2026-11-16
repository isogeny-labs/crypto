#!/usr/bin/env bash
# Thin orchestration for the frozen protocol (PROTOCOL.md §5): one Memcheck and one Callgrind process per target,
# then the native analyzer. Usage: run.sh <path to ct_dynamic_audit binary> <raw output dir> <analysis output dir>
set -euo pipefail
BIN=$1 RAW=$2 OUT=$3
mkdir -p "$RAW" "$OUT"
{
  echo "binary_sha256 $(sha256sum "$BIN" | cut -d' ' -f1)"
  echo "valgrind $(valgrind --version)"
  echo "rustc $(rustc -V)"
  echo "commit $(git -C "$(dirname "$0")" rev-parse HEAD)"
  echo "host $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2-) | $(nproc) vCPU | $(uname -r)"
} > "$RAW/ENV.txt"
for t in $("$BIN" list | cut -f1); do
  valgrind --tool=memcheck --xml=yes --xml-file="$RAW/memcheck-$t.xml" --leak-check=no --track-origins=no \
    --num-callers=40 --error-limit=no "$BIN" taint "$t" 2>>"$RAW/stderr.log"
  valgrind --tool=callgrind --collect-atstart=no --callgrind-out-file="$RAW/callgrind.$t.%p" \
    "$BIN" ir "$t" 2>>"$RAW/stderr.log"
done
"$BIN" analyze "$RAW" "$OUT"
