#!/usr/bin/env bash
# make_requests_m112.sh -- write Amendment 14's -m112 requests for the cells of draft Amendment 18 (decision D5).
# Each is derived from its frozen requests_r2i/ request by common.sh's m112_request: memory 112Gi, --max-dense
# 27900000000 (Amendment 7 A's formula at 112Gi), job-name suffix -m112, a sentence appended to provenance.note; every
# other field copied unchanged. It then prints the jq -S diff of each pair, which must list exactly those four paths.
# The t = 13 draw 0 cell is included because Amendment 14 says a further attempt of it "runs under it".
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd); . "$here/common.sh"
mkdir -p "$here/requests_m112"
for c in t14-k0-c6 t14-k1-c6 t13-k0-c6; do
  src="$study/requests_r2i/pfd-x-$c.json" dst="$here/requests_m112/pfd-x-$c-m112.json"
  m112_request "$src" > "$dst"
  paths=$(jq -n --slurpfile a "$src" --slurpfile b "$dst" \
    '[ ($a[0] | paths(scalars)) as $p | select(($a[0] | getpath($p)) != ($b[0] | getpath($p))) | $p | map(tostring) | join(".") ] | join(" ")')
  echo "$(sha256sum "$dst" | cut -d' ' -f1)  ${dst#"$here/"}  changed: $paths  stage: $(jq -r '.job.command[1]' "$dst")"
done
