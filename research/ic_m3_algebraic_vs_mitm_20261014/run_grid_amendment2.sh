#!/bin/sh
# Amendment 2: the m = 4 grid with the narrow engines in a wide-vars build.
# Thin orchestration only. Writes runs/m4-engines (must not exist).
set -eu
cd "$(dirname "$0")/../.."
cargo build --release --features wide-vars --example ic_algebraic_floor
B=target/release/examples/ic_algebraic_floor
S=research/ic_m3_algebraic_vs_mitm_20261014
test ! -e $S/runs/m4-engines
$B run --m 4 --cells 1:11:3,0:13:4,1:17:5,1:19:5,1:23:6,1:29:8,0:31:8 \
  --arms f4-chain,f6-ic,sat \
  --pool 1024 --kyes 6 --kno 6 --budget-secs 120 --mem-gib 4 --stop-after 4 \
  --seed 20261014 --out $S/runs/m4-engines 2> $S/runs-m4-engines.log
