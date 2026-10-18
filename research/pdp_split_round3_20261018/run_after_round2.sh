#!/bin/sh
# Round 3 (PROTOCOL.md), started once round 2's resumed runs report
# `all done`.  Launch detached: setsid nohup ./research/pdp_split_round3_20261018/run_after_round2.sh &
set -u
D2=research/pdp_split_slots_20261012
D3=research/pdp_split_round3_20261018
log() { echo "$1 $(date -u +%FT%TZ) $2" >> "$D3/progress.txt"; }
log wait "for round 2 all done"
until grep -q "all done" "$D2/progress.txt" 2>/dev/null; do sleep 60; done
mkdir -p "$D3/sessions" "$D3/runs-B" "$D3/audits"
for n in 13 17 19; do
  log "A-n$n" start
  target/release/ecbench run --wait --spec "$D3/SPEC-A-n$n.json" --out "$D3/sessions/A-n$n" --cpus none \
    > "$D3/sessions/A-n$n.log" 2>&1
  log "A-n$n" "end exit=$?"
done
log "C-n15" start
target/release/ecbench run --wait --spec "$D3/SPEC-C-n15.json" --out "$D3/sessions/C-n15" --cpus none \
  > "$D3/sessions/C-n15.log" 2>&1
log "C-n15" "end exit=$?"
for s in A-n13 A-n17 A-n19 C-n15; do
  target/release/ecbench verify --dir "$D3/sessions/$s" --replay-all --out "$D3/audits/$s.audit.json" \
    > "$D3/audits/$s.verify.log" 2>&1
  log "audit-$s" "end exit=$?"
done
log "B-ell" start
timeout 14400 target/release/pdp_split_dreg --cells 3:0:23:4,3:0:23:5,3:0:23:6,3:0:23:7,3:0:23:8 --base closure \
  --plans 3 --guesses 8 --targets 2 --slots-per-target 2 --seed 20261018 --budget-secs 120 --max-free 24 \
  --lambda-targets 16 --lambda-systems 128 --out "$D3/runs-B/ell-series.jsonl" 2> "$D3/runs-B/ell-series.jsonl.stderr"
log "B-ell" "end exit=$?"
log "B-ell9" start
timeout 28800 target/release/pdp_split_dreg --cells 3:0:23:9 --base closure \
  --plans 3 --guesses 8 --targets 2 --slots-per-target 2 --seed 20261018 --budget-secs 600 --max-free 24 \
  --lambda-targets 4 --lambda-systems 8 --out "$D3/runs-B/ell9.jsonl" 2> "$D3/runs-B/ell9.jsonl.stderr"
log "B-ell9" "end exit=$?"
log "B-g" start
timeout 14400 target/release/pdp_split_dreg --cells 3:0:23:7 --base closure \
  --plans 3 --guesses 8,10,12,14 --targets 2 --slots-per-target 2 --seed 20261018 --budget-secs 120 --max-free 24 \
  --lambda-targets 16 --lambda-systems 128 --out "$D3/runs-B/g-series.jsonl" 2> "$D3/runs-B/g-series.jsonl.stderr"
log "B-g" "end exit=$?"
log all done
