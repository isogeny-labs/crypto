#!/usr/bin/env bash
# Round-2 final check of the committed tree (/work/trees/fin): Makefile defaults (plus ARCH),
# host tests, device selftests, FES digests against the CPU (and main on fes32), 3 macaulay runs.
set -uo pipefail
W=/work; OUT=$W/out_fin2; mkdir -p $OUT; I=$W/inputs2; T=$W/trees/fin/gpu
export CUDA_MODULE_LOADING=EAGER
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
cp $W/trees/fin/REV $OUT/
( cd $T/fes && make test > $OUT/fes-test.log 2>&1; echo rc=$? >> $OUT/fes-test.log; make cuda host-worker ARCH=$ARCH > $OUT/fes-build.log 2>&1; ./fes_cuda > $OUT/fes-selftest.log 2>&1 )
( cd $T/macaulay && make test > $OUT/mac-test.log 2>&1; echo rc=$? >> $OUT/mac-test.log; make bench ARCH=$ARCH > $OUT/mac-build.log 2>&1
  ./bench selftest 64 > $OUT/mac-selftest.log 2>&1; ./bench occupancy > $OUT/mac-occupancy.log 2>&1
  MAC_SMEM=0 ./bench selftest 64 > $OUT/mac-selftest-global.log 2>&1
  for r in 1 2 3; do ./bench throughput > $OUT/mac-tp-$r.log 2>&1; done )
{
  for f in fes16 fes24m12 fes28m16 fes26 fes30; do
    echo "$f cpu  $($T/fes/fes_solve --in $I/$f.in | tail -n +2 | sort -n | sha256sum | cut -c1-16)"
    echo "$f cuda $($T/fes/fes_cuda --in $I/$f.in | tail -n +2 | sort -n | sha256sum | cut -c1-16)"
  done
  echo "fes32 main $($W/trees/main/gpu/fes/fes_cuda --in $I/fes32.in 2>/dev/null | tail -n +2 | sort -n | sha256sum | cut -c1-16)"
  echo "fes32 cuda $($T/fes/fes_cuda --in $I/fes32.in | tail -n +2 | sort -n | sha256sum | cut -c1-16)"
} > $OUT/digests.txt 2>&1
cd $W && tar czf out_fin2.tgz out_fin2 && sha256sum out_fin2.tgz > out_fin2.tgz.sha256 && echo FINAL_DONE
