#!/usr/bin/env bash
# Round-2 validation, in order (validator-written). Run from the study directory.
# SCR = scratch directory (session archives are extracted there one at a time; ~0.5 GB free needed).
# Usage: bash validation/round2/run_all.sh SCR SNAPSHOT_SHA VALIDATION_ID
set -eu
SCR=$1; SNAP=$2; VID=$3
EV=../../benchmarks/20261023-volcano-census-followup
bash validation/round2/draw_sample.sh runs/progress.jsonl "$SNAP" > "$SCR/sample.txt"
bash validation/round2/replay_sample.sh . ../../target/release/ecbench \
  ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35 "$SCR" "$SCR/sample.txt" "$SCR/replay.jsonl"
python3 -I validation/round2/rederive.py . "$SCR/derived" 2
python3 -I validation/round2/compare_extracts.py . "$SCR/derived/rows.csv.gz" "$SCR/derived/extract_compare.json"
python3 -I validation/round2/recompute.py . "$SCR/derived/rows.csv.gz" "$SCR/derived/recompute.json"
python3 -I validation/round2/report_check.py . "$SCR/derived/report_check.json"
python3 -I validation/round2/orphans_check.py . > "$SCR/derived/orphans.json"
for o in $(jq -r 'select(.status=="done") | .out | sub("^sessions/";"")' runs/progress.jsonl); do
  jq -c --arg s "$o" '{s:$s, ok, st:.session_status, rec:.records, ver:.verified_records, rep:(.replays|length), repok:([.replays[]|select(.reproduced)]|length), bin:.auditor_binary_sha256}' "sessions/$o.audit.json"
done > "$SCR/derived/phase_b_audits.jsonl"
python3 -I validation/round2/build_validation.py . "$SCR/derived" "$SCR/replay.jsonl" "$EV" "$SNAP"
mkdir -p "$EV/replay-receipts/round2"; cp "$SCR"/receipts/*.json.xz "$EV/replay-receipts/round2/"
python3 -I validation/round2/write_outputs.py . "$SCR" "$EV" "$SNAP" "$VID"
python3 -I validation/round2/write_md.py "$EV"
(cd ../.. && python3 scripts/validation_gate.py evidence benchmarks/20261023-volcano-census-followup)
