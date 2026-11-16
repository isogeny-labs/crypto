#!/bin/sh
# AMENDMENT-1.md: `analyze` refuses an incomplete ladder and, on the complete
# one, reproduces the committed analysis byte for byte. Run from anywhere after
# run.sh (it needs main/*.tsv, which run.sh regenerates).
set -eu
cd "$(dirname "$0")"
SRC=../../examples/coincidence_ladder.rs
BIN=${CL_BIN:-bin/coincidence_ladder}
mkdir -p bin audits
[ -x "$BIN" ] || rustc -O -o "$BIN" "$SRC"
T=$(mktemp -d "${TMPDIR:-/tmp}/cl.XXXXXX")
trap 'rm -rf "$T"' EXIT
refused() { # label, files...
  label=$1; shift
  if "$BIN" analyze "$@" > "$T/out" 2> "$T/err"; then echo "NOT REFUSED: $label" >&2; return 1; fi
  [ ! -s "$T/out" ] || { echo "reading printed for: $label" >&2; return 1; }
  echo "refused: $label: $(cat "$T/err")"
}
for b in 16 20 24 28 32; do
  awk '$3 == 0' "pilot/pilot-$b.tsv" > "$T/theta0-$b.tsv"
done
{
  refused "pilot TSVs filtered to theta index 0" "$T"/theta0-*.tsv
  refused "pilot TSVs without the 32-bit rung" pilot/pilot-16.tsv pilot/pilot-20.tsv pilot/pilot-24.tsv pilot/pilot-28.tsv
  refused "main 32-bit rung only" main/main-32.tsv
} | tee audits/refusal.txt
"$BIN" analyze main/main-*.tsv > "$T/analysis.json"
cmp "$T/analysis.json" analysis/analysis.json
echo "analyze on main/*.tsv reproduces analysis/analysis.json byte for byte" | tee -a audits/refusal.txt
