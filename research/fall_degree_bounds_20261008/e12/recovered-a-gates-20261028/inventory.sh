#!/bin/sh
set -eu
jq -e '.id == "20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f"' /work/job.json >/dev/null
jq '{id,deadline}' /work/job.json
cd /work/fall-degree-source/research/fall_degree_bounds_20261008/e7/validate
set -- *.jsonl
[ "$#" -le 32 ]
for path do
  [ -f "$path" ] && [ ! -L "$path" ]
  stat -c '%n %s' "$path"
  sha256sum "$path"
done
