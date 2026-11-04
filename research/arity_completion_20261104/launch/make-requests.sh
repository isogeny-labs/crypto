#!/usr/bin/env bash
# Writes the three isolab.eks.command-request/v1 documents for the arity
# completion from launch/image.json (the published study image, by digest) and
# the current commit. Idempotent; the requests are committed next to this
# script so a card can bind them by sha256.
#   make-requests.sh [RESUME_PREFIX_KOBLITZ] [RESUME_PREFIX_PRIME_MAIN] [RESUME_PREFIX_PRIME_T4]
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../../.." && pwd)
image=$(jq -er .image_uri "$here/image.json")
rev=$(git -C "$root" rev-parse HEAD)
kob_sha=$(jq -er .freeze.protocol_sha256 "$here/../koblitz_m5_m6/protocol.json")
pt_sha=$(jq -er .freeze.protocol_sha256 "$here/../prime_tower_m5/protocol.json")
stamp=${ARITY_RUN_STAMP:-20261104}
req() { # file job_name cpu memory seconds profile instance wait study protocol_sha command...
  local file=$1 name=$2 cpu=$3 mem=$4 secs=$5 prof=$6 inst=$7 wait=$8 study=$9 psha=${10}; shift 10
  jq -n --arg name "$name" --arg image "$image" --argjson cpu "$cpu" --arg mem "$mem" --argjson secs "$secs" \
    --arg prof "$prof" --arg inst "$inst" --argjson wait "$wait" --arg rev "$rev" --arg study "$study" --arg psha "$psha" \
    --args '{schema:"isolab.eks.command-request/v1",
      job:({job_name:$name,image:$image,command:$ARGS.positional,cpu:$cpu,memory:$mem,seconds:$secs,
            isolation_profile:$prof,retries:0,wait_seconds:$wait} + (if $inst=="" then {} else {instance_type:$inst} end)),
      provenance:{harness:"claude-code-main-session",source_revision:$rev,study:$study,protocol_sha256:$psha,
        note:"research/arity_completion_20261104/launch/README.md: command Job on Iso Lab EKS; no L2 admission exists for command Jobs, so seconds are diagnostic only"}}' \
    -- "$@" > "$here/$file"
  echo "$file"
}
req request-koblitz.json "arity-koblitz-m56-$stamp" 4 12Gi 60900 l2 c8a.2xlarge 600 \
  koblitz_m5_m6 "$kob_sha" /opt/study/launch/job-koblitz.sh ${1:+"$1"}
req request-prime-main.json "arity-prime-m5-main-$stamp" 2 10Gi 42000 standard "" 600 \
  prime_tower_m5 "$pt_sha" /opt/study/launch/job-prime-tower.sh main ${2:+"$2"}
req request-prime-t4.json "arity-prime-m5-t4-$stamp" 2 120Gi 174600 standard "" 300 \
  prime_tower_m5 "$pt_sha" /opt/study/launch/job-prime-tower.sh t4 ${3:+"$3"}
