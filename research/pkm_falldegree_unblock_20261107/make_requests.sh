#!/usr/bin/env bash
# Writes requests/*.json: isolab.eks.command-request/v1 documents for the prime fall-degree study.
#   make_requests.sh [IMAGE_JSON] [STAMP]
# With no IMAGE_JSON (written by image/build_layer.sh) it uses a PLACEHOLDER image digest (64 zeros) and
# the current HEAD of /home/user/crypto as source_revision, so the documents validate but are not launchable.
# With IMAGE_JSON it takes .image_uri and .source_revision from it (the image's own source commit).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); repo=$(cd "$here/../.." && pwd)
repository=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks
if [[ -n "${1:-}" ]]; then image=$(jq -er .image_uri "$1"); rev=$(jq -er .source_revision "$1"); ph=''
else image="$repository@sha256:$(printf '0%.0s' $(seq 64))"; rev=$(git -C "$repo" rev-parse HEAD)
     ph='PLACEHOLDER image digest (no image exists for this revision); replace via make_requests.sh IMAGE_JSON. '; fi
stamp=${2:-20261108}
psha=$(sha256sum "$repo/research/prime_fall_degree_20261027/PREREGISTRATION.md" | cut -d' ' -f1)
mkdir -p "$here/requests"; rm -f "$here"/requests/*.json
req() { # name cpu memory seconds note command...
  local name=$1 cpu=$2 mem=$3 secs=$4 note=$5; shift 5
  jq -n --arg name "$name-$stamp" --arg image "$image" --argjson cpu "$cpu" --arg mem "$mem" --argjson secs "$secs" \
    --arg rev "$rev" --arg psha "$psha" --arg note "$ph$note" --args \
    '{schema:"isolab.eks.command-request/v1",
      job:{job_name:$name,image:$image,command:$ARGS.positional,cpu:$cpu,memory:$mem,seconds:$secs,
           isolation_profile:"standard",retries:0,wait_seconds:300},
      provenance:{harness:"claude-code-subagent",source_revision:$rev,study:"prime_fall_degree",
                  protocol_sha256:$psha,note:$note}}' -- "$@" > "$here/requests/$name.json"
}
E=/usr/local/bin/entry.sh
# Registered cells (run.sh), unchanged; Amendment 2 chains. 14 vCPU fits a 16-vCPU node; pool cap is 16 vCPU / 128 GiB: one job at a time.
req pfd-a 14 56Gi 172800 "stage A + F4 chain: g1 hky-A g3 d-A cap-A. seconds is a proposal (worst-case sum of per-system budgets ~3.8e5 s; the proposed 172800 s is below it)." $E g1 hky-A g3 d-A cap-A
req pfd-c 14 56Gi  86400 "stage C: g1 g4 hky-C g3. seconds is a proposal." $E g1 g4 hky-C g3
req pfd-b 14 56Gi 172800 "stage B: g1 hky-B d-B g3 (LAUNCH.md omitted B). m=3 N=21 / m=4 N=20 closures hit the program's own max_entries and read censored. seconds is a proposal." $E g1 hky-B d-B g3
# Amendment 7 A/B: one capped F4 run per job. DENSE = largest multiple of 1e8 with 4*C <= request - 8 GiB.
cap() { # t k c mem dense
  req "pfd-x-t$1-k$2-c$3" 14 "$4" 90000 "Amendment 7 B bracket step: Kummer m=2 P1 t=$1 draw $2 cap $3, --budget 86400 --max-dense $5. Run only after G3 passed (pfd-a)." $E "cap:$1:$2:$3:$5"
}
for k in 0 1; do
  cap 12 $k 5 56Gi 12800000000;  cap 12 $k 6 56Gi 12800000000
  cap 13 $k 5 112Gi 27900000000; cap 13 $k 6 112Gi 27900000000
  cap 14 $k 6 112Gi 27900000000
done
# Amendment 8 (m=4): max_entries = floor((112Gi-8GiB)/8B) = 13958643712, max_cols 4e6, budget 172800.
m4() { req "pfd-m4-t$1-k$2-c$3" 14 112Gi 176400 "Amendment 8 m=4 closure: t=$1 draw $2 hky-cmax $3 --budget 172800. Run after G3. t=5 c=7 and t=6 are NOT generated: projected 80-510 GB exceeds the 128 GiB pool." $E "m4:$1:$2:$3:13958643712:4000000"; }
for k in 0 1; do m4 4 $k 7; m4 5 $k 6; done
ls "$here/requests" | wc -l
