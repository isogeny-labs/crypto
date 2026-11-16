#!/usr/bin/env bash
# Rebuild the report of research/isogeny_class_ic_yield_20261102: pool and analyse P3 shards with p3h,
# run the Rust generator (REPORT.md, REPORT.typ, SVGs), render REPORT.pdf with Typst.
# Light on CPU and disk: one rustc compile of one file, p3h analyze on a few MB, typst render.
# Env: P3H (path to p3h), TMPDIR (scratch), REPORT_PY (python with the typst wheel, rendering only).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STUDY="$(dirname "$HERE")"
ROOT="$(cd "$STUDY/../.." && pwd)"
P3H="${P3H:-$ROOT/target/release/p3h}"
TMP="$(mktemp -d "${TMPDIR:-/tmp}/isoyield-report.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT
FULL="$STUDY/p3_ecc2k130/full"
ADIR="$HERE/data/p3_analysis"
mkdir -p "$ADIR" "$HERE/figures"

# 1. compile the generator
nice -n 10 rustc -O --edition 2021 "$HERE/gen/gen_report.rs" -o "$TMP/gen_report"

# 2. P3 pooled analyses: masked first, then unmasked (PROTOCOL section 4 blinding order).
emit() { # stem...: cat each present file (gz preferred) to stdout
  for s in "$@"; do
    if [ -f "$FULL/$s.jsonl.gz" ]; then gzip -dc "$FULL/$s.jsonl.gz"
    elif [ -f "$FULL/$s.jsonl" ]; then cat "$FULL/$s.jsonl"; fi
  done
}
rm -f "$ADIR"/*.txt
if [ -x "$P3H" ]; then
  pool() { # name stems...
    local name="$1"; shift
    emit "$@" > "$TMP/$name.jsonl"
    [ -s "$TMP/$name.jsonl" ] || { echo "pending: no rows for $name" > "$ADIR/$name.txt"; return 0; }
    for mode in masked unmasked; do
      flag=""; [ "$mode" = unmasked ] && flag="--unmask"
      # per-instance family-minima lines are not archived (the generator tabulates recorded values); rerun p3h for them
      { "$P3H" analyze --rows "$TMP/$name.jsonl" $flag 2>&1 || true; } | sed '/^== presentation-family minima/q' > "$ADIR/$name.$mode.txt"
      echo "(per-instance lines omitted: rerun p3h analyze --rows <pooled file> for the full listing)" >> "$ADIR/$name.$mode.txt"
    done
  }
  pool p3a_main_k3 p3a_main_k3
  pool p3a_main_k4 p3a_main_k4.s0 p3a_main_k4.s1
  pool p3a_main_k5 p3a_main_k5
  pool p3a_ctrl_k3 p3a_ctrl_k3
  pool p3a_ctrl_k4 p3a_ctrl_k4.s0 p3a_ctrl_k4.s1
  pool p3a_m2 p3a_m2
  pool p3b_main_k34 p3b_main_k34
  pool p3b_main_k5 p3b_main_k5.s0 p3b_main_k5.s1
  pool p3b_stdv p3b_stdv.s0a p3b_stdv.s0b p3b_stdv.s0c p3b_stdv.s1a p3b_stdv.s1b p3b_stdv.s1c
  pool p3b_badapt p3b_badapt
  pool p3b_bsparse p3b_bsparse
  pool p3b_ctrl p3b_ctrl
  pool c2_n131 p3b_stdv.s0a p3b_stdv.s0b p3b_stdv.s0c p3b_stdv.s1a p3b_stdv.s1b p3b_stdv.s1c p3b_badapt p3b_bsparse
  pool c2_n43 p3a_ctrl_k3 p3a_ctrl_k4.s0 p3a_ctrl_k4.s1
else
  echo "p3h not found at $P3H: P3 analyses not archived in this build" > "$ADIR/NOT_BUILT.txt"
fi

# 3. generate REPORT.md, REPORT.typ, SVGs and data
"$TMP/gen_report" --study "$STUDY" --out "$HERE"

# 4. render the PDF (document rendering only)
PY="${REPORT_PY:-python3}"
if ! "$PY" -c "import typst" 2>/dev/null; then
  "$PY" -m venv "$TMP/venv"
  "$TMP/venv/bin/pip" install -q typst
  PY="$TMP/venv/bin/python"
fi
(cd "$HERE" && "$PY" -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')")
echo "built $HERE/REPORT.pdf"
