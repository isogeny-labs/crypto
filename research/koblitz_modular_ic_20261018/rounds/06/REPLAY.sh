#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
replay=${2:-$(mktemp -d)}
mkdir -p "$replay"
for name in q3-equivalent-quotients q3b-first-graph-check q2-natural-quotient gl2b-integral-polarizations s1-reverse-gram m4-direct-modular-relations; do
  cp "$expected/$name.json" "$replay/"
done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in images check-images seven-fibers quotients lean reverse-screen reverse-screen-negative split-fiber reverse-algebras-negative reverse-divisor; do
  "$runner" "--round06-$command" "$replay"
done
"$runner" --verify-round06 "$replay"
for name in i1-degree210-images i1b-image-checks i1c-seven-fiber-cancellation i2-zero-column-matrix-check q4-split-quotient-equations q4b-independent-checks r3-reverse-fiber-screen r3-reverse-fiber-screen-negative r3b-complete-split-fiber r3c-complete-pullback-fiber-negative r3d-even-reverse-divisor r3e-coordinate-checks; do
  cmp "$expected/$name.json" "$replay/$name.json"
done
for i in {00..11}; do
  cmp "$expected/r3b-negative-fiber-neighbor-$i.json" "$replay/r3b-negative-fiber-neighbor-$i.json"
done
cmp "$expected/KoblitzRound06Matrices.lean" "$replay/KoblitzRound06Matrices.lean"
cmp "$study/lean/KoblitzRound06Matrices.lean" "$replay/KoblitzRound06Matrices.lean"
printf 'PASS: 24 JSON records and generated Lean source replay byte for byte; separate checks pass.\n'
