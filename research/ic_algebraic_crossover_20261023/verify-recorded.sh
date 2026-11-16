#!/usr/bin/env bash
# Verify archived evidence in a fresh scratch directory; never overwrite runs.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
(cd "$study/evidence" && sha256sum -c SHA256SUMS)
tar -xJf "$study/evidence/runs-local-20261023.tar.xz" -C "$scratch"
cargo build --release --locked --example ic_campaign_report_data \
  --example ic_structured_small_audit --example ic_guess_symmetry
bin_root=${CARGO_TARGET_DIR:-target}/release/examples
"$bin_root/ic_campaign_report_data" "$scratch" > "$scratch/report-data.json"
cmp "$study/report-data.json" "$scratch/report-data.json"
"$bin_root/ic_structured_small_audit" > "$scratch/small-support.json"
if cmp -s "$study/pipeline-receipts/structured-n7-support.json" "$scratch/small-support.json"; then
  printf '%s\n' 'Finite n7 support matches its original source-bound receipt.'
else
  # The merged base changed ic_boundary.rs. Keep its new source hash in a
  # separate receipt; never normalize it away or rewrite the original.
  cmp "$study/pipeline-receipts/structured-n7-support-new-main-20261025.json" "$scratch/small-support.json"
  printf '%s\n' 'Finite n7 support matches the separately source-bound merged-base receipt.'
fi
"$bin_root/ic_guess_symmetry" "$study/inputs/guess-ell8.jsonl" \
  "$scratch/identity.json" "$scratch/symmetry.json"
cmp "$study/proofs/s3-swap.identity.json" "$scratch/identity.json"
cmp "$study/proofs/guess-swap.receipt.json" "$scratch/symmetry.json"
printf 'PASS: archive hashes, native raw-data recomputation, support audit and identity replay\n'
