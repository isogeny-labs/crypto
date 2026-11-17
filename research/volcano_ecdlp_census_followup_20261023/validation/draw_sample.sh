#!/usr/bin/env bash
# Draw the replay sample (validator-written; adapted from the census
# research/volcano_ecdlp_census_20261017/validation/draw_sample.sh).
#
# 1. Forced: every V5 crater session that carries rho-cubic outside Tcr
#    (V5-D-L0-c0, V5-N-L0-c0, V5-ctl-L0-c0; the positive-control sessions).
# 2. One session per (volcano, family) stratum present in runs/progress.jsonl
#    (status == "done"): index = SHA-256("NONCE|V|FAMILY") mod n over the
#    stratum's specs sorted by name (C locale), computed digit by digit.
# 3. Extra Tcr draws "NONCE|V5|Tcr|k" (k = 1, 2, ...) until 3 distinct Tcr
#    sessions are in the sample; extra M draws "NONCE|M|k" over all M specs
#    until 2 distinct M sessions are in the sample.
# 4. One of the three redone (.r2) sessions: SHA-256("NONCE|r2") mod 3.
# The nonce is the branch HEAD that published the data, fixed before the draw.
# Output: one session name per line (the basename of the progress line's
# `out`, i.e. <spec> or <spec>.r2). Usage: draw_sample.sh PROGRESS_JSONL NONCE
set -eu
PROG=$1; NONCE=$2
export LC_ALL=C
done_specs=$(jq -r 'select(.status=="done") | .spec' "$PROG" | sort)
out_of() { jq -r --arg s "$1" 'select(.status=="done" and .spec==$s) | .out' "$PROG" | sed 's#^sessions/##'; }
hidx() { printf '%s' "$1" | sha256sum | awk -v n="$2" '{r=0; for(i=1;i<=length($1);i++){d=index("0123456789abcdef",substr($1,i,1))-1; r=(r*16+d)%n}; print r}'; }
picked=""
add() { case " $picked " in *" $1 "*) ;; *) picked="$picked $1"; out_of "$1";; esac; }
for s in V5-D-L0-c0 V5-N-L0-c0 V5-ctl-L0-c0; do add "$s"; done
for key in $(echo "$done_specs" | awk -F- '{print $1"|"$2}' | sort -u); do
  v=${key%%|*}; fam=${key#*|}
  list=$(echo "$done_specs" | awk -F- -v v="$v" -v f="$fam" '$1==v && $2==f')
  n=$(echo "$list" | wc -l)
  idx=$(hidx "$NONCE|$v|$fam" "$n")
  add "$(echo "$list" | sed -n "$((idx+1))p")"
done
count() { echo "$picked" | tr ' ' '\n' | grep -c -- "$1" || true; }
tl=$(echo "$done_specs" | grep -- '-Tcr-'); tn=$(echo "$tl" | wc -l); k=1
while [ "$(count -Tcr-)" -lt 3 ]; do add "$(echo "$tl" | sed -n "$(( $(hidx "$NONCE|V5|Tcr|$k" "$tn") + 1 ))p")"; k=$((k+1)); done
ml=$(echo "$done_specs" | grep -- '-M-'); mn=$(echo "$ml" | wc -l); k=1
while [ "$(count -M-)" -lt 2 ]; do add "$(echo "$ml" | sed -n "$(( $(hidx "$NONCE|M|$k" "$mn") + 1 ))p")"; k=$((k+1)); done
r2=$(jq -r 'select(.status=="done" and (.out|endswith(".r2"))) | .spec' "$PROG" | sort)
add "$(echo "$r2" | sed -n "$(( $(hidx "$NONCE|r2" 3) + 1 ))p")"
