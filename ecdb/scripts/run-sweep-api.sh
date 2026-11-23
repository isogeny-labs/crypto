#!/usr/bin/env bash
# Resolve every cell of one checked-in ecdb.research_sweep/v1 manifest through
# the read API. Output is JSONL: one snapshot envelope per cell. This selects
# candidates; it does not claim measurements or freeze membership.
set -euo pipefail

manifest=${1:?usage: run-sweep-api.sh MANIFEST [OUTPUT.jsonl]}
output=${2:-/dev/stdout}
: "${ECDB_URL:?set ECDB_URL to the ecdb API base URL}"

command -v curl >/dev/null
command -v jq >/dev/null
command -v sha256sum >/dev/null
jq -e '.schema == "ecdb.research_sweep/v1" and (.cells | type == "array")' "$manifest" >/dev/null

headers=(--fail-with-body --silent --show-error --header 'accept: application/json')
if [[ -n ${ECDB_TOKEN:-} ]]; then headers+=(--header "authorization: Bearer ${ECDB_TOKEN}"); fi

name=$(jq -r .name "$manifest")
version=$(jq -r .version "$manifest")
sort=$(jq -r '.selection_policy.order_by // "icv1_slug"' "$manifest")
order=$(jq -r '.selection_policy.order // "asc"' "$manifest")
excluded_families=$(jq -c '.selection_policy.exclude_families // []' "$manifest")
exclude_supersingular=$(jq -r '.selection_policy.exclude_supersingular // false' "$manifest")
manifest_sha256=$(sha256sum "$manifest" | awk '{print $1}')
tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT
: >"$tmpdir/results.jsonl"

while IFS= read -r cell; do
  key=$(jq -r .key <<<"$cell")
  target=$(jq -r '.target_curves // 1 | if . > 1000 then 1000 else . end' <<<"$cell")
  fetch_limit=$target
  if [[ $excluded_families != '[]' || $exclude_supersingular == true ]]; then
    fetch_limit=$((target * 4))
    if ((fetch_limit > 1000)); then fetch_limit=1000; fi
  fi
  query=$(jq -r '.curve_filter | to_entries | map("\(.key | @uri)=\(.value | tostring | @uri)") | join("&")' <<<"$cell")
  raw=$(curl "${headers[@]}" "${ECDB_URL%/}/v1/curves?${query}&limit=${fetch_limit}&sort=${sort}&order=${order}")
  selected=$(jq -c --argjson excluded "$excluded_families" --argjson reject_ss "$exclude_supersingular" --argjson target "$target" '
    [.items[] | select((.family as $f | ($excluded | index($f)) | not) and ($reject_ss == false or .is_supersingular != true))]
    | sort_by(.field_bits, .icv1_slug, .id) | .[:$target]' <<<"$raw")
  jq -cn --arg schema 'ecdb.sweep_cell_snapshot/v1' --arg sweep "$name" --argjson version "$version" \
    --arg manifest_sha256 "$manifest_sha256" --arg cell_key "$key" --argjson cell "$cell" \
    --argjson raw_total "$(jq '.total' <<<"$raw")" --argjson selected "$selected" \
    '{schema:$schema,sweep:$sweep,version:$version,manifest_sha256:$manifest_sha256,cell_key:$cell_key,
      cell:$cell,raw_total:$raw_total,selected_count:($selected|length),items:$selected}' \
    >>"$tmpdir/results.jsonl"
done < <(jq -c '.cells[]' "$manifest")

if [[ $output == /dev/stdout ]]; then
  command cat "$tmpdir/results.jsonl"
else
  install -m 0644 "$tmpdir/results.jsonl" "$output"
  printf '%s\n' "$output"
fi
