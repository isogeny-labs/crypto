#!/usr/bin/env bash
# Writes the isolab.eks.command-request/v1 documents for the row (NOT RUN).
#   make-request.sh IMAGE.json PROTOCOL_SHA256 STAMP
# IMAGE.json is the receipt of the layered study image (fields image_uri,
# source_revision); both must be real before this runs. The Job names are unique
# per STAMP; a rerun is a new experiment, never a retry.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
image=$(jq -er .image_uri "$1"); rev=$(jq -er .source_revision "$1"); psha=$2; stamp=$3
[[ "$rev" =~ ^[a-f0-9]{40}$ && "$psha" =~ ^[a-f0-9]{64}$ ]]
req() { # file job cpu memory seconds instance wait command...
  local file=$1 name=$2 cpu=$3 mem=$4 secs=$5 inst=$6 wait=$7; shift 7
  jq -n --arg name "$name" --arg image "$image" --argjson cpu "$cpu" --arg mem "$mem" --argjson secs "$secs" \
    --arg inst "$inst" --argjson wait "$wait" --arg rev "$rev" --arg psha "$psha" --args \
    '{schema:"isolab.eks.command-request/v1",
      job:({job_name:$name,image:$image,command:$ARGS.positional,cpu:$cpu,memory:$mem,seconds:$secs,
            isolation_profile:"standard",retries:0,wait_seconds:$wait} + (if $inst=="" then {} else {instance_type:$inst} end)),
      provenance:{harness:"claude-code-main-session",source_revision:$rev,study:"prime_ic_algebraic_row",protocol_sha256:$psha,
        note:"research/prime_ic_leads_20261110/algebraic_row_spec/README.md: counted S, command Job (no native L2 admission), wall time descriptive only"}}' \
    -- "$@" > "$here/$file"
  echo "$here/$file"
}
# Job 1: the row (run + in-job replay-all). 2 vCPU is the command-request minimum.
req request-row.json "alg-row-$stamp" 2 4Gi 54000 "" 3000 \
  /opt/study/job-algebraic-row.sh run /opt/study/spec.json row
# Job 2: independent replay on another host class; INSTANCE must differ from job 1's actual node family.
# ROW_RESULTS_PREFIX is job 1's s3://.../runs/<job>/<pod-uid>/ (known only after job 1 finishes).
req request-replay.json "alg-row-replay-$stamp" 2 4Gi 28800 "${REPLAY_INSTANCE:?set REPLAY_INSTANCE to a different x86 family}" 3000 \
  /opt/study/job-algebraic-row.sh replay "${ROW_RESULTS_PREFIX:?set to job 1 results prefix}" row
