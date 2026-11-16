#!/usr/bin/env bash
# Copy every pod file under out/ and rho/ into raw/ (skips files already fetched with the same size).
set -u
export RP_STATE=${RP_STATE:-/tmp/claude-0/rp_quartic-k26-clean}
K=/home/user/crypto/research/runpod_kit_20261112/rp.sh; N=bdr-quartic-k26-clean
D=$(cd "$(dirname "$0")" && pwd)
$K ls $N | python3 -c "import json,sys;[print(p,s) for p,s in json.load(sys.stdin) if p.startswith(('out/','rho/'))]" | while read p s; do
  mkdir -p "$D/raw/$(dirname $p)"
  [ -f "$D/raw/$p" ] && [ "$(stat -c %s "$D/raw/$p")" = "$s" ] && continue
  $K get $N "$p" "$D/raw/$p"
done
