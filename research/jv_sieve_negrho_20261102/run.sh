#!/usr/bin/env bash
# Frozen orchestration for PROTOCOL.md. Usage: run.sh replay | run.sh runs
set -u
cd /home/user/crypto
D=research/jv_sieve_negrho_20261102; R=$D/results
B=${JV_BIN:-/tmp/claude-0/-home-user/e05ef163-36d2-5181-b9b7-869571ded707/scratchpad/jv_cover_rho_frozen}
OLD=research/jv_cover_negrho_rescore_20261101/results/inputs/old_rhostrong
export B R D
case "${1:-}" in
replay)  # control 3: p=1451 seed 1 run 0 of #3278, bit-identical except wall_ms (compared by the fit tool)
  a=$(date +%s)
  $B --exp rhostrong --sizes 1451 --first-seed 1 --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run 0 --json $R/replay/new_p1451_s1_run0.json > $R/replay/new_p1451_s1_run0.log 2>&1
  echo "exit $? wall_s $(( $(date +%s) - a ))" >> $R/replay/wall.txt
  cp $OLD/rhostrong_p1451_run0.json $R/replay/old_p1451_s1_run0.json ;;
runs)
  LAUNCH_DEADLINE=7200; export LAUNCH_DEADLINE; export T0=$(date +%s)
  echo "T0 $(date -u +%FT%TZ)" >> $R/walls.txt
  job() { # seed run
    local s=$1 r=$2 a=$(date +%s) L=rhostrong_p1511_s${1}_run${2}
    [ -s $R/runs/$L.json ] && return 0
    if [ $(( a - T0 )) -gt $LAUNCH_DEADLINE ]; then echo "$L skipped: launch deadline" >> $R/walls.txt; return 0; fi
    timeout 5400 $B --exp rhostrong --sizes 1511 --first-seed $s --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run $r --json $R/runs/$L.json > $R/runs/$L.log 2>&1
    echo "$L exit $? wall_s $(( $(date +%s) - a )) at_s $(( $(date +%s) - T0 ))" >> $R/walls.txt
  }
  export -f job
  for r in 0 1; do for s in $(seq 17 24); do echo "$s $r"; done; done | xargs -P4 -L1 bash -c 'job $0 $1'
  echo "ALL DONE total_s $(( $(date +%s) - T0 ))" >> $R/walls.txt ;;
*) echo "usage: run.sh replay|runs"; exit 2 ;;
esac
