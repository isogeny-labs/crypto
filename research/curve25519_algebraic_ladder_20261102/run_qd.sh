#!/usr/bin/env bash
# Q-D grid (PROTOCOL.md Amendment 2): thin orchestration over the native harness.
set -u
cd "$(dirname "$0")/../.."
out=research/curve25519_algebraic_ladder_20261102/analysis/f4cost.jsonl
for t in 3 4 5 6 7 8 9; do
  for g in 0 1 2 3; do
    [ "$g" -lt "$t" ] || continue
    ./target/release/curve25519_family f4cost 20 bits "$t" "$g" 200 $((2026110800 + t)) 600 >> "$out"
  done
  ./target/release/curve25519_family f4cost 20 isogeny "$t" 0 200 $((2026110800 + t)) 600 >> "$out"
done
echo DONE >> "$out.status"
