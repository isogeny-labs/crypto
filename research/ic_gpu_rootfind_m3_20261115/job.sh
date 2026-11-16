#!/usr/bin/env bash
# Runs on the GPU pod. /work/rm3 holds: REV, ecbench (release binary of REV), rootfind/ (gpu/rootfind
# sources of REV), specs/. Writes /work/out_rm3.   job.sh
# Builds rootfind_serve_cuda next to ecbench (worker=spawn arms find it there), starts
# `rootfind_serve_cuda --daemon /work/rm3/rootfind_serve.sock` (worker=daemon arms find the socket
# there) and logs its start time, then runs the smoke, rf3 and rf2d sessions, each replay-audited.
set -uo pipefail
W=/work/rm3; OUT=/work/out_rm3; mkdir -p $OUT
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
pack() { (cd /work && tar czf out_rm3.tgz out_rm3 && sha256sum out_rm3.tgz > out_rm3.tgz.sha256); }
nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt; nproc > $OUT/nproc.txt
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
$W/rootfind_serve_cuda --daemon $W/rootfind_serve.sock 2> $OUT/daemon.log &
DPID=$!
until [ -S $W/rootfind_serve.sock ] || ! kill -0 $DPID 2>/dev/null; do sleep 0.01; done
T1=$(date +%s.%N)
echo "daemon start to socket: $(python3 -c "print(round($T1 - $T0, 3))") s" >> $OUT/daemon-start.txt
log "daemon pid $DPID up: $(cat $OUT/daemon-start.txt); $(head -2 $OUT/daemon.log | tr '\n' ' ')"
# The backend is not in the records: require the CUDA backend here, in the daemon and the spawned worker.
grep -q "cuda: " $OUT/daemon.log && grep -q "^cuda: " $OUT/serve-info.txt || { log "ABORT: worker not on the CUDA backend"; pack; exit 1; }
$W/ecbench host > $OUT/host.json 2>&1
for s in smoke rf3 rf2d; do
  log "ecbench run $s"
  timeout 14400 $W/ecbench run --spec $W/specs/$s.json --out $OUT/sessions/$s --cpus none --allow-busy \
    > $OUT/run-$s.log 2>&1; log "run $s rc=$? $(tail -1 $OUT/run-$s.log)"
  timeout 14400 $W/ecbench verify --dir $OUT/sessions/$s --replay-all --exit-code \
    --out $OUT/audit-$s.json > $OUT/audit-$s.log 2>&1; log "audit $s rc=$? $(tail -1 $OUT/audit-$s.log)"
  $W/ecbench table --dir $OUT/sessions/$s > $OUT/table-$s.md 2>&1
  pack
done
for sz in 1024 2048; do
  $W/ecbench compare --dir $OUT/sessions/rf3 --a rf3-cpu-F$sz --b rf3-gpu-F$sz > $OUT/compare-rf3-gpu-F$sz.txt 2>&1
  $W/ecbench compare --dir $OUT/sessions/rf3 --a rf3-cpu-F$sz --b rf3-gpud-F$sz > $OUT/compare-rf3-gpud-F$sz.txt 2>&1
done
$W/ecbench compare --dir $OUT/sessions/rf2d --a rf2-gpu-F4096 --b rf2-gpud-F4096 > $OUT/compare-rf2-spawn-daemon.txt 2>&1
kill $DPID; wait $DPID 2>/dev/null
log "done"
pack
