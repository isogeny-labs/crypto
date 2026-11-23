#!/bin/sh
# Thin orchestration: hash every evidence file named in ledger.list and write
#   evidence/ledger.tex   (rows for the paper's appendix table)
#   EVIDENCE.md           (the same table for readers of the directory)
# Run from papers/ic-negative-results/ with the three repositories checked out
# side by side under $ROOT (default /home/user). Read-only on the evidence.
set -eu
ROOT="${ROOT:-/home/user}"
here=$(cd "$(dirname "$0")" && pwd)
list="$here/ledger.list"
tex="$here/ledger.tex"
md="$here/../EVIDENCE.md"
{
  printf '%s\n' '\begin{longtable}{@{}l p{10.6cm} l@{}}' '\toprule' 'id & repository:path & sha256 (16) \\' '\midrule' '\endhead'
} > "$tex"
{
  echo "# Evidence ledger for papers/ic-negative-results"
  echo
  echo "Every number in the paper cites one of these files as [E#]. The SHA-256 is"
  echo "of the file as read when this ledger was generated (first 16 hex digits;"
  echo "the full digest is in \`evidence/ledger.full\`). Repository heads at that time:"
  for r in crypto crypto-autoresearcher; do
    echo "- \`$r\` at \`$(git -C "$ROOT/$r" rev-parse --short=12 HEAD)\`"
  done
  echo
  echo "| id | repository:path | sha256 (16) | what |"
  echo "|:--|:--|:--|:--|"
} > "$md"
: > "$here/ledger.full"
while IFS='|' read -r id repo path note; do
  f="$ROOT/$repo/$path"
  [ -f "$f" ] || { echo "missing $f" >&2; exit 1; }
  full=$(sha256sum "$f" | cut -d' ' -f1)
  short=$(printf %s "$full" | cut -c1-16)
  printf '%s  %s:%s\n' "$full" "$repo" "$path" >> "$here/ledger.full"
  printf '| %s | `%s:%s` | `%s` | %s |\n' "$id" "$repo" "$path" "$short" "$note" >> "$md"
  # LaTeX row; the path is typeset with \nolinkurl so it can break at slashes.
  printf '%s & \\nolinkurl{%s:%s} & \\texttt{%s} \\\\\n' "$id" "$repo" "$path" "$short" >> "$tex"
done < "$list"
printf '%s\n' '\bottomrule' '\end{longtable}' >> "$tex"
echo "ledger written: $(wc -l < "$list") entries"
