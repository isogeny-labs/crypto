#!/usr/bin/env bash
set -euo pipefail
study=/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028
bundle="$study/evidence/n83-factorbase-variants/r29-attempt-02/source-v12"
protocol="$study/protocol-factor-base-variants-v12-r29-retry.json"
protocol_sha="$protocol.sha256"
source_index="$study/factor-base-variant-source-index-v12-r29-retry.json"
source_index_sha="$source_index.sha256"
binary="$study/target/release/factor_base"
runner="$study/run-base-variant-v2.sh"
bit=29
out=/tmp/n83-candidate-attempts/r29-attempt-02
results_uri=s3://isolab-results-104186811093-us-east-2/runs/n83-fb-variants-r29-20261031-r2/
[[ ! -e "$out" ]] || { echo "fresh output directory required: $out" >&2; exit 2; }
(cd "$study" && sha256sum -c "$(basename "$protocol_sha")")
(cd "$study" && sha256sum -c "$(basename "$source_index_sha")")
checks=$(mktemp)
trap 'rm -f "$checks"' EXIT
jq -r '.entries[] | "\(.sha256)  \(.path)"' "$source_index" > "$checks"
(cd "$bundle" && sha256sum -c "$checks")
actual_binary=$(sha256sum "$binary" | cut -d' ' -f1)
expected_binary=$(jq -r '.extensions.binary_sha256' "$protocol")
[[ "$actual_binary" == "$expected_binary" ]] || { echo "instrument binary hash mismatch: $actual_binary != $expected_binary" >&2; exit 3; }
actual_runner=$(sha256sum "$runner" | cut -d' ' -f1)
expected_runner=$(jq -r '.analysis_plan.script.runner_sha256' "$protocol")
[[ "$actual_runner" == "$expected_runner" ]] || { echo "runner hash mismatch: $actual_runner != $expected_runner" >&2; exit 3; }
if [[ "${VERIFY_ONLY:-0}" == 1 ]]; then
  echo "v11 prelaunch identities verified; output directory has not been created"
  exit 0
fi
mkdir -p "$(dirname "$out")"
exec "$runner" "$binary" "$bit" "$out" "$results_uri" "$protocol"
