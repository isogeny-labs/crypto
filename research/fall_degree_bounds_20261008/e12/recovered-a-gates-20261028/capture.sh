#!/bin/sh
set -eu
jq -e '.id == "20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f" and .deadline == 1793326711' /work/job.json >/dev/null
cd /work/fall-degree-source/research/fall_degree_bounds_20261008/e7/validate
set -- A_m3_13-5.jsonl inputs_m4_9-3.jsonl m3_5-2.jsonl m3_7-3.jsonl x_m3_11-4.jsonl x_m3_13-4.jsonl x_m4_9-3.jsonl
for path do
  [ -f "$path" ] && [ ! -L "$path" ]
  [ "$(stat -c %s "$path")" -le 2048 ]
done
scratch=$(mktemp -d /var/tmp/e12a-gate-rescue.XXXXXX)
trap 'rm -rf "$scratch"' EXIT
sha256sum "$@" > "$scratch/files.sha256"
tar -czf "$scratch/gates.tar.gz" -- "$@"
sha256sum -c "$scratch/files.sha256" >/dev/null
[ "$(stat -c %s "$scratch/gates.tar.gz")" -le 12000 ]
sha256sum "$scratch/gates.tar.gz" | cut -d ' ' -f 1
base64 -w0 "$scratch/gates.tar.gz"
printf '\n'
