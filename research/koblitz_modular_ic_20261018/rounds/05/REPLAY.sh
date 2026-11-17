#!/usr/bin/env bash
set -euo pipefail
round=$(cd "$(dirname "$0")" && pwd)
study=$(cd "$round/../.." && pwd)
expected=${1:-"$round/runs"}
replay=${2:-$(mktemp -d)}
mkdir -p "$replay"
cp "$expected/m5-interior-function.json" "$expected/gl2b-integral-polarizations.json" "$expected/q2-natural-quotient.json" "$replay/"
cp "$expected/m1-modular-base.json" "$expected/m4-direct-modular-relations.json" "$replay/"
cargo build --release --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --round05 "$replay"
"$runner" --round05-extend "$replay"
"$runner" --round05-lean "$replay"
"$runner" --round05-poles "$replay"
"$runner" --verify-round05 "$replay"
for name in s1-reverse-gram m6-interior-factors q3-equivalent-quotients m6b-closed-zero-orbits q3b-first-graph-check m7-poles-and-symbolic-row; do
  cmp "$expected/$name.json" "$replay/$name.json"
done
cmp "$expected/KoblitzRound05Quotients.lean" "$replay/KoblitzRound05Quotients.lean"
cmp "$study/lean/KoblitzRound05Quotients.lean" "$replay/KoblitzRound05Quotients.lean"
printf 'PASS: six JSON records and generated Lean replay byte for byte; separate checks pass.\n'
