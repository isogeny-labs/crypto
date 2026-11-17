#!/usr/bin/env bash
# PROTOCOL.md "Measures": the band (exact sums) of every session, the paired
# comparisons of every named pair, saved under sessions/<S>/comparisons/, then
# analysis/summary.json (the native examples/ic_planner_exhaustive_summary.rs,
# which reads only these outputs and the null JSONs). Re-running it reproduces
# every derived file.
set -euo pipefail
cd "$(dirname "$0")/.."
E=../../target/release/ecbench
W=0,0.01,0.03,0.1,0.3,1
R="R-p16 R-p18 R-p36 R-p37"; B="B-hb19 B-hb23 B-hb25 B-hb27"; K="K-k47 K-k57 K-k41 K-k53 K-k59 K-k61"
for s in $R $B $K; do
  $E band --dir sessions/$s --weights $W --json analysis/band-$s.json > analysis/band-$s.txt
done
cmp() { # session a b weight...
  local s=$1 a=$2 b=$3; shift 3
  for w in "$@"; do
    if [ "$w" = 0 ]; then $E compare --dir sessions/$s --a $a --b $b --save > /dev/null
    else $E compare --dir sessions/$s --a $a --b $b --weight $w --save > /dev/null; fi
  done
}
for s in $R; do
  for b in res2 res4 res8 res16 res32 m141; do cmp $s plan $b 0 0.1 0.3 1; done
  for b in plan res2 res4; do cmp $s rho $b 0 1; done
done
for s in $B; do
  for b in dm1 dp1; do cmp $s plan $b 0 0.1 0.3 1; done
  cmp $s rho plan 0 1
done
for s in $K; do
  cmp $s pipeline pipeline71 0 0.1 0.3 1
  cmp $s pipeline descent 0          # descent has uncounted native work: w = 0 only
  cmp $s rho pipeline 0 1
  cmp $s rho descent 0
done
cargo run --release --example ic_planner_exhaustive_summary
