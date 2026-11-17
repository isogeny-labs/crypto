#!/usr/bin/env bash
# Validator recompute driver (thin orchestration only; the arithmetic is in
# recompute.jq and headline.jq).  Reads the six committed raw reports, writes
# validation/recompute.json (per instance) and validation/headline.json
# (claimed README table vs recomputed).
#
#   research/fes_then_batched_20261112/validation/recompute.sh
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
study="$(cd "$here/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
for name in koblitz-n13 char2-n15 koblitz-n17 koblitz-n19 char2-n21 koblitz-n23; do
  jq --arg name "$name" -f "$here/recompute.jq" "$study/results/$name.json" >"$tmp/$name.json"
done
jq -s '.' "$tmp"/koblitz-n13.json "$tmp"/char2-n15.json "$tmp"/koblitz-n17.json \
  "$tmp"/koblitz-n19.json "$tmp"/char2-n21.json "$tmp"/koblitz-n23.json >"$here/recompute.json"
jq --slurpfile claimed "$here/claimed-readme-table.json" -f "$here/headline.jq" \
  "$here/recompute.json" >"$here/headline.json"
echo "wrote $here/recompute.json $here/headline.json"
