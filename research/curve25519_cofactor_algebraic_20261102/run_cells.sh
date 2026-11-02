#!/bin/bash
# Frozen protocol runner: 5 cells x {lifts=0, lifts=1}, interleaved by pair, sequential.
BIN=/home/user/frozen-bin/curve25519_family_lead2
D="$(cd "$(dirname "$0")" && pwd)/analysis"
OUT=$D/f4cost_lifts.jsonl
mkdir -p "$D"
CELLS="isogeny:4:0:2026111204 isogeny:5:0:2026111205 isogeny:6:0:2026111206 bits:5:3:2026111205 bits:6:3:2026111206"
for c in $CELLS; do
  IFS=: read form t g seed <<<"$c"
  for L in 0 1; do
    name="${form}_t${t}_g${g}_lifts${L}"
    ( ulimit -v 3500000; timeout 900 "$BIN" f4cost 20 "$form" "$t" "$g" 200 "$seed" 600 2 full "$L" \
        > "$D/$name.json" 2> "$D/$name.stderr" )
    rc=$?
    if [ $rc -eq 0 ] && jq -c . "$D/$name.json" >> "$OUT" 2>/dev/null; then
      echo "$name ok" >> "$D/$name.status"
    else
      echo "$name failed exit=$rc" >> "$D/$name.status"
    fi
    cat "$D/$name.status"
  done
done
