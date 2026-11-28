# Depth certificates for `LFD(k)`: what is proved, for which depths, and by what

2026-11-26, Claude Code. Owner priority 5: advance the unresolved Lean depth certificates
without conflating finite results and asymptotic theorems.

Three kinds of evidence are kept apart here:

- **(A) All-depth theorems** hold for every depth `k` in a stated range, and are proved in the
  Lean kernel or on paper.
- **(B) Finite-depth certificates** hold for specific `(p, A, B, t, k)` and are checked by the
  independent C checker (`native/checker/chaincheck.c`). Lean enters only through the generic
  lemmas that turn a checked closure into a bound, and through the depth transfer.
- **(C) Open statements** have neither kind of proof.

No row below derives a growth law from finite data.

## (A) Theorems for every depth

| Statement | Range | Status |
| --- | --- | --- |
| `sd = ref = LFD` for the chain system (T2) | all `k`, `p` prime | Lean `lastFallDegree_eq_solvingDegree_chain` |
| depth monotonicity `LFD(k') ≤ LFD(k)` for `k' ≤ k`, same `(p, A, B, t)` (T4) | all `k' ≤ k` | Lean `one_mem_closure_of_le`, `solvingDegree_mono` |
| `1 ∉ V_c ⇒ LFD > c`, `1 ∈ V_c ⇒ LFD ≤ c` | all `k` | Lean `lt_solvingDegree_of_not_mem`, `solvingDegree_le_of_mem` |
| `LFD(k) ≥ 3` | all `k ≥ 2`, any `(A, B, t)` | Lean `two_lt_solvingDegree` |
| **`LFD(k) ≥ 4`** (T6) | **all `k ≥ 2`, `p ≠ 2`, `A + 3t² ≠ 0`, any `B`** | **Lean, new: `three_lt_solvingDegree` (`lean/ChainSd/LowerFour.lean`)** |
| unitness of the admissible family (T3) | every admissible `p`, `k` | Lean `family_S3_ne_zero`; grid-to-unit bridge paper only |
| `LFD(k) ≤ 2k + 1` (T7) | all `k ≥ 1`, `A = 2, B = 1` | paper only (§6) |
| Theorems A and A′: a `k`-free cofactor/shift hypothesis implies `1 ∉ V_c(k)` (T10, T13) | conditional | Lean `one_not_mem_closure_of_cofactorHyp`, `one_not_mem_closure_of_shiftHyp` |

**The new Lean theorem (T6).** At depth 2, `W = span {g, σ_i g}` is 3-closed. An element of `W`
of weight at most 2 has zero coefficients at the four weight-3 characters `(3,1)`, `(3,2)`,
`(1,3)` and `(2,3)`. These four equations reduce, in each coordinate, to

    (−2t)·c₀ + (−2t² − 2A)·c₁ = 0,    c₀ − 2t·c₁ = 0,

with determinant `−2(3t² + A)`. So every `c_i = 0`, the only admitted shifts are those of
multiples of `g`, and `1 ∉ V_3(2)`. Depth monotonicity (Lean) gives `1 ∉ V_3(k)` for every
`k ≥ 2`, and hence `LFD(k) ≥ 4`.

- **Hypotheses:** the formal statement needs only `p ≠ 2` and `A + 3t² ≠ 0`. The report's
  `t ≠ 0` is not used.
- **Non-vacuity:** `hyp_F1` and `hyp_F2` check the hypothesis on both certificate instances.
- **Axioms:** the theorem depends only on `propext`, `Classical.choice` and `Quot.sound`
  (`reports/axioms.txt`). There is no `sorry`, `native_decide` or custom axiom.
- **Build:** `lake build` completes with 8,934 jobs on Lean 4.34.1 and Mathlib v4.34.1.

## (B) Finite-depth certificates

All are on the admissible instances F1 (`p = 12289, n = 11`) and F2 (`p = 65537, n = 3`). The
C-checker certificates are in `certs/`, with `"verified": true`.

| `k` | Lower bound | Upper bound | `LFD(k)` | Evidence |
| --- | --- | --- | --- | --- |
| 2–5 (F1) | `≥ 4`: Lean T6 for every `k`; also C cert `1 ∉ V_3` | C cert `1 ∈ V_4` | **4** | lower: Lean; upper: C checker |
| 6–9 (F1) | C cert `1 ∉ V_4` | C cert `1 ∈ V_5` | **5** | C checker (both) |
| 10 (F2) | C cert `1 ∉ V_5` | none | **≥ 6** | C checker |
| 11–15 (F2) | `1 ∉ V_5(10)` transferred by Lean T4 | Lean T7 is not applicable (paper, and only for `A = 2, B = 1`) | **≥ 6** | C checker + Lean transfer |

With the new Lean T6, the `k = 2..5` lower bounds no longer depend on the C checker. The exact
value `LFD = 4` there still needs the checker for the upper side. Nothing in this table bounds
`LFD(k)` for `k > 15`.

## (C) Open

| Statement | Why it is open | Concrete next step and cost |
| --- | --- | --- |
| exact `LFD(10)` (expected 6) | no upper certificate | `1 ∈ V_6(10)` on `C(20, ≤6) = 60,460` columns. The dense engine took about 1 h at 21,700 columns, so about 20 h at cubic scaling on this 4-core host. A sparse or structured derivation is needed. |
| exact `LFD(15)` (expected 7) | no certificate at `D = 6, 7` | lower `1 ∉ V_6(15)` on 768,212 columns; upper on 2,629,575 columns (§13.6), out of reach of available hosts |
| finite certificates inside the Lean kernel | `Finsupp` arithmetic is noncomputable, and the closures have 15–21,700 columns | needs a computable model of `ChainAlg p k` with a proved bridge, plus a reflective checker; still the cleanest route for `k = 2..5` (at most 386 columns) |
| linear lower bound `LFD(k) ≥ ck − C` (T9) | reduced to Missing Lemma Π (Lean Theorems A and A′), which a counting heuristic predicts to fail | this is an asymptotic question; finite certificates cannot settle it |
| `LFD(k) ≤ 2k + 1` in Lean (T7) | paper proof only | the next all-depth formalization target |
