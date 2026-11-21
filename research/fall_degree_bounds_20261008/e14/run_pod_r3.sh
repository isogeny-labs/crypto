#!/usr/bin/env bash
# E14 r3 (Amendment 31) on one RunPod CPU pod. Thin orchestration only: every computation is
# `hky_par ladder` (MODE=a: E14a draw 3 alone) or `dreg_linear_guess --engine pe` (MODE=b: E14b controls).
# Env (set at pod creation by e14/launch_r3.sh):
#   MODE (a|b), CAP_USD, MAX_HOURS, SRC_COMMIT, PATHS_SHA256, TREE_SHA256, OUT_TOKEN,
#   POD_CTL_KEY (RunPod API key for self-delete), POST_HOURS, MAX_HOURS_FALLBACK. RunPod injects RUNPOD_POD_ID.
# Amendment 32: no persistent volume exists on RunPod CPU pods; after finishing the pod serves for POST_HOURS
# (18 h) and then deletes itself; the controller deletes it earlier, after its pull.
set -u
W=/workspace; SRC=/root/src; OUT=$W/out; SERVE=$W/serve/$OUT_TOKEN
mkdir -p "$SRC" "$OUT" "$SERVE"
T0=$(date +%s)
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$OUT/run.log"; }
pack() { tar czf "$W/out.tgz.part" -C "$W" out 2>/dev/null && mv -f "$W/out.tgz.part" "$SERVE/out.tgz" && sha256sum "$SERVE/out.tgz" | cut -d' ' -f1 >"$SERVE/out.tgz.sha256"; }
delete_self() {
  { set +x; } 2>/dev/null
  [ -n "${RUNPOD_POD_ID:-}" ] && [ -n "${POD_CTL_KEY:-}" ] && \
    curl -sS -m 60 -X DELETE -H "Authorization: Bearer $POD_CTL_KEY" "https://rest.runpod.io/v1/pods/$RUNPOD_POD_ID" >>"$OUT/run.log" 2>&1
  set -x
}
finish() {
  log "finish: $1"; echo "$1" >"$OUT/final_status"; pack; sync
  # Amendment 32: RunPod CPU pods get no persistent volume, so a stopped pod would lose out/. Keep serving
  # for POST_HOURS (default 18) so a late controller can still pull; the controller deletes the pod after
  # its pull; only then, as a cost bound, the pod deletes itself.
  sleep $(( ${POST_HOURS:-18} * 3600 )); delete_self; sleep infinity
}
ln -sfn "$OUT" "$SERVE/out"
echo "$(date -u +%FT%TZ) boot: script started (pid $$)" >>"$OUT/run.log"
if [ -s "$OUT/final_status" ]; then   # Amendment 31: restarted after finishing: serve only, never recompute
  echo "$(date -u +%FT%TZ) restart after finish ($(cat "$OUT/final_status")): serving only" >>"$OUT/run.log"; pack; sleep infinity
fi
if [ -s "$OUT/started" ]; then        # restarted mid-run (host event): stop, do not retry (Amendment 31)
  echo "$(date -u +%FT%TZ) restart mid-run: not retried" >>"$OUT/run.log"; echo "restarted_mid_run" >"$OUT/final_status"; pack; sleep infinity
fi
date -u +%FT%TZ >"$OUT/started"
# Amendment 24: full command trace to a served file; the HTTP server is started by the pod's start
# command before this script runs (python3 is in the image), so a stall anywhere below is visible.
PS4='+ $(date -u +%T) '; exec 2>>"$OUT/trace.log"; set -x
pgrep -f 'http.server 8080' >/dev/null || ( cd "$W/serve" && python3 -m http.server 8080 --bind 0.0.0.0 >"$OUT/http.log" 2>&1 ) &
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >"$OUT/apt.log" 2>&1; echo "$(date -u +%FT%TZ) boot: apt update exit=$?" >>"$OUT/run.log"
apt-get install -y -qq jq curl git >>"$OUT/apt.log" 2>&1; echo "$(date -u +%FT%TZ) boot: apt base exit=$?" >>"$OUT/run.log"
apt-get install -y -qq build-essential pkg-config libssl-dev time >>"$OUT/apt.log" 2>&1; echo "$(date -u +%FT%TZ) boot: apt build tools exit=$?" >>"$OUT/run.log"
# Amendment 31 cap: deadline = min(MAX_HOURS, floor(CAP_USD / costPerHr) h) from start
{ set +x; } 2>/dev/null
COST=$(curl -sS -m 60 -H "Authorization: Bearer ${POD_CTL_KEY:-}" "https://rest.runpod.io/v1/pods/${RUNPOD_POD_ID:-x}" | jq -r '.costPerHr // empty')
set -x
HRS=$(awk -v c="$COST" -v cap="${CAP_USD:-30}" -v mx="${MAX_HOURS:-16}" 'BEGIN{ if (c+0 <= 0) { print 0; exit } h = int(cap / c); print (h < mx ? h : mx) }')
[ "$HRS" -gt 0 ] || { HRS=${MAX_HOURS_FALLBACK:-18}; COST="unread (fallback limit ${HRS} h)"; }
DEADLINE_EPOCH=$(( T0 + HRS*3600 ))
left() { echo $(( DEADLINE_EPOCH - $(date +%s) )); }
( while sleep 600; do pack; done ) &
( while [ "$(left)" -gt 120 ]; do sleep 60; done; log "deadline"; pkill -TERM -f dreg_linear_guess; pkill -TERM -f hky_par; sleep 30; finish deadline ) &
log "pod ${RUNPOD_POD_ID:-?} mode ${MODE:-?} start; costPerHr $COST; limit ${HRS} h; deadline $(date -u -d @"$DEADLINE_EPOCH" +%FT%TZ)"
{ grep MemTotal /proc/meminfo; nproc; lscpu | grep 'Model name'; echo "costPerHr $COST"; } >"$OUT/host.txt"
if ! command -v cargo >/dev/null; then
  curl -sSf https://sh.rustup.rs -o "$W/rustup.sh" && sh "$W/rustup.sh" -y --profile minimal >"$OUT/rustup.log" 2>&1 || finish "rustup_failed"
  . "$HOME/.cargo/env"
fi
rustc --version >>"$OUT/host.txt"; log "rust ready"; pack
# --- source: sparse partial clone of the frozen commit (Amendment 20, G-src-a/b/c)
S=research/fall_degree_bounds_20261008
# Amendment 26: the path list postdates the frozen commit; fetch it from PATHS_COMMIT (hash-pinned by PATHS_SHA256)
curl -sS -m 120 -o "$W/source_paths.txt" "https://raw.githubusercontent.com/isogeny-labs/crypto/${PATHS_COMMIT:-43438f5ab6eb6d1844b8d0465c933ad079099326}/$S/e13/source_paths.txt" || finish "paths_download_failed"
echo "$PATHS_SHA256  $W/source_paths.txt" | sha256sum -c - >>"$OUT/run.log" 2>&1 || finish "G-src-b_failed"
cd "$SRC" && git init -q . && git remote add origin https://github.com/isogeny-labs/crypto.git \
  && git sparse-checkout init --no-cone && git sparse-checkout set --stdin <"$W/source_paths.txt" \
  && git fetch -q --filter=blob:none --depth 1 origin "$SRC_COMMIT" && git checkout -q FETCH_HEAD || finish "clone_failed"
[ "$(git rev-parse HEAD)" = "$SRC_COMMIT" ] || finish "G-src-a_failed"
TREE=$(git ls-tree -r HEAD -- $(tr '\n' ' ' <"$W/source_paths.txt") | sha256sum | cut -d' ' -f1)
echo "tree $TREE" >>"$OUT/run.log"; [ "$TREE" = "$TREE_SHA256" ] || finish "G-src-c_failed"
log "source verified $SRC_COMMIT"; pack
# --- builds
# Amendment 28: record the CPU quota and pressure, run a short parallel probe, and bound the build
{ echo "cpu.max: $(cat /sys/fs/cgroup/cpu.max 2>/dev/null || echo n/a)"; echo "cpu.pressure: $(head -1 /proc/pressure/cpu 2>/dev/null || echo n/a)"; } >>"$OUT/host.txt"
{ set +x; } 2>/dev/null   # Amendment 29: never trace the busy loops
P0=$(date +%s%N); PIDS=""; for i in $(seq 1 32); do ( n=0; while [ $n -lt 300000 ]; do n=$((n+1)); done ) & PIDS="$PIDS $!"; done; wait $PIDS; P1=$(date +%s%N)   # Amendment 30: wait only on the probe jobs
S0=$(date +%s%N); ( n=0; while [ $n -lt 300000 ]; do n=$((n+1)); done ); S1=$(date +%s%N)
PAR=$(awk -v p=$((P1-P0)) -v s=$((S1-S0)) 'BEGIN{ printf "%.1f", 32*s/p }')
set -x
log "cpu probe: effective parallelism $PAR of 32"; echo "parallelism $PAR" >>"$OUT/host.txt"; pack
awk -v x="$PAR" 'BEGIN{ exit (x+0 < 12) ? 0 : 1 }' && finish "cpu_probe_low_$PAR"
timeout 1500 cargo build --release --example dreg_linear_guess --example hky_par >"$OUT/build.log" 2>&1 || finish "build_failed_or_timeout"
log "build done"
P=$SRC/target/release/examples/dreg_linear_guess; H=$SRC/target/release/examples/hky_par
sha256sum "$P" "$H" >"$OUT/sha256_binaries"
# --- G0 (Amendment 17)
log "G0"
cargo test --release --lib -- fast_chained_count_matches_the_reference chained_count partial_enlargement >"$OUT/g0.log" 2>&1 \
  && grep -q "test result: ok. [1-9]" "$OUT/g0.log" || finish "G0_failed"
TIMEV=""; [ -x /usr/bin/time ] && TIMEV="/usr/bin/time -v"
case "${MODE:-}" in
a)
  # --- E14a draw 3 alone through `hky_par ladder` on E10's draw-3 record (Amendment 31)
  jq -c 'select(.draw==3)' $S/e10/run-r3-B/out/E_3-21-7.jsonl >"$OUT/d3_input.jsonl"
  sha256sum "$OUT/d3_input.jsonl" >>"$OUT/run.log"
  [ "$(wc -l <"$OUT/d3_input.jsonl")" = 1 ] || finish "input_not_one_record"
  kL='{draw,d_f,d_f_at_least,per_degree:(.per_degree|map_values(del(.secs)))}'
  # GL: ladder at degree cap 4 must reproduce E10's draw-3 record on the timing-free key
  log "GL"
  RAYON_NUM_THREADS=32 "$H" ladder "$OUT/d3_input.jsonl" --m 3 --cmax 4 >"$OUT/gl.jsonl" 2>"$OUT/gl.err" || finish "GL_exit"
  diff <(jq -c "$kL" "$OUT/d3_input.jsonl") <(jq -c "$kL" "$OUT/gl.jsonl") >"$OUT/gl.diff" || finish "GL_differs"
  log "gates passed"; pack
  log "start A_3-21-7_d3 (hky_par ladder cmax 5, 32 threads)"; sA=$(date +%s)
  $TIMEV env RAYON_NUM_THREADS=32 "$H" ladder "$OUT/d3_input.jsonl" --m 3 --cmax 5 >"$OUT/A_3-21-7_d3.jsonl" 2>"$OUT/A_3-21-7_d3.err"
  log "done A_3-21-7_d3 exit=$? $(( $(date +%s) - sA ))s"; pack
  finish done ;;
b)
  # --- E14b: the three unmeasured 6:10:3 controls, pe at 8 threads each, concurrently, 6 h box each
  k='[.scan[]|{degree,closure_refuted,saturated,closure_rank}]'
  log "G1 (8 threads)"
  RAYON_NUM_THREADS=8 "$P" --engine pe --to 4 --max-cols 140000 --draw 0 --sum 15=7+0 --fix 23=0,24=1 \
    research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl >"$OUT/g1_8.jsonl" 2>"$OUT/g1_8.err" || finish "G1_exit"
  diff <(jq -c "$k" research/dreg_invariant_guess_20261029/runs/S3.d0.b4.jsonl) <(jq -c "$k" "$OUT/g1_8.jsonl") >"$OUT/g1_8.diff" || finish "G1_differs"
  log "gates passed"; pack
  BP=""
  for d in 1 3 4; do
    ( name=B_ctl_6-10-3_d$d; log "start $name (8 threads)"; s=$(date +%s)
      $TIMEV timeout -s TERM 21600 env RAYON_NUM_THREADS=8 "$P" --engine pe --batch 64 --to 4 --max-cols 600000 --draw $d \
        $S/e9/run-r3/out/E_6-10-3.jsonl >"$OUT/$name.jsonl" 2>"$OUT/$name.err"
      log "done $name exit=$? $(( $(date +%s) - s ))s"; pack ) &
    BP="$BP $!"
  done
  wait $BP   # only the three draw jobs (Amendment 30 lesson)
  finish done ;;
*) finish "bad_mode_${MODE:-unset}" ;;
esac
