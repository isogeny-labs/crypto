#!/usr/bin/env bash
# Builds fig_enc_control.svg from raw/*.jsonl (jq + the Rust SVG tool), then PNG/PDF.
set -eu
cd "$(dirname "$0")"
SVG=../../target/release/examples/enc_control_svg
R=raw/rows.jsonl; A=raw/addendum_rows.jsonl
{
  # a: D vs k on random targets (draw 0, random0), finished runs only
  jq -r 'select(.kind=="f4" and .target=="random0" and .draw==0 and (.timed_out|not) and (.arm|IN("tower-H","coset","tower-rand","bits-H","bits-rand"))) | ["a_k",.arm,.k,.D]|@tsv' $R
  # b, c: dose at k = 8 (r = 0 is tower-H, r = 256 is tower-rand)
  H8=$(jq -s '[.[]|select(.kind=="f4" and .arm=="tower-H" and .k==8 and .target=="random0")][0].field_ops' $R)
  for k in 6 8; do
  jq -r --argjson k $k 'select(.kind=="f4" and .target=="random0" and .draw==0 and .k==$k and (.arm|IN("tower-H","dose","tower-rand"))) | ["b_dose","k=\($k)",(if .arm=="tower-H" then 0 elif .arm=="tower-rand" then .size else .r end),.D]|@tsv' $R
  done
  for k in 6 7 8; do
  HK=$(jq -s --argjson k $k '[.[]|select(.kind=="f4" and .arm=="tower-H" and .k==$k and .target=="random0")][0].field_ops' $R)
  jq -r --argjson k $k --argjson h $HK 'select(.kind=="f4" and .target=="random0" and .draw==0 and .k==$k and (.arm|IN("tower-H","dose","tower-rand"))) | ["c_opsdose","k=\($k)",(if .arm=="tower-H" then 0 elif .arm=="tower-rand" then .size else .r end),(.field_ops/$h)]|@tsv' $R
  done
  # d: coset union, D vs j per k
  [ -s $A ] && jq -r 'select(.kind=="f4" and .arm=="cosetunion" and .target=="random0" and .draw==0 and (.timed_out|not)) | ["d_cu","k=\(.k)",.r,.D]|@tsv' $A
} > fig_data.tsv
$SVG < fig_data.tsv > fig_enc_control.svg
# No SVG rasterizer (rsvg-convert) or LaTeX on this box: SVG only.
