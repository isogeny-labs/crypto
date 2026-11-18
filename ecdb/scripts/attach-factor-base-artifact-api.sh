#!/usr/bin/env bash
# Link an already-uploaded, content-addressed object to an existing factor base.
# The endpoint records the URI; it does not upload or remotely verify bytes.
set -euo pipefail

request=${1:?usage: attach-factor-base-artifact-api.sh REQUEST.json}
: "${ECDB_URL:?set ECDB_URL}"
: "${ECDB_WRITE_TOKEN:?set ECDB_WRITE_TOKEN}"
command -v curl >/dev/null
command -v jq >/dev/null
jq -e '.factor_base and .role and .sha256 and .storage_uri and .agent.name' "$request" >/dev/null

curl --fail-with-body --silent --show-error \
  --header 'content-type: application/json' \
  --header 'accept: application/json' \
  --header "authorization: Bearer ${ECDB_WRITE_TOKEN}" \
  --data-binary "@$request" \
  "${ECDB_URL%/}/v1/factor-base-artifacts"
printf '\n'
