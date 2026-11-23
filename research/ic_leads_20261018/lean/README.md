# Lean proofs for `LEMMAS.md` (Lemmas 1 and 2)

`ICLemmas.lean` is a standalone core Lean 4 file, toolchain
`leanprover/lean4:v4.34.1` (`lean-toolchain`). It uses no Mathlib, because
the container that wrote it had about 1 GB of disk left after installing the
toolchain. Each algebraic statement is over `FieldOps`, a structure carrying
exactly the field axioms its proof uses. Every field satisfies them, so each
theorem holds for every field, `F_{2^n}` included. `F₂` is `Bool` with `xor`.

## Verification

```sh
lean research/ic_leads_20261018/lean/ICLemmas.lean            # compiles, no output
tools/lean/proof-gate.sh research/ic_leads_20261018/lean/ICLemmas.lean
# PASS  26 thm  0 trusted
```

The receipts are in `receipts/` (`build.log`, `audit.log`, `status`). The
transitive axioms of every reported theorem are a subset of `propext` and
`Quot.sound`. There is no `sorry`, no `native_decide` and no user axiom.
`lean4checker --fresh` was not run because it is not installed here. A
deliberately false goal (`example : False := by exact functional_const`)
is rejected with a type mismatch.

## Proof map

| `LEMMAS.md` statement | Lean declaration | status |
|---|---|---|
| Lemma 1, the heart of the proof: a functional vanishing on every `x₁x₂u` (`x₁, x₂ ∈ V`, `u ∈ K`, `V ∋ v ≠ 0`) is zero | `functional_const` (stronger: any function constant on those products is constant; linearity is not used) | proved |
| Lemma 1, as stated: the `n` coordinate forms of `(x₁, x₂, u) ↦ x₁x₂u` on `V × V × K` are linearly independent over `F₂` | `lemma1_no_dependency`: every `c ∈ F₂ⁿ` with `Σ cᵢ coordᵢ(x₁x₂u) = 0` everywhere is zero, given coordinates with a dual basis | proved |
| Lemma 1, the step "the squared parts of `S₃` have Boolean degree ≤ 2" | `sq_add_char2`: `(a + b)² = a² + b²` in characteristic 2 | the algebraic identity is proved; see bridge 2 |
| Lemma 2: the relation phase exceeds `T` unless `R·A·c ≤ T` | `lemma2_relation_phase` | proved (counts in `ℕ`; `c` is any integer lower bound on one call's cost, e.g. `⌈C(N,≤D)^w⌉` for real `w ≥ 1`) |
| Lemma 2, budget form `c ≤ B = T/(R·A)` | `lemma2_budget` (with `⌊T/(R·A)⌋`) | proved |
| Lemma 2, `C(N, ≤D) ≤ (Ne/D)^D` and the logarithmic form `D_adm ≈ n(1/2 − 1/m)/(w log₂(Ne/D))` | — | **not formalized** (bridge 3) |

## Missing bridges (the full theorems stay open in Lean)

1. **From `FieldOps` and coordinates to `F_{2^n}`.** Instantiating
   `FieldOps` from a field, and the coordinates with dual basis from a
   polynomial basis of `F_{2^n}` over `F₂`, is standard. It needs Mathlib's
   `Field` and finite-dimensional `Basis`/`Basis.coord`, which were not
   available here.
2. **From the field identity to Boolean degree.** The statement that the
   degree-3 part of the Weil restriction of `S₃` is exactly the coordinate
   forms of `x₁x₂u` needs a model of Weil restriction as Boolean polynomials:
   coordinates of products as quadratic forms in coordinate bits, and the
   coordinates of `a²` as linear in the bits of `a`. Only the algebraic
   identity behind it, `sq_add_char2`, is formalized.
3. **The binomial estimate.** `Σ_{i≤D} C(N, i) ≤ (Ne/D)^D` needs the real
   exponential (Mathlib `Real.exp`, `Real.add_one_le_exp`) and is used only
   for Lemma 2's asymptotic remark. The inequality the lemma's conclusion
   rests on is the multiplicative bound above, which is proved.

With Mathlib, bridges 1 and 3 are short. Bridge 2 needs a Weil-restriction
library, which is a project of its own.
