#!/bin/bash
# request-a<N>.json (isolab.eks.command-request/v1) from image.json.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); attempt=${1:?attempt}
jq -n --arg name "scanp-main-epyc-20261111-a$attempt" --arg image "$(jq -er .image_uri "$here/image.json")" \
  --arg rev "$(jq -er .build_info.launch_revision "$here/image.json")" \
  '{schema:"isolab.eks.command-request/v1",
    job:{job_name:$name,image:$image,command:["/opt/scanp/job.sh"],cpu:4,memory:"12Gi",seconds:10800,
         isolation_profile:"l2",instance_type:"c8a.2xlarge",retries:0,wait_seconds:600},
    provenance:{harness:"claude-code-main-session",source_revision:$rev,
      study:"research/ic_tool_program/explorations/scan-stages-main-epyc-20261111",
      note:"stage diagnostic (R04 probes) on the EPYC class rule v9 ran on; a record, not a round; no L2 admission for command Jobs"}}' \
  > "$here/request-a$attempt.json"
echo "$here/request-a$attempt.json"
