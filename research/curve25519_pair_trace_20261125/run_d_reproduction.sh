#!/usr/bin/env bash
set -euo pipefail

study_dir=$(cd "$(dirname "$0")" && pwd)
new_bin=${PAIR_TRACE_BIN:-/tmp/curve25519-projective-verify/target/release/examples/curve25519_pair_trace}
old_bin=${QUOTIENT_LAYOUT_BIN:-/tmp/curve25519-projective-verify/target/release/examples/curve25519_quotient_layout}
new_sha=4f0d9d328e74c2d166ac30e12206dd6d3a8c58d372c36b8d227c154efbcc916a
old_sha=5972800a79a50b27d725d961cbc60ecc512a9c3c1923a73c2b38b5aa9c3da9d4

test "$(sha256sum "$new_bin" | cut -d' ' -f1)" = "$new_sha"
test "$(sha256sum "$old_bin" | cut -d' ' -f1)" = "$old_sha"

for implementation in new old; do
  mkdir -p "$study_dir/preflight/d-reproduction/$implementation"
  if test "$implementation" = new; then
    bin=$new_bin
  else
    bin=$old_bin
  fi
  for kind in random planted; do
    if test "$kind" = random; then count=4; else count=4; fi
    for ((index=0; index<count; index++)); do
      out="$study_dir/preflight/d-reproduction/$implementation/${kind}-d7-i${index}-D.json"
      tmp="${out}.partial"
      RAYON_NUM_THREADS=1 "$bin" cell --arm D --d 7 --kind "$kind" \
        --index "$index" --cap 120 --dmax 24 --seed 2026112503 >"$tmp"
      mv "$tmp" "$out"
    done
  done
done
