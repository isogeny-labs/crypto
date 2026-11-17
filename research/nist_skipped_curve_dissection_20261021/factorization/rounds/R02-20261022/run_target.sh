#!/usr/bin/env bash
# Serialize a lineage so another launcher can safely reuse its finished search.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
id=$1 out=$2 hint=$3
[[ $id =~ ^[a-zA-Z0-9_-]+$ ]]
mkdir -p "$(dirname -- "$out")"
exec 9> "$out.lock"
flock -x 9
if [[ -f $out/result.json && -f $out/accounting.json ]]; then
  expected=$(jq -r --arg id "$id" '.[]|select(.id==$id)|.value' "$here/baseline-results.json")
  jq -e --arg id "$id" --arg expected "$expected" \
    '.id==$id and .value==$expected and .product_verified and
     .known_factors_prime and .certificates_replay_valid and .verification.verified' \
    "$out/result.json" >/dev/null
  echo "Reused completed search record: $id ($(jq -r .status "$out/result.json"))"
  exit 0
fi
bash "$here/run_target_search.sh" "$id" "$out" "$hint"
