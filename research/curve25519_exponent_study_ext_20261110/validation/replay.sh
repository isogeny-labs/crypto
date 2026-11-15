#!/usr/bin/env bash
# Validator replay of two cheap extension cells with the archived frozen binary.
# Uses the runner's exact command line (scripts/run_ext.py run_cell) with the
# recorded seed 20261108 + 1000*m + d. Output: replay_<cell>.out next to this script.
# The dispatch's literal seeds (20265112, 20265114) do not follow that formula
# (they are 20261108 + 4004 and + 4006) and were not used.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
BIN="${BIN:-/home/user/bin-archive/curve25519_family-dc2c30a4}"
sha256sum "$BIN"
for d in 12 14; do
  m=3; seed=$((20261108 + 1000 * m + d))
  t0=$(date +%s.%N)
  bash -c "ulimit -v 12000000; RAYON_NUM_THREADS=1 $BIN f4sub 20 sub $d 0 mont $m sym 6 $seed 10800" \
    > "$HERE/replay_k20_m3_sym_d${d}_g0.out" 2> "$HERE/replay_k20_m3_sym_d${d}_g0.err"
  rc=$?
  t1=$(date +%s.%N)
  echo "d=$d seed=$seed exit=$rc wall_s=$(echo "$t1 - $t0" | bc) started=$(date -u -d @"${t0%.*}" +%FT%TZ)"
done
