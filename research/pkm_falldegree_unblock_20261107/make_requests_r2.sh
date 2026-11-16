#!/usr/bin/env bash
# Round 2 (slug r2pkm). Usage: make_requests_r2.sh IMAGE_JSON [STAMP]
# 1. runs make_requests.sh IMAGE_JSON (rewrites requests/*.json with the image digest and its source commit);
# 2. writes requests_pending_amendment/*.json: the Amendment R2-B null cells (entry_r2.sh m4n: stage). These are
#    REQUIRES AMENDMENT FREEZE and live outside requests/ so that no glob over requests/ can launch them.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); repo=$(cd "$here/../.." && pwd)
img=${1:?image.json}; stamp=${2:-20261108}
"$here/make_requests.sh" "$img" "$stamp"
image=$(jq -er .image_uri "$img"); rev=$(jq -er .source_revision "$img")
psha=$(sha256sum "$repo/research/prime_fall_degree_20261027/PREREGISTRATION.md" | cut -d' ' -f1)
out=$here/requests_pending_amendment; mkdir -p "$out"; rm -f "$out"/*.json
for k in 0 1; do for tc in 4:7 5:6; do t=${tc%:*}; c=${tc#*:}; name=pfd-m4n-t$t-k$k-c$c
  jq -n --arg name "$name-$stamp" --arg image "$image" --arg rev "$rev" --arg psha "$psha" \
    --arg note "REQUIRES AMENDMENT FREEZE (AMENDMENT_R2_B_m4_null.md): shape-matched null, Kummer m=4 t=$t draw $k hky-cmax $c, --controls null --null-chain. Run after G3 (pfd-a) and after its Kummer counterpart pfd-m4-t$t-k$k-c$c. Do not launch before the amendment is frozen in PREREGISTRATION.md." \
    --args '{schema:"isolab.eks.command-request/v1",
      job:{job_name:$name,image:$image,command:$ARGS.positional,cpu:14,memory:"112Gi",seconds:176400,
           isolation_profile:"standard",retries:0,wait_seconds:300},
      provenance:{harness:"claude-code-subagent",source_revision:$rev,study:"prime_fall_degree",protocol_sha256:$psha,note:$note}}' \
    -- /usr/local/bin/entry_r2.sh "m4n:$t:$k:$c:13958643712:4000000" > "$out/$name.json"
done; done
ls "$out" | wc -l
