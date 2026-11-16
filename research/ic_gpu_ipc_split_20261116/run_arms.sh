#!/bin/bash
# Three arms of ic bench on the fes-f2-gpu sweeps, interleaved (arm order rotates per repeat), same sweep,
# FES_GPU_TIMING=1 (Rust side) and FES_SERVE_TIMING=1 (worker side).
#   A  ic-before (before the direct pack, text --serve protocol)  + fes_cuda
#   B  ic-main   (direct pack + binary --serve-bin protocol)      + fes_cuda
#   C  ic-main + null_bin (parses the request, answers "no solutions": the pipe round-trip floor)
#   run_arms.sh WORKDIR SWEEP_DIR OUT_DIR
set -u
W=$1; SW=$2; OUT=$3; mkdir -p "$OUT"
export FES_GPU_TIMING=1 FES_SERVE_TIMING=1
ARMS=(A B C)
for inst in ${INSTANCES:-koblitz-n23 char2-n25}; do
  for rep in 1 2 3; do
    for i in 0 1 2; do
      arm=${ARMS[$(( (i + rep - 1) % 3 ))]}
      case $arm in
        A) BIN=$W/ic-before; WK=$W/gpu/fes/fes_cuda; BUD=120 ;;
        B) BIN=$W/ic-main;   WK=$W/gpu/fes/fes_cuda; BUD=120 ;;
        C) BIN=$W/ic-main;   WK=$W/gpu/fes/null_bin; BUD=60 ;;
      esac
      o="$OUT/${inst}_${rep}_${arm}"
      ICX_FES_WORKER=$WK "$BIN" bench --json --sweep "$SW/${inst}-gpu.json" --out "$o.json" \
        --solver-budget-seconds $BUD --rho-runs 2 --max-trials 20000000 > "$o.log" 2>&1
      rc=$?
      sleep 3   # the worker prints its timing at EOF, after ic exits
      echo "$inst rep$rep $arm rc=$rc $(grep -h 'fes-timing' "$o.log" | tail -1) | $(grep -h 'serve-bin-timing\|serve-timing' "$o.log" | tail -1)"
    done
  done
done
