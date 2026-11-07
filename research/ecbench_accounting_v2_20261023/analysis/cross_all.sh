#!/bin/bash
set -eu
D=$(dirname "$0")/../sessions
for n in j0 prime koblitz large-prime; do
  case $n in j0) s=j0-frozen;; *) s=frozen-$n;; esac
  echo "{\"spec\":\"$s\"}" 
  jq -n --slurpfile a $D/$s-branch/records.jsonl --slurpfile b $D/$s-main/records.jsonl -f $(dirname "$0")/cross_binary.jq
done | jq -s '[range(0; length; 2) as $i | .[$i] + .[$i+1]]'
