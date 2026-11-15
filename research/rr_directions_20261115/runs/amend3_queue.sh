#!/bin/bash
# Amendment 3: stop F4 targets after the 4th m = 11 draw summary, then run the F1
# remainder (s4 without F4 at l >= 6), then F4 targets m = 13 at 4 draws x 16 targets.
cd /home/user/crypto/research/rr_directions_20261115
P=/home/user/crypto/target/release/examples/rr_quadratic_panel
A=/home/user/crypto/target/release/examples/rr_sat_amortise
export RAYON_NUM_THREADS=1
while [ "$(grep -c '"targets_summary"' runs/f4_targets_20261115/results.jsonl)" -lt 4 ] && kill -0 14353 2>/dev/null; do sleep 30; done
pkill -f "rr_sat_amortise targets --out runs/f4_targets_20261115 --m 11 --targets 64" ; sleep 2
pkill -f "rr_sat_amortise targets --out runs/f4_targets_20261115 --m 13 --targets 64"
set -x
$P --out runs/f1_ladder_20261115 --m 13 --ell 6 --first-draw 1 --arms norm,rr4 --solvers sat,f4 --planted 4 --unplanted 4 --budget-s 600 --seed 20261115
$P --out runs/f1_ladder_20261115 --m 13 --ell 6 --first-draw 1 --arms s4 --solvers sat --planted 4 --unplanted 4 --budget-s 600 --seed 20261115
$P --out runs/f1_ladder_20261115 --m 17 --ell 6 --first-draw 1 --arms norm,rr4 --solvers sat,f4 --planted 4 --unplanted 4 --budget-s 600 --seed 20261115
$P --out runs/f1_ladder_20261115 --m 17 --ell 6 --first-draw 1 --arms s4 --solvers sat --planted 4 --unplanted 4 --budget-s 600 --seed 20261115
$P --out runs/f1_ladder_20261115 --m 17 --ell 7 --arms norm,rr4 --solvers sat,f4 --planted 4 --unplanted 4 --budget-s 600 --seed 20261115
$P --out runs/f1_ladder_20261115 --m 17 --ell 7 --arms s4 --solvers sat --planted 4 --unplanted 4 --budget-s 600 --seed 20261115
$A targets --out runs/f4_targets_20261115 --m 13 --targets 16 --draws 0,1,2,3 --budget-s 600 --seed 20261115
echo EXIT $?
