#!/bin/bash
# Runs on a Modal GPU container.  /work/ab holds ic-before, ic-after (release binaries of the same
# source, differing only in the direct pack), gpu/fes (sources) and exp/ (this directory: run_ab.sh,
# sweeps).  Builds the CUDA serve worker, checks it, then runs the before/after A/B of ic bench with
# both timers on, so each log carries the Rust-side `fes-timing:` line and the worker-side
# `serve-timing:` line.  Writes $RESULTS (default /results).
set -uo pipefail
cd /work/ab || exit 1
export DEBIAN_FRONTEND=noninteractive
R=${RESULTS:-/results}; mkdir -p "$R"
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a "$R/progress.log"; }
nvidia-smi -q > "$R/nvidia-smi-q.txt" 2>&1; nvcc --version > "$R/nvcc.txt" 2>&1; lscpu > "$R/lscpu.txt" 2>&1; nproc > "$R/nproc.txt"
nvidia-smi --query-gpu=name,driver_version,clocks.max.sm,power.limit,memory.total --format=csv,noheader | tee "$R/gpu.txt"
sha256sum ic-before ic-after > "$R/binaries.sha256"
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
log "building gpu/fes for $ARCH"
( cd gpu/fes && make cuda host-worker ARCH=$ARCH > "$R/fes-build.log" 2>&1 ) ; rc=$?
if [ $rc -ne 0 ] || [ ! -x gpu/fes/fes_cuda ]; then log "BUILD FAILED rc=$rc"; tail -40 "$R/fes-build.log"; exit 3; fi
( cd gpu/fes && ./fes_cuda > "$R/fes-selftest.log" 2>&1 ) && log "fes_cuda selftest done" || { log "fes_cuda selftest FAILED"; tail -5 "$R/fes-selftest.log"; }
printf '3 2\n0\n1 2 0\n0 0 0 0 0 3\n2 1\n1\n1 1\n0 0 0\n' > "$R/serve-smoke.in"
gpu/fes/fes_cuda --serve < "$R/serve-smoke.in" > "$R/serve-smoke-cuda.txt" 2>&1
gpu/fes/fes_solve --serve < "$R/serve-smoke.in" > "$R/serve-smoke-host.txt" 2>&1
if cmp -s "$R/serve-smoke-cuda.txt" "$R/serve-smoke-host.txt"; then log "serve smoke: cuda == host"; else log "serve smoke: DIFFER (stopping)"; cat "$R/serve-smoke-cuda.txt" "$R/serve-smoke-host.txt"; exit 4; fi
export FES_SERVE_TIMING=1 ICX_FES_WORKER=$PWD/gpu/fes/fes_cuda INSTANCES="${INSTANCES:-koblitz-n23 char2-n25}"
log "A/B on $INSTANCES"
bash exp/run_ab.sh "$PWD/ic-before" "$PWD/ic-after" "$PWD/exp" "$R/ab" 2>&1 | tee "$R/ab.out"
log "done"
