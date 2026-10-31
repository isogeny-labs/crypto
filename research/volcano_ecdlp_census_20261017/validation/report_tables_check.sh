#!/usr/bin/env bash
# Check that REPORT.md's level-effect (s4), paired-source (s6) and IC-vs-rho
# (s7) table cells equal results.json rounded as printed. Prints one line per
# cell that differs and a summary line. Usage: report_tables_check.sh STUDY
set -eu
STUDY=$1; R=$STUDY/analysis/results.json; REP=$STUDY/REPORT.md
abbr='{"equivalent":"E","different":"D","inconclusive":"I"}'
{
  jq -r --argjson ab "$abbr" '.level_effect | to_entries[] | (.key | split("|")) as [$v, $a] | .value.R | to_entries[] |
    ["L", $v, $a, "L\(.key)/L0", .value.S.ratio, .value.S.ci95[0], .value.S.ci95[1], $ab[.value.S.verdict]] | @tsv' "$R"
  jq -r --argjson ab "$abbr" '.paired_source | to_entries[] | (.key | split("|")) as [$v, $a, $f] |
    ["P", $v, $a, $f, .value.ratio_src_over_D, .value.ci95[0], .value.ci95[1], $ab[.value.verdict]] | @tsv' "$R"
  jq -r '.ic_vs_rho | to_entries[] | .key as $v | (.value.levels | to_entries[] | .key as $l | .value.prices | to_entries[] |
    ["I", $v, $l, .key, .value.ratio_ic_over_rho, .value.ci95[0], .value.ci95[1], "-"]),
    (.value.pooled | to_entries[] | ["I", $v, "all", .key, .value.ratio_ic_over_rho, .value.ci95[0], .value.ci95[1], "-"]) | @tsv' "$R"
  jq -r '(.ic_vs_rho_cubic_crater // {} | .prices // {} | to_entries[] | ["C", "V3", "0", .key, .value.ratio_ic_over_rho_cubic, .value.ci95[0], .value.ci95[1], "-"]) | @tsv' "$R"
} > /dev/stdout | awk -F'\t' -v rep="$REP" '
BEGIN {
  # parse REPORT.md tables
  while ((getline line < rep) > 0) {
    if (line ~ /^## 4\./) sec = 4; else if (line ~ /^## 6\./) sec = 6; else if (line ~ /^## 7\./) sec = 7; else if (line ~ /^## /) sec = 0
    if (line ~ /^\*\*V[123]\*\*/) { vol = substr(line, 3, 2) }
    if (line !~ /^\|/ || line ~ /^\|:--/) continue
    nc = split(line, c, "|"); for (i = 2; i < nc; i++) { gsub(/^ +| +$/, "", c[i]) }
    if (sec == 4 && c[2] ~ /^(rho|bsgs|ic)/) { for (L = 1; L <= 4; L++) cell["L", vol, c[2], "L" L "/L0"] = c[2 + L] }
    if (sec == 6 && c[2] ~ /^V[123]$/ && c[4] ~ /\[/) { split("Ua/D Ub/D S/D N/D", F, " "); for (i = 1; i <= 4; i++) cell["P", c[2], c[3], F[i]] = c[3 + i] }
    if (sec == 7 && (c[2] ~ /^[0-9]+$/ || c[2] == "all")) { split("0.0 0.1 1.0", PP, " "); for (i = 1; i <= 3; i++) cell["I", vol, c[2], PP[i]] = (c[2] == "all" ? c[3 + i] : c[3 + i]) }
    if (sec == 7 && vol == "V3" && c[2] ~ /^0, matched rho = `rho-cubic`/) { split("0.0 0.1 1.0", PP, " "); for (i = 1; i <= 3; i++) cell["C", "V3", "0", PP[i]] = c[3 + i] }
  }
}
{
  if ($1 == "I" || $1 == "C") { k = $1 SUBSEP $2 SUBSEP $3 SUBSEP $4
                   want = sprintf("%.1f [%.1f, %.1f]", $5, $6, $7) }
  else { k = $1 SUBSEP $2 SUBSEP $3 SUBSEP $4; want = sprintf("%.3f [%.3f, %.3f] %s", $5, $6, $7, $8) }
  n++; nt[$1]++
  if (!(k in cell)) { miss++; print "NOT IN REPORT:", $0; next }
  if (cell[k] != want) { bad++; print "DIFF:", $1, $2, $3, $4, "report=" cell[k], "results=" want } else ok++
}
END { printf "report_table_cells_checked=%d (level %d, paired %d, ic_vs_rho %d, ic_vs_rho_cubic_crater %d) equal=%d differ=%d missing=%d\n", n, nt["L"], nt["P"], nt["I"], nt["C"] + 0, ok, bad + 0, miss + 0 }'
