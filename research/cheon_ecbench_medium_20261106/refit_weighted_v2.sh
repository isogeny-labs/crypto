#!/usr/bin/env bash
# Refit every weighted fit-version-2 Cheon bound record from the options it
# records, so it carries the per-size pricing band that fit version 2 now
# writes on weighted fits too (src/cryptanalysis/ecbench/bounds.rs). The
# sessions, audits, labels, notes, filters, seeds and resamples are the
# record's own; only the band is added. Run from the repository root, with
# the sessions' records.jsonl pulled from the artifact store.
set -euo pipefail
E=${ECBENCH:-./target/release/ecbench}
for r in docs/bounds/records/cheon-ecbench-*-w*.json; do
  [ "$(jq -r '.fit_options.fit_version' "$r")" = 2 ] || continue
  args=()
  while IFS= read -r d; do args+=(--dir "$d"); done < <(jq -r '.provenance.sessions[].dir' "$r")
  while IFS= read -r a; do args+=(--audit "$a"); done < <(jq -r '.provenance.audits[].path' "$r")
  while IFS= read -r c; do args+=(--curve "$c"); done < <(jq -r '.fit_options.curves[]' "$r")
  tier=$(jq -r '.fit_options.tier // empty' "$r"); [ -n "$tier" ] && args+=(--tier "$tier")
  "$E" bound fit "${args[@]}" \
    --arm "$(jq -r '[.provenance.sessions[].arm]|unique|.[0]' "$r")" \
    --label "$(jq -r '.label' "$r")" --notes "$(jq -r '.notes' "$r")" \
    --oracle "$(jq -r '.fit_options.oracle' "$r")" \
    --weight "$(jq -r '.fit_options.weight' "$r")" \
    --resamples "$(jq -r '.fit_options.resamples' "$r")" --seed "$(jq -r '.fit_options.seed' "$r")" \
    --fit-version 2 --root . --out "$r" >/dev/null
  echo "refit $r"
done
