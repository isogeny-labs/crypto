#!/usr/bin/env bash
# Runs on the GPU pod after job.sh (needs /work/trees/cand and /work/inputs). FES O(1) Gray walk
# (/work/trees/fes2/gpu/fes) against main's symmetric walk, plus the semaev default-build check
# (/work/trees/fes2/gpu/semaev). Arms: M (main fes_cuda, shim from job.sh), S (new tree,
# -DFES_WALK_SYM=1: main's walk, A/A control), G (new tree, Gray walk). Writes /work/out_fes.
#   job_fes.sh ROUNDS
set -uo pipefail
ROUNDS=${1:-5}
W=/work; OUT=$W/out_fes; mkdir -p $OUT; I=$W/inputs
export CUDA_MODULE_LOADING=EAGER
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
NV="-O3 -std=c++17 -arch=$ARCH -lineinfo"
shim() {  # same timing lines as job.sh
  python3 - "$1" <<'P'
import sys
p=sys.argv[1]; s=open(p).read()
if '__icg_t0' in s: sys.exit(0)
s=s.split('\n')
i=next(k for k,l in enumerate(s) if '<<<' in l)
j=next(k for k in range(i,len(s)) if 'cudaDeviceSynchronize' in s[k])
s.insert(j+1,'    { auto __icg_t1 = std::chrono::steady_clock::now(); fprintf(stderr, "KERNEL_MS %.4f\\n", std::chrono::duration<double, std::milli>(__icg_t1 - __icg_t0).count()); }')
s.insert(i,'    auto __icg_t0 = std::chrono::steady_clock::now();')
s.insert(0,'#include <chrono>\n#include <cstdio>')
open(p,'w').write('\n'.join(s))
P
}
F=$W/trees/fes2/gpu/fes; shim $F/fes.cu
( cd $F && make test > $OUT/test.log 2>&1 && make test N=16 M=3 SEED=11 >> $OUT/test.log 2>&1; echo rc=$? >> $OUT/test.log
  make clean >/dev/null; make gen >/dev/null 2>&1
  nvcc $NV -Xptxas -v -DFES_WALK_SYM=1 -o fes_cuda_sym fes.cu > $OUT/build-S.log 2>&1
  nvcc $NV -Xptxas -v -o fes_cuda_gray fes.cu > $OUT/build-G.log 2>&1 )
declare -A BIN=([M]=$W/trees/cand/gpu/fes/fes_cuda [S]=$F/fes_cuda_sym [G]=$F/fes_cuda_gray)
gen() { awk -v n=$1 -v m=$2 -v s=$3 'BEGIN{srand(s); printf "%d %d\n%d\n", n, m, int(rand()*2^m);
  for(i=0;i<n;i++) printf "%d ", int(rand()*2^m); print "";
  for(i=0;i<n*(n+1)/2;i++) printf "%d ", int(rand()*2^m); print ""}'; }
gen 30 28 2 > $I/fes30.in; gen 24 12 5 > $I/fes24m12.in
sha256sum $I/fes30.in $I/fes24m12.in >> $OUT/inputs.sha256
log "built"
{
  for a in M S G; do echo "selftest $a: $(cd $F && ${BIN[$a]} 2>/dev/null | tail -1)"; done
  for f in fes16 fes24m12 fes26; do
    echo "$f cpu $($W/trees/cand/gpu/fes/fes_solve --in $I/$f.in | tail -n +2 | sort -n | sha256sum | cut -c1-16) n=$($W/trees/cand/gpu/fes/fes_solve --in $I/$f.in | tail -n +2 | wc -l)"
    for a in M S G; do o=$(timeout 600 ${BIN[$a]} --in $I/$f.in 2>/dev/null); echo "$f $a $(echo "$o" | tail -n +2 | sort -n | sha256sum | cut -c1-16) n=$(echo "$o" | tail -n +2 | wc -l)"; done
  done
} > $OUT/identity.txt 2>&1
log "identity: $(tr '\n' ';' < $OUT/identity.txt)"
for r in $(seq 1 $ROUNDS); do
  for a in M G S; do
    for w in "fes26:" "fes26:--low-bits 10" "fes30:" "fes24m12:"; do
      f=${w%%:*}; extra=${w#*:}
      { timeout 300 ${BIN[$a]} --in $I/$f.in $extra > /dev/null; } 2>&1 | grep KERNEL_MS | sed "s/^/$f${extra:+lb10} $a $r /" >> $OUT/kernel_ms.txt
    done
  done
  log "round $r"
done
# semaev: the default build must now succeed on CUDA 12.x (portable path) and pass its selftest
Z=$W/trees/fes2/gpu/semaev
( cd $Z && make bench2 ARCH=$ARCH > $OUT/semaev-default-build.log 2>&1; echo rc=$? >> $OUT/semaev-default-build.log
  timeout 900 ./bench2 selftest > $OUT/semaev-selftest.log 2>&1; echo rc=$? >> $OUT/semaev-selftest.log )
log "semaev: build $(tail -1 $OUT/semaev-default-build.log) selftest $(grep -c 'selftest ok' $OUT/semaev-selftest.log)"
log "done"
cd $W && tar czf out_fes.tgz out_fes && sha256sum out_fes.tgz > out_fes.tgz.sha256
