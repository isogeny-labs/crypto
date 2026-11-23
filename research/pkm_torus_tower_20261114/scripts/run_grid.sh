#!/usr/bin/env bash
# Grid of PROTOCOL.md cells (+ A1 arms). One process per arm; cells sequential; timeout 1200 per cell.
set -u
cd "$(dirname "$0")/.."
P2=${P2:-73727}   # second prime = -1 mod 2^13 (replication of arm T)
run() { # arm p m t seed target dmax
  local f="runs/${1}_p${2}_m${3}_t${4}_s${5}_${6}.jsonl"
  [ -s "$f" ] && grep -q -E "certified|censored" "$f" && return
  : > "$f"
  timeout 1200 python3 -I scripts/raw_closure.py --arm $1 --p $2 --m $3 --t $4 --seed $5 --target $6 --dmax $7 --quiet --out "$f" >/dev/null 2>"runs/${1}_p${2}_m${3}_t${4}_s${5}_${6}.err"
  echo "$(date -u +%H:%M:%S) $1 p$2 m$3 t$4 s$5 $6 exit=$?" >> runs/progress.txt
}
arm_K() { for s in 1 2; do for t in 1 2 3 4 5; do run K 40961 2 $t $s random 7; done; for t in 1 2 3; do run K 40961 3 $t $s random 7; done; done; run K 40961 2 3 1 planted 6; }
arm_S() { for s in 1 2; do for t in 1 2 3 4 5; do run S 40961 2 $t $s random 7; done; for t in 1 2 3; do run S 40961 3 $t $s random 7; done; done; run S 40961 2 3 1 planted 6; }
arm_T() { for s in 1 2; do for t in 1 2 3 4 5; do run T 8191 2 $t $s random 7; done; for t in 1 2 3; do run T 8191 3 $t $s random 7; done; done; run T 8191 2 3 1 planted 6; for t in 2 3 4; do run T $P2 2 $t 1 random 7; done; }
arm_M() { for s in 1 2; do for t in 1 2 3 4 5; do run M 40961 2 $t $s random 7; done; for t in 1 2 3; do run M 40961 3 $t $s random 7; done; done; run M 40961 2 3 1 planted 6; }
arm_C() { for s in 1 2; do for t in 1 2 3 4; do run C 8191 2 $t $s random 6; done; for t in 1 2; do run C 8191 3 $t $s random 6; done; done; run C 8191 2 3 1 planted 6; run C 8191 2 5 1 random 5; }
arm_K & arm_S & arm_T & arm_M & arm_C &
wait
echo "$(date -u +%H:%M:%S) GRID DONE" >> runs/progress.txt
