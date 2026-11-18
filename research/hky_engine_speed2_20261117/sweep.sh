#!/bin/bash
# sweep.sh "M T CMAX;M T CMAX;…" LABEL [hky3 flags…]: one line, per cell the multiply-adds of every closure
# with at least 5,000 columns, the rank over the columns when it stopped, and whether 1 entered.
set -u
V2=${V2:-/home/user/crypto-wt/hky-speed/target/release/examples/pkm_tower_pilot}
TMP=${TMPDIR:-/tmp}/hky3sweep.$$; mkdir -p $TMP
cells=$1; label=$2; shift 2
line="$label"
IFS=';' read -ra CS <<< "$cells"
for cell in "${CS[@]}"; do
  set -- $cell "$@"; m=$1; t=$2; cmax=$3; shift 3
  "$(dirname "$0")/cell.sh" $V2 $m $t $cmax $TMP/o.jsonl --engine hky3 "$@" > /dev/null
  r=$(jq -r 'select(has("closures")) | select(.t == '$t') | . as $r | [range(0; .closures|length) as $i | .closures[$i] | select(.cols >= 5000)
      | "c\(.c):\(.muladds / 1e7 | round / 100)e9 \(($r.hky3.stats[$i].part_rank | add) / .cols * 100 | round)%\(if .one then "+1" else "" end) \(.ms/100|round/10)s"] | join(", ")' $TMP/o.jsonl)
  line="$line | ($m,$t) $r"
done
echo "$line"; rm -rf $TMP
