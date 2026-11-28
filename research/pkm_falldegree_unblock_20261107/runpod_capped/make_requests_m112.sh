#!/usr/bin/env bash
# make_requests_m112.sh -- RETIRED (owner decision 24, 2026-11-21): the record of the request of owner decision 23,
# t = 14, draw 0, cap 7 at Amendment 14's -m112 setting (pfd-x-t14-k0-c7-m112). That cell was found already measured
# (results/SNAPSHOT_20261117: oversize at step 35, censored), and its names are off the allow-list. Kept self-contained
# so the record reproduces: derived from the frozen requests_r2i/pfd-x-t14-k0-c6.json, command[1] cap:14:0:6:12800000000
# -> cap:14:0:7:27900000000, memory 56Gi -> 112Gi, job-name suffix c6 -> c7-m112, one sentence appended to the note.
# The current request is written by make_requests_d54.sh.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); study=$(cd "$here/.." && pwd)
src="$study/requests_r2i/pfd-x-t14-k0-c6.json" dst="$here/requests_m112/pfd-x-t14-k0-c7-m112.json"; rel=${src#"$study/"}
jq -e '.job.memory == "56Gi" and .job.command == ["/usr/local/bin/entry.sh", "cap:14:0:6:12800000000"]
       and (.job.job_name | startswith("pfd-x-t14-k0-c6-"))' "$src" >/dev/null
mkdir -p "$here/requests_m112"
jq --arg rel "$rel" '.job.job_name |= sub("^pfd-x-t14-k0-c6"; "pfd-x-t14-k0-c7-m112")
  | .job.memory = "112Gi"
  | .job.command[1] = "cap:14:0:7:27900000000"
  | .provenance.note += " Amendment 33: cap 7 (Amendment 7 B next step) at Amendment 14 -m112 (112Gi, --max-dense 27900000000, name suffix -m112), derived from \($rel) by runpod_capped/make_requests_m112.sh; other fields unchanged."' "$src" > "$dst"
paths=$(jq -n --slurpfile a "$src" --slurpfile b "$dst" \
  '[ ($a[0] | paths(scalars)) as $p | select(($a[0] | getpath($p)) != ($b[0] | getpath($p))) | $p | map(tostring) | join(".") ] | join(" ")')
added=$(jq -n --slurpfile a "$src" --slurpfile b "$dst" '[ ($b[0] | paths(scalars)) as $p | select(($a[0] | getpath($p)) == null) | $p | map(tostring) | join(".") ] | join(" ")')
echo "$(sha256sum "$dst" | cut -d' ' -f1)  ${dst#"$here/"}  changed: $paths  added: $added  stage: $(jq -r '.job.command[1]' "$dst")  memory: $(jq -r '.job.memory' "$dst")  job_name: $(jq -r '.job.job_name' "$dst")  note length: $(jq '.provenance.note | length' "$dst")"
