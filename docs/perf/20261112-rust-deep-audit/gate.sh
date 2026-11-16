#!/bin/bash
# usage: gate.sh BIN LABEL  -> runs the three round-1 gate specs, compares normalised records to base-*, replays all
# S: work directory holding the base-*/ gate sessions (as run: the session scratchpad)
S=${S:-$PWD}; D=${D:-$(cd "$(dirname "$0")/../20261110-wall-only-rust-fixes" && pwd)}
B=$1; L=$2; cd $S
for g in prime koblitz tensor; do
  rm -rf $L-$g; $B run --spec $D/gate-$g.json --out $L-$g --cpus none --allow-busy --quiet > $L-$g.log 2>&1; rc=$?
  hb=$(sh $D/norm-records.sh base-$g/records.jsonl | sha256sum | cut -c1-16); hc=$(sh $D/norm-records.sh $L-$g/records.jsonl | sha256sum | cut -c1-16)
  $B verify --dir $L-$g --replay-all --out $L-$g.audit.json > $L-$g.verify.log 2>&1; vr=$?
    rep=$(jq -r "\"ok=\(.ok) replays=\([.replays[]|select(.reproduced)]|length)/\(.replays|length)\"" $L-$g.audit.json 2>/dev/null)
  echo "$L $g run_rc=$rc records=$(wc -l < $L-$g/records.jsonl) base=$hb cand=$hc $([ $hb = $hc ] && echo IDENTICAL || echo DIFFERENT) verify_rc=$vr $rep"
done
