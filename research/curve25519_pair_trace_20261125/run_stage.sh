#!/usr/bin/env bash
set -euo pipefail

if test "$#" -ne 1; then
  echo "usage: $0 7|9|19" >&2
  exit 2
fi
d=$1
case "$d" in
  7) random_count=4; planted_count=4; cap=120 ;;
  9) random_count=4; planted_count=4; cap=180 ;;
  19) random_count=8; planted_count=4; cap=300 ;;
  *) echo "stage must be 7, 9, or 19" >&2; exit 2 ;;
esac

study_dir=$(cd "$(dirname "$0")" && pwd)
bin=${PAIR_TRACE_BIN:-/tmp/curve25519-projective-verify/target/release/examples/curve25519_pair_trace}
expected_sha=4f0d9d328e74c2d166ac30e12206dd6d3a8c58d372c36b8d227c154efbcc916a
test "$(sha256sum "$bin" | cut -d' ' -f1)" = "$expected_sha"

out_dir="$study_dir/raw/d$d"
mkdir -p "$out_dir"
for kind in random planted; do
  if test "$kind" = random; then count=$random_count; else count=$planted_count; fi
  for ((index=0; index<count; index++)); do
    for arm in D R P; do
      out="$out_dir/${kind}-d${d}-i${index}-${arm}.json"
      tmp="${out}.partial"
      RAYON_NUM_THREADS=1 "$bin" cell --arm "$arm" --d "$d" --kind "$kind" \
        --index "$index" --cap "$cap" --dmax 24 --seed 2026112503 >"$tmp"
      mv "$tmp" "$out"
    done
  done
done
