#!/usr/bin/env bash
# research/ic_gpu_rootfind_la_20261115. Runs on the GPU pod. /work/rla holds: REV, ecbench (release binary of REV), rootfind/ (gpu/rootfind
# sources of REV), specs/. Writes /work/out_rla.   job.sh
# Builds rootfind_serve_cuda next to ecbench (worker=spawn arms find it there), starts
# `rootfind_serve_cuda --daemon /work/rla/rootfind_serve.sock` (worker=daemon arms find the socket
# there) and logs its start time, then runs the smoke, rf3 and rf2d sessions, each replay-audited.
set -uo pipefail
W=/work/rla; OUT=/work/out_rla; mkdir -p $OUT
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
pack() { (cd /work && tar czf out_rla.tgz out_rla && sha256sum out_rla.tgz > out_rla.tgz.sha256); }
(cat /sys/fs/cgroup/cpu.max; cat /sys/fs/cgroup/cpu/cpu.cfs_quota_us /sys/fs/cgroup/cpu/cpu.cfs_period_us) > $OUT/cgroup-cpu.txt 2>&1; nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt; nproc > $OUT/nproc.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
cp $W/REV $OUT/
FL="-O3 -std=c++17 -lineinfo -gencode arch=compute_$CC,code=$ARCH -Xcompiler -pthread"
( cd $W/rootfind && make clean >/dev/null 2>&1; make test > $OUT/test-serve.log 2>&1; echo rc=$? >> $OUT/test-serve.log
  make -j4 serve serve-cuda ARCH=$ARCH NVCCFLAGS="$FL" > $OUT/build-serve.log 2>&1; echo rc=$? >> $OUT/build-serve.log )
cp $W/rootfind/rootfind_serve_cuda $W/
$W/rootfind_serve_cuda --info > $OUT/serve-info.txt 2>&1
sha256sum $W/ecbench $W/rootfind_serve_cuda $W/rootfind/rootfind_serve > $OUT/sha256.txt
log "serve: $(cat $OUT/serve-info.txt) build $(tail -1 $OUT/build-serve.log) test $(grep -c 'ALL PASS' $OUT/test-serve.log)"
# The daemon: started once, timed from launch to listening (its log line says so).
T0=$(date +%s.%N)
rm -f $W/rootfind_serve.sock
RF_SERVE_TIMING=1 $W/rootfind_serve_cuda --daemon $W/rootfind_serve.sock 2> $OUT/daemon.log &
DPID=$!
until grep -q listening $OUT/daemon.log 2>/dev/null || ! kill -0 $DPID 2>/dev/null; do sleep 0.01; done
T1=$(date +%s.%N)
echo "daemon start to socket: $(python3 -c "print(round($T1 - $T0, 3))") s" >> $OUT/daemon-start.txt
log "daemon pid $DPID up: $(cat $OUT/daemon-start.txt); $(head -2 $OUT/daemon.log | tr '\n' ' ')"
# The backend is not in the records: require the CUDA backend here, in the daemon and the spawned worker.
grep -q "cuda: " $OUT/daemon.log && grep -q "^cuda: " $OUT/serve-info.txt || { log "ABORT: worker not on the CUDA backend"; kill $DPID 2>/dev/null; pack; exit 1; }
$W/ecbench host > $OUT/host.json 2>&1
for s in smoke rf3la; do
  log "ecbench run $s"
  timeout 14400 $W/ecbench run --spec $W/specs/$s.json --out $OUT/sessions/$s --cpus none --allow-busy \
    > $OUT/run-$s.log 2>&1; log "run $s rc=$? $(tail -1 $OUT/run-$s.log)"
  timeout 14400 $W/ecbench verify --dir $OUT/sessions/$s --replay-all --exit-code \
    --out $OUT/audit-$s.json > $OUT/audit-$s.log 2>&1; log "audit $s rc=$? $(tail -1 $OUT/audit-$s.log)"
  $W/ecbench table --dir $OUT/sessions/$s > $OUT/table-$s.md 2>&1
  pack
done
for sz in 2048 4096; do
  $W/ecbench compare --dir $OUT/sessions/rf3la --a rf3-gpup-gauss-F$sz --b rf3-gpup-echelon-F$sz > $OUT/compare-gpu-F$sz.txt 2>&1
  $W/ecbench compare --dir $OUT/sessions/rf3la --a rf3-cpu-gauss-F$sz --b rf3-cpu-echelon-F$sz > $OUT/compare-cpu-F$sz.txt 2>&1
done
$W/ecbench compare --dir $OUT/sessions/rf3la --a rf3-gpup-echelon-F4096 --b rf3-gpup-echelon-aa-F4096 > $OUT/compare-aa.txt 2>&1
kill $DPID; wait $DPID 2>/dev/null
log "done"
pack
