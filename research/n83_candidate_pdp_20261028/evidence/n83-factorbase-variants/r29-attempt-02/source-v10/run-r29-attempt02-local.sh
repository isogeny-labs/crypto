#!/usr/bin/env bash
set -euo pipefail
study=/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028
bundle="$study/evidence/n83-factorbase-variants/r29-attempt-02/source-v10"
protocol="$study/protocol-factor-base-variants-v10-r29-retry.json"
protocol_sha="$protocol.sha256"
source_index="$study/factor-base-variant-source-index-v10-r29-retry.json"
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
[[ "$(sha256sum "$binary" | cut -d' ' -f1)" == "$(jq -r '.extensions.binary_sha256' "$protocol")" ]]
[[ "$(sha256sum "$runner" | cut -d' ' -f1)" == "$(jq -r '.analysis_plan.script.runner_sha256' "$protocol")" ]]
mkdir -p "$(dirname "$out")"
exec "$runner" "$binary" "$bit" "$out" "$results_uri" "$protocol"
