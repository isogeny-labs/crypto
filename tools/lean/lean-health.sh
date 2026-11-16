#!/usr/bin/env bash
# Advisory Lean health report (skill lean-instance-gate). Never fails a build:
# it measures how far each study re-derives instead of instantiating.
#
#   tools/lean/lean-health.sh                 # table over every study under research/
#   tools/lean/lean-health.sh --diff REF      # advisory notes for .lean files changed since REF
#   tools/lean/lean-health.sh --json          # one JSON object per study
#
# Per study (the directory holding a lakefile, else the file's directory):
#   files            .lean files
#   crypto_lean      files importing CryptoLean (or the lakefile requires crypto-lean)
#   heartbeats       `set_option maxHeartbeats` / `maxRecDepth` occurrences
#   native           `native_decide` occurrences
#   theorems         theorem/lemma declarations
#   top_family       largest family of theorem names equal after removing digits,
#                    with its size (a large family suggests a missing generic checker)
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

mode=table
ref=""
case "${1:-}" in
  --diff) mode=diff; ref="${2:?--diff needs a ref}" ;;
  --json) mode=json ;;
  ""|--table) ;;
  -h|--help) sed -n '2,18p' "$0"; exit 0 ;;
  *) echo "unknown option $1" >&2; exit 2 ;;
esac

study_of() {  # nearest ancestor with a lakefile, else the file's directory
  local d
  d=$(dirname "$1")
  local start=$d
  while [ "$d" != "." ] && [ "$d" != "/" ]; do
    if [ -f "$d/lakefile.toml" ] || [ -f "$d/lakefile.lean" ]; then echo "$d"; return; fi
    d=$(dirname "$d")
  done
  echo "$start"
}

uses_crypto_lean() {  # $1 file, $2 study dir
  grep -qE '^import CryptoLean' "$1" && return 0
  grep -qsiE 'crypto-lean|crypto_lean|CryptoLean' "$2/lakefile.toml" "$2/lakefile.lean" 2>/dev/null
}

if [ "$mode" = diff ]; then
  mapfile -t changed < <(git diff --name-only --diff-filter=AM "$ref"...HEAD -- '*.lean' 2>/dev/null || true)
  if [ "${#changed[@]}" -eq 0 ]; then echo "lean-health: no .lean files changed since $ref"; exit 0; fi
  notes=0
  for f in "${changed[@]}"; do
    [ -f "$f" ] || continue
    s=$(study_of "$f")
    added=$(git diff "$ref"...HEAD -- "$f" | grep -cE '^\+.*set_option (maxHeartbeats|maxRecDepth)' || true)
    nat=$(git diff "$ref"...HEAD -- "$f" | grep -cE '^\+.*native_decide' || true)
    if [ "$added" -gt 0 ]; then echo "::notice file=$f::lean-instance-gate: $added new maxHeartbeats/maxRecDepth; find the missing law or record the argument"; notes=$((notes+1)); fi
    if [ "$nat" -gt 0 ]; then echo "::notice file=$f::lean-instance-gate: $nat new native_decide; needs tools/lean/trusted-native.txt or a checker with a soundness theorem"; notes=$((notes+1)); fi
    if ! uses_crypto_lean "$f" "$s"; then echo "::notice file=$f::lean-instance-gate: study $s does not use crypto-lean; check for an existing structure before re-deriving"; notes=$((notes+1)); fi
    fam=$({ grep -hoE '^(theorem|lemma) [A-Za-z_.'"'"'0-9]+' "$f" || true; } | awk '{print $2}' | sed -E 's/[0-9]+//g' | sort | uniq -c | sort -rn | awk 'NR==1 && $1>=20 {print $1" x "$2}')
    if [ -n "$fam" ]; then echo "::notice file=$f::lean-instance-gate: theorem family $fam; consider one generic checker with a soundness theorem"; notes=$((notes+1)); fi
  done
  echo "lean-health: ${#changed[@]} changed file(s), $notes advisory note(s) (never blocking)"
  exit 0
fi

tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT
git ls-files -- 'research/**/*.lean' '*.lean' | grep -v '^tools/lean/selftest' | sort -u > "$tmp.files" || true
while IFS= read -r f; do
  [ -f "$f" ] || continue
  s=$(study_of "$f")
  cl=0; uses_crypto_lean "$f" "$s" && cl=1
  hb=$(grep -cE 'set_option (maxHeartbeats|maxRecDepth)' "$f" || true)
  nd=$(grep -cE 'native_decide' "$f" || true)
  { grep -hoE '^(theorem|lemma) [A-Za-z_.'"'"'0-9]+' "$f" || true; } | awk -v s="$s" '{n=$2; gsub(/[0-9]+/,"",n); print "N\t" s "\t" n}' >> "$tmp"
  printf 'F\t%s\t%s\t%s\t%s\n' "$s" "$cl" "$hb" "$nd" >> "$tmp"
done < "$tmp.files"
rm -f "$tmp.files"

body=$(awk -F'\t' -v mode="$mode" '
  $1=="F" { files[$2]++; cl[$2]+=$3; hb[$2]+=$4; nd[$2]+=$5 }
  $1=="N" { th[$2]++; fam[$2 SUBSEP $3]++ }
  END {
    for (k in fam) { split(k, p, SUBSEP); if (fam[k] > best[p[1]]) { best[p[1]] = fam[k]; bname[p[1]] = p[2] } }
    for (s in files) {
      if (mode == "json")
        printf "{\"study\":\"%s\",\"files\":%d,\"crypto_lean\":%d,\"heartbeats\":%d,\"native\":%d,\"theorems\":%d,\"top_family\":%d,\"top_family_name\":\"%s\"}\n", s, files[s], cl[s], hb[s], nd[s], th[s]+0, best[s]+0, bname[s]
      else
        printf "%d\t| %s | %d | %d | %d | %d | %d | %s |\n", best[s]+hb[s], s, files[s], cl[s], hb[s], nd[s], th[s]+0, (best[s] > 0 ? best[s] " x " bname[s] : "-")
    }
  }' "$tmp")
if [ "$mode" = json ]; then
  printf '%s\n' "$body"
  exit 0
fi
echo "| study | files | crypto_lean | heartbeats | native | theorems | top_family |"
echo "|:--|--:|--:|--:|--:|--:|:--|"
printf '%s\n' "$body" | sort -t$'\t' -k1,1nr | cut -f2-
awk -F'\t' '$1=="F"{F++; C+=$3; H+=$4; D+=$5; if(!($2 in seen)){seen[$2]=1; S++}} $1=="N"{T++} END{printf "\nTotal: %d studies, %d files, %d import crypto-lean, %d heartbeat overrides, %d native_decide, %d theorems. Advisory only (skill lean-instance-gate).\n", S, F, C, H, D, T}' "$tmp"
