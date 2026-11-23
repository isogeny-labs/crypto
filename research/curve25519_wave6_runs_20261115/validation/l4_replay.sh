#!/usr/bin/env bash
# Validator replays for L4 (each cell runs in well under a second). Writes <name>.out/.err into $1.
# Usage: l4_replay.sh <out_dir>
set -u
S=${1:?out dir}; mkdir -p "$S"
NEW=/home/user/ctarget-f1/release/curve25519_family     # expected sha256 5abc70ba...3746
OLD=/home/user/bin-archive/curve25519_family-dc2c30a4   # expected sha256 dc2c30a4...3faf
sha256sum "$NEW" "$OLD"
run() { name=$1; shift; ( ulimit -v 6000000; env RAYON_NUM_THREADS=1 "$@" ) > "$S/$name.out" 2> "$S/$name.err"; echo "$name exit $?"; }
run new_sym19_log env F4SUB_STEP_LOG=1 "$NEW" f4sub 20 sub 19 0 mont 2 sym 6 20263127 1800
run new_sym19 "$NEW" f4sub 20 sub 19 0 mont 2 sym 6 20263127 1800
run new_ef19 "$NEW" f4sub 20 sub 19 0 mont 2 sym-efirst 6 20263127 1800
run new_ef57 "$NEW" f4sub 20 sub 57 0 mont 2 sym-efirst 6 20263165 1800
run new_sym57 "$NEW" f4sub 20 sub 57 0 mont 2 sym 6 20263165 1800
# exploratory (validator only, not producer evidence): step logs of the e-first cells and of the d = 57 sym cell
run new_ef57_log env F4SUB_STEP_LOG=1 "$NEW" f4sub 20 sub 57 0 mont 2 sym-efirst 6 20263165 1800
run new_ef19_log env F4SUB_STEP_LOG=1 "$NEW" f4sub 20 sub 19 0 mont 2 sym-efirst 6 20263127 1800
run new_sym57_log env F4SUB_STEP_LOG=1 "$NEW" f4sub 20 sub 57 0 mont 2 sym 6 20263165 1800
# frozen reference binary on the two recorded cells
run old_sym19 "$OLD" f4sub 20 sub 19 0 mont 2 sym 6 20263127 1800
run old_sym57 "$OLD" f4sub 20 sub 57 0 mont 2 sym 6 20263165 1800
