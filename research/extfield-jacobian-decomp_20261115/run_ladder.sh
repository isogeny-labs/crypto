#!/bin/bash
# Reproduces the registered ladder. Run from /home/user/crypto.
set -u
export GIT_HEAD=$(git log -1 --format=%H)
BIN=/home/user/crypto/target/release/extfield_jac_decomp
OUT=/home/user/crypto/research/extfield-jacobian-decomp_20261115/results
mkdir -p "$OUT"
for t in "67 300" "131 100"; do set -- $t; $BIN oracle $1 1 $2 > "$OUT/oracle_q$1.json"; done
for q in 67 131 251 509 1021 2039; do
  for s in 1 2 3; do
    $BIN run $q $s both 0 > "$OUT/run_q${q}_s${s}_t0.json"
  done
done
for q in 131 251; do
  $BIN run $q 1 both 1 > "$OUT/run_q${q}_s1_t1.json"
done
# Supplemental (not registered): larger oracle-agreement samples, new curve seed 2.
$BIN oracle 67 2 3000 > "$OUT/oracle_supp_q67.json"
$BIN oracle 131 2 1000 > "$OUT/oracle_supp_q131.json"
