#!/usr/bin/env bash
# Runs on the GPU pod. End-to-end IC solves (`ic bench --sweep`) with the descent oracle's solver
# set to inherited-f4, fes-f2 and fes-f2-gpu. /work/e2e holds: ic (release binary of the commit in
# REV), gpu/fes (sources), sweeps/. Writes /work/out_e2e.   job_e2e.sh
set -uo pipefail
W=/work/e2e; OUT=/work/out_e2e; mkdir -p $OUT
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
nvidia-smi -q > $OUT/nvidia-smi-q.txt 2>&1; nvcc --version > $OUT/nvcc.txt; lscpu > $OUT/lscpu.txt; nproc > $OUT/nproc.txt
CC=$(nvidia-smi --query-gpu=compute_cap --format=csv,noheader | head -1 | tr -d .); ARCH=sm_$CC
cp $W/REV $OUT/; sha256sum $W/ic > $OUT/ic.sha256
( cd $W/gpu/fes && make cuda host-worker ARCH=$ARCH > $OUT/fes-build.log 2>&1 && ./fes_cuda > $OUT/fes-selftest.log 2>&1 )
sha256sum $W/gpu/fes/fes_cuda >> $OUT/ic.sha256
# serve smoke: the same two requests to the CUDA and host workers must give the same replies
printf '3 2\n0\n1 2 0\n0 0 0 0 0 3\n2 1\n1\n1 1\n0 0 0\n' > $OUT/serve-smoke.in
$W/gpu/fes/fes_cuda --serve < $OUT/serve-smoke.in > $OUT/serve-smoke-cuda.txt 2>&1
$W/gpu/fes/fes_solve --serve < $OUT/serve-smoke.in > $OUT/serve-smoke-host.txt 2>&1
cmp -s $OUT/serve-smoke-cuda.txt $OUT/serve-smoke-host.txt && log "serve smoke: cuda == host" || log "serve smoke: DIFFER"
export ICX_FES_WORKER=$W/gpu/fes/fes_cuda
declare -A TMO=([koblitz-n23]=2400 [char2-n25]=3600 [char2-n29]=5400)
for s in koblitz-n23 char2-n25 char2-n29; do
  log "start $s"
  timeout ${TMO[$s]} $W/ic bench --json --sweep $W/sweeps/$s.json --out $OUT/$s.json \
    --solver-budget-seconds 120 --rho-runs 16 --max-trials 20000000 > $OUT/$s.log 2>&1
  log "end $s rc=$? $(grep -c '"verified": true' $OUT/$s.json 2>/dev/null) verified-true"
  cd /work && tar czf out_e2e.tgz out_e2e && sha256sum out_e2e.tgz > out_e2e.tgz.sha256; cd - >/dev/null
done
log "done"
