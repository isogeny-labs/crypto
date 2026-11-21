#!/usr/bin/env bash
# make_requests_m112.sh -- write the one request of Amendment 33 (owner decision 23): t = 14, draw 0, cap 7 at Amendment
# 14's -m112 setting. No frozen pfd-x-t14-k0-c7-m112 request exists on main (searched 2026-11-21: requests*/, runpod/,
# results/ hold none; the other session launched its cap-7 jobs from stage strings), so it is derived from the frozen
# requests_r2i/pfd-x-t14-k0-c6.json by common.sh's c7_m112_request: command[1] cap:14:0:6:12800000000 ->
# cap:14:0:7:27900000000 (Amendment 7 B cap 6 -> 7; Amendment 14 --max-dense), memory 56Gi -> 112Gi, job-name suffix
# c6 -> c7-m112, one sentence appended to provenance.note; every other field copied unchanged. It then prints the paths
# that differ from the source, which must be exactly those four, and the file's SHA-256 (pinned in common.sh).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
mkdir -p "$here/requests_m112"
src="$study/$STUDY_SRC_REL" dst=$STUDY_REQUEST
c7_m112_request "$src" > "$dst"
paths=$(jq -n --slurpfile a "$src" --slurpfile b "$dst" \
  '[ ($a[0] | paths(scalars)) as $p | select(($a[0] | getpath($p)) != ($b[0] | getpath($p))) | $p | map(tostring) | join(".") ] | join(" ")')
added=$(jq -n --slurpfile a "$src" --slurpfile b "$dst" '[ ($b[0] | paths(scalars)) as $p | select(($a[0] | getpath($p)) == null) | $p | map(tostring) | join(".") ] | join(" ")')
echo "$(sha256sum "$dst" | cut -d' ' -f1)  ${dst#"$here/"}  changed: $paths  added: $added  stage: $(jq -r '.job.command[1]' "$dst")  memory: $(jq -r '.job.memory' "$dst")  job_name: $(jq -r '.job.job_name' "$dst")  note length: $(jq '.provenance.note | length' "$dst")"
