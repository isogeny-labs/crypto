#!/usr/bin/env bash
# Record one typed, provenance-bearing curve, factor-base or FBM1 observation.
set -euo pipefail

request=${1:?usage: record-feature-api.sh REQUEST.json}
: "${ECDB_URL:?set ECDB_URL}"
: "${ECDB_WRITE_TOKEN:?set ECDB_WRITE_TOKEN}"
command -v curl >/dev/null
command -v jq >/dev/null
jq -e '
  .feature_key and .agent.name and
  ([has("curve"),has("factor_base"),has("fbm_id")] | map(select(.)) | length == 1) and
  ((has("value")) or (has("missing_reason")))' "$request" >/dev/null

curl --fail-with-body --silent --show-error \
  --header 'content-type: application/json' \
  --header 'accept: application/json' \
  --header "authorization: Bearer ${ECDB_WRITE_TOKEN}" \
  --data-binary "@$request" \
  "${ECDB_URL%/}/v1/features/observations"
printf '\n'
