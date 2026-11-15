#!/usr/bin/env bash
# parse.sh -- summarise msolve -v 2 logs: one TSV line per <cell>.log in DIR.
# Usage: parse.sh [-H] DIR      (-H prints a header line first)
# Columns: file D lastprod width sumcells quotient_dim exit wall Dsel
#   Dsel     largest degree of any round that was selected, including a round cut
#            off by the timeout (its row has no "new" field); for completed runs Dsel = D
#   D        largest degree (first column) over the F4 round rows
#   lastprod largest degree of a round with new > 0
#   width    largest column count; sumcells = sum of rows*cols over rounds
#   quotient_dim from "Dimension of quotient: N" (NA if absent)
#   exit/wall from the trailing "exit=<rc> wall=<sec>" line (NA if absent)
# Round rows look like:  4  26  62  109 x 165   4.79%   17 new   9 zero ...
set -euo pipefail
hdr=0
if [ "${1:-}" = "-H" ]; then hdr=1; shift; fi
[ $# -eq 1 ] && [ -d "$1" ] || { echo "usage: $0 [-H] DIR" >&2; exit 2; }
[ $hdr -eq 1 ] && printf 'file\tD\tlastprod\twidth\tsumcells\tquotient_dim\texit\twall\tDsel\n'
shopt -s nullglob
for f in "$1"/*.log; do
  awk -v name="$(basename "$f")" '
    $1 ~ /^[0-9]+$/ && $2 ~ /^[0-9]+$/ && $3 ~ /^[0-9]+$/ && $4 ~ /^[0-9]+$/ && $5 == "x" &&
    $6 ~ /^[0-9]+$/ && $7 ~ /^[0-9.]+%$/ && $8 ~ /^[0-9]+$/ && $9 == "new" && $10 ~ /^[0-9]+$/ && $11 == "zero" {
      deg = $1 + 0; r = $4 + 0; c = $6 + 0; nw = $8 + 0
      if (deg > D) D = deg
      if (c > W) W = c
      cells += r * c
      if (nw > 0 && deg > LP) LP = deg
    }
    $1 ~ /^[0-9]+$/ && $5 == "x" && $4 ~ /^[0-9]+$/ { if ($1 + 0 > DS) DS = $1 + 0 }
    !haveq && match($0, /Dimension of quotient:[ \t]*[0-9]+/) {
      s = substr($0, RSTART, RLENGTH); sub(/.*:[ \t]*/, "", s); q = s; haveq = 1
    }
    !havee && match($0, /exit=[0-9]+ wall=[0-9.]+/) {
      s = substr($0, RSTART, RLENGTH); split(s, a, /[= ]/); ex = a[2]; wl = a[4]; havee = 1
    }
    END {
      printf "%s\t%d\t%d\t%d\t%.0f\t%s\t%s\t%s\t%d\n", name, D, LP, W, cells,
             (haveq ? q : "NA"), (havee ? ex : "NA"), (havee ? wl : "NA"), DS
    }' "$f"
done
