#!/usr/bin/env bash
# Runs on the GPU pod. /work/rp holds: ic (release binary of REV), gpu/fes, gpu/rootfind, sweeps/
# (the exact specs of research/ic_gpu_e2e_20261113). Writes /work/out_rp.
#   1. Replay of the end-to-end sweeps (inherited-f4, fes-f2, fes-f2-gpu) on a fresh host, with the
#      CUDA worker's opt-in timers (FES_SERVE_TIMING=1: busy and kernel GPU time per call), which
#      bound what batching systems per launch could save.
#   2. rootfind: sampled phase profile (RF_PROFILE=1 build) and a plain build for the perturbation.
set -uo pipefail
W=/work/rp; OUT=/work/out_rp; mkdir -p $OUT
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
cp $W/REV $OUT/
RFF="-O3 -std=c++17 -lineinfo -gencode arch=compute_$CC,code=$ARCH -Xcompiler -pthread -Xptxas -v"
# rootfind builds are slow (~13 min each): start both in the background now
mkdir -p $W/rfp $W/rf0; cp -r $W/gpu/rootfind/. $W/rfp/; cp -r $W/gpu/rootfind/. $W/rf0/
( cd $W/rfp && make -j2 bench ARCH=$ARCH RF_PROFILE=1 NVCCFLAGS="$RFF" > $OUT/rf-build-profile.log 2>&1; echo rc=$? >> $OUT/rf-build-profile.log ) &
( cd $W/rf0 && make -j2 bench ARCH=$ARCH NVCCFLAGS="$RFF" > $OUT/rf-build-plain.log 2>&1; echo rc=$? >> $OUT/rf-build-plain.log ) &
( cd $W/gpu/fes && make cuda host-worker ARCH=$ARCH > $OUT/fes-build.log 2>&1 && ./fes_cuda > $OUT/fes-selftest.log 2>&1 )
log "fes built: $(tail -1 $OUT/fes-selftest.log)"
sha256sum $W/ic $W/gpu/fes/fes_cuda > $OUT/sha256.txt
export ICX_FES_WORKER=$W/gpu/fes/fes_cuda FES_SERVE_TIMING=1
declare -A TMO=([koblitz-n23]=2400 [char2-n25]=3600 [char2-n29]=5400)
for s in koblitz-n23 char2-n25 char2-n29; do
  log "start $s"
  timeout ${TMO[$s]} $W/ic bench --json --sweep $W/sweeps/$s.json --out $OUT/$s.json \
    --solver-budget-seconds 120 --rho-runs 16 --max-trials 20000000 > $OUT/$s.log 2>&1
  log "end $s rc=$? $(grep -c '"verified": true' $OUT/$s.json 2>/dev/null) verified-true; $(grep -h 'serve-bin-timing' $OUT/$s.log | tr '\n' ' ')"
  cd /work && tar czf out_rp.tgz out_rp && sha256sum out_rp.tgz > out_rp.tgz.sha256; cd - >/dev/null
done
unset FES_SERVE_TIMING
wait
log "rootfind built: profile $(tail -1 $OUT/rf-build-profile.log), plain $(tail -1 $OUT/rf-build-plain.log)"
sha256sum $W/rfp/bench $W/rf0/bench >> $OUT/sha256.txt 2>&1
( cd $W/rfp && timeout 900 ./bench selftest > $OUT/rf-selftest-profile.log 2>&1 )
log "rf selftest (profile build): $(grep -c 'SELFTEST PASSED' $OUT/rf-selftest-profile.log)"
for r in 1 2 3; do
  ( cd $W/rfp && timeout 900 ./bench profile > $OUT/rf-profile-large-$r.log 2>&1 && timeout 900 ./bench profile 61 1024 262144 > $OUT/rf-profile-small-$r.log 2>&1 )
  ( cd $W/rf0 && timeout 900 ./bench throughput > $OUT/rf-plain-large-$r.log 2>&1 && timeout 900 ./bench throughput 61 1024 262144 > $OUT/rf-plain-small-$r.log 2>&1 )
  log "rf round $r"
done
log "done"
cd /work && tar czf out_rp.tgz out_rp && sha256sum out_rp.tgz > out_rp.tgz.sha256
