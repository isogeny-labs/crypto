#!/usr/bin/env bash
# make_requests_d54.sh -- write the one request of Amendment 33 (owner decision 24): t = 14, draw 0, cap 7 at
# --max-dense 54000000000 on the 256 GB pod (t14_bigmem/DRAFT_AMENDMENT.md). No frozen request for this job exists, so it
# is derived from the frozen requests_r2i/pfd-x-t14-k0-c6.json by common.sh's d54_request: command[1]
# cap:14:0:6:12800000000 -> cap:14:0:7:54000000000, memory 56Gi -> 238Gi, job-name suffix c6 -> c7-d54, one sentence
# appended to provenance.note; every other field copied unchanged. It prints the paths that differ from the source,
# which must be exactly those four, and the file's SHA-256 (pinned in common.sh).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
mkdir -p "$here/requests_d54"
src="$study/$STUDY_SRC_REL" dst=$STUDY_REQUEST
d54_request "$src" > "$dst"
paths=$(jq -n --slurpfile a "$src" --slurpfile b "$dst" \
  '[ ($a[0] | paths(scalars)) as $p | select(($a[0] | getpath($p)) != ($b[0] | getpath($p))) | $p | map(tostring) | join(".") ] | join(" ")')
added=$(jq -n --slurpfile a "$src" --slurpfile b "$dst" '[ ($b[0] | paths(scalars)) as $p | select(($a[0] | getpath($p)) == null) | $p | map(tostring) | join(".") ] | join(" ")')
echo "$(sha256sum "$dst" | cut -d' ' -f1)  ${dst#"$here/"}  changed: $paths  added: $added  stage: $(jq -r '.job.command[1]' "$dst")  memory: $(jq -r '.job.memory' "$dst")  job_name: $(jq -r '.job.job_name' "$dst")  note length: $(jq '.provenance.note | length' "$dst")"
