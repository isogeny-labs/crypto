#!/bin/bash
# FES-K13-VALIDATED-20261112 attempt a1 on Runpod (launch 2). Inputs by presigned GET; logs and results by presigned PUT.
set -u
W=/work; mkdir -p $W/out; cd $W
log() { echo "$(date -u +%FT%TZ) $*" >> $W/out/pod.log; }
beat() { curl -fsS -m 60 -X PUT -T $W/out/pod.log "$1" >/dev/null 2>&1 || true; }
log "start $(hostname)"; beat "$LOG1_URL"
command -v curl >/dev/null || { apt-get update -qq && apt-get install -y -qq curl ca-certificates; }
curl -fsS -m 300 -o ecbench "$BIN_URL"; log "bin_get=$?"
curl -fsS -m 60 -o spec.json "$SPEC_URL"; log "spec_get=$?"
chmod +x ecbench
sha256sum ecbench spec.json > $W/out/inputs.sha256; cat $W/out/inputs.sha256 >> $W/out/pod.log
echo "$BIN_SHA  ecbench" | sha256sum -c - >>$W/out/pod.log 2>&1 || { log "binary hash mismatch"; FAIL=1; }
echo "$SPEC_SHA  spec.json" | sha256sum -c - >>$W/out/pod.log 2>&1 || { log "spec hash mismatch"; FAIL=1; }
ldd ./ecbench >>$W/out/pod.log 2>&1
beat "$LOG2_URL"
if [ -z "${FAIL:-}" ]; then
  nproc >$W/out/nproc.txt; grep -m1 'model name' /proc/cpuinfo >$W/out/cpu.txt; uname -a >$W/out/uname.txt
  ./ecbench host --json >$W/out/host.json 2>$W/out/host.err; log "host=$?"
  log "run"
  timeout 3000 ./ecbench run --spec spec.json --out $W/out/session --lock $W/bench.lock >$W/out/run.stdout 2>&1; log "run_exit=$?"
  if ! [ -f $W/out/session/session.json ]; then
    log "run refused or failed without --allow-busy; retrying with --allow-busy (disclosed)"
    tail -5 $W/out/run.stdout >> $W/out/pod.log
    rm -rf $W/out/session
    timeout 3000 ./ecbench run --spec spec.json --out $W/out/session --lock $W/bench.lock --allow-busy >$W/out/run-allow-busy.stdout 2>&1; log "run_allow_busy_exit=$?"
  fi
  beat "$LOG3_URL"
  log "verify"
  timeout 3000 ./ecbench verify --dir $W/out/session --replay-all --exit-code >$W/out/audit.stdout 2>&1; log "verify_exit=$?"
fi
log "upload"
tar -C $W -czf $W/results.tgz out
sha256sum $W/results.tgz >> $W/out/pod.log
curl -fsS -m 300 -X PUT -T $W/results.tgz "$PUT_URL"; log "results_put=$?"
beat "$LOG4_URL"
sleep infinity
