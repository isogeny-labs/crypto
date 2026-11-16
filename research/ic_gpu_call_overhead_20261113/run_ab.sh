#!/bin/bash
# Before/after A/B of the Rust-side preparation cost of a fes-f2-gpu call, on the host-emulation
# serve worker (no GPU).  ic bench on the fes-f2-gpu koblitz sweeps, three pairs per instance,
# arm order alternating per pair, FES_GPU_TIMING=1.
#   run_ab.sh BEFORE_IC AFTER_IC SWEEP_DIR OUT_DIR     (SWEEP_DIR holds koblitz-n19-gpu.json, koblitz-n23-gpu.json)
set -u
BEFORE=$1; AFTER=$2; SW=$3; OUT=$4; mkdir -p "$OUT"
export FES_GPU_TIMING=1 ICX_FES_WORKER="${ICX_FES_WORKER:-$PWD/gpu/fes/fes_solve}"
for inst in ${INSTANCES:-koblitz-n19 koblitz-n23}; do
  for rep in 1 2 3; do
    if [ $((rep % 2)) = 1 ]; then order="before after"; else order="after before"; fi
    for arm in $order; do
      [ $arm = before ] && BIN=$BEFORE || BIN=$AFTER
      o="$OUT/${inst}_${rep}_${arm}"
      "$BIN" bench --json --sweep "$SW/${inst}-gpu.json" --out "$o.json" --solver-budget-seconds 120 --rho-runs 2 --max-trials 20000000 > "$o.log" 2>&1
      echo "$inst rep$rep $arm rc=$? $(grep fes-timing "$o.log" | tail -1)"
    done
  done
done
