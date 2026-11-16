#!/usr/bin/env bash
# Refit every weighted fit-version-2 bound record in docs/bounds/records (the
# Cheon records and any other) from the options it records, so it carries the per-size pricing band that fit version 2 now
# writes on weighted fits too (src/cryptanalysis/ecbench/bounds.rs). The
# sessions, audits, labels, notes, filters, seeds and resamples are the
# record's own; only the band is added. Run from the repository root, with
# the sessions' records.jsonl pulled from the artifact store.
set -euo pipefail
E=${ECBENCH:-./target/release/ecbench}
for r in docs/bounds/records/*.json; do
  jq -e '(.fit_options.weight // null) != null and (.fit_options.fit_version // 1) >= 2' "$r" >/dev/null || continue
  args=()
  while IFS= read -r d; do args+=(--dir "$d"); done < <(jq -r '.provenance.sessions[].dir' "$r")
  while IFS= read -r a; do args+=(--audit "$a"); done < <(jq -r '.provenance.audits[].path' "$r")
  while IFS= read -r c; do args+=(--curve "$c"); done < <(jq -r '.fit_options.curves[]' "$r")
  tier=$(jq -r '.fit_options.tier // empty' "$r"); [ -n "$tier" ] && args+=(--tier "$tier")
  oracle=$(jq -r '.fit_options.oracle // empty' "$r"); [ -n "$oracle" ] && args+=(--oracle "$oracle")
  "$E" bound fit "${args[@]}" \
    --arm "$(jq -r '[.provenance.sessions[].arm]|unique|.[0]' "$r")" \
    --label "$(jq -r '.label' "$r")" --notes "$(jq -r '.notes // ""' "$r")" \
    --weight "$(jq -r '.fit_options.weight' "$r")" \
    --resamples "$(jq -r '.fit_options.resamples' "$r")" --seed "$(jq -r '.fit_options.seed' "$r")" \
    --fit-version 2 --root . --out "$r" >/dev/null
  echo "refit $r"
done
