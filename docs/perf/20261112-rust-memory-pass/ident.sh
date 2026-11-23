#!/bin/bash
# usage: ident.sh BIN LABEL -> runs every Phase A spec, compares normalised records with base-*, replays
S=${S:-$PWD}; N=$(dirname "$0")/../20261110-wall-only-rust-fixes/norm-records.sh
B=$1; L=$2; cd $S/m-rss
for f in ${SPECS:-$S/m-specs}/*.json; do n=$(basename $f .json); rm -rf $L-$n
  $B run --spec $f --out $L-$n --cpus none --allow-busy --quiet >/dev/null 2>&1
  hb=$(sh $N base-$n/records.jsonl | sha256sum | cut -c1-16); hc=$(sh $N $L-$n/records.jsonl | sha256sum | cut -c1-16)
  $B verify --dir $L-$n --replay-all --out $L-$n.audit.json >/dev/null 2>&1; ok=$(jq -r .ok $L-$n.audit.json)
  echo "$L $n $([ $hb = $hc ] && echo IDENTICAL || echo DIFFERENT) replay_ok=$ok rss_kib=$(jq .time.max_rss_kib $L-$n/records.jsonl) minflt=$(jq .time.minor_faults $L-$n/records.jsonl)"
done
