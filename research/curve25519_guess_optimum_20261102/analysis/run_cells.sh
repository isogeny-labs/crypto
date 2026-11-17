#!/usr/bin/env bash
# One cell per call: run_cells.sh <stage_tag> <k> <t> <G> <seed> <cap_s>  (appends to results/status/uptime files)
# Driver for PROTOCOL.md. Sequential use only. shell + jq.
set -u
BIN=/home/user/frozen-bin/curve25519_family_leadB
D=$(cd "$(dirname "$0")" && pwd); ERR=$D/stderr; mkdir -p "$ERR"
tag=$1; k=$2; t=$3; G=$4; seed=$5; cap=$6
OUT=$D/cells.jsonl; ST=$OUT.status
tmp=$(mktemp); s=$(date +%s); up=$(uptime | sed 's/.*load average: //')
( ulimit -v 3500000; timeout 1000 "$BIN" f4cost "$k" isogeny "$t" "$G" 100 "$seed" "$cap" 2 full 0 ) >"$tmp" 2>"$ERR/$tag-k$k-t$t-g$G.err"
rc=$?; e=$(( $(date +%s) - s ))
if [ $rc -eq 0 ] && jq -e . "$tmp" >/dev/null 2>&1; then
  jq -c --arg tag "$tag" --argjson seed "$seed" --argjson cap "$cap" --arg load "$up" --argjson el "$e" \
    '. + {stage:$tag,seed:$seed,cap_s:$cap,load_at_start:$load,elapsed_s:$el}' "$tmp" >>"$OUT"
  echo "ok $tag k=$k t=$t G=$G seed=$seed rc=0 elapsed_s=$e load=$up" >>"$ST"
else
  echo "failed $tag k=$k t=$t G=$G seed=$seed rc=$rc elapsed_s=$e load=$up" >>"$ST"
fi
rm -f "$tmp"
