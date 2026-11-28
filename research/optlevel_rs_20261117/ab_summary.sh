#!/usr/bin/env bash
# Per kernel and variant: n, median, min, max of ops/s (or bytes/s with a second argument "bytes"),
# and the spread (max - min)/median.  usage: ab_summary.sh AB.tsv [bytes]
set -euo pipefail
col=5; [ "${2:-}" = bytes ] && col=6
tail -n +2 "$1" | sort -t$'\t' -k1,1 -k4,4 -k${col},${col}g | awk -F'\t' -v c=$col '
function flush() { if (n) { med = (n % 2) ? v[(n + 1) / 2] : (v[n / 2] + v[n / 2 + 1]) / 2;
    printf "%s\t%s\t%d\t%.4e\t%.4e\t%.4e\t%.3f\n", k, var, n, med, v[1], v[n], (v[n] - v[1]) / med } n = 0 }
{ if ($1 != k || $4 != var) { flush(); k = $1; var = $4 } v[++n] = $c }
END { flush() }' | (echo -e "kernel\tvariant\tn\tmedian\tmin\tmax\tspread"; cat)
