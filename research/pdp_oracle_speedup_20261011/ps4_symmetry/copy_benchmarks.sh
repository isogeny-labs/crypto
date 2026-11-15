#!/usr/bin/env bash
# AGENTS.md, "Performance results also land in benchmarks/": copy the visual record of the PS-4
# run report (PDF, SVG graphs, 150 dpi PNG renders of the same plotting source) into
# benchmarks/20261011-pdp-ps4-symmetry/.  The study directory stays the source of truth; run this
# again after regenerating the report and never edit a copy alone.
#
# This entry is NOT an admitted row of benchmarks/README.md: the validation gate (AGENTS.md,
# docs/VALIDATION_GATE.md) admits a row only with a replay certificate, a validator report and a
# matched rho reference, and a PS-4 stage/counting result has none of them (and the pre-gate
# freeze may only shrink).  The entry's README says so.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
S=research/pdp_oracle_speedup_20261011
R=$S/reports/run-20261011T2346Z
B=benchmarks/20261011-pdp-ps4-symmetry
mkdir -p "$B"
cp "$R/ps4_report.pdf" "$B/report.pdf"
for g in candidate_factor decision leaf mechanism; do
  cp "$R/ps4_$g.svg" "$B/$g.svg"
  cp "$R/ps4_$g.png" "$B/$g.png"
done
cat > "$B/README.md" <<'EOF'
# PS-4 candidate reduction by symmetry (Frobenius-orbit representatives, 2-torsion)

Visual copies of the run report `research/pdp_oracle_speedup_20261011/reports/run-20261011T2346Z/` (`report.pdf` here is its
`ps4_report.pdf`) and its four graphs (SVG, with a 150 dpi PNG render of each, from the same plotting source
`research/pdp_oracle_speedup_20261011/ps4_symmetry/plot_ps4.py`). The study directory
[`research/pdp_oracle_speedup_20261011/ps4_symmetry/`](../../research/pdp_oracle_speedup_20261011/ps4_symmetry/) holds the raw
records, sources, protocol results and README and is the source of truth; these files are copies made by
`ps4_symmetry/copy_benchmarks.sh`.

**Standing: unvalidated, not an admitted row.** A stage/counting measurement of the pairs-and-solve loop on toy fields (112
targets, 14 settings, counted field operations; no wall-time headline, no end-to-end cost, no `S`, no ratio to rho). It has no
replay certificate, validator report or matched rho reference, so it carries no `VALIDATION.json`, is not listed as a row of
`benchmarks/README.md` (the gate admits no such row and the pre-gate freeze may only shrink), and nothing here claims validation.

What the report states (measured, scope in the study README): candidate pairs per target fall by `m/reps` (7.667 to 40.697, about `n`)
against the ordered unreduced loop and counted multiplications keep 0.997 to 1.056 of that factor; the reduced loop on a single
target returned the unreduced verdict on 84 of 112 targets (answered no on 28 of 66 decomposable ones) and reproduces the unreduced
solution set on 112/112 only over all conjugates of the target; `T = (0,1)` preserves no Hamming-weight base (0 images in the base,
14 settings) and halves the pairs exactly on the T-closed base `B±` (94 targets); leaf cost per pair, weight base against the
subspace control: 0.953 to 1.078 counted multiplications, 1.316 to 1.488 model bit operations at matched degree. Two host classes
(Xeon 2.80 GHz and 2.10 GHz), never pooled; shared container, no isolation level claimed.
EOF
echo "wrote $B"
ls -la "$B"
