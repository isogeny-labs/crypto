#!/bin/sh
# Builds REPORT.md and report.typ from analysis/report_body.md and the generated tables, then the PDF.
# Usage: build_report.sh PYTHON_WITH_TYPST BODY.md OUTDIR BASENAME
set -eu
R=research/ecbench_allocator_confound_20261023; A=$R/analysis; T=$A/tables
PY=$1; BODY=$2; OUT=$3; BASE=$4
awk -v T="$T" -v mode=md '
function table(spec,   n,h,i,line,sep) {
  n = split(spec, h, "|"); name = h[1]
  line = "|"; sep = "|"
  for (i = 2; i <= n; i++) { line = line " " h[i] " |"; sep = sep ":--|" }
  print line; print sep
  while ((getline row < (T "/" name ".rows.md")) > 0) print row
  close(T "/" name ".rows.md")
}
/^\{\{table / { s = $0; sub(/^\{\{table /, "", s); sub(/\}\}$/, "", s); table(s); next }
/^\{\{img /   { s = $0; sub(/^\{\{img /, "", s); sub(/\}\}$/, "", s); split(s, p, "|"); print "![" p[2] "](" p[1] ".png)"; print "[SVG](" p[1] ".svg)"; next }
{ print }' $BODY > $OUT/$BASE.md
{
cat <<'HDR'
#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm), numbering: "1")
#set text(size: 9pt)
#set par(justify: false)
#set table(inset: 3pt, stroke: 0.4pt + luma(150))
#show table: it => { show raw: set text(size: 7pt); set text(size: 7pt); it }
#show raw: set text(size: 8pt)
HDR
awk -v T="$T" '
function cells(line,   a, n, i, out) {
  sub(/^\|[ ]*/, "", line); sub(/[ ]*\|[ ]*$/, "", line)
  n = split(line, a, /[ ]*\|[ ]*/); out = ""
  for (i = 1; i <= n; i++) out = out "[" a[i] "], "
  return out
}
function flush(   i) {
  if (nrows == 0) return
  printf "#table(columns: %d,\n", ncols
  printf "  table.header(%s),\n", hdr
  for (i = 1; i <= nrows; i++) printf "  %s\n", rows[i]
  print ")\n"
  nrows = 0; intable = 0
}
/^\{\{table / {
  flush()
  s = $0; sub(/^\{\{table /, "", s); sub(/\}\}$/, "", s); n = split(s, h, "|")
  hd = ""; for (i = 2; i <= n; i++) hd = hd "[*" h[i] "*], "
  printf "#table(columns: %d,\n  table.header(%s),\n", n - 1, hd
  while ((getline row < (T "/" h[1] ".rows.typ")) > 0) printf "  %s\n", row
  close(T "/" h[1] ".rows.typ"); print ")\n"; next
}
/^\{\{img / {
  flush()
  s = $0; sub(/^\{\{img /, "", s); sub(/\}\}$/, "", s); split(s, p, "|")
  printf "#figure(image(\"%s.svg\", width: 100%%), caption: [%s])\n\n", p[1], p[2]; next
}
/^\|/ {
  if (!intable) { intable = 1; nrows = 0; hdr = ""; getline_state = 0 }
  if (hdr == "") { hdr = cells($0); gsub(/\[/, "[*", hdr); gsub(/\], /, "*], ", hdr); n2 = gsub(/\*\], /, "&", hdr); ncols = n2; next }
  if ($0 ~ /^\|[:\- |]+\|$/) next
  rows[++nrows] = cells($0); next
}
{ flush() }
/^### / { sub(/^### /, "=== "); print; next }
/^## /  { sub(/^## /, "== "); print; next }
/^# /   { sub(/^# /, "= "); print; next }
{ print }
END { flush() }' $BODY | sed -e 's/\*\*\([^*]*\)\*\*/*\1*/g' > $OUT/$BASE.typ.tmp
} > $OUT/$BASE.typ
cat $OUT/$BASE.typ.tmp >> $OUT/$BASE.typ; rm $OUT/$BASE.typ.tmp
sed -i '/^#set\|^#show\|^#table\|^#figure\|^  /! s/#\([0-9]\)/\\#\1/g' $OUT/$BASE.typ
$PY -c "import typst; typst.compile('$OUT/$BASE.typ', output='$OUT/$BASE.pdf', root='.')"
