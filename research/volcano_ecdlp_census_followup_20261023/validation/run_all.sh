#!/usr/bin/env bash
# Rebuild the follow-up validation from the committed snapshot (validator-written).
# Usage: run_all.sh SCRATCH [SNAPSHOT_SHA]   (run from anywhere; never builds ecbench)
# Disk: rederive.py streams records out of the archives (no unpack); replay_sample.sh
# unpacks one session at a time into SCRATCH and deletes it (stops below 300 MB free).
set -eu
SCR=$1; SNAP=${2:-$(git -C "$(dirname "$0")" rev-parse HEAD)}
V=$(cd "$(dirname "$0")" && pwd); STUDY=$(dirname "$V"); REPO=$(git -C "$V" rev-parse --show-toplevel)
BENCH=$REPO/benchmarks/20261023-volcano-census-followup
ECB=$REPO/target/release/ecbench
mkdir -p "$SCR"
"$V/draw_sample.sh" "$STUDY/runs/progress.jsonl" "$SNAP" > "$SCR/sample.txt"
date -u +%FT%TZ > "$SCR/replay_started_utc"
"$V/replay_sample.sh" "$STUDY" "$ECB" "$SCR" "$SCR/sample.txt" "$SCR/replay.jsonl"
python3 -I "$V/rederive.py" "$STUDY" "$SCR/rows"
python3 -I "$V/compare_extracts.py" "$STUDY" "$SCR/rows" "$SCR/compare_extracts.json"
python3 -I "$V/recompute.py" "$STUDY" "$SCR/rows" "$SCR/recompute.json"
python3 -I "$V/hm_precise.py" "$STUDY" "$SCR/rows" "$SCR/hm_precise.json"
python3 -I "$V/mixed_check.py" "$STUDY" "$SCR/rows" "$SCR/mixed_check.json"
python3 -I "$V/extras.py" "$STUDY" "$SCR/rows" "$SCR/extras.json"
python3 -I "$V/report_check.py" "$STUDY" "$SCR/recompute.json" "$SCR/report_check.json"
# Phase B audit receipts and orphan comparison (inline checks recorded in the certificate)
python3 -I "$V/phase_b_checks.py" "$STUDY" "$SCR"
python3 -I "$V/session_provenance.py" "$STUDY" "$SCR/provenance.json"
python3 -I "$V/build_validation.py" "$STUDY" "$SCR" "$BENCH" "$SNAP"
python3 -I "$V/write_report.py" "$STUDY" "$SCR" "$BENCH" "$SNAP"
