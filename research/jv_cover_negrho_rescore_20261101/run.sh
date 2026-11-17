#!/usr/bin/env bash
# Frozen orchestration for PROTOCOL.md: new negation-map rho runs only (run indices 2..K+1), 4 single-thread jobs at a time.
# Usage: run.sh   (from anywhere; logs to results/new/*.log, walls to results/walls.txt)
set -u
cd /home/user/crypto
D=research/jv_cover_negrho_rescore_20261101; R=$D/results
B=${JV_BIN:-/tmp/claude-0/-home-user/e05ef163-36d2-5181-b9b7-869571ded707/scratchpad/jv_cover_rescore}
K=12                                   # new runs per instance (run indices 2..13)
LAUNCH_DEADLINE=$((3*3600+36*60))      # no new job after T0 + 3 h 36 min
mkdir -p $R/new
{ echo "host $(lscpu | sed -n 's/^Model name: *//p') nproc $(nproc)"; sha256sum $B; echo "T0 $(date -u +%FT%TZ)"; } >> $R/versions.txt
export B R LAUNCH_DEADLINE; export T0=$(date +%s)
job() { # p seed run
  local p=$1 s=$2 r=$3 a=$(date +%s)
  local L=rhostrong_p${p}_s${s}_run${r}
  [ -s $R/new/$L.json ] && return 0
  if [ $(( a - T0 )) -gt $LAUNCH_DEADLINE ]; then echo "$L skipped: launch deadline" >> $R/walls.txt; return 0; fi
  timeout 5400 $B --exp rhostrong --sizes $p --first-seed $s --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run $r --json $R/new/$L.json > $R/new/$L.log 2>&1
  echo "$L exit $? wall_s $(( $(date +%s) - a )) at_s $(( $(date +%s) - T0 ))" >> $R/walls.txt
}
export -f job
# order: by run index (a round = the 9 instances of one run index), longest primes first inside a round
for r in $(seq 2 $((K+1))); do for p in 1601 1451 1301; do for s in 1 2 3; do echo "$p $s $r"; done; done; done \
  | xargs -P4 -L1 bash -c 'job $0 $1 $2'
echo "ALL DONE total_s $(( $(date +%s) - T0 ))" >> $R/walls.txt
