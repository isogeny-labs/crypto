#!/usr/bin/env bash
# Frozen driver for PROTOCOL.md. Usage: bash run.sh inputs | replay | cover | rho   (from anywhere)
set -u
cd /home/user/crypto
D=research/jv_sieve_line_vs_rho_20261103; R=$D/results
B=${JV_BIN:-/tmp/claude-0/-home-user/e05ef163-36d2-5181-b9b7-869571ded707/scratchpad/jv_cover_round3_frozen}
SB=research/jv_sieve_negrho_20261102/results
[ -f $R/FROZEN.sha ] || { echo "results/FROZEN.sha missing: freeze first" >&2; exit 2; }
[ "$(sha256sum $B | cut -d' ' -f1)" = "$(grep 'jv_cover_round3_frozen$' $R/FROZEN.sha | cut -d' ' -f1)" ] || { echo "binary differs from FROZEN.sha" >&2; exit 2; }
(cd $D && sha256sum -c --quiet <(grep -E ' (PROTOCOL.md)$' results/FROZEN.sha)) || { echo "frozen PROTOCOL.md changed" >&2; exit 2; }
export B R D
case "${1:-}" in
inputs) # reuse study B's 16 p=1511 rho records unchanged
  mkdir -p $R/rho $R/inputs
  for f in $SB/runs/rhostrong_p1511_s*_run*.json; do cp -n $f $R/rho/; done
  (cd $SB/runs && sha256sum rhostrong_p1511_s*_run*.json) > $R/inputs/studyB_p1511.sha256
  (cd $R/rho && sha256sum -c --quiet ../inputs/studyB_p1511.sha256) && echo inputs ok ;;
replay) # R1 and R2: bit-identical except wall_ms; both must pass before the rho queue starts
  mkdir -p $R/replay; rm -f $R/replay/REPLAY_OK
  cp $SB/runs/rhostrong_p1511_s17_run0.json $R/replay/B_p1511_s17_run0.old.json
  cp $SB/replay/old_p1451_s1_run0.json $R/replay/O_p1451_s1_run0.old.json
  a=$(date +%s)
  $B --exp rhostrong --sizes 1511 --first-seed 17 --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run 0 --json $R/replay/B_p1511_s17_run0.new.json > $R/replay/B.log 2>&1 &
  $B --exp rhostrong --sizes 1451 --first-seed 1 --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run 0 --json $R/replay/O_p1451_s1_run0.new.json > $R/replay/O.log 2>&1 &
  wait
  echo "replay wall_s $(( $(date +%s) - a ))" >> $R/walls.txt
  ok=1
  for t in B_p1511_s17_run0 O_p1451_s1_run0; do
    if [ "$(jq -S 'map(del(.wall_ms))' $R/replay/$t.old.json | sha256sum)" = "$(jq -S 'map(del(.wall_ms))' $R/replay/$t.new.json | sha256sum)" ]; then echo "$t identical"; else echo "$t DIFFERS"; ok=0; fi
  done | tee $R/replay/REPLAY.txt
  grep -q DIFFERS $R/replay/REPLAY.txt || touch $R/replay/REPLAY_OK ;;
cover) # P, H, U on seeds 17-24 at each prime (study A's options)
  for P in 1009 1511 1823; do
    for arm in P H U; do
      case $arm in P) E="";; H) E="JV_FB_LINE=4096 JV_FB_LINE_MODE=horner";; U) E="JV_FB_LINE=32768 JV_FB_LINE_MODE=table";; esac
      L=${arm}_$P; [ -s $R/cover/$L.json ] && continue
      s0=$(date +%s)
      env -u JV_FB_LINE -u JV_FB_LINE_MODE JV_LIN=sparse $E $B --exp sieve --stop 64 --sizes $P --first-seed 17 --seeds 8 --rho-runs 0 --json $R/cover/$L.json > $R/cover/$L.log 2>&1
      echo "cover $L rc=$? wall=$(( $(date +%s)-s0 ))s" >> $R/walls.txt
    done
  done ;;
rho) # matched rho queue; blocked until the replays pass
  [ -f $R/replay/REPLAY_OK ] || { echo "replay controls not passed" >&2; exit 2; }
  export LAUNCH_DEADLINE=9000 T0=$(date +%s)
  echo "rho T0 $(date -u +%FT%TZ)" >> $R/walls.txt
  job() { # p seed run
    local p=$1 s=$2 k=$3 a=$(date +%s) L=rhostrong_p${1}_s${2}_run${3}
    [ -s $R/rho/$L.json ] && return 0
    if [ $(( a - T0 )) -gt $LAUNCH_DEADLINE ]; then echo "$L skipped: launch deadline" >> $R/walls.txt; return 0; fi
    timeout 3600 $B --exp rhostrong --sizes $p --first-seed $s --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run $k --json $R/rho/$L.json > $R/rho/$L.log 2>&1
    echo "$L exit $? wall_s $(( $(date +%s) - a )) at_s $(( $(date +%s) - T0 ))" >> $R/walls.txt
  }
  export -f job
  { for s in 17 18 19 20; do echo "1823 $s 0"; done
    for k in 0 1; do for s in $(seq 17 24); do echo "1009 $s $k"; done; done
    for s in 21 22 23 24; do echo "1823 $s 0"; done; } | xargs -P4 -L1 bash -c 'job $0 $1 $2'
  echo "RHO DONE total_s $(( $(date +%s) - T0 ))" >> $R/walls.txt ;;
*) echo "usage: run.sh inputs|replay|cover|rho"; exit 2 ;;
esac
