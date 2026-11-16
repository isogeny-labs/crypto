#!/usr/bin/env bash
# Validator follow-up runs, sequential, at most 2 threads. Usage: post_replay.sh <binary> <outdir>
set -u
B=$1; O=$2; mkdir -p $O; W=$O/walls.txt
cd /home/user/crypto
run() { local label=$1; shift; local a=$(date +%s); "$@" > $O/$label.log 2>&1; echo "$label exit $? wall_s $(( $(date +%s) - a )) $(date -u +%FT%TZ)" >> $W; }
export RAYON_NUM_THREADS=2
# a. rhodp determinism at p=251 seed 1, 2 threads, twice
run rhodp_p251_s1_A timeout 900 $B --exp rhodp --sizes 251 --first-seed 1 --seeds 1 --rho-runs 2 --dp-bits 12 --threads 2 --json $O/rhodp_p251_s1_A.json
run rhodp_p251_s1_B timeout 900 $B --exp rhodp --sizes 251 --first-seed 1 --seeds 1 --rho-runs 2 --dp-bits 12 --threads 2 --json $O/rhodp_p251_s1_B.json
# b. rhostrong determinism at p=503 seeds 1-2 (2 instances in parallel), twice
run rhostrong_p503_A timeout 900 $B --exp rhostrong --sizes 503 --first-seed 1 --seeds 2 --neg 1 --batch 16 --r 256 --dp-bits 16 --run 0 --json $O/rhostrong_p503_A.json
run rhostrong_p503_B timeout 900 $B --exp rhostrong --sizes 503 --first-seed 1 --seeds 2 --neg 1 --batch 16 --r 256 --dp-bits 16 --run 0 --json $O/rhostrong_p503_B.json
# c. instrument comparison at p=251 seed 1: rebuilt binary, fast (default) and JV_LIN=sparse
run dlp_p251_s1_fast timeout 1800 $B --exp dlp --sizes 251 --first-seed 1 --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64 --json $O/dlp_p251_s1_fast.json
JV_LIN=sparse run dlp_p251_s1_sparse timeout 1800 $B --exp dlp --sizes 251 --first-seed 1 --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64 --json $O/dlp_p251_s1_sparse.json
echo "PHASE_ABC_DONE $(date -u +%FT%TZ)" >> $W
# d. reproduction control with the incumbent linear stage: p=1009 seed 1, JV_LIN=sparse
JV_LIN=sparse run dlp_p1009_s1_sparse timeout 3600 $B --exp dlp --sizes 1009 --first-seed 1 --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64 --json $O/dlp_p1009_s1_sparse.json
echo "ALL_DONE $(date -u +%FT%TZ)" >> $W
