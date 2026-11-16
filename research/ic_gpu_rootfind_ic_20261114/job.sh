#!/usr/bin/env bash
# Runs on the GPU pod. /work/ric holds: REV, ecbench (release binary of REV), rootfind/ (gpu/rootfind
# sources of REV), specs/. Writes /work/out_ric.   job.sh ROUNDS
#  Part 1 (index calculus): rootfind_serve_cuda next to ecbench; ecbench smoke + main spec, replay
#    audit, table, compare cpu vs gpu arms.
#  Part 2 (Eytzinger prefetch, gpu/rootfind RF_EYT_PREFETCH): six builds of one tree, device
#    selftests first, ROUNDS rotated rounds of ./bench throughput at two workloads.
set -uo pipefail
ROUNDS=${1:-5}
W=/work/ric; OUT=/work/out_ric; mkdir -p $OUT
export CUDA_MODULE_LOADING=EAGER
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
pack() { (cd /work && tar czf out_ric.tgz out_ric && sha256sum out_ric.tgz > out_ric.tgz.sha256); }
nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt; nproc > $OUT/nproc.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
cp $W/REV $OUT/
FL="-O3 -std=c++17 -lineinfo -gencode arch=compute_$CC,code=$ARCH -Xcompiler -pthread -Xptxas -v"

# ---- Part 1 -----------------------------------------------------------------------------
( cd $W/rootfind && make clean >/dev/null 2>&1; make test > $OUT/test-serve.log 2>&1; echo rc=$? >> $OUT/test-serve.log
  make -j4 serve serve-cuda ARCH=$ARCH NVCCFLAGS="$FL" > $OUT/build-serve.log 2>&1; echo rc=$? >> $OUT/build-serve.log )
cp $W/rootfind/rootfind_serve_cuda $W/
$W/rootfind_serve_cuda --info > $OUT/serve-info.txt 2>&1
sha256sum $W/ecbench $W/rootfind_serve_cuda $W/rootfind/rootfind_serve > $OUT/sha256-ic.txt
log "serve: $(cat $OUT/serve-info.txt) build $(tail -1 $OUT/build-serve.log) test $(grep -c 'ALL PASS' $OUT/test-serve.log)"
$W/ecbench host > $OUT/host.json 2>&1
for s in smoke main; do
  log "ecbench run $s"
  timeout 14400 $W/ecbench run --spec $W/specs/$s.json --out $OUT/sessions/$s --cpus none --allow-busy \
    > $OUT/run-$s.log 2>&1; log "run $s rc=$?"
  timeout 14400 $W/ecbench verify --dir $OUT/sessions/$s --replay-all --exit-code \
    --out $OUT/audit-$s.json > $OUT/audit-$s.log 2>&1; log "audit $s rc=$? $(tail -1 $OUT/audit-$s.log)"
  $W/ecbench table --dir $OUT/sessions/$s > $OUT/table-$s.md 2>&1
  pack
done
for sz in 4096 16384; do
  $W/ecbench compare --dir $OUT/sessions/main --a rf2-cpu-F$sz --b rf2-gpu-F$sz > $OUT/compare-F$sz.txt 2>&1
  $W/ecbench compare --dir $OUT/sessions/main --a rho-neg --b rf2-gpu-F$sz > $OUT/compare-rho-F$sz.txt 2>&1
done
$W/ecbench compare --dir $OUT/sessions/main --a rho-neg --b rho-neg-aa > $OUT/compare-rho-aa.txt 2>&1
pack

# ---- Part 2 -----------------------------------------------------------------------------
# arm -> "PREFETCH L2"
declare -A V=([D]="0 0" [D2]="0 0" [P3]="3 0" [P4]="4 0" [P5]="5 0" [P4L2]="4 1")
ARMS="D P3 P4 P5 P4L2 D2"
build() {
  local a=$1; set -- ${V[$a]}
  rm -rf $W/b/$a; mkdir -p $W/b/$a; cp -r $W/rootfind/. $W/b/$a/
  ( cd $W/b/$a && make clean >/dev/null 2>&1
    make test RF_EYT_PREFETCH=$1 RF_EYT_PREFETCH_L2=$2 > $OUT/test-$a.log 2>&1; echo rc=$? >> $OUT/test-$a.log
    make -j2 bench ARCH=$ARCH RF_EYT_PREFETCH=$1 RF_EYT_PREFETCH_L2=$2 NVCCFLAGS="$FL" > $OUT/build-$a.log 2>&1
    echo rc=$? >> $OUT/build-$a.log )
}
for a in $ARMS; do build $a & done; wait
log "built: $(for a in $ARMS; do echo -n "$a:$(tail -1 $OUT/build-$a.log) test:$(grep -c 'ALL PASS' $OUT/test-$a.log) "; done)"
sha256sum $W/b/*/bench > $OUT/sha256-bench.txt
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
  set -- $OK; for i in $(seq 2 $r); do x=$1; shift; set -- "$@" $x; done
  for a in "$@"; do
    ( cd $W/b/$a && timeout 900 ./bench throughput > $OUT/tp-$a-$r.log 2>&1
      timeout 900 ./bench throughput 61 1024 262144 > $OUT/tpsmall-$a-$r.log 2>&1 )
  done
  log "round $r ($*)"
done
log "done"
pack
