#!/usr/bin/env bash
# E14 (Amendment 19) on one RunPod CPU pod. Thin orchestration only: every computation is
# `dreg_linear_guess --engine pe` (E14a, E14b) or `e12/run_window.sh` (E14c).
# Env (set at pod creation): SRC_URL, SRC_SHA256, OUT_URL (presigned PUT for out.tgz).
# RunPod injects RUNPOD_POD_ID and RUNPOD_API_KEY.
set -u
W=/workspace; SRC=$W/src; OUT=$W/out; mkdir -p "$SRC" "$OUT"
T0=$(date +%s)
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$OUT/run.log"; }
upload() { tar czf "$W/out.tgz" -C "$W" out 2>/dev/null && curl -sS -m 300 -X PUT -T "$W/out.tgz" "$OUT_URL" >/dev/null && log "uploaded $(sha256sum "$W/out.tgz" | cut -c1-16)"; }
finish() {
  log "finish: $1"; echo "$1" >"$OUT/final_status"; upload
  [ -n "${RUNPOD_POD_ID:-}" ] && [ -n "${RUNPOD_API_KEY:-}" ] && \
    curl -sS -m 60 -X DELETE -H "Authorization: Bearer $RUNPOD_API_KEY" "https://rest.runpod.io/v1/pods/$RUNPOD_POD_ID" >>"$OUT/run.log" 2>&1
  sleep infinity
}
apt-get update -qq >/dev/null 2>&1; apt-get install -y -qq jq build-essential pkg-config libssl-dev curl time >/dev/null 2>&1
# USD 100 hard cap: deadline = min(36 h, floor(100 / costPerHr) h) from start
COST=$(curl -sS -m 60 -H "Authorization: Bearer ${RUNPOD_API_KEY:-}" "https://rest.runpod.io/v1/pods/${RUNPOD_POD_ID:-x}" | jq -r '.costPerHr // empty')
HRS=$(awk -v c="$COST" 'BEGIN{ if (c+0 <= 0) { print 0; exit } h = int(100 / c); print (h < 36 ? h : 36) }')
[ "$HRS" -gt 0 ] || { HRS=${MAX_HOURS_FALLBACK:-24}; COST="unread (fallback limit ${HRS} h)"; }
DEADLINE_EPOCH=$(( T0 + HRS*3600 ))
left() { echo $(( DEADLINE_EPOCH - $(date +%s) )); }
( while sleep 600; do upload; done ) &
( while [ "$(left)" -gt 120 ]; do sleep 60; done; log "deadline"; pkill -TERM -f dreg_linear_guess; pkill -TERM -f hky_window; sleep 30; finish deadline ) &
log "pod ${RUNPOD_POD_ID:-?} start; costPerHr $COST; limit ${HRS} h; deadline $(date -u -d @"$DEADLINE_EPOCH" +%FT%TZ)"
{ grep MemTotal /proc/meminfo; nproc; lscpu | grep 'Model name'; echo "costPerHr $COST"; } >"$OUT/host.txt"
if ! command -v cargo >/dev/null; then
  curl -sSf https://sh.rustup.rs -o "$W/rustup.sh" && sh "$W/rustup.sh" -y --profile minimal >"$OUT/rustup.log" 2>&1 || finish "rustup_failed"
  . "$HOME/.cargo/env"
fi
rustc --version >>"$OUT/host.txt"
curl -sS -m 600 -o "$W/src.tgz" "$SRC_URL" || finish "source_download_failed"
echo "$SRC_SHA256  $W/src.tgz" | sha256sum -c - >>"$OUT/run.log" 2>&1 || finish "source_hash_mismatch"
tar xzf "$W/src.tgz" -C "$SRC" && cd "$SRC" || finish "extract_failed"
S=research/fall_degree_bounds_20261008
cargo build --release --example dreg_linear_guess >"$OUT/build_pe.log" 2>&1 || finish "build_pe_failed"
P=$SRC/target/release/examples/dreg_linear_guess
sha256sum "$P" >"$OUT/sha256_dreg_linear_guess"
log "G0"
cargo test --release --lib -- fast_chained_count_matches_the_reference chained_count partial_enlargement >"$OUT/g0.log" 2>&1 \
  && grep -q "test result: ok. [1-9]" "$OUT/g0.log" || finish "G0_failed"
log "G1"
RAYON_NUM_THREADS=64 "$P" --engine pe --to 4 --max-cols 140000 --draw 0 --sum 15=7+0 --fix 23=0,24=1 \
  research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl >"$OUT/g1.jsonl" 2>"$OUT/g1.err" || finish "G1_exit"
k='[.scan[]|{degree,closure_refuted,saturated,closure_rank}]'
diff <(jq -c "$k" research/dreg_invariant_guess_20261029/runs/S3.d0.b4.jsonl) <(jq -c "$k" "$OUT/g1.jsonl") >"$OUT/g1.diff" || finish "G1_differs"
TIMEV=""; [ -x /usr/bin/time ] && TIMEV="/usr/bin/time -v"
log "gates passed"; upload
pe() { # name, threads, max_cols, to, file, draw
  local name=$1 th=$2 mc=$3 to=$4 f=$5 d=$6
  log "start $name"; local s=$(date +%s)
  $TIMEV env RAYON_NUM_THREADS=$th "$P" --engine pe --batch 64 --to "$to" --max-cols "$mc" --draw "$d" "$f" >"$OUT/$name.jsonl" 2>"$OUT/$name.err"
  log "done $name exit=$? $(( $(date +%s) - s ))s"; upload
}
# E14b: the three unmeasured 6:10:3 controls (64 threads, sequential)
for d in 1 3 4; do pe B_ctl_6-10-3_d$d 64 600000 4 $S/e9/run-r3/out/E_6-10-3.jsonl $d; done
log "E14b finished"
# E14a (40 threads, draws in order) and E14c (24 threads), concurrently
( for d in 0 1 3 4; do pe A_3-21-7_d$d 40 1000000 5 $S/e10/run-r3-B/out/E_3-21-7.jsonl $d; done; log "E14a finished" ) &
APID=$!
( RAYON_NUM_THREADS=24 bash $S/e12/run_window.sh "$OUT/C" 4:28:7 >"$OUT/C.log" 2>&1; log "E14c exit=$?" ) &
CPID=$!
wait $APID; wait $CPID
finish done
