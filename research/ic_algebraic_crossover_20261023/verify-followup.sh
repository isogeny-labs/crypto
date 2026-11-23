#!/usr/bin/env bash
# Pure native reconstruction from immutable archived outputs, no experiments.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
study="$root/research/ic_algebraic_crossover_20261023"
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
for archive in runs-local-followup-20261025-01.tar.xz runs-instruction-profiles-n13-20261025-01.tar.xz; do
  (cd "$study/evidence" && sha256sum -c "$archive.sha256")
  tar -xJf "$study/evidence/$archive" -C "$scratch"
done
mkdir -p "$scratch/inputs" "$scratch/pipeline-runs"
cp -R "$study/inputs/fresh" "$scratch/inputs/"
cp -R "$study/pipeline-runs/structured-n13-20261025-01" "$scratch/pipeline-runs/"
cd "$root"
cargo build --release --example ic_hky_probe_summary --example ic_instruction_profile_audit
bin_root=${CARGO_TARGET_DIR:-target}/release/examples
"$bin_root/ic_hky_probe_summary" "$scratch" > "$scratch/hky.json"
cmp "$scratch/hky.json" "$study/pipeline-receipts/hky-probes-20261025/receipt.json"
"$bin_root/ic_instruction_profile_audit" "$scratch" "$scratch/runs/instruction-profiles-n13-20261025-01" > "$scratch/profiles.json"
cmp "$scratch/profiles.json" "$study/pipeline-receipts/instruction-profiles-n13-20261025-01/receipt.json"
cmp "$scratch/runs/guess-cap200k-20261025-01/cell-0000-d6-k0-b0/rows.jsonl" "$study/pipeline-receipts/guess-zero-draw6-20261025/row.json"
printf '%s\n' 'PASS: archived HKY inputs/statuses, profile scalars/counters/parts and original pilot row reproduce.'
