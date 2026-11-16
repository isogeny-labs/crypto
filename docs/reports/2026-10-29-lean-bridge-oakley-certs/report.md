# Lean bridge for S3Roots and Pratt certificates for the Oakley orders

2026-10-29, from the CFSG exposure audit (crypto-autoresearcher PR #2690).
Rendered report with the diagram: [report.pdf](report.pdf) (source `report.typ`,
diagram `diagram.svg`). The hypothesis-delta tables for every new theorem are
in the PDF; the summary is below.

**Task 1** (`papers/rootfinding-decomposition/lean/S3Roots.lean`):
`addX_eq_mathlib_addX` (needs `a₁=a₂=0`, `xR≠xF`, `DecidableEq K`; no curve
equation), `pointX_add_eq_xPlus` (adds `Nonsingular` at both points),
`pointX_add_neg_eq_xMinus` (adds `a₃=0`), `S3_factor_mathlib`. `S_m`, `m ≥ 4`
is still not formalized.

**Task 2** (`research/weak_curve_leaderboard_20261012/lean/OakleyPrimality.lean`):
`oakley3Prime_prime`, `oakley4Prime_prime` via Pratt certificates checked by
`decide +kernel` (no `native_decide`). Independent stdlib checker
the Rust checker `src/bin/check_pratt_certificates.rs` passed locally (18 certificates).
It proves primality of the integers only, not that they are RFC point orders.

**Status:** the Lean builds, fresh-kernel replay and axiom audits have not been
run locally (no Lean toolchain installed in this session); they are pending the
CI runs linked in the PR.
