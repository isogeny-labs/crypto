#!/usr/bin/env bash
# Thin orchestration only: re-run the cheapest committed attempt (arm A, target b0,
# repetition 1) with the freshly built producer and compare its deterministic
# counters with the committed receipt. The solver is deterministic for a given
# phase seed, so status, conflicts, decisions and propagations must match
# exactly; CPU and wall seconds are not compared. Run from the repository root.
set -euo pipefail
d=research/phase_paired_native_20261026
bin="${CARGO_TARGET_DIR:-target}/release/examples/phase_paired_fixed_budget"
tmp="${RUNNER_TEMP:-$(mktemp -d)}/determinism"
mkdir -p "$tmp"
rm -f "$tmp/rerun.jsonl"
budget=$(jq -r 'select(.type=="header") | .conflict_budget' "$d/results/receipts_n13.jsonl")
"$bin" run --out "$tmp/rerun.jsonl" --forms subgroup_xor --reps 1 --targets b0 --arms A \
  --conflict-budget "$budget" --deadline-secs 600 --priority core > /dev/null
sel='select(.type=="attempt" and .rep==1 and .form=="subgroup_xor" and .arm=="A" and .target_id=="b0") | [.status,.conflicts,.decisions,.propagations,.restarts,.learnt_clauses]'
want=$(jq -c "$sel" "$d/results/receipts_n13.jsonl")
got=$(jq -c "$sel" "$tmp/rerun.jsonl")
echo "committed: $want"
echo "re-run   : $got"
[ "$want" = "$got" ] && echo "deterministic counters match" || { echo "MISMATCH"; exit 1; }
