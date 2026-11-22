#!/bin/bash
# Registered ladder for round 2. Run from /home/user/crypto after building
#   CARGO_TARGET_DIR=/home/user/crypto/target cargo build --release --bin extfield_jac_decomp -j2
# Two worker processes at a time (shared 4-CPU machine). Raw JSON per run in results/.
set -u
export GIT_HEAD=$(git log -1 --format=%H)
BIN=/home/user/crypto/target/release/extfield_jac_decomp
OUT=/home/user/crypto/research/extfield-jacobian-decomp_20261121/results
mkdir -p "$OUT"
export LP_TIME_CAP_S=${LP_TIME_CAP_S:-1200}
job() { # q seed tag
  [ -s "$OUT/lp_q$1_s$2_t$3.json" ] || $BIN lp $1 $2 $3 > "$OUT/lp_q$1_s$2_t$3.json"
}
rho() { [ -s "$OUT/rho_q$1_s$2.json" ] || $BIN run $1 $2 rho 0 > "$OUT/rho_q$1_s$2.json"; }
for q in 131 251 509 1021 2039; do
  for s in 1 2 3; do rho $q $s; done
done
for q in 131 251 509 1021 2039; do
  for s in 1 2 3; do
    job $q $s 0 &
    # at most two concurrent workers
    while [ "$(jobs -r | wc -l)" -ge 2 ]; do sleep 1; done
  done
done
wait
for q in 131 251 509; do job $q 1 1 & while [ "$(jobs -r | wc -l)" -ge 2 ]; do sleep 1; done; done
wait
