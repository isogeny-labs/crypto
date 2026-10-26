#!/bin/bash
# Replays every session of docs/ic/validation/replay-plan.json with `ecbench verify` and compares the outcome
# with the plan's `expect`.  Exit 1 on new drift, on a stale `drift` entry, or on a missing receipt.
# usage: replay_plan.sh <ecbench-binary> <outdir>         Needs jq.  Run from the repository root.
set -u
EB="$1"; OUT="$2"; mkdir -p "$OUT"
plan=docs/ic/validation/replay-plan.json
bad=0
n=$(jq '.sessions | length' $plan)
for i in $(seq 0 $((n - 1))); do
  dir=$(jq -r ".sessions[$i].dir" $plan); rep=$(jq -r ".sessions[$i].replay" $plan); exp=$(jq -r ".sessions[$i].expect" $plan)
  tag=$(echo "$dir" | sed 's#research/##; s#/sessions/#__#; s#/#_#g')
  flag="--replay-all"; [ "$rep" != all ] && flag="--replay $rep"
  "$EB" verify --dir "$dir" $flag --out "$OUT/$tag.receipt.json" > /dev/null 2> "$OUT/$tag.stderr"
  [ -s "$OUT/$tag.receipt.json" ] || { echo "$dir: no receipt"; bad=1; continue; }
  total=$(jq '.replays | length' "$OUT/$tag.receipt.json"); ok=$(jq '[.replays[] | select(.reproduced)] | length' "$OUT/$tag.receipt.json")
  if [ "$ok" = "$total" ]; then got=identical; else got=drift; fi
  if [ "$got" = "$exp" ]; then echo "$dir: $got ($ok/$total), as planned"
  elif [ "$got" = drift ]; then echo "$dir: NEW DRIFT ($ok/$total reproduced) on a session planned as identical"; bad=1
  else echo "$dir: STALE ENTRY: planned as drift but all $total replays reproduce; remove or update its plan entry"; bad=1; fi
done
exit $bad
