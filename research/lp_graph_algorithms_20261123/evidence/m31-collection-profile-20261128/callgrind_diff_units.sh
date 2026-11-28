#!/usr/bin/env bash
# Function costs of (B - A) / DIV from two callgrind profiles.
# Usage: diff.sh A.out B.out DIV excl|incl TOP
set -euo pipefail
a=$1; b=$2; div=$3; mode=$4; top=$5
inc=no; [ "$mode" = incl ] && inc=yes
dump() {
  callgrind_annotate --auto=no --threshold=100 --inclusive=$inc "$1" | awk '
    /PROGRAM TOTALS/ { gsub(",", "", $1); print "__TOTAL__\t" $1; next }
    /events annotated/ || /^[ \t]*=>/ { next }
    match($0, /^[ \t]*[0-9,]+[ \t]+\([^)]*\)[ \t]+/) {
      n = $1; gsub(",", "", n); name = substr($0, RLENGTH + 1)
      sub(/[ \t]*\[[^]]*\]$/, "", name)
      if (name ~ /^\// && index(name, ":")) name = substr(name, index(name, ":") + 1)
      print name "\t" n }'
}
awk -F'\t' -v div="$div" '
  FNR == NR { A[$1] += $2; next } { B[$1] += $2 }
  END { for (k in A) if (!(k in B)) B[k] += 0
        t = (B["__TOTAL__"] - A["__TOTAL__"]) / div; printf "TOTAL\t%.0f\n", t
        for (k in B) if (k != "__TOTAL__") { v = (B[k] - A[k]) / div; if (v > 0) printf "%.0f\t%s\n", v, k } }' \
  <(dump "$a") <(dump "$b") |
  { IFS=$'\t' read -r _ total; printf "total %.3f G per unit (%s)\n" "$(echo "$total/1e9" | bc -l)" "$mode"
    sort -t$'\t' -k1,1nr | head -n "$top" | awk -F'\t' -v t="$total" '{ s=$2; gsub(/prime31_algebraic_ic::|alloc::|core::|std::|hashbrown::/, "", s); printf "%8.1f M %5.1f%%  %s\n", $1/1e6, 100*$1/t, substr(s,1,140) }'; }
