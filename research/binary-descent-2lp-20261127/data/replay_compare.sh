#!/usr/bin/env bash
# Bitwise comparison (wall times excluded) of the replayed n=30 ci=0 ti=0 D2
# record with the original evaluation record.
set -eu
cd "$(dirname "$0")"
sel='select(.arm=="D2" and .case.n==30 and .case.ci==0 and .case.ti==0) | del(.wall_ns, .check_wall_ns)'
a=$(jq -cS "$sel" runs_d2.jsonl | sha256sum | cut -d' ' -f1)
b=$(jq -cS "$sel" replay_d2.jsonl | sha256sum | cut -d' ' -f1)
echo "original $a"; echo "replay   $b"
[ "$a" = "$b" ] && echo "IDENTICAL (all fields except wall_ns, check_wall_ns)" || { echo DIFFERENT; exit 1; }
