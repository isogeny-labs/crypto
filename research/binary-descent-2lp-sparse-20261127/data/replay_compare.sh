#!/usr/bin/env bash
# Bitwise compare of the replay record against the run record (all fields but wall times).
set -eu
cd "$(dirname "$0")"
strip='del(.wall_ns, .check_wall_ns)'
a=$(jq -c "select(.arm==\"D2S\" and .case.n==30 and .case.ci==0 and .case.ti==0) | $strip" runs_d2s.jsonl)
b=$(jq -c "$strip" replay_d2s.jsonl | head -1)
if [ "$a" == "$b" ]; then echo "replay n=30 ci=0 ti=0 D2S: IDENTICAL in every field but wall times ($(echo "$a" | sha256sum | cut -c1-16))"; else echo "replay MISMATCH"; diff <(echo "$a" | jq -S .) <(echo "$b" | jq -S .) | head -40; fi
