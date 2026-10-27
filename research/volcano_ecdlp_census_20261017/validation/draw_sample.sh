#!/usr/bin/env bash
# Draw the replay sample (validator-written).
#
# One session per (volcano, family) stratum present in runs/progress.jsonl
# (status == "done"): index = SHA-256("NONCE|V|FAMILY") mod n over the
# stratum's sessions sorted by name (C locale), computed digit by digit;
# plus every V3 crater session that carries rho-cubic, forced first. The
# nonce is the branch HEAD commit that published the data, fixed before the
# draw. Usage: draw_sample.sh PROGRESS_JSONL NONCE
set -eu
PROG=$1; NONCE=$2
export LC_ALL=C
done_specs=$(jq -r 'select(.status=="done") | .spec' "$PROG" | sort)
picked="V3-D-L0-c0 V3-N-L0-c0 V3-ctl-L0-c0"
for s in $picked; do echo "$s"; done
for key in $(echo "$done_specs" | awk -F- '{print $1"|"$2}' | sort -u); do
  v=${key%%|*}; fam=${key#*|}
  list=$(echo "$done_specs" | awk -F- -v v="$v" -v f="$fam" '$1==v && $2==f')
  n=$(echo "$list" | wc -l)
  h=$(printf '%s|%s|%s' "$NONCE" "$v" "$fam" | sha256sum | awk '{print $1}')
  idx=$(echo "$h" | awk -v n="$n" '{r=0; for(i=1;i<=length($0);i++){d=index("0123456789abcdef",substr($0,i,1))-1; r=(r*16+d)%n}; print r}')
  s=$(echo "$list" | sed -n "$((idx+1))p")
  case " $picked " in *" $s "*) ;; *) echo "$s"; picked="$picked $s";; esac
done
