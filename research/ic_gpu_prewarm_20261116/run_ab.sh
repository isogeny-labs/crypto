#!/bin/bash
# A/B of the early worker spawn on one binary: ICX_FES_PREWARM unset (arm on) against ICX_FES_PREWARM=0 (arm off), and a
# second off arm (off2) as the noise floor; arm order rotates per repeat.  The worker is START_MS slower to start than the
# real one (a sleep before it execs it) to stand in for a CUDA context's initialisation on a host without a GPU.
#   run_ab.sh IC SWEEP_JSON OUT_DIR [START_MS=150] [REPS=4]      (IC_WORKER = the real worker, default gpu/fes/fes_solve)
set -u
IC=$1; SW=$2; OUT=$3; START_MS=${4:-150}; REPS=${5:-4}; mkdir -p "$OUT"
REAL=${IC_WORKER:-$PWD/gpu/fes/fes_solve}
WRAP="$OUT/slow_start.sh"
printf '#!/bin/sh\nsleep %s\nexec %s "$@"\n' "$(awk "BEGIN{print $START_MS/1000}")" "$REAL" > "$WRAP"; chmod +x "$WRAP"
ARMS=(on off off2)
for rep in $(seq 1 $REPS); do
  for i in 0 1 2; do
    arm=${ARMS[$(( (i + rep - 1) % 3 ))]}
    o="$OUT/${arm}_${rep}"
    if [ $arm = on ]; then PRE=""; else PRE="0"; fi
    t0=$(date +%s.%N)
    if [ -z "$PRE" ]; then env ICX_FES_WORKER="$WRAP" "$IC" bench --json --sweep "$SW" --out "$o.json" --solver-budget-seconds 60 --rho-runs 1 --max-trials 2000000 > "$o.log" 2>&1
    else env ICX_FES_PREWARM=0 ICX_FES_WORKER="$WRAP" "$IC" bench --json --sweep "$SW" --out "$o.json" --solver-budget-seconds 60 --rho-runs 1 --max-trials 2000000 > "$o.log" 2>&1; fi
    rc=$?; t1=$(date +%s.%N)
    echo "rep$rep $arm rc=$rc total_s=$(awk "BEGIN{printf \"%.3f\", $t1-$t0}")"
  done
done
