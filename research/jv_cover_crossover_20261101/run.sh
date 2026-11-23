#!/usr/bin/env bash
# Thin orchestration for PROTOCOL.md. Sequential phases; at most 4 cores busy. Usage: run.sh
set -u
cd /home/user/crypto
D=research/jv_cover_crossover_20261101; R=$D/results
B=${JV_BIN:-/tmp/claude-0/-home-user/e05ef163-36d2-5181-b9b7-869571ded707/scratchpad/jv_cover_new}
PRED=research/jv_cover_p_scaling_20261024/results
mkdir -p $R; walls=$R/walls.txt
{ echo "host $(lscpu | sed -n 's/^Model name: *//p') nproc $(nproc)"; sha256sum $B; echo "T0 $(date -u +%FT%TZ)"; } >> $R/versions.txt
T0=$(date +%s); now() { date +%s; }
LAUNCH_DEADLINE=$((6*3600+1800))   # no new job after T0 + 6.5 h
run() { # label timeout args...   (stdout -> log, json -> $R/label.json)
  local label=$1 to=$2; shift 2; local a=$(now)
  timeout $to $B "$@" --json $R/$label.json > $R/$label.log 2>&1
  echo "$label exit $? wall_s $(( $(now) - a )) at_s $(( $(now) - T0 ))" >> $walls
}
# 0. calibration: rhodp at 251, seeds 1..2, 8 runs per seed, dp_bits 12
for s in 1 2; do run calib_p251_s$s 600 --exp rhodp --sizes 251 --first-seed $s --seeds 1 --rho-runs 8 --dp-bits 12 --threads 4; done
# 1. reproduction control
run repro_dlp_p1009_s1 2400 --exp dlp --sizes 1009 --first-seed 1 --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64
# 2. cover arm (1301 s1 reused from the pre-freeze timing probe: same binary, same command)
cp $R/probe/dlp_p1301_s1.json $R/dlp_p1301_s1.json; cp $R/probe/dlp_p1301_s1.log $R/dlp_p1301_s1.log
echo "dlp_p1301_s1 reused from probe wall_s 843" >> $walls
for p in 1301 1451 1601; do for s in 1 2 3; do
  [ -s $R/dlp_p${p}_s$s.json ] && continue
  run dlp_p${p}_s$s 3600 --exp dlp --sizes $p --first-seed $s --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64
done; done
# 3. rhostrong: 3 instances in parallel (3 cores), runs 0 and 1
for p in 1301 1451 1601; do for r in 0 1; do
  run rhostrong_p${p}_run$r 3600 --exp rhostrong --sizes $p --first-seed 1 --seeds 3 --neg 1 --batch 16 --r 256 --dp-bits 16 --run $r
done; done
# 4. rhodp: 3 runs per instance (PROTOCOL.md "Budget"), 4 threads, one process per instance; the JSON is rewritten after every run
for p in 1301 1451 1601; do for s in 1 2 3; do
  if [ $(( $(now) - T0 )) -gt $LAUNCH_DEADLINE ]; then echo "rhodp_p${p}_s$s skipped: launch deadline" >> $walls; continue; fi
  case $p in 1301) to=2400;; 1451) to=3000;; *) to=3900;; esac
  run rhodp_p${p}_s$s $to --exp rhodp --sizes $p --first-seed $s --seeds 1 --rho-runs 3 --dp-bits 18 --threads 4
done; done
echo "ALL DONE total_s $(( $(now) - T0 ))" >> $walls
