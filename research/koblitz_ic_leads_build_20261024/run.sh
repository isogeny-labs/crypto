#!/usr/bin/env bash
# G1 and P1 of PROTOCOL.md (degree 4), sequential, one process per run.
# Usage: run.sh OUTDIR   (from the repository root)
set -u
OUT=${1:?outdir}; mkdir -p "$OUT"
BIN=target/release/examples/pe_closure
DR=research/dreg_closure_driver_20261016/draws
one() { # tag file draw mode
  tag=$1; f=$2; k=$3; mode=$4
  { time "$BIN" --draws "$DR/$f.jsonl" --draw "$k" --degree 4 --mode "$mode" --batch 64 \
      > "$OUT/$tag-$f.d$k.jsonl"; } 2> "$OUT/$tag-$f.d$k.log"
  echo "$(date -u +%H:%M:%SZ) $tag $f d$k exit $?" >> "$OUT/queue.log"
}
for spec in m3-22-6:0 m3-22-6:1 m3-22-6:2 m3-22-6:3 m3-24-6:0 m3-24-6:1 m3-24-6:2 m3-24-6:3 \
            m4-11-5:8 m4-11-5:67 m4-11-5:107 m4-11-5:143 m4-12-5:24 m4-12-5:26 m4-12-5:47 m4-12-5:52; do
  f=${spec%:*}; k=${spec#*:}
  [ -s "$OUT/g1-$f.d$k.jsonl" ] || one g1 "$f" "$k" partial
done
for spec in m4-11-5:0 m4-11-5:1; do f=${spec%:*}; k=${spec#*:}; one control "$f" "$k" partial; done
for spec in m3-22-6:0 m3-22-6:1 m3-22-6:2 m3-22-6:3 m4-11-5:8 m4-11-5:67 m4-11-5:107 m4-11-5:143; do
  f=${spec%:*}; k=${spec#*:}; one p1full "$f" "$k" full
done
echo DONE >> "$OUT/queue.log"
