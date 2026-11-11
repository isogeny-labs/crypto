#!/usr/bin/env bash
# Kept for docs/bounds/records repair 2026-11-11 (BOUNDS-REPAIR.md). Usage: OUT=dir research/prime_medium_rho_cpc_20261109/regen_bound_records.sh
# Re-fit the six prime-medium CPC bound records with their recorded inputs,
# citing the committed audit receipts. Run from the repository root.
set -euo pipefail
OUT=${OUT:?}
for f in docs/bounds/records/prime-medium-cpc-rho-*.json; do
  args=()
  while read -r d; do args+=(--dir "$d"); done < <(jq -r '.provenance.sessions[].dir' "$f")
  while read -r a; do args+=(--audit "$a"); done < <(jq -r '.provenance.audits[].path' "$f")
  arm=$(jq -r '.provenance.sessions[0].arm' "$f")
  label=$(jq -r '.label' "$f")
  notes=$(jq -r '.notes' "$f")
  w=$(jq -r '.fit_options.weight // empty' "$f")
  wa=(); [ -n "$w" ] && wa=(--weight "$w")
  jq -e '.fit_options | .tier==null and .curves==[] and .resamples==2000 and .seed==20261005 and .fit_version==2 and (.unit==null) and (.param_rule==null) and (.oracle==null)' "$f" >/dev/null
  ./target/release/ecbench bound fit "${args[@]}" --arm "$arm" --label "$label" --notes "$notes" "${wa[@]}" \
    --resamples 2000 --seed 20261005 --fit-version 2 --root . --out "$OUT/$(basename "$f")"
done
