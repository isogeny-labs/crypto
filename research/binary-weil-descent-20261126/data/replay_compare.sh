#!/usr/bin/env bash
# Bitwise compare of every non-wall field of a replayed record with the
# primary record of the same case.
set -eu
r=$(jq -c 'del(.wall_ns, .check_wall_ns)' "$2")
k=$(jq -r '"\(.case.n) \(.case.ci) \(.case.ti)"' <<<"$r")
set -- $k "$1"
p=$(jq -c --argjson n $1 --argjson c $2 --argjson t $3 'select(.case.n==$n and .case.ci==$c and .case.ti==$t) | del(.wall_ns, .check_wall_ns)' "$4")
if [ "$(jq -S . <<<"$p" | sha256sum)" = "$(jq -S . <<<"$r" | sha256sum)" ]; then echo "replay n=$1 ci=$2 ti=$3: all non-wall fields bitwise equal ($(jq -S . <<<"$r" | sha256sum | cut -c1-16))"; else echo "replay n=$1 ci=$2 ti=$3: MISMATCH"; diff <(jq -S . <<<"$p") <(jq -S . <<<"$r") | head; fi
