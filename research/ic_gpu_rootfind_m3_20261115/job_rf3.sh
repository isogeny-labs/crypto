#!/usr/bin/env bash
# Amendment 1: the rf3 session with specs/rf3-amend1.json on the same pod and binaries as job.sh.
set -uo pipefail
W=/work/rm3; OUT=/work/out_rm3
log() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a $OUT/progress.log; }
pack() { (cd /work && tar czf out_rm3.tgz out_rm3 && sha256sum out_rm3.tgz > out_rm3.tgz.sha256); }
cp $W/specs/rf3-amend1.json $OUT/
rm -f $W/rootfind_serve.sock   # the first daemon's socket file outlives it
T0=$(date +%s.%N)
$W/rootfind_serve_cuda --daemon $W/rootfind_serve.sock 2> $OUT/daemon2.log &
DPID=$!
until grep -q "listening" $OUT/daemon2.log 2>/dev/null || ! kill -0 $DPID 2>/dev/null; do sleep 0.01; done
echo "daemon (2nd start) start to socket: $(python3 -c "print(round($(date +%s.%N) - $T0, 3))") s" >> $OUT/daemon-start.txt
grep -q "cuda: " $OUT/daemon2.log || { log "ABORT: daemon not on the CUDA backend"; pack; exit 1; }
log "amendment 1: daemon up: $(tail -1 $OUT/daemon-start.txt)"
mv $OUT/sessions/rf3 $OUT/sessions/rf3-refused 2>/dev/null
log "ecbench run rf3 (amendment 1)"
timeout 14400 $W/ecbench run --spec $W/specs/rf3-amend1.json --out $OUT/sessions/rf3 --cpus none --allow-busy \
  > $OUT/run-rf3-amend1.log 2>&1; log "run rf3 rc=$? $(tail -1 $OUT/run-rf3-amend1.log)"
timeout 14400 $W/ecbench verify --dir $OUT/sessions/rf3 --replay-all --exit-code \
  --out $OUT/audit-rf3.json > $OUT/audit-rf3.log 2>&1; log "audit rf3 rc=$? $(tail -1 $OUT/audit-rf3.log)"
$W/ecbench table --dir $OUT/sessions/rf3 > $OUT/table-rf3.md 2>&1
for sz in 1024 2048; do
  $W/ecbench compare --dir $OUT/sessions/rf3 --a rf3-cpu-F$sz --b rf3-gpu-F$sz > $OUT/compare-rf3-gpu-F$sz.txt 2>&1
  $W/ecbench compare --dir $OUT/sessions/rf3 --a rf3-cpu-F$sz --b rf3-gpud-F$sz > $OUT/compare-rf3-gpud-F$sz.txt 2>&1
done
$W/ecbench compare --dir $OUT/sessions/rf3 --a rf3-cpu-F1024 --b rf3-cpuaa-F1024 > $OUT/compare-rf3-aa.txt 2>&1
kill $DPID; wait $DPID 2>/dev/null
log "done (amendment 1)"
pack
