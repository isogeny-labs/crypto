#!/usr/bin/env bash
# Bind the six requests to the image built from the commit that lands this
# protocol (PROTOCOL.md section 11), then validate each with the repository's
# own validator. Writes requests/final/J*.request.json; the drafts stay as
# they are. Run from the repository root:
#   research/prime_medium_tier_rho_20261107/finalize_requests.sh \
#     104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:<64 hex> \
#     <40-hex source commit> <64-hex ecbench sha256>
set -euo pipefail
[[ $# == 3 ]] || { echo "usage: $0 IMAGE_URI SOURCE_REVISION ECBENCH_SHA256" >&2; exit 2; }
D=research/prime_medium_tier_rho_20261107
V=taskq/deploy/eks-karpenter/remote/validate-request
mkdir -p "$D/requests/final"
for j in J1 J2 J3 J4 J5 J6; do
  out="$D/requests/final/$j.request.json"
  jq --arg image "$1" --arg rev "$2" --arg bin "$3" '
    .job.build = {image_uri: $image, source_revision: $rev, ecbench_sha256: $bin}
    | .provenance.source_revision = $rev' "$D/requests/$j.request.json" > "$out"
  # The spec inside must be byte-for-byte the committed spec.
  cmp <(jq -S .job.spec "$out") <(jq -S . "$D/specs/$j.json")
  "$V" "$out" > /dev/null
  printf '%s %s\n' "$(sha256sum "$out" | cut -d' ' -f1)" "$out"
done
