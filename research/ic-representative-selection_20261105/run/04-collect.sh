#!/usr/bin/env bash
# Stage 1b: download one finished job's S3 payload and verify every hash, then check
# the receipt binds the request, image and binary. Fails closed.
#   run/04-collect.sh <job-name>
source "$(dirname "$0")/common.sh"
job=$1
r=$D/receipts/$job/result.json
jq -e '.state=="succeeded" and .exit_code==0' "$r" > /dev/null
uri=$(jq -er .results_uri "$r")
dest=$D/sessions/$job
mkdir -p "$dest"
aws s3 cp --recursive "$uri" "$dest" --only-show-errors
( cd "$dest" && sha256sum -c --quiet SHA256SUMS )
jq -e '.accepted' "$dest/l2-gate.json" > /dev/null
jq -e '.ok' "$dest/audit.json" > /dev/null
[[ $(jq -er .ecbench_sha256 $D/inputs/image.json) == $(cut -d' ' -f1 "$dest/binary.sha256") ]]
cmp <(jq -S . "$dest/input-spec.json") <(jq -S . "$(find $D/inputs/specs -name "$job.json")")
echo "$job collected"
