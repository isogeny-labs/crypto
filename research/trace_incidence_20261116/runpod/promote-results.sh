#!/usr/bin/env bash
# Administrative custody: verified bytes only, no scientific recomputation.
set -euo pipefail
plan=${1:?promotion request JSON required}
bucket=setec-astronomy-artifacts
stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
while IFS=$'\t' read -r source_key digest bytes; do
  [[ $source_key =~ ^store/runpod/trace-incidence/[a-f0-9]{40}/[0-9]+/results.tar.gz$ ]] || exit 2
  [[ $digest =~ ^[a-f0-9]{64}$ && $bytes =~ ^[0-9]+$ ]] || exit 2
  ((bytes>0&&bytes<=33554432)) || exit 2
  aws s3 cp --only-show-errors "s3://$bucket/$source_key" "$stage/source.tar.gz"
  test "$(stat -c %s "$stage/source.tar.gz")" -eq "$bytes"
  printf '%s  %s\n' "$digest" "$stage/source.tar.gz" | sha256sum -c -
  destination="store/sha256/${digest:0:2}/$digest"
  status=0
  aws s3api put-object --bucket "$bucket" --key "$destination" --body "$stage/source.tar.gz" --if-none-match '*' > "$stage/put.json" 2> "$stage/put.error" || status=$?
  # An existing immutable object is accepted only after the same readback check.
  aws s3 cp --only-show-errors "s3://$bucket/$destination" "$stage/readback.tar.gz"
  printf '%s  %s\n' "$digest" "$stage/readback.tar.gz" | sha256sum -c -
  printf 'TRACE_CAS_PROMOTION='
  jq -nc --arg source "$source_key" --arg destination "$destination" --arg digest "$digest" --argjson bytes "$bytes" --argjson put_status "$status" \
    '{schema:"trace-runpod-cas-promotion/v1",source_key:$source,destination_key:$destination,sha256:$digest,bytes:$bytes,put_cli_exit:$put_status,readback_matches:true}'
done < <(jq -r '.objects[]|[.key,.sha256,(.bytes|tostring)]|@tsv' "$plan")
