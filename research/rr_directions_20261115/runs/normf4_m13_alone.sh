#!/bin/bash
# Norm F4 at m = 13, draw 7 (draws 0-6 recorded), 8 GiB cap, started only when the targets run (pid 14353)
# and the follow-up chain (rr_quadratic_panel --out runs/f1|f3|f5) have exited, so it runs alone.
while pgrep -f "followups_chain.sh|amend3_queue.sh" >/dev/null; do sleep 60; done
export RAYON_NUM_THREADS=1 F4_F2_MAX_MATRIX_WORDS=1073741824
/home/user/crypto/target/release/examples/rr_quadratic_panel --out runs/rust_panel_20261115_normf4 --m 13 --first-draw 7 --arms norm --solvers f4 --budget-s 600 --seed 20261115
echo EXIT $?
