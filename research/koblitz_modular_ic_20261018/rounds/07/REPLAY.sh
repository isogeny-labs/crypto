#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
replay=${2:-$(mktemp -d)}
mkdir -p "$replay/odd-verified"
for name in r3-reverse-fiber-screen r3-reverse-fiber-screen-negative r3b-complete-split-fiber r3d-even-reverse-divisor m4-direct-modular-relations gl2b-integral-polarizations; do
  cp "$expected/$name.json" "$replay/"
done
for i in {00..11}; do cp "$expected/r3b-negative-fiber-neighbor-$i.json" "$replay/"; done
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
for command in norm-seeds forward relations; do "$runner" "--round07-$command" "$replay"; done
"$runner" --verify-round07-relations "$replay"
"$runner" --round07-relations-next "$replay"
"$runner" --verify-round07-next "$replay"
"$runner" --round07-quotients "$replay"
"$runner" --round07-cofactor-equations "$replay"
for command in odd-screen odd-algebras odd-targets odd-divisor; do "$runner" "--round07-$command" "$replay/odd-verified"; done
"$runner" --verify-round07-odd "$replay/odd-verified"
for name in n4-independent-seed-targets n4-even-independent-forward m8-full-map-principal-row m8b-independent-F32-check m9-second-full-map-row m9b-independent-small-field-check m9c-geometric-base-check q5-mixed-translation-atlases q5b-expanded-cofactor-cover; do
  cmp "$expected/$name.json" "$replay/$name.json"
done
for i in {00..11}; do cmp "$expected/n4-negative-neighbor-$i.json" "$replay/n4-negative-neighbor-$i.json"; done
for name in o4-seed-screen o4-complete-seed o4-independent-seed-targets o4-odd-reverse-divisor o4b-odd-coordinate-check; do
  cmp "$expected/odd-verified/$name.json" "$replay/odd-verified/$name.json"
done
for i in {00..11}; do
  cmp "$expected/odd-verified/o4-fiber-neighbor-$i.json" "$replay/odd-verified/o4-fiber-neighbor-$i.json"
  cmp "$expected/odd-verified/o4-target-neighbor-$i.json" "$replay/odd-verified/o4-target-neighbor-$i.json"
done
cmp "$expected/q5b-expanded-cofactor-equations.txt" "$replay/q5b-expanded-cofactor-equations.txt"
cmp "$expected/KoblitzRound07Cofactor.lean" "$replay/KoblitzRound07Cofactor.lean"
cmp "$study/lean/KoblitzRound07Cofactor.lean" "$replay/KoblitzRound07Cofactor.lean"
printf 'PASS:50 new JSON records, expanded cover equations and generated Lean replay byte for byte; separate checks pass.\n'
