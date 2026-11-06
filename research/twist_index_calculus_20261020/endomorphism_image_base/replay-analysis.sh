#!/usr/bin/env bash
# Recompute the summaries from an extracted raw archive, without timing again.
set -euo pipefail
image_root=$(cd -- "$(dirname -- "$0")" && pwd)
image_replay=${1:?new output directory}
mkdir "$image_replay"
image_replay=$(cd -- "$image_replay" && pwd)
cd "$image_root"
cargo build --locked --release --manifest-path native/Cargo.toml
image_native="$image_root/native/target/release/twist-endomorphism-image-study"
jq --arg root "$image_root/runs/jobs" '.jobs |= map(.out=($root+"/"+.id+"/out"))' runs/reconciliation/cold-sources-final-v1.json > "$image_replay/cold-sources.json"
"$image_native" --image-cold-assemble "$image_replay/cold-sources.json" "$image_replay/cold"
image_audit="$image_root/runs/jobs/20261023-170329-twist-image-audit-29b206/out"
"$image_native" --image-report ../quartic_twist_mixed_v1.ndjson \
 "$image_audit/input/reference" "$image_audit/input/candidate" "$image_audit/audit" \
 "$image_audit/input/double-reference" "$image_audit/input/double-candidate" "$image_audit/audit" \
 "$image_replay/cold" "$image_replay/analysis"
diff -u results/full-stream.csv "$image_replay/analysis/full-stream.csv"
diff -u results/cold-costs.csv "$image_replay/analysis/cold-costs.csv"
diff -u results/paired-cold-costs.csv "$image_replay/analysis/paired-cold-costs.csv"
diff -u results/rank-traces.csv "$image_replay/analysis/rank-traces.csv"
printf 'Raw sets, ranks, solutions, cold phases and all four CSV summaries replayed.\n'
