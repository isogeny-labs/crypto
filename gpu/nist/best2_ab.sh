#!/usr/bin/env bash
# RHO_BEST2 A/B for the gpu/nist rho kernel: CPU suites and a PTX/global-table
# bench build for BEST2=0 and 1, then interleaved rho cells at 256 threads,
# W = 16/32/64, both curves, REPS alternating-order repetitions.  Prints the
# raw tool output with CELL/BUILD/STATE marker lines; the rho-walk-leads
# harness (`rho_walk_leads gpuab --log`) parses it.
#   ./best2_ab.sh [ITERS=2048] [REPS=3] [SEEDS=8] [FIRST_SEED=20261027] > best2.log
set -euo pipefail
cd "$(dirname "$0")"
ITERS=${1:-2048}
REPS=${2:-3}
SEEDS=${3:-8}
FIRST_SEED=${4:-20261027}
table_args=()
if [[ $# -ge 5 ]]; then table_args=("$5"); fi  # fixed table / varying starts control
echo "GPU $(nvidia-smi --query-gpu=name,uuid,driver_version,clocks.max.sm,power.limit --format=csv,noheader)"
for b in 0 1; do
  make -s test BEST2=$b
  # Fail closed on a compiler failure; never copy a stale executable.
  make -B bench ARCH=sm_120 NIST_PTX=1 TABLE_SHARED=0 BEST2=$b
  cp bench bench-b$b
  echo "BUILD best2=$b sha256=$(sha256sum bench-b$b | cut -d' ' -f1)"
done
for ((sample = 0; sample < SEEDS; sample++)); do
 seed=$((FIRST_SEED + sample))
 for ((rep = 0; rep < REPS; rep++)); do
  for W in 16 32 64; do
    if (((rep + W / 16) % 2 == 0)); then order="0 1"; else order="1 0"; fi
    pos=0
    for b in $order; do
      echo "CELL seed_id=$seed rep=$rep order=$pos best2=$b W=$W iters=$ITERS"
      ./bench-b$b 256 rho $W $ITERS $seed "${table_args[@]}"
      echo "STATE $(nvidia-smi --query-gpu=clocks.sm,power.draw,temperature.gpu --format=csv,noheader,nounits)"
      pos=$((pos + 1))
    done
  done
done
done
