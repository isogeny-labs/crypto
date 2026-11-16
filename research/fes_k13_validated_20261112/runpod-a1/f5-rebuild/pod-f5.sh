#!/bin/bash
# F5: rebuild ecbench from git archive of e9d5dbdccd with pinned rustc 1.97.0, replay the session, re-run the frozen spec.
set -u
W=/work; mkdir -p $W/out; cd $W
log() { echo "$(date -u +%FT%TZ) $*" >> $W/out/pod.log; }
beat() { curl -fsS -m 60 -X PUT -T $W/out/pod.log "$1" >/dev/null 2>&1 || true; }
log "start $(hostname) nproc=$(nproc) $(grep -m1 'model name' /proc/cpuinfo)"; beat "$LOG1_URL"
curl -fsS -m 600 -o src.tgz "$SRC_URL"; log "src_get=$?"
curl -fsS -m 300 -o session.tgz "$SESSION_URL"; log "session_get=$?"
curl -fsS -m 300 -o ecbench-orig "$BIN_URL"; log "orig_bin_get=$?"
curl -fsS -m 60 -o spec.json "$SPEC_URL"; log "spec_get=$?"
sha256sum src.tgz session.tgz ecbench-orig spec.json | tee $W/out/inputs.sha256 >> $W/out/pod.log
mkdir -p src && tar -xzf src.tgz -C src && tar -xzf session.tgz -C $W
(apt-get update -qq && DEBIAN_FRONTEND=noninteractive apt-get install -y -qq pkg-config libssl-dev build-essential cmake) >>$W/out/apt.log 2>&1; log "apt=$?"
export RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:$PATH
curl -fsS --proto '=https' --tlsv1.2 https://sh.rustup.rs -o rustup-init.sh && sh rustup-init.sh -y -q --profile minimal --default-toolchain 1.97.0 >>$W/out/rustup.log 2>&1; log "rustup=$?"
rustc -vV > $W/out/rustc.txt 2>&1; cargo -V >> $W/out/rustc.txt 2>&1; cat $W/out/rustc.txt >> $W/out/pod.log
beat "$LOG2_URL"
cd src && T0=$(date +%s); cargo build --release --locked --bin ecbench > $W/out/build.log 2>&1; log "build=$? secs=$(( $(date +%s)-T0 ))"; cd $W
cp src/target/release/ecbench $W/out/ecbench-rebuilt 2>/dev/null
sha256sum $W/out/ecbench-rebuilt >> $W/out/pod.log
beat "$LOG3_URL"
B=$W/out/ecbench-rebuilt
$B host --json > $W/out/host.json 2>/dev/null
$B verify --dir $W/session --replay-all --exit-code > $W/out/replay-rebuilt.stdout 2>&1; log "replay_rebuilt=$?"
tail -1 $W/out/replay-rebuilt.stdout >> $W/out/pod.log
$B run --spec spec.json --out $W/out/rerun-session --lock $W/bench.lock > $W/out/rerun.stdout 2>&1; log "rerun=$?"
$B verify --dir $W/out/rerun-session --replay-all --exit-code > $W/out/rerun-audit.stdout 2>&1; log "rerun_audit=$?"
tail -1 $W/out/rerun-audit.stdout >> $W/out/pod.log
log "upload"
tar -C $W -czf $W/results.tgz out
sha256sum $W/results.tgz >> $W/out/pod.log
curl -fsS -m 600 -X PUT -T $W/results.tgz "$PUT_URL"; log "results_put=$?"
beat "$LOG4_URL"
sleep infinity
