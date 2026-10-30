# Sign-safe Kummer differential walk: frozen invariant gate

Frozen before execution on 2026-10-22 UTC. This is a necessary algebraic gate
for the Kummer lead in `research/four_followups_20261021/PROTOCOL.md`; it is not
an end-to-end collector or a fast Kummer implementation.

## Cohort and checks

Use the immutable 21 Scholten models and 60 public targets per model from
`research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`.
Read only `rho_neg` source rows. For each of 1,260 targets, independently
recompute the planted source scalar and transfer its generator and target to
the genus-2 Jacobian. Use six fixed coefficient pairs (including identity and
inverse cases) and one seed-derived pair. For each pair, form the quotient
classes K(A), K(B), K(A-B), and enumerate the four lifts of K(A),K(B).
Keep pairs whose difference projects to K(A-B). The resulting K(A+B) must be
unique and equal to the signed Jacobian oracle. Given K(A+B), K(A+B+G),
and signed G, enumerate the two lifts and require unique signed recovery.

The quotient representation in this gate is the canonical Mumford divisor
`min(D,-D)`, and differential addition is an exhaustive Jacobian reference
oracle over its four possible lifts. It proves only the quotient recurrence
and sign-label recovery invariant. It does **not** measure a coordinate-only
Kummer xADD/xDBL, demonstrate faster arithmetic, or price a DLP solve.

For each model additionally test the identity, equal/inverse inputs, and a
nonzero rational Weierstrass two-torsion divisor using the independently
recorded rational root set. The model and target checks must retain failures
as raw rows. No summary may count a partial row as passed.

## Output contract

`results/target-gate.jsonl` has one JSON object per target with workload ID,
source scalar check, seven case counts, differential output and signed-recovery
statuses. `results/model-exceptions.jsonl` has one row per model. The manifest
records source archive/hash, commands, program revision, SHA-256 files, counts,
and explicit nulls for the 36-cell candidate/rho ratios. A source-linked
report, quantitative SVG and PNG, and PDF accompany the raw records.

An accepted result is a **reference invariant gate**. Fast Kummer coordinates,
exception-safe differential formulas, cold full-collector integration, 1,260
source-verified DLP solves, and matched rho costs remain required under the
original protocol.
