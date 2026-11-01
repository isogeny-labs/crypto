#!/bin/bash
# Rerun driver for the prime-field leads 20261031 (frozen in RERUN_PLAN.md before any data).
# usage: run_stage.sh build | parity_c2 | parity_l2b | l2b | c2m | l2a | l2a_fit
# Every stage writes only under this directory. Counted metrics only; wall time is used
# only as the frozen kill caps. Stages are run one at a time, never concurrently.
set -euo pipefail
H=$(cd "$(dirname "$0")" && pwd)
L=$(cd "$H/.." && pwd)
B=${RERUN_BUILD:-/home/user/cargo-rerun}
export CARGO_TARGET_DIR=$B
stage=${1:?stage}
need() { command -v "$1" >/dev/null || { echo "missing tool: $1" >&2; exit 3; }; }
case "$stage" in
  build)
    need cargo; need rustc; need msolve
    mkdir -p "$B/bin"
    (cd "$L/L2b_dreg_trajectory/gen" && cargo build -q --release)
    (cd "$L/L2b_dreg_trajectory/rsswatch" && cargo build -q --release)
    (cd "$L/C2_volcano/harness" && cargo build -q --release)
    rustc -O -o "$B/bin/l2a" "$L/L2a_slope_e_m3/src/l2a.rs"
    rustc -O -o "$B/bin/l2a_stats" "$L/L2a_slope_e_m3/src/stats.rs"
    sha256sum "$B/release/l2b-gen" "$B/release/rsswatch" "$B/release/c2_volcano" "$B/bin/l2a" "$B/bin/l2a_stats" "$(command -v msolve)" | tee "$H/binaries.sha256"
    dpkg -l msolve | tail -1 | tee "$H/msolve_version.txt"
    ;;
  parity_c2)
    # P1: the 32 committed S-instance msolve inputs, original timeout 300 s, 4 threads.
    D=$H/parity/C2_S; rm -rf "$D"; mkdir -p "$D/msolve"
    cp "$L/C2_volcano/S/msolve/"*.ms "$D/msolve/"
    sha256sum "$D/msolve/"*.ms > "$D/inputs.sha256"
    "$B/release/c2_volcano" msolve "$D" 300 4 2>&1 | tee "$D/msolve_stage.log"
    ;;
  parity_l2b)
    # P2: the three probe cells that completed on the original host, original cap 600 s.
    export L2B_GEN=$B/release/l2b-gen RSSWATCH=$B/release/rsswatch MSOLVE=$(command -v msolve)
    export L2B_RESULTS=$H/parity/L2b_probes L2B_WORK=$H/parity/L2b_probes/work L2B_RSSCAP_KB=$((6*1024*1024))
    rm -rf "$L2B_RESULTS"; mkdir -p "$L2B_RESULTS"
    for p in 4093 16381 65521; do echo "$p 9 9 real"; done \
      | xargs -P 3 -L 1 sh -c '"'"$L/L2b_dreg_trajectory/run_cell.sh"'" $0 $1 $2 $3 600'
    ;;
  l2b)
    # Stage 1: the ladder of run_ladder.sh, same lines in the same order, cap 1500 s.
    # Long cells (k = 20, 18) with 2 in parallel, then the k <= 16 cells with 4 in parallel;
    # memory cap 6 GiB per run (the host has 15 GiB; RERUN_PLAN.md, amendment A2).
    export L2B_GEN=$B/release/l2b-gen RSSWATCH=$B/release/rsswatch MSOLVE=$(command -v msolve)
    export L2B_RESULTS=$H/L2b/results L2B_WORK=$H/L2b/work L2B_RSSCAP_KB=$((6*1024*1024))
    rm -rf "$H/L2b"; mkdir -p "$L2B_RESULTS"
    RC="$L/L2b_dreg_trajectory/run_cell.sh"
    { for t in 1; do for a in real ctrl; do echo "1048573 1 $t $a"; done; done
      for t in 1 2; do for a in real ctrl; do echo "262139 1 $t $a"; done; done; } \
      | xargs -P 2 -L 1 sh -c '"'"$RC"'" $0 $1 $2 $3 1500'
    { for p in 65521 16381 4093 1021; do
        for c in 1 2; do for t in 1 2 3 4; do for a in real ctrl; do echo "$p $c $t $a"; done; done; done
      done; } | xargs -P 4 -L 1 sh -c '"'"$RC"'" $0 $1 $2 $3 1500'
    bash "$L/L2b_dreg_trajectory/summarize.sh"
    ;;
  c2m)
    # Stage 2: the 16 committed M-instance msolve inputs, 4 threads, cap 3600 s (amendment A3).
    D=$H/C2_M; rm -rf "$D"; mkdir -p "$D/msolve"
    cp "$L/C2_volcano/M/msolve/"*.ms "$D/msolve/"
    sha256sum "$D/msolve/"*.ms > "$D/inputs.sha256"
    "$B/release/c2_volcano" msolve "$D" 3600 4 2>&1 | tee "$D/msolve_stage.log"
    ;;
  l2a)
    # Stage 3: the full L2a ladder s = 6..12, 4 threads, frozen 20-minute cell guard.
    D=$H/L2a; rm -rf "$D"; mkdir -p "$D"
    "$B/bin/l2a" "$D" 6 12 4 2> "$D/run.log"
    ;;
  l2a_fit)
    D=$H/L2a; mkdir -p "$D/results"
    "$B/bin/l2a_stats" "$D/attempts.csv" "$D/results" 2>&1 | tee "$D/fit.log"
    ;;
  *) echo "unknown stage $stage" >&2; exit 2 ;;
esac
