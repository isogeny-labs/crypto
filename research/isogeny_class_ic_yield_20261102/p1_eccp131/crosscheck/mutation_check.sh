#!/usr/bin/env bash
# Positive control: the cross-check must flag a deliberately broken engine.  Copies rust/ and crosscheck/ to a scratch
# directory, applies one mutation to the copy of engine.rs, builds and runs on one prime; expects failures > 0.
# Usage: mutation_check.sh SCRATCH TOWER.json P     env: CARGO_TARGET_DIR (reuse the one that holds the crypto build)
set -euo pipefail
SC=${1:?scratch}; TW=${2:?tower}; P=${3:?prime}; HERE=$(cd "$(dirname "$0")" && pwd)
for m in M1 M2; do
  rm -rf "$SC/$m"; mkdir -p "$SC/$m"; cp -r "$HERE/../rust" "$HERE" "$SC/$m/"; rm -rf "$SC/$m/rust/target" "$SC/$m/crosscheck/target" "$SC/$m/crosscheck/results"
  sed -i "s#path = \"../../../..\"#path = \"$(cd "$HERE/../../../.." && pwd)\"#" "$SC/$m/crosscheck/Cargo.toml"
done
# M1: wrong sign in the pivot application for columns with id % 7 == 3
sed -i 's|\*a = a.sub(fct.mul(\*v));|*a = if *id % 7 == 3 { a.add(fct.mul(*v)) } else { a.sub(fct.mul(*v)) };|' "$SC/M1/rust/src/engine.rs"
# M2: the staircase ignores divisibility by the last variable
sed -i '0,/for v in 0..n {/s//for v in 0..n - 1 {/' "$SC/M2/rust/src/engine.rs"
for m in M1 M2; do
  grep -q "id % 7\|0..n - 1" "$SC/$m/rust/src/engine.rs" || { echo "mutation $m not applied"; exit 2; }
  (cd "$SC/$m/crosscheck" && touch src/main.rs ../rust/src/*.rs && cargo build --release --offline 2>&1 | tail -1)
  "${CARGO_TARGET_DIR:-$SC/$m/crosscheck/target}/release/p1_crosscheck" --p "$P" --tower "$TW" --configs m2t3,m3t2 --curves 1 --null 1 \
      --planted 1 --abscissa 1 --ladder 1 --dense-every 1 --out "$SC/$m.rows.jsonl" 2>&1 | tail -2 || true
done
