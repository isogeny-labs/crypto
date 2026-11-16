#!/usr/bin/env bash
# PROTOCOL.md (R1') on one RunPod CPU pod. Source by presigned GET (sha256 checked); the
# heartbeat log and the results tarball by presigned PUT. No credentials on the pod.
# Env: SRC_URL SRC_SHA SYS_SHA LOG_URL PUT_URL [PAR=4] [DENSE_T=1800] [SPARSE_T=3600] [BUDGET=36000]
set -u
W=/work; mkdir -p $W/out/runs; cd $W
PAR=${PAR:-4}; DENSE_T=${DENSE_T:-1800}; SPARSE_T=${SPARSE_T:-3600}; BUDGET=${BUDGET:-36000}
log() { echo "$(date -u +%FT%TZ) $*" >> $W/out/pod.log; }
beat() { curl -fsS -m 60 -X PUT -T $W/out/pod.log "$LOG_URL" >/dev/null 2>&1 || true; }
finish() {
  log "upload"; tar -C $W -czf $W/results.tgz out
  sha256sum $W/results.tgz >> $W/out/pod.log
  curl -fsS -m 900 -X PUT -T $W/results.tgz "$PUT_URL"; log "results_put=$?"; beat
  sleep infinity
}
fail() { log "FAIL: $*"; finish; }
START=$(date +%s)
log "start $(hostname)"; nproc > out/nproc.txt; free -g > out/mem.txt
grep -m1 'model name' /proc/cpuinfo > out/cpu.txt; uname -a > out/uname.txt; beat
( while sleep 300; do
    echo "$(date -u +%FT%TZ) progress $(cat out/runs/*.jsonl 2>/dev/null | wc -l) lines" >> out/pod.log; beat
  done ) &
command -v curl >/dev/null || { apt-get update -qq && apt-get install -y -qq curl ca-certificates; }
apt-get update -qq >/dev/null 2>&1; apt-get install -y -qq build-essential pkg-config >/dev/null 2>&1
curl -fsS -m 1800 -o src.tgz "$SRC_URL" || fail "source download"
echo "$SRC_SHA  src.tgz" | sha256sum -c - >> out/pod.log 2>&1 || fail "source hash mismatch"
mkdir -p src && tar -C src -xzf src.tgz || fail "source unpack"
curl -fsS https://sh.rustup.rs | sh -s -- -y -q --profile minimal --default-toolchain stable >> out/build.log 2>&1
. "$HOME/.cargo/env"; rustc -V >> out/pod.log; cargo -V >> out/pod.log; beat
D=$W/src/research/representation_leads_20261109
( cd $D/derive && cargo build --release >> $W/out/build.log 2>&1 && cargo test --release >> $W/out/build.log 2>&1 ) \
  || fail "derive build or tests"
$D/derive/target/release/replead-derive export > $W/systems.jsonl || fail "export"
echo "$SYS_SHA  $W/systems.jsonl" | sha256sum -c - >> out/pod.log 2>&1 || fail "systems hash mismatch (PROTOCOL.md: stop)"
( cd $W/src && export CARGO_PROFILE_RELEASE_DEBUG=0 \
  && cargo build --release --example replead_closure >> $W/out/build.log 2>&1 \
  && cargo test --release --example replead_closure >> $W/out/build.log 2>&1 ) || fail "instrument build or tests"
BIN=$W/src/target/release/examples/replead_closure
sha256sum $BIN $D/derive/target/release/replead-derive $W/systems.jsonl > out/binaries.sha256
log "built; running"; beat

# One cell: draws 0..19, arms rotated by draw index (PROTOCOL.md "Variable ledger").
run_cell() {
  local C=$1 out=$W/out/runs/$1.jsonl arms=(chain_s3 e2_aux chain_trace quad_generic)
  for K in $(seq 0 19); do
    for i in 0 1 2 3; do
      local A=${arms[$(( (i + K) % 4 ))]} tmp
      [ $(( $(date +%s) - START )) -gt $BUDGET ] && { echo "{\"cell\":\"$C\",\"draw\":$K,\"arm\":\"$A\",\"budget_exhausted\":true}" >> $out; continue; }
      tmp=$(timeout $DENSE_T $BIN $W/systems.jsonl --cell $C --draw $K --arm $A --from 2 --to 6 --max-cols 70000); local rc=$?
      [ -n "$tmp" ] && echo "$tmp" >> $out
      [ $rc -eq 124 ] && echo "{\"cell\":\"$C\",\"draw\":$K,\"arm\":\"$A\",\"engine\":\"dense\",\"timeout_s\":$DENSE_T,\"measured\":false}" >> $out
      if echo "$tmp" | grep -q '"instrument_fault":true'; then echo "{\"cell\":\"$C\",\"void\":true}" >> $out; return; fi
      echo "$tmp" | grep -q '"closure_refuted":true' && continue
      local d0; d0=$(echo "$tmp" | grep '"measured":false' | grep -o '"degree":[0-9]*' | head -1 | cut -d: -f2)
      [ -z "$d0" ] && continue   # dense measured through degree 6 without refuting: censored at 6
      for Dg in $(seq $d0 6); do
        tmp=$(timeout $SPARSE_T $BIN $W/systems.jsonl --cell $C --draw $K --arm $A --sparse $Dg --max-entries 1500000000); rc=$?
        [ -n "$tmp" ] && echo "$tmp" >> $out
        if [ $rc -eq 124 ]; then echo "{\"cell\":\"$C\",\"draw\":$K,\"arm\":\"$A\",\"engine\":\"sparse\",\"degree\":$Dg,\"timeout_s\":$SPARSE_T,\"measured\":false}" >> $out; break; fi
        if echo "$tmp" | grep -q '"instrument_fault":true'; then echo "{\"cell\":\"$C\",\"void\":true}" >> $out; return; fi
        echo "$tmp" | grep -q '"closure_refuted":true' && break
        echo "$tmp" | grep -q '"measured":false' && break
      done
    done
  done
  echo "$(date -u +%FT%TZ) cell $C done" >> $W/out/pod.log
}
export -f run_cell; export W BIN START BUDGET DENSE_T SPARSE_T
printf '%s\n' n11-m3-l3-a0 n11-m3-l3-a1 n11-m3-l4-a0 n11-m3-l4-a1 n13-m3-l3-a0 n13-m3-l3-a1 n11-m4-l3-a0 n11-m4-l3-a1 \
  | xargs -P $PAR -I{} bash -c 'run_cell {}'
log "all cells done after $(( $(date +%s) - START )) s"
finish
