#!/usr/bin/env bash
# Resolve a saved dataset recipe and execute its read-only selection query.
# Output is a JSON envelope with the exact recipe, columns and rows; applying
# and freezing the recipe's split policy is a separate producer step.
set -euo pipefail

name=${1:?usage: export-dataset-api.sh DATASET [OUTPUT.json] [LIMIT]}
output=${2:-/dev/stdout}
limit=${3:-5000}
: "${ECDB_URL:?set ECDB_URL}"
[[ $limit =~ ^[1-9][0-9]*$ ]] || { printf 'LIMIT must be a positive integer\n' >&2; exit 2; }
if ((limit > 5000)); then printf 'LIMIT may not exceed the API maximum of 5000\n' >&2; exit 2; fi

command -v curl >/dev/null
command -v jq >/dev/null
headers=(--fail-with-body --silent --show-error --header 'accept: application/json')
if [[ -n ${ECDB_TOKEN:-} ]]; then headers+=(--header "authorization: Bearer ${ECDB_TOKEN}"); fi

encoded_name=$(jq -rn --arg v "$name" '$v|@uri')
recipe=$(curl "${headers[@]}" "${ECDB_URL%/}/v1/datasets/${encoded_name}")
selection=$(jq -er .selection_query <<<"$recipe")
sql="SELECT * FROM (${selection}) AS dataset_export LIMIT ${limit}"
payload=$(jq -cn --arg sql "$sql" --argjson limit "$limit" '{sql:$sql,limit:$limit}')
rows=$(curl "${headers[@]}" --header 'content-type: application/json' --data-binary "$payload" "${ECDB_URL%/}/v1/sql")
envelope=$(jq -cn --arg schema 'ecdb.dataset_export/v1' --argjson recipe "$recipe" --argjson result "$rows" \
  '{schema:$schema,dataset:{name:$recipe.name,version:$recipe.version,subject_scope:$recipe.subject_scope,
    definition_sha256:$recipe.definition_sha256,selection_query:$recipe.selection_query,split_policy:$recipe.split_policy,
    seed:$recipe.seed,features:$recipe.features},result:$result}')

if [[ $output == /dev/stdout ]]; then
  printf '%s\n' "$envelope"
else
  tmpdir=$(mktemp -d)
  trap 'rm -rf "$tmpdir"' EXIT
  printf '%s\n' "$envelope" >"$tmpdir/export.json"
  install -m 0644 "$tmpdir/export.json" "$output"
  printf '%s\n' "$output"
fi
