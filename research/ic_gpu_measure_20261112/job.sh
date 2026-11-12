#!/usr/bin/env bash
# Runs on the GPU pod. Implements cards S, F, M, R and Z of docs/perf/20261112-ic-gpu-audit.md
# for two source trees, /work/trees/base and /work/trees/cand (git archives of gpu/), plus an
# A/A arm (a second build of base, /work/trees/base2). Writes everything under /work/out.
#   job.sh ROUNDS
# Timing shim: spmv.cu and fes.cu print nothing about time, so every arm's working copy gets the
# same two inserted lines (steady_clock before the launch, print after the cudaDeviceSynchronize
# that follows it). The shim is identical in every arm and is applied to the copy, not the tree.
set -uo pipefail
ROUNDS=${1:-5}
W=/work; OUT=$W/out; mkdir -p $OUT
export CUDA_MODULE_LOADING=EAGER
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }

nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1
nvidia-smi --query-gpu=name,compute_cap,driver_version,clocks.max.sm,clocks.max.mem --format=csv > $OUT/gpu.csv
nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt; uname -a > $OUT/uname.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .)
ARCH=sm_$CC; echo $ARCH > $OUT/arch.txt; log "arch $ARCH"
NV="-O3 -std=c++17 -arch=$ARCH -lineinfo"

shim() {  # shim FILE  -> insert the timing lines around the first <<< launch
  python3 - "$1" <<'P'
import re,sys
p=sys.argv[1]; s=open(p).read().split('\n')
i=next(k for k,l in enumerate(s) if '<<<' in l)
j=next(k for k in range(i,len(s)) if 'cudaDeviceSynchronize' in s[k])
s.insert(j+1,'    { auto __icg_t1 = std::chrono::steady_clock::now(); fprintf(stderr, "KERNEL_MS %.4f\\n", std::chrono::duration<double, std::milli>(__icg_t1 - __icg_t0).count()); }')
s.insert(i,'    auto __icg_t0 = std::chrono::steady_clock::now();')
s.insert(0,'#include <chrono>\n#include <cstdio>')
open(p,'w').write('\n'.join(s))
P
}

for T in base base2 cand; do
  D=$W/trees/$T/gpu; L=$OUT/build-$T; mkdir -p $L
  shim $D/spmv/spmv.cu; shim $D/fes/fes.cu
  log "build $T"
  ( cd $D/spmv && make host-worker cuda-worker NVCCFLAGS="$NV" > $L/spmv.log 2>&1 && \
    nvcc $NV -Xptxas -v -c spmv.cu -o /dev/null > $L/spmv.ptxas 2>&1 ) || log "spmv build FAILED $T"
  ( cd $D/fes && make host-worker cuda ARCH=$ARCH > $L/fes.log 2>&1 && \
    make nvcc-check ARCH=$ARCH NVCCFLAGS="$NV -Xptxas -v" > $L/fes.ptxas 2>&1 ) || log "fes build FAILED $T"
  ( cd $D/macaulay && make bench ARCH=$ARCH NVCCFLAGS="$NV -Xptxas -v" > $L/macaulay.log 2>&1 ) || log "macaulay build FAILED $T"
  ( cd $D/rootfind && make bench ARCH=$ARCH > $L/rootfind.log 2>&1 && \
    make ptxas-info ARCH=$ARCH > $L/rootfind.ptxas 2>&1 ) || log "rootfind build FAILED $T"
done
# semaev is unchanged between base and cand: one tree, default build (clmad) and the portable one.
( cd $W/trees/cand/gpu/semaev && make bench2 ARCH=$ARCH > $OUT/semaev-default-build.log 2>&1 \
  && cp bench2 bench2_default; make clean >/dev/null; \
  make bench2 ARCH=$ARCH NVCCFLAGS="$NV -DGF2N_CLMAD=0 -Xptxas -v" > $OUT/semaev-portable-build.log 2>&1 \
  && cp bench2 bench2_portable ) ; log "semaev builds done"

# ---------- identity ----------
C=$OUT/identity; mkdir -p $C
log "host oracles"
( cd $W/trees/cand/gpu/spmv && make oracle-check > $C/spmv-oracle.log 2>&1; ./spmv-oracle-check >> $C/spmv-oracle.log 2>&1 )
for t in cand base; do
  ( cd $W/trees/$t/gpu/fes && make test > $C/fes-test-$t.log 2>&1 && make test N=16 M=3 SEED=11 >> $C/fes-test-$t.log 2>&1; echo rc=$? >> $C/fes-test-$t.log )
  ( cd $W/trees/$t/gpu/macaulay && make test > $C/macaulay-test-$t.log 2>&1; echo rc=$? >> $C/macaulay-test-$t.log )
  ( cd $W/trees/$t/gpu/rootfind && make test > $C/rootfind-test-$t.log 2>&1; echo rc=$? >> $C/rootfind-test-$t.log )
done
( cd $W/trees/cand/gpu/semaev && make test > $C/semaev-test.log 2>&1; echo rc=$? >> $C/semaev-test.log )

log "device selftests"
for t in base cand; do
  G=$W/trees/$t/gpu
  ( cd $G/fes && timeout 600 ./fes_cuda > $C/fes-selftest-$t.log 2>&1; echo rc=$? >> $C/fes-selftest-$t.log )
  ( cd $G/macaulay && timeout 600 ./bench selftest 64 > $C/macaulay-selftest-$t.log 2>&1; echo rc=$? >> $C/macaulay-selftest-$t.log
    ./bench occupancy > $C/macaulay-occupancy-$t.log 2>&1 )
  ( cd $G/rootfind && timeout 900 ./bench selftest > $C/rootfind-selftest-$t.log 2>&1; echo rc=$? >> $C/rootfind-selftest-$t.log
    timeout 900 ./bench p256 > $C/rootfind-p256-$t.log 2>&1; echo rc=$? >> $C/rootfind-p256-$t.log )
done
for v in default portable; do
  B=$W/trees/cand/gpu/semaev/bench2_$v; [ -x $B ] || continue
  ( cd $W/trees/cand/gpu/semaev && timeout 900 $B selftest > $C/semaev-selftest-$v.log 2>&1; echo rc=$? >> $C/semaev-selftest-$v.log
    $B occupancy > $C/semaev-occupancy-$v.log 2>&1 )
done

log "workload inputs"
I=$W/inputs; mkdir -p $I
awk 'BEGIN{srand(1); r=65536; c=65536; l=32; m="2305843009213693951"; z=32;
  printf "SPMV1 %d %d %d %s %d\n", r, c, l, m, r*z;
  for(i=0;i<=r;i++) printf "%d ", i*z; print "";
  for(i=0;i<r*z;i++) printf "%d ", int(rand()*c); print "";
  for(i=0;i<r*z;i++) printf "%.0f ", int(rand()*2^52); print "";
  for(i=0;i<c*l;i++) printf "%.0f ", int(rand()*2^52); print ""}' > $I/spmv.in
gen() { awk -v n=$1 -v m=$2 -v s=$3 'BEGIN{srand(s); printf "%d %d\n%d\n", n, m, int(rand()*2^m);
  for(i=0;i<n;i++) printf "%d ", int(rand()*2^m); print "";
  for(i=0;i<n*(n+1)/2;i++) printf "%d ", int(rand()*2^m); print ""}'; }
gen 26 28 1 > $I/fes26.in; gen 16 3 11 > $I/fes16.in
sha256sum $I/* > $OUT/inputs.sha256

log "worker digests"
{
  echo "spmv cpu  $(timeout 900 $W/trees/base/gpu/spmv/spmv_solve --in $I/spmv.in | sha256sum)"
  for t in base cand; do echo "spmv $t $(timeout 300 $W/trees/$t/gpu/spmv/spmv_cuda --in $I/spmv.in 2>/dev/null | sha256sum)"; done
  for f in fes26 fes16; do
    echo "$f cpu  $(timeout 900 $W/trees/base/gpu/fes/fes_solve --in $I/$f.in | tail -n +2 | sort -n | sha256sum) n=$($W/trees/base/gpu/fes/fes_solve --in $I/$f.in | tail -n +2 | wc -l)"
    for t in base cand; do
      o=$(timeout 600 $W/trees/$t/gpu/fes/fes_cuda --in $I/$f.in 2>/dev/null); echo "$f $t $(echo "$o" | tail -n +2 | sort -n | sha256sum) n=$(echo "$o" | tail -n +2 | wc -l) head=$(echo "$o" | head -1)"
    done
  done
} > $C/digests.txt 2>&1

# ---------- timing: A/A (base vs base2) then A/B (base vs cand), alternating ----------
TM=$OUT/timing; mkdir -p $TM
run_arm() {  # run_arm ARM TREE ROUND
  local a=$1 t=$2 r=$3 G=$W/trees/$2/gpu
  { timeout 300 $G/spmv/spmv_cuda --in $I/spmv.in > /dev/null; } 2>&1 | grep KERNEL_MS | sed "s/^/spmv $a $r /" >> $TM/kernel_ms.txt
  { timeout 300 $G/fes/fes_cuda --in $I/fes26.in > /dev/null; } 2>&1 | grep KERNEL_MS | sed "s/^/fes26 $a $r /" >> $TM/kernel_ms.txt
  { timeout 300 $G/fes/fes_cuda --in $I/fes26.in --low-bits 10 > /dev/null; } 2>&1 | grep KERNEL_MS | sed "s/^/fes26lb10 $a $r /" >> $TM/kernel_ms.txt
  ( cd $G/macaulay && timeout 900 ./bench throughput > $TM/macaulay-$a-$r.log 2>&1 )
  ( cd $G/rootfind && timeout 900 ./bench throughput > $TM/rootfind-$a-$r.log 2>&1 )
}
for r in $(seq 1 $ROUNDS); do log "A/A round $r"; run_arm A base $r; run_arm A2 base2 $r; done
for r in $(seq 1 $ROUNDS); do log "A/B round $r"; run_arm A base $r; run_arm B cand $r; done
for v in default portable; do
  B=$W/trees/cand/gpu/semaev/bench2_$v; [ -x $B ] || continue
  for r in 1 2 3; do ( cd $W/trees/cand/gpu/semaev && timeout 900 $B throughput > $TM/semaev-$v-$r.log 2>&1 ); done
done
nvidia-smi -q -d CLOCK,TEMPERATURE,POWER,PERFORMANCE > $OUT/nvidia-smi-after.txt 2>&1
log "done"
cd $W && tar czf out.tgz out && sha256sum out.tgz > out.tgz.sha256
