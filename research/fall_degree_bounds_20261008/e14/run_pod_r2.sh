#!/usr/bin/env bash
# E14 r2 (Amendment 20) on one RunPod CPU pod. Thin orchestration only: every computation is
# `hky_par newcells` (E14a), `dreg_linear_guess --engine pe` (E14b) or `e12/run_window.sh` (E14c).
# Env (set at pod creation by e14/launch_r2.sh):
#   SRC_COMMIT, PATHS_SHA256, TREE_SHA256, OUT_TOKEN, POD_CTL_KEY (RunPod API key for self-delete),
#   MAX_HOURS_FALLBACK (default 18). RunPod injects RUNPOD_POD_ID.
set -u
W=/workspace; SRC=$W/src; OUT=$W/out; SERVE=$W/serve/$OUT_TOKEN
mkdir -p "$SRC" "$OUT" "$SERVE"
T0=$(date +%s)
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$OUT/run.log"; }
pack() { tar czf "$W/out.tgz.part" -C "$W" out 2>/dev/null && mv -f "$W/out.tgz.part" "$SERVE/out.tgz" && sha256sum "$SERVE/out.tgz" | cut -d' ' -f1 >"$SERVE/out.tgz.sha256"; }
delete_self() {
  [ -n "${RUNPOD_POD_ID:-}" ] && [ -n "${POD_CTL_KEY:-}" ] && \
    curl -sS -m 60 -X DELETE -H "Authorization: Bearer $POD_CTL_KEY" "https://rest.runpod.io/v1/pods/$RUNPOD_POD_ID" >>"$OUT/run.log" 2>&1
}
finish() {
  log "finish: $1"; echo "$1" >"$OUT/final_status"; pack
  # keep serving for 6 hours (Amendment 21) so the controller can take the final pull, then delete the pod
  sleep 21600; delete_self; sleep infinity
}
ln -sfn "$OUT" "$SERVE/out"
( cd "$W/serve" && python3 -m http.server 8080 --bind 0.0.0.0 >"$OUT/http.log" 2>&1 ) &
apt-get update -qq >/dev/null 2>&1; apt-get install -y -qq jq git build-essential pkg-config libssl-dev curl time >/dev/null 2>&1
# USD 100 hard cap: deadline = min(36 h, floor(100 / costPerHr) h) from start
COST=$(curl -sS -m 60 -H "Authorization: Bearer ${POD_CTL_KEY:-}" "https://rest.runpod.io/v1/pods/${RUNPOD_POD_ID:-x}" | jq -r '.costPerHr // empty')
HRS=$(awk -v c="$COST" 'BEGIN{ if (c+0 <= 0) { print 0; exit } h = int(100 / c); print (h < 36 ? h : 36) }')
[ "$HRS" -gt 0 ] || { HRS=${MAX_HOURS_FALLBACK:-18}; COST="unread (fallback limit ${HRS} h)"; }
DEADLINE_EPOCH=$(( T0 + HRS*3600 ))
left() { echo $(( DEADLINE_EPOCH - $(date +%s) )); }
( while sleep 600; do pack; done ) &
( while [ "$(left)" -gt 120 ]; do sleep 60; done; log "deadline"; pkill -TERM -f dreg_linear_guess; pkill -TERM -f hky_par; pkill -TERM -f hky_window; sleep 30; finish deadline ) &
log "pod ${RUNPOD_POD_ID:-?} start; costPerHr $COST; limit ${HRS} h; deadline $(date -u -d @"$DEADLINE_EPOCH" +%FT%TZ)"
{ grep MemTotal /proc/meminfo; nproc; lscpu | grep 'Model name'; echo "costPerHr $COST"; } >"$OUT/host.txt"
if ! command -v cargo >/dev/null; then
  curl -sSf https://sh.rustup.rs -o "$W/rustup.sh" && sh "$W/rustup.sh" -y --profile minimal >"$OUT/rustup.log" 2>&1 || finish "rustup_failed"
  . "$HOME/.cargo/env"
fi
rustc --version >>"$OUT/host.txt"
# --- source: sparse partial clone of the frozen commit (Amendment 20, G-src-a/b/c)
S=research/fall_degree_bounds_20261008
curl -sS -m 120 -o "$W/source_paths.txt" "https://raw.githubusercontent.com/isogeny-labs/crypto/$SRC_COMMIT/$S/e13/source_paths.txt" || finish "paths_download_failed"
echo "$PATHS_SHA256  $W/source_paths.txt" | sha256sum -c - >>"$OUT/run.log" 2>&1 || finish "G-src-b_failed"
cd "$SRC" && git init -q . && git remote add origin https://github.com/isogeny-labs/crypto.git \
  && git sparse-checkout init --no-cone && git sparse-checkout set --stdin <"$W/source_paths.txt" \
  && git fetch -q --filter=blob:none --depth 1 origin "$SRC_COMMIT" && git checkout -q FETCH_HEAD || finish "clone_failed"
[ "$(git rev-parse HEAD)" = "$SRC_COMMIT" ] || finish "G-src-a_failed"
TREE=$(git ls-tree -r HEAD -- $(tr '\n' ' ' <"$W/source_paths.txt") | sha256sum | cut -d' ' -f1)
echo "tree $TREE" >>"$OUT/run.log"; [ "$TREE" = "$TREE_SHA256" ] || finish "G-src-c_failed"
log "source verified $SRC_COMMIT"
# --- builds
cargo build --release --example dreg_linear_guess --example hky_par >"$OUT/build.log" 2>&1 || finish "build_failed"
P=$SRC/target/release/examples/dreg_linear_guess; H=$SRC/target/release/examples/hky_par
sha256sum "$P" "$H" >"$OUT/sha256_binaries"
# --- G0, G1 (Amendment 17)
log "G0"
cargo test --release --lib -- fast_chained_count_matches_the_reference chained_count partial_enlargement >"$OUT/g0.log" 2>&1 \
  && grep -q "test result: ok. [1-9]" "$OUT/g0.log" || finish "G0_failed"
k='[.scan[]|{degree,closure_refuted,saturated,closure_rank}]'
g1() { # threads -> wall seconds; output in g1_<threads>.jsonl
  local th=$1 s=$(date +%s)
  RAYON_NUM_THREADS=$th "$P" --engine pe --to 4 --max-cols 140000 --draw 0 --sum 15=7+0 --fix 23=0,24=1 \
    research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl >"$OUT/g1_$th.jsonl" 2>"$OUT/g1_$th.err" || return 1
  diff <(jq -c "$k" research/dreg_invariant_guess_20261029/runs/S3.d0.b4.jsonl) <(jq -c "$k" "$OUT/g1_$th.jsonl") >"$OUT/g1_$th.diff" || return 2
  echo $(( $(date +%s) - s ))
}
log "G1 (64 threads)"
T64=$(g1 64) || finish "G1_failed"
# --- G2: hky_par reproduces the E10 (3,21,7) records at degree 4
log "G2"
RAYON_NUM_THREADS=64 "$H" newcells --cells 3:21:7 --seed 20261012 --keep 4 --cmax 4 >"$OUT/g2.jsonl" 2>"$OUT/g2.err" || finish "G2_exit"
k2='{draw,x_r,v_basis,solutions,d_f,d_f_at_least,per_degree}'
diff <(jq -c "$k2" $S/e10/run-r3-B/out/E_3-21-7.jsonl) <(jq -c "$k2" "$OUT/g2.jsonl") >"$OUT/g2.diff" || finish "G2_differs"
# --- G3: pe scaling pilot (8, 16 threads; 64 from G1)
log "G3"
T8=$(g1 8) || finish "G3_failed_8"; T16=$(g1 16) || finish "G3_failed_16"
TH_PE=64; T_PE=$T64
[ "$T16" -lt "$T_PE" ] && { TH_PE=16; T_PE=$T16; }
[ "$T8" -lt "$T_PE" ] && { TH_PE=8; T_PE=$T8; }
printf '{"t8":%s,"t16":%s,"t64":%s,"th_pe":%s,"t_pe":%s}\n' "$T8" "$T16" "$T64" "$TH_PE" "$T_PE" >"$OUT/g3.json"
log "gates passed; pe threads $TH_PE ($T_PE s at 140k columns)"; pack
TIMEV=""; [ -x /usr/bin/time ] && TIMEV="/usr/bin/time -v"
# --- E14a: hky_par degree-5 closure at (3,21,7); 12 h without a finished draw -> killed
log "start A_3-21-7 (hky_par cmax 5, 64 threads)"; sA=$(date +%s)
( $TIMEV env RAYON_NUM_THREADS=64 "$H" newcells --cells 3:21:7 --seed 20261012 --keep 4 --cmax 5 >"$OUT/A_3-21-7.jsonl" 2>"$OUT/A_3-21-7.err"; echo "exit=$?" >>"$OUT/A_3-21-7.err" ) &
APID=$!
while kill -0 $APID 2>/dev/null; do
  sleep 60
  if [ ! -s "$OUT/A_3-21-7.jsonl" ] && [ $(( $(date +%s) - sA )) -gt 43200 ]; then log "E14a: no draw in 12 h, killed"; pkill -TERM -f "hky_par newcells"; break; fi
done
wait $APID 2>/dev/null; log "done A_3-21-7 $(( $(date +%s) - sA ))s, draws finished: $(wc -l <"$OUT/A_3-21-7.jsonl")"; pack
# --- E14b: the three unmeasured 6:10:3 controls, pe, TH_PE threads, 6 h each
if [ "$T_PE" -gt 1800 ]; then
  log "E14b skipped: G3 pe too slow ($T_PE s)"; echo "skipped_g3" >"$OUT/E14b_status"
else
  for d in 1 3 4; do
    name=B_ctl_6-10-3_d$d; log "start $name ($TH_PE threads)"; s=$(date +%s)
    $TIMEV timeout -s TERM 21600 env RAYON_NUM_THREADS=$TH_PE "$P" --engine pe --batch 64 --to 4 --max-cols 600000 --draw $d \
      $S/e9/run-r3/out/E_6-10-3.jsonl >"$OUT/$name.jsonl" 2>"$OUT/$name.err"
    log "done $name exit=$? $(( $(date +%s) - s ))s"; pack
  done
fi
log "E14b finished"
# --- E14c: (4,28,7) windows until the deadline
( RAYON_NUM_THREADS=24 bash $S/e12/run_window.sh "$OUT/C" 4:28:7 >"$OUT/C.log" 2>&1; log "E14c exit=$?" )
finish done
