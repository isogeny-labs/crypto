#!/usr/bin/env bash
# Round 2 (2026-11-13), runs on the GPU pod. Trees: /work/trees/main (git archive of origin/main
# c5678a98ae) and /work/trees/new (this branch). Writes /work/out2. Every arm is checked for identity
# before it is timed; every comparison has an A/A arm.
#   FES: M (main fes_cuda = Gray walk), G (new, FES_WALK=1 Gray, A/A), B4..B8 (new, block walk K).
#   macaulay: M (main bench, shared u16 1024 thr), MA (main bench again, A/A), D1024/D512 (new, MAC_UPDATE_2D=1).
#   rootfind: launch knobs on main's source: R128_0 (default), R128_0b (A/A), R64_0, R256_0, R128_2.
#   job_round2.sh ROUNDS
set -uo pipefail
ROUNDS=${1:-5}
W=/work; OUT=$W/out2; mkdir -p $OUT; I=$W/inputs2; mkdir -p $I
export CUDA_MODULE_LOADING=EAGER
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
nvidia-smi --query-gpu=name,compute_cap,driver_version --format=csv > $OUT/gpu.csv
NV="-O3 -std=c++17 -arch=$ARCH -lineinfo"
shim() {
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

# ---------------- rootfind builds first (slow, background) ----------------
RFB=$W/rf2; mkdir -p $RFB
rf_build() {  # rf_build NAME BLOCK MINB
  rm -rf $RFB/$1; cp -r $W/trees/main/gpu/rootfind $RFB/$1
  ( cd $RFB/$1 && make clean >/dev/null 2>&1; make -j2 bench ARCH=$ARCH RF_BLOCK=$2 RF_MIN_BLOCKS=$3 \
      NVCCFLAGS="-O3 -std=c++17 -lineinfo -gencode arch=compute_$CC,code=$ARCH -Xcompiler -pthread -Xptxas -v" \
      > $OUT/rf-build-$1.log 2>&1 || echo BUILD_FAILED >> $OUT/rf-build-$1.log )
}
( rf_build R128_0 128 0 & rf_build R128_0b 128 0 & rf_build R64_0 64 0 & rf_build R256_0 256 0 & rf_build R128_2 128 2 & wait; echo built > $RFB/BUILT ) &
RF_PID=$!

# ---------------- FES ----------------
FM=$W/trees/main/gpu/fes; FN=$W/trees/new/gpu/fes
shim $FM/fes.cu; shim $FN/fes.cu
( cd $FM && make cuda host-worker ARCH=$ARCH > $OUT/fes-build-M.log 2>&1 )
( cd $FN && make test > $OUT/fes-test.log 2>&1 && make test N=16 M=3 SEED=11 >> $OUT/fes-test.log 2>&1; echo rc=$? >> $OUT/fes-test.log
  make gen >/dev/null 2>&1
  nvcc $NV -Xptxas -v -DFES_WALK=1 -o fes_G fes.cu > $OUT/fes-build-G.log 2>&1
  for k in 4 5 6 7 8; do nvcc $NV -Xptxas -v -DFES_WALK=2 -DFES_BLOCK_BITS=$k -o fes_B$k fes.cu > $OUT/fes-build-B$k.log 2>&1; done )
declare -A FB=([M]=$FM/fes_cuda [G]=$FN/fes_G [B4]=$FN/fes_B4 [B5]=$FN/fes_B5 [B6]=$FN/fes_B6 [B7]=$FN/fes_B7 [B8]=$FN/fes_B8)
FARMS="M B6 G B4 B5 B7 B8"
gen() { awk -v n=$1 -v m=$2 -v s=$3 'BEGIN{srand(s); printf "%d %d\n%d\n", n, m, int(rand()*2^m);
  for(i=0;i<n;i++) printf "%d ", int(rand()*2^m); print "";
  for(i=0;i<n*(n+1)/2;i++) printf "%d ", int(rand()*2^m); print ""}'; }
gen 16 3 11 > $I/fes16.in; gen 24 12 5 > $I/fes24m12.in; gen 26 28 1 > $I/fes26.in
gen 30 28 2 > $I/fes30.in; gen 32 28 3 > $I/fes32.in; gen 28 16 7 > $I/fes28m16.in
sha256sum $I/* > $OUT/inputs.sha256
log "fes built; identity"
{
  for a in $FARMS; do echo "selftest $a: $(cd $FN && ${FB[$a]} 2>/dev/null | tail -1)"; done
  for f in fes16 fes24m12 fes28m16 fes26 fes30; do
    c=$($FM/fes_solve --in $I/$f.in | tail -n +2 | sort -n | sha256sum | cut -c1-16)
    echo "$f cpu $c n=$($FM/fes_solve --in $I/$f.in | tail -n +2 | wc -l)"
    for a in $FARMS; do o=$(timeout 600 ${FB[$a]} --in $I/$f.in 2>/dev/null); d=$(echo "$o" | tail -n +2 | sort -n | sha256sum | cut -c1-16)
      echo "$f $a $d n=$(echo "$o" | tail -n +2 | wc -l) $([ "$d" = "$c" ] && echo SAME || echo DIFF)"; done
  done
  # fes32: no CPU reference (2^32 points on one core); arms must agree with each other
  for a in $FARMS; do echo "fes32 $a $(timeout 600 ${FB[$a]} --in $I/fes32.in 2>/dev/null | tail -n +2 | sort -n | sha256sum | cut -c1-16)"; done
} > $OUT/fes-identity.txt 2>&1
log "fes identity: $(grep -c SAME $OUT/fes-identity.txt) SAME, $(grep -c DIFF $OUT/fes-identity.txt) DIFF"
for r in $(seq 1 $ROUNDS); do
  for a in $FARMS; do
    for f in fes24m12 fes26 fes28m16 fes30 fes32; do
      { timeout 300 ${FB[$a]} --in $I/$f.in > /dev/null; } 2>&1 | grep KERNEL_MS | sed "s/^/$f $a $r /" >> $OUT/fes-kernel_ms.txt
    done
  done
  log "fes round $r"
done

# ---------------- macaulay ----------------
MM=$W/trees/main/gpu/macaulay; MN=$W/trees/new/gpu/macaulay
( cd $MM && make bench ARCH=$ARCH NVCCFLAGS="$NV -Xptxas -v" > $OUT/mac-build-M.log 2>&1 && cp bench bench_M && cp bench bench_MA )
( cd $MN && make test > $OUT/mac-test.log 2>&1; echo rc=$? >> $OUT/mac-test.log
  make bench ARCH=$ARCH NVCCFLAGS="$NV -Xptxas -v -DMAC_UPDATE_2D=1" > $OUT/mac-build-D.log 2>&1 && cp bench bench_D )
rm -rf $W/mac2_small; cp -r $MN $W/mac2_small
( cd $W/mac2_small && make clean >/dev/null && make bench P=271 ROWS=64 COLS=80 ARCH=$ARCH NVCCFLAGS="$NV -DMAC_UPDATE_2D=1" > $OUT/mac-build-small.log 2>&1 )
marm() { case $1 in M) (cd $MM && ./bench_M "${@:2}");; MA) (cd $MM && ./bench_MA "${@:2}");;
  D1024) (cd $MN && MAC_SMEM_THREADS=1024 ./bench_D "${@:2}");; D512) (cd $MN && MAC_SMEM_THREADS=512 ./bench_D "${@:2}");; esac; }
for a in M D1024 D512; do marm $a selftest 64 > $OUT/mac-selftest-$a.log 2>&1; log "mac selftest $a: $(grep -c 'selftest ok' $OUT/mac-selftest-$a.log)"; done
for e in 1024 512; do (cd $W/mac2_small && MAC_SMEM_THREADS=$e ./bench selftest 64 > $OUT/mac-selftest-small-$e.log 2>&1); log "mac selftest small $e: $(grep -c 'selftest ok' $OUT/mac-selftest-small-$e.log)"; done
for r in $(seq 1 $ROUNDS); do for a in M D1024 MA D512; do marm $a throughput > $OUT/mac-tp-$a-$r.log 2>&1; done; log "mac round $r"; done

# ---------------- rootfind launch knobs ----------------
wait $RF_PID; log "rootfind built: $(grep -l BUILD_FAILED $OUT/rf-build-*.log | wc -l) failed"
RARMS="R128_0 R64_0 R256_0 R128_2 R128_0b"
for a in $RARMS; do (cd $RFB/$a && timeout 900 ./bench selftest > $OUT/rf-selftest-$a.log 2>&1; timeout 900 ./bench p256 >> $OUT/rf-selftest-$a.log 2>&1)
  log "rf selftest $a: $(grep -c 'SELFTEST PASSED' $OUT/rf-selftest-$a.log)/2"; done
for r in $(seq 1 $ROUNDS); do for a in $RARMS; do
  (cd $RFB/$a && timeout 900 ./bench throughput > $OUT/rf-tp-$a-$r.log 2>&1; timeout 900 ./bench throughput 61 1024 262144 > $OUT/rf-tpsmall-$a-$r.log 2>&1)
done; log "rf round $r"; done
# profiler availability (explains, never proves): one kernel of the default build
( cd $RFB/R128_0 && timeout 600 ncu --launch-count 1 --section SpeedOfLight --section Occupancy --section LaunchStats \
    -k regex:k_decompose_sorted ./bench throughput 61 65536 65536 > $OUT/rf-ncu.txt 2>&1; echo rc=$? >> $OUT/rf-ncu.txt )
log "ncu: $(tail -1 $OUT/rf-ncu.txt) $(grep -c 'ERR_NVGPUCTRPERM' $OUT/rf-ncu.txt) perm-errors"
nvidia-smi -q -d CLOCK,TEMPERATURE,POWER,PERFORMANCE > $OUT/nvidia-smi-after.txt 2>&1
log "done"
cd $W && tar czf out2.tgz out2 && sha256sum out2.tgz > out2.tgz.sha256
