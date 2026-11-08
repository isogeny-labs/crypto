#!/usr/bin/env bash
# make-requests.sh [ATTEMPT] -- writes the isolab.eks.command-request/v1 documents for P1 round 3
# (PROTOCOL-R3.md) from image.json (the published study image, by digest). ATTEMPT (default a1)
# names a resubmission after a Spot loss (a2, a3; at most two per system, PROTOCOL-R3).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
att=${1:-a1}
image=$(jq -er .image_uri "$here/image.json")
rev=$(jq -er .source_revision "$here/build-info.json"); [[ "$rev" =~ ^[a-f0-9]{40}$ ]]
psha=$(sha256sum "$here/../PROTOCOL-R3.md" | cut -d' ' -f1)
req() { # file name cpu mem secs wait command...
  local file=$1 name=$2 cpu=$3 mem=$4 secs=$5 wait=$6; shift 6
  jq -n --arg name "$name" --arg image "$image" --argjson cpu "$cpu" --arg mem "$mem" --argjson secs "$secs" \
    --argjson wait "$wait" --arg rev "$rev" --arg psha "$psha" --args \
    '{schema:"isolab.eks.command-request/v1",
      job:{job_name:$name,image:$image,command:$ARGS.positional,cpu:$cpu,memory:$mem,seconds:$secs,
           isolation_profile:"standard",retries:0,wait_seconds:$wait},
      provenance:{harness:"claude-code-main-session",source_revision:$rev,study:"split_prime_ecdlp_20261107/p1_rr_dickson",
        protocol_sha256:$psha,note:"PROTOCOL-R3.md: T24 m5 t4 to completion; command Job, no L2 admission; wall time diagnostic only"}}' \
    -- "$@" > "$here/$file"
  echo "$file"
}
req request-smoke.json "p1r3-smoke-20261108" 2 4Gi 1800 900 /bin/bash -c \
  'set -x; aws --version; echo 1+1 | bc; mkdir -p /work/results; P1TOOL=/usr/local/bin/p1tool OUT_DIR=/work/results/smoke VLIMIT=3000000 TLIMIT=20 bash /opt/study/calibration/run_cells_r3.sh rr; rc=$?; aws s3 cp /work/results/ "$ISOLAB_RESULTS_URI" --recursive --only-show-errors; exit $rc'
req "request-rr-$att.json" "p1r3-rr-$att-20261108" 2 64Gi 100800 600 /opt/study/launch/job-r3.sh rr
req "request-t-$att.json" "p1r3-t-$att-20261108" 2 60Gi 100800 600 /opt/study/launch/job-r3.sh t
