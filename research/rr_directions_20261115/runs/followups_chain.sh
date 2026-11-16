#!/bin/bash
# F1 -> F3 -> F5 (PREREG_FOLLOWUPS.md), sequential, default F4 cap, 600 s budget, seed 20261115.
P=/home/user/crypto/target/release/examples/rr_quadratic_panel
export RAYON_NUM_THREADS=1
set -x
$P --out runs/f1_ladder_20261115 --m 13 --ell 3,4,5,6 --arms norm,rr4,s4 --solvers sat,f4 --planted 4 --unplanted 4 --budget-s 600 --seed 20261115
$P --out runs/f1_ladder_20261115 --m 17 --ell 4,5,6,7 --arms norm,rr4,s4 --solvers sat,f4 --planted 4 --unplanted 4 --budget-s 600 --seed 20261115
for m in 13 15 17; do l=$((m/4)); $P --out runs/f3_norm4_20261115 --m $m --ell $l,$((l+1)) --arms norm4 --solvers sat,f4 --planted 4 --unplanted 4 --budget-s 600 --seed 20261115; done
for D in 4 5; do $P --out runs/f5_xl_d${D}_20261115 --m 7,9,11,13 --arms norm --solvers xl --xl-degree $D --planted 4 --unplanted 4 --budget-s 600 --seed 20261115; done
echo EXIT $?
