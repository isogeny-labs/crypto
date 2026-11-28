#!/usr/bin/env bash
# Per-call function costs from two callgrind profiles of one arm:
# (Ir[reps=11] - Ir[reps=1]) / 10, exclusive or inclusive.
# Usage: callgrind_diff.sh ARM excl|incl DIR TOP   (DIR holds ARM-1.out, ARM-11.out)
set -euo pipefail
arm=$1; mode=$2; dir=$3; top=$4
inc=no; [ "$mode" = incl ] && inc=yes
dump() {
  callgrind_annotate --auto=no --threshold=100 --inclusive=$inc "$1" | awk '
    /PROGRAM TOTALS/ { gsub(",", "", $1); print "__TOTAL__\t" $1; next }
    /events annotated/ || /^[ \t]*=>/ { next }
    match($0, /^[ \t]*[0-9,]+[ \t]+\([^)]*\)[ \t]+/) {
      n = $1; gsub(",", "", n)
      name = substr($0, RLENGTH + 1)
      sub(/[ \t]*\[[^]]*\]$/, "", name)
      if (name ~ /^\// && index(name, ":")) name = substr(name, index(name, ":") + 1)
      gsub(/::\{closure#[0-9]+\}/, "::{closure}", name)
      print name "\t" n
    }'
}
awk -F'\t' -v arm="$arm" -v mode="$mode" -v top="$top" '
  FNR == NR { a[$1] += $2; next }
  { b[$1] += $2 }
  END {
    for (k in a) if (!(k in b)) b[k] += 0
    total = (b["__TOTAL__"] - a["__TOTAL__"]) / 10
    printf "%s %s per MST call; total %.3f M\n", arm, (mode == "incl" ? "inclusive" : "exclusive"), total / 1e6
    for (k in b) if (k != "__TOTAL__") { v = (b[k] - a[k]) / 10; if (v > 0) printf "%.0f\t%s\n", v, k }
  }' <(dump "$dir/$arm-1.out") <(dump "$dir/$arm-11.out") |
  { IFS= read -r head; echo "$head"; total=$(sed 's/.*total \([0-9.]*\) M/\1/' <<<"$head")
    sort -t$'\t' -k1,1nr | head -n "$top" | awk -F'\t' -v t="$total" '{
      s = $2; gsub(/crypto_lib::cryptanalysis::large_prime_filter::/, "", s)
      gsub(/<alloc::|alloc::|core::|std::/, "", s)
      printf "%8.3f M %5.1f%%  %s\n", $1 / 1e6, 100 * $1 / (t * 1e6), substr(s, 1, 150) }'; }
