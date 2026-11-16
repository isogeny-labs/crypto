#!/usr/bin/env bash
# Rebuilds REPORT.pdf from README.md, TABLES.md and figs/ (layout only; numbers come from summary.json).
# Needs: pandoc, and `pip install typst` in a venv (PY=/tmp/report-venv/bin/python).
set -euo pipefail
cd "$(dirname "$0")"
PY=${PY:-/tmp/report-venv/bin/python}
python3 tables.py .
$PY plot.py .
cap() { echo "![](figs/$1.svg)"; echo; echo "*$2 Study jv_sieve_strong_rho_20261019, instances seeds 1–16 per size, 95% bootstrap intervals; source summary.json.*"; echo; }
{ tail -n +2 README.md; echo; echo "## Figures"; echo
  cap ratio_vs_l "Figure 1. Sieve over strong rho (R3), paired by instance, four prices; 1 is parity."
  cap exponent "Figure 2. Mean total F_p multiplications against ℓ; fitted slopes in the legend (theory 1/3 sieve, 1/2 rho)."
  cap rho_factorial "Figure 3. Rho factorial at p = 251, 503, 1009: negation map and batched inversion, harness price."
  cap dense_vs_sparse "Figure 4. Dense over sparse FGLM in the sieve's descent, paired on the same instances."
  cat TABLES.md; } > /tmp/report_full.md
pandoc -f gfm -t typst /tmp/report_full.md | sed -E '/^<[^>]+>$/d' > REPORT_body.typ
{ cat <<'T'
#set page(paper: "a4", margin: (x: 1.6cm, y: 1.8cm), numbering: "1")
#set text(size: 9pt)
#set table(inset: 3pt, stroke: 0.4pt + luma(160))
#show table: set text(size: 6.5pt)
#align(center)[
  #text(14pt, weight: "bold")[The Joux–Vitse sieve on E(F#sub[p⁶]) against a strong rho] \
  #text(9pt)[Weak class only (never prime order); the cover's Jacobian has genus 3. Study `research/jv_sieve_strong_rho_20261019`, run 2026-10-19/20, protocol frozen before data. Every number is copied from `summary.json`, computed by `examples/jv_sieve_study.rs`.]
]
T
cat REPORT_body.typ; } > REPORT.typ
$PY -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
echo built REPORT.pdf
