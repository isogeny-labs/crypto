#!/usr/bin/env bash
# Final identity + timing check of the committed tree (/work/trees/final, git archive of HEAD):
# every directory built with its own Makefile defaults (plus ARCH), selftests, worker digests
# against the CPU, and 3 macaulay throughput runs at the default path. Writes /work/out_final.
set -uo pipefail
W=/work; OUT=$W/out_final; mkdir -p $OUT; I=$W/inputs; T=$W/trees/final/gpu
export CUDA_MODULE_LOADING=EAGER
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
cp $W/trees/final/REV $OUT/
( cd $T/spmv && make cuda-worker host-worker > $OUT/spmv-build.log 2>&1; echo rc=$? >> $OUT/spmv-build.log )
( cd $T/fes && make test > $OUT/fes-test.log 2>&1; echo rc=$? >> $OUT/fes-test.log; make cuda host-worker ARCH=$ARCH > $OUT/fes-build.log 2>&1; ./fes_cuda > $OUT/fes-selftest.log 2>&1 )
( cd $T/macaulay && make test > $OUT/mac-test.log 2>&1; echo rc=$? >> $OUT/mac-test.log; make bench ARCH=$ARCH > $OUT/mac-build.log 2>&1
  ./bench selftest 64 > $OUT/mac-selftest.log 2>&1; ./bench occupancy > $OUT/mac-occupancy.log 2>&1
  for r in 1 2 3; do ./bench throughput > $OUT/mac-tp-$r.log 2>&1; done )
( cd $T/semaev && make test > $OUT/semaev-test.log 2>&1; make bench2 ARCH=$ARCH > $OUT/semaev-build.log 2>&1; echo rc=$? >> $OUT/semaev-build.log; ./bench2 selftest > $OUT/semaev-selftest.log 2>&1 )
{
  echo "spmv cpu  $($T/spmv/spmv_solve --in $I/spmv.in | sha256sum | cut -c1-16)"
  echo "spmv cuda $($T/spmv/spmv_cuda --in $I/spmv.in | sha256sum | cut -c1-16)"
  for f in fes16 fes24m12 fes26 fes30; do
    echo "$f cpu  $($T/fes/fes_solve --in $I/$f.in | tail -n +2 | sort -n | sha256sum | cut -c1-16)"
    echo "$f cuda $($T/fes/fes_cuda --in $I/$f.in | tail -n +2 | sort -n | sha256sum | cut -c1-16)"
  done
} > $OUT/digests.txt 2>&1
cuobjdump --list-elf $T/spmv/spmv_cuda > $OUT/spmv-elf.txt 2>&1
cd $W && tar czf out_final.tgz out_final && sha256sum out_final.tgz > out_final.tgz.sha256 && echo FINAL_DONE
