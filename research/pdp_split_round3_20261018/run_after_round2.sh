#!/bin/sh
# Round 3 (PROTOCOL.md), started once round 2 reports `all done`, as
# restartable units (round 2's amendment 3 pattern): a complete session
# or stage file is skipped; an incomplete one is set aside, never overwritten.
# Launch detached: setsid nohup ./research/pdp_split_round3_20261018/run_after_round2.sh &
set -u
D2=research/pdp_split_slots_20261012
D3=research/pdp_split_round3_20261018
mkdir -p "$D3/sessions" "$D3/runs-B" "$D3/audits"
log() { echo "$1 $(date -u +%FT%TZ) $2" >> "$D3/progress.txt"; }
session_done() { [ -f "$D3/sessions/$1/session.json" ] && grep -q '"status": *"complete"' "$D3/sessions/$1/session.json"; }
set_aside() {
  [ -d "$D3/sessions/$1" ] || return 0
  k=1; while [ -e "$D3/sessions/$1-interrupted-$k" ]; do k=$((k+1)); done
  mv "$D3/sessions/$1" "$D3/sessions/$1-interrupted-$k"
  [ -f "$D3/sessions/$1.log" ] && mv "$D3/sessions/$1.log" "$D3/sessions/$1-interrupted-$k.log"
  log "$1" "set aside as $1-interrupted-$k"
}
session() {  # name spec
  session_done "$1" && return 0
  set_aside "$1"
  log "$1" start
  target/release/ecbench run --wait --spec "$D3/$2" --out "$D3/sessions/$1" --cpus none > "$D3/sessions/$1.log" 2>&1
  log "$1" "end exit=$?"
}
stage() {  # name timeout args...
  name=$1; t=$2; shift 2
  OUT="$D3/runs-B/$name.jsonl"
  [ -s "$OUT.done" ] && return 0
  [ -e "$OUT" ] && mv "$OUT" "$OUT.interrupted-$(date -u +%H%M%S)"
  log "B-$name" start
  timeout "$t" target/release/pdp_split_dreg "$@" --out "$OUT" 2> "$OUT.stderr"
  rc=$?
  [ $rc -eq 0 ] && date -u +%FT%TZ > "$OUT.done"
  log "B-$name" "end exit=$rc"
}
log run "start; waiting for round 2 all done"
until grep -q "all done" "$D2/progress.txt" 2>/dev/null; do sleep 60; done
session A-n13 SPEC-A-n13.json
session A-n19 SPEC-A-n19.json
session C-n15 SPEC-C-n15.json
session A-n17 SPEC-A-n17.json
COMMON="--base closure --plans 3 --targets 2 --slots-per-target 2 --seed 20261018 --max-free 24"
for l in 4 5 6 7 8; do
  stage "ell$l" 7200 --cells 3:0:23:$l --guesses 8 $COMMON --budget-secs 120 --lambda-targets 16 --lambda-systems 128
done
for g in 8 10 12 14; do
  stage "ell7-g$g" 7200 --cells 3:0:23:7 --guesses $g $COMMON --budget-secs 120 --lambda-targets 16 --lambda-systems 128
done
for s in A-n13 A-n19 C-n15 A-n17; do
  [ -s "$D3/audits/$s.audit.json" ] && continue
  session_done "$s" || continue
  target/release/ecbench verify --dir "$D3/sessions/$s" --replay-all --out "$D3/audits/$s.audit.json.tmp" \
    > "$D3/audits/$s.verify.log" 2>&1 && mv "$D3/audits/$s.audit.json.tmp" "$D3/audits/$s.audit.json"
  log "audit-$s" "end exit=$?"
done
stage ell9 28800 --cells 3:0:23:9 --guesses 8 $COMMON --budget-secs 600 --lambda-targets 4 --lambda-systems 8
log all done
