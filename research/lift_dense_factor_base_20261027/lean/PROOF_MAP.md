# Proof map — lift-dense factor bases

Required by AGENTS.md, "Mathematical results should include Lean proofs".
Status as of 2026-10-27: **the central theorem is built and audited.**

- **Build and toolchain.** `lake build` in this directory is clean, with no
  warnings. The toolchain is `leanprover/lean4:v4.34.1` and Mathlib is
  `d13f23b723b8a846827a245b89c10fc7d3f11612`, the repository pins (see
  [`gate-receipt/versions.txt`](gate-receipt/versions.txt)).
- **Proof gate.** `tools/lean/proof-gate.sh research/lift_dense_factor_base_20261027/lean`
  returns **PASS: 11 theorems, 0 trusted `native_decide`, 0 failing.**
- **Axioms.** Every theorem depends only on `propext`, `Classical.choice`
  and `Quot.sound` (receipt: [`gate-receipt/audit.log`](gate-receipt/audit.log)).
- **No escape hatches.** There is no `sorry`, `admit`, custom axiom or
  `native_decide`.
- **Not run.** `lean4checker --fresh` was not available in this session.

| report statement | Lean declaration | exact hypotheses | status |
|:--|:--|:--|:--|
| Lemma 1, trace tower: Tr_{M/K}(y u) = Tr_{L/K}(y · Tr_{M/L} u) | `LiftDense.trace_mul_algebraMap` | fields K ⊆ L ⊆ M, scalar tower, finite-dimensional | **proved** |
| Theorem 2, trace form: Tr(yu + a + c/(yu)) = Tr(a) | `LiftDense.twisted_line_trace_const` | Tr_{M/L}(u) = 0, Tr_{M/L}(c u⁻¹) = 0; holds for every y ∈ L (y = 0 included, since 0⁻¹ = 0 in Lean) | **proved** |
| O2: Tr(z²) = Tr(z) over F₂ | `LiftDense.trace_sq` | M a finite field, `Algebra (ZMod 2) M` | **proved** (via Frobenius `frobeniusAlgEquivOfAlgebraic` and `Algebra.trace_eq_of_algEquiv`) |
| O2: Tr(c²/x²) = Tr(c/x) | `LiftDense.trace_sq_div` | as above | **proved** |
| O1: Artin–Schreier in characteristic 2, ∃z, z² + z = w ⟺ Tr_{M/F₂}(w) = 0 | `LiftDense.artin_schreier` (with `asMap`, `ker_asMap`, `trace_asMap`) | M a finite field, `Algebra (ZMod 2) M` | **proved**: the range of z ↦ z² + z is contained in ker Tr, and both have dimension n − 1 |
| Lift criterion: for x ≠ 0, x lifts to y² + xy = x³ + ax² + b ⟺ Tr(x + a + b/x²) = 0 | `LiftDense.lifts_iff` | x ≠ 0 | **proved** |
| Theorem 2, lift form: for y ∈ L \ {0}, yu lifts to E_{a, c²} ⟺ Tr(a) = 0 (all-lift line if Tr(a) = 0, all-twist if Tr(a) = 1) | `LiftDense.twisted_line_lifts` | M finite with `Algebra (ZMod 2) M`; intermediate field L, finite-dimensional, scalar tower; b = c²; Tr_{M/L}(u) = 0; Tr_{M/L}(c u⁻¹) = 0; u ≠ 0; y ≠ 0 | **proved** |
| Proposition 3, u-count: n = 2 {0, q−1}; n = 3 {0, q−1, 2(q−1), q²−1}; n = 4 a multiple of q−1 | — | — | **open (O3)**. The projective-zero-set classification is not formalised. It is checked exhaustively at (12,6), (15,5), (18,6), (18,9), (16,4) and on 4096 sampled b at (21,7), (20,5) (`census`), with 0 violations |
| Yield identity: mean ordered yield \|F\|^m/#E, or (\|F\|^m − C_O − C_T)/(#E − 2) on targets ≠ O, T | — | — | **open**: a counting argument over the group law, not attempted |
| Weil cap: all-lift dimension ≤ N/2 + 1 | — | needs Weil's bound for Kloosterman sums, which is not in Mathlib | **open** |

Scope notes:
- The theorems are over an arbitrary finite field M with `Algebra (ZMod 2) M`,
  that is, of characteristic 2, and an arbitrary intermediate field L. The
  report's F_{2^N} ⊇ F_{2^d} is the special case M = GF(2, N), L = GF(2, d).
- The curve is y² + xy = x³ + ax² + b with b = c². Every element of a finite
  field of characteristic 2 is a square, so this loses no generality. The
  theorem takes c, not √b, as the parameter.
- In the lift form, "lifts" means an F_q-rational y exists. Points on the
  twist are exactly the abscissae for which no such y exists. That
  equivalence with the twist is the standard one and is not itself
  formalised here.

Rebuild: `lake exe cache get && lake build`. Audit:
`bash tools/lean/proof-gate.sh research/lift_dense_factor_base_20261027/lean`
from the repository root.
