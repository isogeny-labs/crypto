#!/bin/bash
# Writes request.json (isolab.eks.command-request/v1) from image.json and the
# launch commit.  A Spot loss is resumed by a new request whose job name has
# the next attempt number and whose command carries the lost pod's S3 prefix:
#   make-request.sh <attempt> [<resume prefix>]
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
attempt=${1:?attempt number}
image=$(jq -er .image_uri "$here/image.json")
rev=$(jq -er .build_info.launch_revision "$here/image.json")
psha=$(sha256sum "$here/../PROTOCOL.md" | cut -d' ' -f1)
jq -n --arg name "rule-v9-20261106-a$attempt" --arg image "$image" --arg rev "$rev" --arg psha "$psha" \
  --args '{schema:"isolab.eks.command-request/v1",
    job:{job_name:$name,image:$image,command:$ARGS.positional,cpu:4,memory:"12Gi",seconds:43200,
         isolation_profile:"l2",instance_type:"c8a.2xlarge",retries:0,wait_seconds:600},
    provenance:{harness:"claude-code-main-session",source_revision:$rev,study:"research/ic_tool_program/rule/v9",
      protocol_sha256:$psha,
      note:"rule v9 and its control v9c on Iso Lab EKS (launch/README.md); command Jobs have no L2 admission, so the in-process pairing of IC and matched rho is the comparison"}}' \
  -- /opt/rulev9/job.sh ${2:+"$2"} > "$here/request-a$attempt.json"
echo "$here/request-a$attempt.json"
