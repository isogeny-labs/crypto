#!/usr/bin/env bash
# Replay check for amendment A2: every (arm, t) row of the T=200 run must reappear
# in the T=800 run with identical deterministic fields (everything except ns and job).
set -euo pipefail
a=$1; b=$2
key='select(.kind=="target") | del(.ns, .job) | tojson'
jq -c "$key" "$a" | sort > /tmp/rc_a.$$
jq -c "select(.kind==\"target\" and .t < 200) | del(.ns, .job) | tojson" "$b" | sort > /tmp/rc_b.$$
na=$(wc -l < /tmp/rc_a.$$); nb=$(wc -l < /tmp/rc_b.$$)
nd=$(comm -3 /tmp/rc_a.$$ /tmp/rc_b.$$ | wc -l)
echo "{\"replay_rows_T200\":$na,\"replay_rows_T800_t_lt_200\":$nb,\"differing_rows\":$nd,\"ok\":$([ "$na" = "$nb" ] && [ "$nd" = 0 ] && echo true || echo false)}"
rm -f /tmp/rc_a.$$ /tmp/rc_b.$$
