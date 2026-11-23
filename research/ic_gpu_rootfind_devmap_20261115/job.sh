#!/usr/bin/env bash
# research/ic_gpu_rootfind_devmap_20261115. Runs on the GPU pod. /work/rdm holds: REV, ecbench (release binary of REV), rootfind/ (gpu/rootfind
# sources of REV), specs/. Writes /work/out_rdm.   job.sh
# Builds rootfind_serve_cuda next to ecbench (worker=spawn arms find it there), starts
# `rootfind_serve_cuda --daemon /work/rdm/rootfind_serve.sock` (worker=daemon arms find the socket
# there) and logs its start time, then runs the smoke, rf3 and rf2d sessions, each replay-audited.
set -uo pipefail
W=/work/rdm; OUT=/work/out_rdm; mkdir -p $OUT
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
pack() { (cd /work && tar czf out_rdm.tgz out_rdm && sha256sum out_rdm.tgz > out_rdm.tgz.sha256); }
(cat /sys/fs/cgroup/cpu.max; cat /sys/fs/cgroup/cpu/cpu.cfs_quota_us /sys/fs/cgroup/cpu/cpu.cfs_period_us) > $OUT/cgroup-cpu.txt 2>&1; nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt; nproc > $OUT/nproc.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
cp $W/REV $OUT/
FL="-O3 -std=c++17 -lineinfo -gencode arch=compute_$CC,code=$ARCH -Xcompiler -pthread"
( cd $W/rootfind && make clean >/dev/null 2>&1; make test > $OUT/test-serve.log 2>&1; echo rc=$? >> $OUT/test-serve.log
  make -j4 serve serve-cuda ARCH=$ARCH NVCCFLAGS="$FL" > $OUT/build-serve.log 2>&1; echo rc=$? >> $OUT/build-serve.log )
cp $W/rootfind/rootfind_serve_cuda $W/
$W/rootfind_serve_cuda --info > $OUT/serve-info.txt 2>&1
$W/rootfind_serve_cuda --bench-pairs > $OUT/bench-pairs.txt 2>&1; BPRC=$?
log "bench-pairs rc=$BPRC: $(tr '\n' ' ' < $OUT/bench-pairs.txt)"
[ $BPRC -eq 0 ] || { log "ABORT: pair kernels disagree or failed"; pack; exit 1; }
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
for s in smoke rf3devmap; do
  log "ecbench run $s"
  timeout 14400 $W/ecbench run --spec $W/specs/$s.json --out $OUT/sessions/$s --cpus none --allow-busy \
    > $OUT/run-$s.log 2>&1; log "run $s rc=$? $(tail -1 $OUT/run-$s.log)"
  timeout 14400 $W/ecbench verify --dir $OUT/sessions/$s --replay-all --exit-code \
    --out $OUT/audit-$s.json > $OUT/audit-$s.log 2>&1; log "audit $s rc=$? $(tail -1 $OUT/audit-$s.log)"
  $W/ecbench table --dir $OUT/sessions/$s > $OUT/table-$s.md 2>&1
  pack
done
$W/ecbench compare --dir $OUT/sessions/rf3devmap --a rf3-gpup-echelon-F4096 --b rf3-gpup-echelon-aa-F4096 > $OUT/compare-aa.txt 2>&1
$W/ecbench band --dir $OUT/sessions/rf3devmap --json $OUT/band-rf3devmap.json > $OUT/band-rf3devmap.txt 2>&1
kill $DPID; wait $DPID 2>/dev/null
log "done"
pack
