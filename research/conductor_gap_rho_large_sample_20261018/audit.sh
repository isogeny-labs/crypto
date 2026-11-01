#!/usr/bin/env bash
# Audit every session: --replay-all for tiers S and M, --replay 24 for tier L (PROTOCOL.md, "Measurement").
set -u
HERE=$(cd "$(dirname "$0")" && pwd); ROOT=$(cd "$HERE/../.." && pwd); SLOTS=${SLOTS:-5}
NAMES=${NAMES:-"s1a-ell8209-q29 s1b-ell8209-q29 s1c-ell8209-q29 s1d-ell8209-q29 s2-ell131101-q37 m1-ell1211653-q45 m2-ell1138019-q45 m3-ell1609903-q45 l1-pairs00-03 l2-pairs04-07 l3-pairs08-15 l4-pairs16-23"}
for name in $NAMES; do
  while [ "$(jobs -rp | wc -l)" -ge "$SLOTS" ]; do wait -n; done
  [ -s "$HERE/audits/$name.audit.json" ] && { echo "skip $name"; continue; }
  case $name in l*|t*) mode="--replay 24";; *) mode="--replay-all";; esac
  echo "$(date -u +%FT%TZ) audit $name $mode"
  ( nice -n 19 "$ROOT/target/release/ecbench" verify --dir "$HERE/sessions/$name" $mode --out "$HERE/audits/$name.audit.json" > "$HERE/audits/$name.log" 2>&1; echo "$(date -u +%FT%TZ) audited $name exit=$?" ) &
done
wait; echo "$(date -u +%FT%TZ) all audits finished"
