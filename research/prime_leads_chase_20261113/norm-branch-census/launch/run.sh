#!/bin/sh
# Frozen non-timing payload for the Iso Lab command Job.
set -eu

if [ "$#" -ne 2 ]; then
  echo 'usage: run.sh SOURCE_REVISION PROTOCOL_SHA256' >&2
  exit 2
fi

source_revision=$1
protocol_sha256=$2
output=${ISOLAB_OUTPUT_DIR:?}
results_uri=${ISOLAB_RESULTS_URI:?}
mkdir -p "$output"

finish() {
  status=$?
  trap - EXIT
  printf '{"schema":"norm.branch-census-exit/v1","exit_status":%s}\n' "$status" \
    > "$output/job-exit.json"
  (
    cd "$output"
    rm -f SHA256SUMS
    for file in *; do
      [ -f "$file" ] || continue
      sha256sum "$file"
    done > SHA256SUMS
  )
  if ! aws s3 cp "$output/" "$results_uri" --recursive --only-show-errors; then
    [ "$status" -ne 0 ] && exit "$status"
    exit 91
  fi
  exit "$status"
}
trap finish EXIT
trap 'exit 143' TERM
trap 'exit 130' INT

cp /opt/isolab/build-info.json "$output/build-info.json"
cp /opt/isolab/norm-branch/protocol.json "$output/protocol.json"

actual_protocol=$(sha256sum "$output/protocol.json" | cut -d' ' -f1)
[ "$actual_protocol" = "$protocol_sha256" ]

producer_sha=$(sha256sum /usr/local/bin/norm-branch-produce | cut -d' ' -f1)
checker_sha=$(sha256sum /usr/local/bin/norm-branch-check | cut -d' ' -f1)
jq -e --arg revision "$source_revision" --arg protocol "$protocol_sha256" \
  --arg producer "$producer_sha" --arg checker "$checker_sha" \
  '.source_revision==$revision
    and .norm_branch_protocol_sha256==$protocol
    and .norm_branch_produce_sha256==$producer
    and .norm_branch_check_sha256==$checker' \
  "$output/build-info.json" >/dev/null

printf '%s  /usr/local/bin/norm-branch-produce\n%s  /usr/local/bin/norm-branch-check\n' \
  "$producer_sha" "$checker_sha" > "$output/binaries.sha256"

/usr/local/bin/norm-branch-produce "$output/artifact.json" \
  "$source_revision" "$protocol_sha256" > "$output/producer.stdout" 2> "$output/producer.stderr"
/usr/local/bin/norm-branch-check "$output/artifact.json" "$output/checker.json" \
  > "$output/checker.stdout" 2> "$output/checker.stderr"

jq -e --arg revision "$source_revision" --arg protocol "$protocol_sha256" \
  '.schema=="norm.branch-census/v1" and .source_revision==$revision
    and .protocol_sha256==$protocol and (.cells|length)==40' \
  "$output/artifact.json" >/dev/null
jq -e --arg revision "$source_revision" --arg protocol "$protocol_sha256" \
  '.schema=="norm.branch-census-check/v1" and .source_revision==$revision
    and .protocol_sha256==$protocol and .pass==true and (.errors|length)==0' \
  "$output/checker.json" >/dev/null

producer_verdict=$(grep -o 'verdict=[A-Z_]*' "$output/producer.stdout")
checker_verdict=$(grep -o 'verdict=[A-Z_]*' "$output/checker.stdout")
[ -n "$producer_verdict" ]
[ "$producer_verdict" = "$checker_verdict" ]
