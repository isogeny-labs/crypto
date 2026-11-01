#!/usr/bin/env bash
# Typst table fragments for the run report, cut column by column from raw/summary.md (text
# processing only: every cell is a cell of the summary, which is itself read from the records).
#   typst_tables.sh raw/summary.md OUT_DIR
set -euo pipefail
export LC_ALL=C
SUMMARY=${1:?summary.md}
OUT=${2:?out dir}
mkdir -p "$OUT"

# rows of "### Table N." as tab-separated, trimmed cells
tab() {
  awk -v T="### Table $1." '
    index($0, T) == 1 { f = 1; next }
    /^### Table/ { f = 0 }
    f && /^\| [0-9]/ {
      n = split($0, a, "|"); line = ""
      for (i = 2; i < n; i++) { gsub(/^ +| +$/, "", a[i]); line = line (i > 2 ? "\t" : "") a[i] }
      print line
    }' "$SUMMARY"
}

# emit(file, ncols, header cells (|-separated), spec): rows come on stdin, tab separated,
# already cut to the wanted columns.
emit() {
  local file=$1 n=$2 header=$3 align=$4
  {
    echo "#table("
    echo "  columns: ($(printf 'auto, %.0s' $(seq 1 $((n - 1)))) auto),"
    echo "  align: ($align),"
    echo "  inset: 3pt, stroke: 0.4pt + luma(170),"
    echo "  table.header($(echo "$header" | awk -F'|' '{for(i=1;i<=NF;i++) printf "[*%s*]%s", $i, (i<NF?", ":"")}')),"
    awk -F'\t' '{ for (i = 1; i <= NF; i++) { c = $i; gsub(/[\\#$]/, "", c); printf "[%s],", c } print "" }'
    echo ")"
  } > "$file"
}

right() { local n=$1; printf 'right, %.0s' $(seq 1 $((n - 1))); printf 'right'; }

# 1. candidates: Table 1 joined with Table 2 on (n, w)
join -t "$(printf '\t')" -j 1 \
  <(tab 1 | awk -F'\t' '{print $1 "_" $2 "\t" $1 "\t" $2 "\t" $4 "\t" $6 "\t" $9 "\t" $10 "\t" $11 "\t" $12 "\t" $14 "\t" $8}' | sort -t "$(printf '\t')" -k1,1) \
  <(tab 2 | awk -F'\t' '{print $1 "_" $2 "\t" $12 "\t" $13 "\t" $18 "\t" $19}' | sort -t "$(printf '\t')" -k1,1) \
  | sort -t "$(printf '\t')" -k3,3n -k2,2n \
  | awk -F'\t' '{print $2 "\t" $3 "\t" $4 "\t" $5 "\t" $11 "\t" $6 "\t" $7 "\t" $8 "\t" $9 "\t" $10 "\t" $12 "\t" $13 "\t" $14 "\t" $15}' \
  | emit "$OUT/ps4_tab_candidates.typ" 14 "n|w|m|reps|short orbits|pairs ordered|pairs unordered|pairs reduced|factor ordered|factor unordered|M factor unordered|M factor ordered|op/pair unordered|op/pair ordered" "$(right 14)"

# 2. decisions: Table 3 and Table 4 (same row order)
paste <(tab 3 | awk -F'\t' '{print $1 "\t" $2 "\t" $3 "\t" $8 "\t" $9 "\t" $10 "\t" $13}') <(tab 4 | awk -F'\t' '{print $12 "\t" $15 "\t" $16}') \
  | awk -F'\t' '{print $1 "\t" $2 "\t" $3 "\t" $8 "\t" $5 "\t" $4 "\t" $6 "\t" $7 "\t" $10}' \
  | emit "$OUT/ps4_tab_decision.typ" 9 "n|w|decomposable|verdict kept (target alone)|R says no|M to witness R / U|refuting|M full scan R / U|closure = U" "$(right 9)"

# 3. agreement with the oracle: Table 4
tab 4 | awk -F'\t' '{print $1 "\t" $2 "\t" $3 "\t" $4 "\t" $5 "\t" $6 "\t" $7 "\t" $8 "\t" $9 "\t" $10 "\t" $11 "\t" $15}' \
  | emit "$OUT/ps4_tab_agree.typ" 12 "n|w|targets|decomposable|panel xR|panel verdict|panel ref_count|U-unord = oracle|U-ord = oracle|R subset|R = predicted|kept solutions" "$(right 12)"

# 4. torsion: Table 10 joined with Table 11
join -t "$(printf '\t')" -j 1 \
  <(tab 10 | awk -F'\t' '{print $1 "_" $2 "\t" $1 "\t" $2 "\t" $3 "\t" $4 "\t" $5 "\t" $6}' | sort -t "$(printf '\t')" -k1,1) \
  <(tab 11 | awk -F'\t' '{print $1 "_" $2 "\t" $3 "\t" $4 "\t" $5 "\t" $8 "\t" $9 "\t" $12 "\t" $13}' | sort -t "$(printf '\t')" -k1,1) \
  | sort -t "$(printf '\t')" -k3,3n -k2,2n \
  | awk -F'\t' '{print $2 "\t" $3 "\t" $4 "\t" $5 "\t" $6 "\t" $7 "\t" $8 "\t" $9 "\t" $10 "\t" $11 "\t" $12 "\t" $13 "\t" $14}' \
  | emit "$OUT/ps4_tab_torsion.typ" 13 "n|w|x-frame base|x-frame image in base|u-frame base|u-frame image in base|targets|m(B+-)|classes|pair factor|M factor|closure = oracle|verdict kept" "$(right 13)"

# 5. leaf: Table 8
tab 8 | awk -F'\t' '{print $1 "\t" $2 "\t" $3 "\t" $5 "\t" $6 "\t" $7 "\t" $8 "\t" $9 "\t" $10 "\t" $14}' \
  | emit "$OUT/ps4_tab_leaf.typ" 10 "n|w|M/pair (ONB)|bits/pair (ONB, model)|n-near l|dn|M/pair weight / control|bits weight / control|bits, control scaled to n|range over l=8..12" "$(right 10)"
echo "wrote $OUT/ps4_tab_*.typ"
