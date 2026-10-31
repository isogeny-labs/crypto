#!/bin/sh
# Amendment 3: restartable units for Parts B, C and the audits.
set -u
D=research/pdp_split_slots_20261012
mkdir -p "$D/sessions" "$D/runs-B" "$D/audits"
log() { echo "$1 $(date -u +%FT%TZ) $2" >> "$D/progress.txt"; }
session_done() { [ -f "$D/sessions/$1/session.json" ] && grep -q '"status": *"complete"' "$D/sessions/$1/session.json"; }
set_aside() {
  [ -d "$D/sessions/$1" ] || return 0
  k=1; while [ -e "$D/sessions/$1-interrupted-$k" ]; do k=$((k+1)); done
  mv "$D/sessions/$1" "$D/sessions/$1-interrupted-$k"
  [ -f "$D/sessions/$1.log" ] && mv "$D/sessions/$1.log" "$D/sessions/$1-interrupted-$k.log"
  log "$1" "set aside as $1-interrupted-$k"
}
log resume3 "start (amendment 3)"
for CELL in 3:1:11:4 3:0:13:4 3:1:17:4 3:1:19:4 3:0:13:5 3:1:17:5; do
  OUT="$D/runs-B/$(echo "$CELL" | tr ':' '-').jsonl"
  if [ -s "$OUT.done" ]; then continue; fi
  [ -e "$OUT" ] && mv "$OUT" "$OUT.interrupted-$(date -u +%H%M%S)"
  log "B-$CELL" start
  timeout 7200 target/release/pdp_split_dreg --cells "$CELL" --base closure --slots-per-target 4 \
    --targets 4 --guesses 0,2,4 --seed 20261012 --budget-secs 300 --max-free 24 \
    --lambda-targets 256 --lambda-systems 512 --out "$OUT" 2>"$OUT.stderr"
  rc=$?
  [ $rc -eq 0 ] && date -u +%FT%TZ > "$OUT.done"
  log "B-$CELL" "end exit=$rc"
done
if ! session_done C-n17; then
  set_aside C-n17
  log "C-n17" start
  target/release/ecbench run --wait --spec "$D/SPEC-C-n17.json" --out "$D/sessions/C-n17" --cpus none \
    > "$D/sessions/C-n17.log" 2>&1
  log "C-n17" "end exit=$?"
fi
for s in A-n11 A-n13 A-n17 C-n17; do
  [ -s "$D/audits/$s.audit.json" ] && continue
  target/release/ecbench verify --dir "$D/sessions/$s" --replay-all --out "$D/audits/$s.audit.json.tmp" \
    > "$D/audits/$s.verify.log" 2>&1 && mv "$D/audits/$s.audit.json.tmp" "$D/audits/$s.audit.json"
  log "audit-$s" "end exit=$?"
done
for s in A-n19 A-n19-02 A-n19-fast; do
  [ -s "$D/audits/$s-interrupted.audit.json" ] && continue
  target/release/ecbench verify --dir "$D/sessions/$s" --replay 10 --allow-interrupted \
    --out "$D/audits/$s-interrupted.audit.json.tmp" > "$D/audits/$s-interrupted.verify.log" 2>&1 \
    && mv "$D/audits/$s-interrupted.audit.json.tmp" "$D/audits/$s-interrupted.audit.json"
  log "audit-$s-interrupted" "end exit=$?"
done
log all done
