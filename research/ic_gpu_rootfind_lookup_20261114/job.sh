#!/usr/bin/env bash
# Runs on the GPU pod. rootfind lookup variants (gpu/rootfind/rootfind.cuh flags RF_SORTED_EYT,
# RF_HASH_PACKED, RF_PROBE_PAIR) against main's lookups, one tree (/work/lk/rootfind, this branch),
# eight builds. Every arm must pass the device selftests before it is timed. Writes /work/out_lk.
#   job.sh ROUNDS
set -uo pipefail
ROUNDS=${1:-5}
W=/work/lk; OUT=/work/out_lk; mkdir -p $OUT
export CUDA_MODULE_LOADING=EAGER
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
cp $W/REV $OUT/
FL="-O3 -std=c++17 -lineinfo -gencode arch=compute_$CC,code=$ARCH -Xcompiler -pthread -Xptxas -v"
# arm name -> "EYT PACKED PAIR"
declare -A V=([M]="0 0 0" [M2]="0 0 0" [E]="1 0 0" [H]="0 1 0" [P]="0 0 1" [EP]="1 0 1" [HP]="0 1 1" [EHP]="1 1 1")
ARMS="M E H P EP HP EHP M2"
build() {
  local a=$1; set -- ${V[$a]}
  rm -rf $W/b/$a; mkdir -p $W/b/$a; cp -r $W/rootfind/. $W/b/$a/
  ( cd $W/b/$a && make clean >/dev/null 2>&1
    make test RF_SORTED_EYT=$1 RF_HASH_PACKED=$2 RF_PROBE_PAIR=$3 > $OUT/test-$a.log 2>&1; echo rc=$? >> $OUT/test-$a.log
    make -j2 bench ARCH=$ARCH RF_SORTED_EYT=$1 RF_HASH_PACKED=$2 RF_PROBE_PAIR=$3 NVCCFLAGS="$FL" > $OUT/build-$a.log 2>&1
    echo rc=$? >> $OUT/build-$a.log )
}
for a in $ARMS; do build $a & done; wait
log "built: $(for a in $ARMS; do echo -n "$a:$(tail -1 $OUT/build-$a.log) test:$(grep -c 'ALL PASS' $OUT/test-$a.log) "; done)"
sha256sum $W/b/*/bench > $OUT/sha256.txt
OK=""
for a in $ARMS; do
  ( cd $W/b/$a && timeout 900 ./bench selftest > $OUT/selftest-$a.log 2>&1
    timeout 900 ./bench selftest 61 4096 4096 >> $OUT/selftest-$a.log 2>&1
    timeout 900 ./bench p256 >> $OUT/selftest-$a.log 2>&1 )
  n=$(grep -c 'SELFTEST PASSED' $OUT/selftest-$a.log)
  log "selftest $a: $n/3 passed"
  [ "$n" = 3 ] && OK="$OK $a"
done
log "timed arms:$OK"
for r in $(seq 1 $ROUNDS); do
  # rotate the arm order each round
  set -- $OK; for i in $(seq 2 $r); do x=$1; shift; set -- "$@" $x; done
  for a in "$@"; do
    ( cd $W/b/$a && timeout 900 ./bench throughput > $OUT/tp-$a-$r.log 2>&1
      timeout 900 ./bench throughput 61 1024 262144 > $OUT/tpsmall-$a-$r.log 2>&1 )
  done
  log "round $r ($*)"
done
log "done"
cd /work && tar czf out_lk.tgz out_lk && sha256sum out_lk.tgz > out_lk.tgz.sha256
