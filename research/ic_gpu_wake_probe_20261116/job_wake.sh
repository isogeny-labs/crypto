#!/bin/bash
# Runs on a Modal GPU container.  /work/ab holds ic-main, gpu/fes (sources) and exp/ (this directory).
set -uo pipefail
cd /work/ab || exit 1
export DEBIAN_FRONTEND=noninteractive
R=${RESULTS:-/results}; mkdir -p "$R"
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a "$R/progress.log"; }
nvidia-smi -q > "$R/nvidia-smi-q.txt" 2>&1; nvcc --version > "$R/nvcc.txt" 2>&1; lscpu > "$R/lscpu.txt" 2>&1; nproc > "$R/nproc.txt"
nvidia-smi --query-gpu=name,driver_version,clocks.max.sm,power.limit,memory.total --format=csv,noheader | tee "$R/gpu.txt"
uname -a > "$R/uname.txt"; cat /sys/devices/system/cpu/cpuidle/current_driver /sys/devices/system/cpu/cpuidle/current_governor > "$R/cpuidle.txt" 2>&1
sha256sum ic-main > "$R/binaries.sha256"
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
log "building gpu/fes for $ARCH"
( cd gpu/fes && make cuda host-worker ARCH=$ARCH > "$R/fes-build.log" 2>&1 ); rc=$?
if [ $rc -ne 0 ] || [ ! -x gpu/fes/fes_cuda ]; then log "BUILD FAILED rc=$rc"; tail -40 "$R/fes-build.log"; exit 3; fi
g++ -O2 -std=c++17 -I gpu/fes -o gpu/fes/null_bin exp/null_bin.cpp > "$R/null-build.log" 2>&1 || { log "null_bin BUILD FAILED"; tail -20 "$R/null-build.log"; exit 3; }
g++ -O2 -std=c++17 -o exp/wake_probe exp/wake_probe.cpp > "$R/probe-build.log" 2>&1 || { log "wake_probe BUILD FAILED"; tail -20 "$R/probe-build.log"; exit 3; }
sha256sum gpu/fes/fes_cuda gpu/fes/null_bin exp/wake_probe >> "$R/binaries.sha256"
( cd gpu/fes && ./fes_cuda > "$R/fes-selftest.log" 2>&1 ) && log "fes_cuda selftest done" || { log "fes_cuda selftest FAILED"; tail -5 "$R/fes-selftest.log"; }
log "cells"
bash exp/run_cells.sh "$PWD" "$R/cells" 2>&1 | tee "$R/cells.out"
log "done"
