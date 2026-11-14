#!/usr/bin/env bash
# Runs on the GPU pod after job.sh. Rootfind sweep variants on one tree (/work/trees/rf, a git
# archive of gpu/rootfind with RF_FUSED_SWEEP): arms R0 (default), R0b (second R0 build, A/A),
# R1 (fused, inlined entry), R2 (fused, entry as a device call). Writes /work/out_rf.
#   job_rootfind.sh ROUNDS
set -uo pipefail
ROUNDS=${1:-5}
W=/work; OUT=$W/out_rf; mkdir -p $OUT
export CUDA_MODULE_LOADING=EAGER
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
nvidia-smi --query-gpu=name,compute_cap,driver_version --format=csv > $OUT/gpu.csv
declare -A V=([B]=0 [R0]=0 [R0b]=0 [R1]=1 [R2]=2)
build_arm() {
  local a=$1 src=$W/trees/rf/gpu/rootfind; rm -rf $W/rf/$a; mkdir -p $W/rf/$a
  # B = the e9d5dbdccd baseline; every arm needs the RF_BIG linkage fix to link at all
  [ $a = B ] && src=$W/trees/base/gpu/rootfind
  cp -r $src/. $W/rf/$a/; ( cd $W/rf/$a && make clean >/dev/null 2>&1 )
  sed -i 's/^#define RF_BIG __host__ __device__ __noinline__$/#define RF_BIG __host__ __device__ __noinline__ inline/' $W/rf/$a/fp64.cuh
  ( cd $W/rf/$a && make test RF_FUSED_SWEEP=${V[$a]} > $OUT/test-$a.log 2>&1; echo rc=$? >> $OUT/test-$a.log
    make -j2 bench ARCH=$ARCH RF_FUSED_SWEEP=${V[$a]} \
      NVCCFLAGS="-O3 -std=c++17 -lineinfo -gencode arch=compute_$CC,code=$ARCH -Xcompiler -pthread -Xptxas -v" \
      > $OUT/build-$a.log 2>&1 || echo BUILD_FAILED >> $OUT/build-$a.log )
  log "built $a"
}
for a in B R0 R0b R1 R2; do build_arm $a & done; wait
for a in B R0 R1 R2; do
  ( cd $W/rf/$a && timeout 900 ./bench selftest > $OUT/selftest-$a.log 2>&1; echo rc=$? >> $OUT/selftest-$a.log
    timeout 900 ./bench selftest 61 4096 4096 > $OUT/selftest4096-$a.log 2>&1; echo rc=$? >> $OUT/selftest4096-$a.log
    timeout 900 ./bench p256 > $OUT/p256-$a.log 2>&1; echo rc=$? >> $OUT/p256-$a.log )
  log "selftest $a: $(grep -h -c 'SELFTEST PASSED' $OUT/selftest-$a.log $OUT/selftest4096-$a.log $OUT/p256-$a.log | tr '\n' ' ')"
done
for r in $(seq 1 $ROUNDS); do
  for a in B R0 R1 R2 R0b; do
    ( cd $W/rf/$a && timeout 900 ./bench throughput > $OUT/tp-$a-$r.log 2>&1 )
    ( cd $W/rf/$a && timeout 900 ./bench throughput 61 1024 262144 > $OUT/tpsmall-$a-$r.log 2>&1 )
  done
  log "round $r"
done
log "done"
cd $W && tar czf out_rf.tgz out_rf && sha256sum out_rf.tgz > out_rf.tgz.sha256
[ -x /work/job_macaulay.sh ] && /work/job_macaulay.sh "$ROUNDS"
