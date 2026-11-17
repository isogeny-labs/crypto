#!/bin/bash
# Wake-up cells: replay captured fes requests against a worker (null or CUDA) with a controlled idle gap between
# calls, the client blocking or spinning for the reply, and the worker blocking or spinning for the next request.
#   run_cells.sh WORKDIR OUT_DIR     (WORKDIR holds ic-main, gpu/fes/{fes_cuda,null_bin}, exp/{wake_probe,sweeps})
set -u
W=$1; OUT=$2; mkdir -p "$OUT"
REQ=$W/req.bin   # kept out of the results: a 5 MB data file
# 1. capture the requests ic main sends to the CUDA worker (same systems as any run of this sweep)
cat > "$W/cap.sh" <<CAP
#!/bin/bash
tee $REQ | $W/gpu/fes/fes_cuda "\$@"
CAP
chmod +x "$W/cap.sh"
ICX_FES_WORKER="$W/cap.sh" "$W/ic-main" bench --json --sweep "$W/exp/koblitz-n23-gpu.json" --out "$OUT/capture.json" \
  --solver-budget-seconds 60 --rho-runs 1 --max-trials 2000000 > "$OUT/capture.log" 2>&1
sleep 3
echo "captured $(stat -c %s "$REQ") bytes of requests"
export REQ
CELLS=()
for worker in null cuda; do for reply in block spin; do for ws in 0 2000; do for gap in 0 100 1000; do
  CELLS+=("$worker $reply $ws $gap"); done; done; done; done
NC=${#CELLS[@]}
for rep in $(seq 1 ${REPS:-3}); do
  for i in $(seq 0 $((NC - 1))); do
    c=${CELLS[$(( (i * 5 + rep * 7) % NC ))]}
    set -- $c; worker=$1; reply=$2; ws=$3; gap=$4
    [ $worker = null ] && WK=$W/gpu/fes/null_bin || WK=$W/gpu/fes/fes_cuda
    line=$(FES_SERVE_SPIN_US=$ws FES_SERVE_TIMING=1 "$W/exp/wake_probe" "$WK" $gap spin $reply ${CELL_N:-1500} 2> "$OUT/err.tmp")
    echo "rep$rep worker=$worker reply=$reply wspin=$ws gap=$gap | $line | $(grep -h 'serve-bin-timing' "$OUT/err.tmp" | tail -1)"
  done
done
