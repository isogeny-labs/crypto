#!/usr/bin/env bash
# Compare replayed records with the originals on exact outputs and op counts.
# usage: replay_compare.sh <original.jsonl> <replay.jsonl>
set -euo pipefail
k='[.arm,.case.n,.case.ci,.case.ti,(.seed_index//-1)]|tostring'
sel='{d,verified,correct,targets,relations,columns,total,steps}'
jq -c "{k:($k),v:$sel}" "$2" | while read -r line; do
  kk=$(jq -r .k <<<"$line"); vv=$(jq -cS .v <<<"$line")
  ov=$(jq -cS --arg kk "$kk" "select(($k)==\$kk)|$sel" "$1")
  if [ "$ov" == "$vv" ]; then echo "EQUAL $kk"; else echo "DIFF  $kk"; fi
done
