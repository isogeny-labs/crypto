#!/usr/bin/env bash
# Runs on the GPU pod. Macaulay shared-resident u16 variant (/work/trees/mac, gpu/macaulay with
# rref_batch_kernel_smem) against the main-branch binary (/work/trees/cand from job.sh).
# Arms: G (main binary), G2 (new binary, MAC_SMEM=0: the same global kernel), S256/S512/S1024
# (new binary, shared path, MAC_SMEM_THREADS). Writes /work/out_mac.   job_macaulay.sh ROUNDS
set -uo pipefail
ROUNDS=${1:-5}
W=/work; OUT=$W/out_mac; mkdir -p $OUT
export CUDA_MODULE_LOADING=EAGER
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
NV="-O3 -std=c++17 -arch=$ARCH -lineinfo -Xptxas -v"
M=$W/trees/mac/gpu/macaulay; G=$W/trees/cand/gpu/macaulay
( cd $M && make test > $OUT/test.log 2>&1; echo rc=$? >> $OUT/test.log
  make bench ARCH=$ARCH NVCCFLAGS="$NV" > $OUT/build.log 2>&1 || echo BUILD_FAILED >> $OUT/build.log
  cp bench bench_default )
# small shape build (p=271 64x80) for a second identity check of both paths
rm -rf $W/mac_small; cp -r $M $W/mac_small
( cd $W/mac_small && make clean >/dev/null && make bench P=271 ROWS=64 COLS=80 ARCH=$ARCH NVCCFLAGS="$NV" > $OUT/build-small.log 2>&1 )
log "built"
arm() {  # arm NAME -> runs "$@" from the arm's directory with its environment
  case $1 in
    G) (cd $G && ./bench "${@:2}") ;;
    G2) (cd $M && MAC_SMEM=0 ./bench_default "${@:2}") ;;
    S256) (cd $M && MAC_SMEM_THREADS=256 ./bench_default "${@:2}") ;;
    S512) (cd $M && MAC_SMEM_THREADS=512 ./bench_default "${@:2}") ;;
    S1024) (cd $M && MAC_SMEM_THREADS=1024 ./bench_default "${@:2}") ;;
  esac
}
for a in G G2 S256 S512 S1024; do
  timeout 900 bash -c "$(declare -f arm); G=$G M=$M; arm $a selftest 64" > $OUT/selftest-$a.log 2>&1; echo rc=$? >> $OUT/selftest-$a.log
  log "selftest $a: $(grep -c 'selftest ok' $OUT/selftest-$a.log) $(head -1 $OUT/selftest-$a.log)"
done
for e in "MAC_SMEM=0" "MAC_SMEM_THREADS=512" "MAC_SMEM_THREADS=1024"; do
  (cd $W/mac_small && env $e timeout 900 ./bench selftest 64 > "$OUT/selftest-small-$e.log" 2>&1; echo rc=$? >> "$OUT/selftest-small-$e.log")
  log "selftest small $e: $(grep -c 'selftest ok' "$OUT/selftest-small-$e.log")"
done
(cd $M && ./bench_default occupancy > $OUT/occupancy.log 2>&1)
for r in $(seq 1 $ROUNDS); do
  for a in G S512 G2 S1024 S256; do
    timeout 900 bash -c "$(declare -f arm); G=$G M=$M; arm $a throughput" > $OUT/tp-$a-$r.log 2>&1
  done
  log "round $r"
done
log "done"
cd $W && tar czf out_mac.tgz out_mac && sha256sum out_mac.tgz > out_mac.tgz.sha256
