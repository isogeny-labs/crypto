#!/usr/bin/env bash
# Draw the round-2 replay sample (validator-written; adapted from the census
# validation's draw_sample.sh).
#
# Sessions are named by the basename of the progress line's `out` (so the
# three redone sessions are `<spec>.r2`). Selection, in this order, without
# duplicates:
#   1. every V5 crater session that carries rho-cubic (V5-D/N/ctl-L0-c0);
#   2. one session per (volcano, family) stratum of status "done":
#      index = SHA-256("NONCE|V|FAMILY") mod n over the stratum's sessions
#      sorted by name (C locale);
#   3. extra draws until the sample holds >= 3 Tcr and >= 2 M sessions and
#      one redone .r2 session: SHA-256("NONCE|V5|Tcr|k") for k = 1, 2, ...
#      and SHA-256("NONCE|r2") over the .r2 sessions.
# The nonce is the snapshot commit under validation, fixed before the draw.
# Usage: draw_sample.sh PROGRESS_JSONL NONCE
set -eu
PROG=$1; NONCE=$2
export LC_ALL=C
pick() { # list key -> one element
  local list=$1 key=$2 n h idx
  n=$(printf '%s\n' "$list" | wc -l)
  h=$(printf '%s' "$key" | sha256sum | awk '{print $1}')
  idx=$(echo "$h" | awk -v n="$n" '{r=0; for(i=1;i<=length($0);i++){d=index("0123456789abcdef",substr($0,i,1))-1; r=(r*16+d)%n}; print r}')
  printf '%s\n' "$list" | sed -n "$((idx+1))p"
}
done_sess=$(jq -r 'select(.status=="done") | .out | sub("^sessions/";"")' "$PROG" | sort)
picked=" "
add() { case "$picked" in *" $1 "*) ;; *) echo "$1"; picked="$picked$1 ";; esac; }
for s in V5-D-L0-c0 V5-N-L0-c0 V5-ctl-L0-c0; do add "$s"; done
for key in $(printf '%s\n' "$done_sess" | awk -F- '{print $1"|"$2}' | sort -u); do
  v=${key%%|*}; fam=${key#*|}
  list=$(printf '%s\n' "$done_sess" | awk -F- -v v="$v" -v f="$fam" '$1==v && $2==f')
  add "$(pick "$list" "$NONCE|$v|$fam")"
done
tcr=$(printf '%s\n' "$done_sess" | awk -F- '$1=="V5" && $2=="Tcr"')
k=1
while [ "$(printf '%s' "$picked" | tr ' ' '\n' | grep -c -- '-Tcr-')" -lt 3 ]; do
  add "$(pick "$tcr" "$NONCE|V5|Tcr|$k")"; k=$((k+1))
done
r2=$(printf '%s\n' "$done_sess" | grep '\.r2$')
add "$(pick "$r2" "$NONCE|r2")"
