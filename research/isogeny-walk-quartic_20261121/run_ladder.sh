#!/bin/bash
# Reproduces the measured ladder of round 2 (about 20 minutes on 2 threads).
# usage: run_ladder.sh   (from /home/user/crypto; builds first)
set -e
cd /home/user/crypto
CARGO_TARGET_DIR=$PWD/target cargo build --release -j2 --bin smith_walk_dlp_ic
export SWI_GIT=$(git rev-parse HEAD)
B=target/release/smith_walk_dlp_ic; R=research/isogeny-walk-quartic_20261121/results; mkdir -p $R
sha256sum $B > $R/binary_sha256.txt; echo $SWI_GIT > $R/git_head.txt
( for p in 101 211; do $B run --p $p --seed 31 --curves 3 --z3 1 --rho-runs 6 > $R/run_p$p.jsonl 2> $R/run_p$p.log; done ) &
( for p in 521 1021; do $B run --p $p --seed 31 --curves 3 --z3 1 --rho-runs 6 > $R/run_p$p.jsonl 2> $R/run_p$p.log; done ) &
wait
