#!/bin/bash
# Resume after the 10:56Z restart (amendment 4). Two chains, then the solo F4 cell, then CMS m = 13.
cd /home/user/crypto/research/rr_directions_20261115
P=/home/user/crypto/target/release/examples/rr_quadratic_panel
A=/home/user/crypto/target/release/examples/rr_sat_amortise
export RAYON_NUM_THREADS=1
C="--planted 4 --unplanted 4 --budget-s 600 --seed 20261115"
(
 set -x
 $P --out runs/f1_ladder_20261115 --m 13 --ell 6 --first-draw 5 --arms rr4 --solvers sat $C
 $P --out runs/f1_ladder_20261115 --m 13 --ell 6 --first-draw 6 --arms norm --solvers sat,f4 $C
 $P --out runs/f1_ladder_20261115 --m 13 --ell 6 --first-draw 6 --arms rr4 --solvers sat $C
 $P --out runs/f1_ladder_20261115 --m 17 --ell 7 --first-draw 1 --arms norm --solvers sat,f4 $C
 $P --out runs/f1_ladder_20261115 --m 17 --ell 7 --first-draw 1 --arms rr4 --solvers sat $C
 $P --out runs/f1_ladder_20261115 --m 17 --ell 7 --first-draw 4 --arms s4 --solvers sat $C
 $A targets --out runs/f4_targets_20261115 --m 13 --targets 16 --draws 0,1,2,3 --budget-s 600 --seed 20261115
 echo CHAIN_A_EXIT $?
) > runs/resume_1105_A.log 2>&1 &
(
 set -x
 $P --out runs/f3_norm4_20261115 --m 15 --ell 4 --first-draw 1 --arms norm4 --solvers sat,f4 $C
 $P --out runs/f3_norm4_20261115 --m 17 --ell 4,5 --arms norm4 --solvers sat,f4 $C
 for D in 4 5; do $P --out runs/f5_xl_d${D}_20261115 --m 7,9,11,13 --arms norm --solvers xl --xl-degree $D $C; done
 echo CHAIN_B_EXIT $?
) > runs/resume_1105_B.log 2>&1 &
wait
set -x
F4_F2_MAX_MATRIX_WORDS=1073741824 $P --out runs/rust_panel_20261115_normf4 --m 13 --first-draw 7 --arms norm --solvers f4 $C
echo SOLO_EXIT $?
