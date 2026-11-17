#!/usr/bin/env bash
# Thin orchestration for PROTOCOL.md. Sequential. Usage: run.sh
set -u
cd "$(dirname "$0")/../.."
D=research/jv_cover_p_scaling_20261024; R=$D/results; B=target/release/examples/jv_cover
mkdir -p $R
{ echo "host $(lscpu | sed -n 's/^Model name: *//p') nproc $(nproc)"; sha256sum $B; } > $R/versions.txt
now() { date +%s; }
T0=$(now); BUDGET=$((150*60))
walls=$R/walls.txt
run() { # label timeout args...
  local label=$1 to=$2; shift 2; local a=$(now)
  timeout $to $B "$@" --json $R/$label.json > $R/$label.log 2>&1
  echo "$label exit $? wall_s $(( $(now) - a ))" >> $walls
}
for p in 53 101 151 251 503 1009; do
  run ccov_p$p 1500 --exp ccov --sizes $p --seeds 1 --residuals 40 --constructed 20 --max-degree 24 --budget 300
done
for p in 53 101 151 251 503; do
  for s in 1 2 3; do
    if [ $p = 251 ] && [ $s = 1 ]; then cp $R/probe/dlp_251_s1.json $R/dlp_p251_s1.json; cp $R/probe/dlp_251_s1.log $R/dlp_p251_s1.log; echo "dlp_p251_s1 reused from probe wall_s 191" >> $walls; continue; fi
    if [ $p -le 251 ]; then rr="--rho-runs 16"; else rr="--rho-runs 0 --rho-ref 1.3"; fi
    run dlp_p${p}_s$s 3600 --exp dlp --sizes $p --first-seed $s --seeds 1 $rr --check-every 64
  done
done
for s in 1 2 3; do
  el=$(( $(now) - T0 ))
  if [ $s -gt 1 ] && [ $el -gt $((BUDGET - 1500)) ]; then echo "dlp_p1009_s$s skipped: cumulative $el s" >> $walls; continue; fi
  run dlp_p1009_s$s 1500 --exp dlp --sizes 1009 --first-seed $s --seeds 1 --rho-runs 0 --rho-ref 1.3 --check-every 64
done
echo "ALL DONE total_s $(( $(now) - T0 ))" >> $walls
