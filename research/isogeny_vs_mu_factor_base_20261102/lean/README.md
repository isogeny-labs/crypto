# Lean: Vélu x-chains admit no monomial coordinates

This directory formalises the obstruction stated in
`../PROTOCOL_Q4_SEARCH.md` ("Structural expectation") and checked
empirically in `../reports/run-20261102T0139Z/` (0 of 11811 Vélu
representations reach S1 density 1).

- **Lean** `leanprover/lean4:v4.34.1`, **Mathlib**
  `d13f23b723b8a846827a245b89c10fc7d3f11612`. This is the same pin as
  `research/p2_halving_carry_20261101`, and its manifest was copied from
  that study.
- **Build:** `lake exe cache get && lake build`. The repository gate
  (`tools/lean/proof-gate.sh research/isogeny_vs_mu_factor_base_20261102/lean`,
  CI `lean-proof-gate.yml`) builds the project and audits transitive
  axioms. The only axioms allowed are `propext`, `Classical.choice` and
  `Quot.sound`.
- **Status:** see "Verification record" below. The session that wrote these
  proofs had no local Lean toolchain (installing it was blocked as external
  code), so the CI gate is the first and only checker.

## Readable proof

Work in projective coordinates over a field K of characteristic ≠ 2.

- **Maps.** A degree-2 map of ℙ¹ is a pair of binary quadratic forms (N, D).
  Its fibre over (T : W) is the form W·N − T·D.
  - A target is a **branch value** when its fibre has discriminant 0.
  - A point is **ramified** when both partial derivatives of a fibre form
    vanish there, i.e. it is a double root of that fibre.
- **Monomial.** A step is *monomial* when it is (λX², Z²) or (λZ², X²) up
  to a scalar, i.e. u ↦ λu^{±2}.
- **Effect of a coordinate change.** Changing coordinates by Möbius matrices
  (M on the source, M′ on the target) pulls fibres back by M⁻¹ and
  scales them by det M′. So it carries branch values to branch values and
  ramified points to ramified points.

1. **Monomial maps.** The fibre of u ↦ λu² over (T : W) has discriminant
   4κ²λ·TW, so its branch values are exactly 0 and ∞. At 0 and at ∞ some
   fibre has a double root. The same holds for u ↦ λ/u².
2. **Vélu steps.** The step with kernel abscissa x₀ is
   x ↦ x + v/(x − x₀), with v = 3x₀² + a.
   - Its fibre over (T : W) has discriminant (T − x₀W)² − 4vW². So its
     branch values are the affine points x₀ ± 2√v, and ∞ is not one.
   - The fibre over (T₀ : W₀) has a double root at (x₀ : 1) only when
     W₀x₀ − T₀ = 0 and 2W₀v = 0. When v ≠ 0 that forces T₀ = W₀ = 0. So
     **a Vélu step is never ramified at its own kernel abscissa.**
3. **Codomain.** The codomain cubic is x³ + (a − 5v)x + (b − 7x₀v). It equals
   (x + 2x₀)((x − x₀)² − 4v). Its root −2x₀ is the dual kernel.
   - Any other next kernel x₁ satisfies (x₁ − x₀)² = 4v. Such an x₁ is a
     **branch value of the first step**.
   - Its own v₁ is 2(x₁ − x₀)(x₁ + 2x₀), which is nonzero.
4. **Contradiction.** Suppose M₁ made step 1 monomial and M₂ made step 2
   monomial, sharing the level-1 coordinate M₁.
   - M₁(x₁) is a branch value of a monomial map, so M₁(x₁) ∈ {0, ∞}.
   - The monomial step 2 then has a double root of some fibre at M₁(x₁).
   - Pulling back, step 2 would be ramified at its own kernel abscissa x₁,
     which (2) rules out.

So no per-level Möbius coordinates make two consecutive steps of a cyclic
2-isogeny x-chain monomial. Any chain of length ≥ 2 contains such a pair.

## Proof map

| Report statement | Lean declaration | Hypotheses (exact) | Status |
|---|---|---|---|
| Fibres of a conjugated map are pulled-back fibres | `fibre_conj` | none (any field) | in gate |
| Discriminant under substitution and scaling | `BQF.disc_comp`, `BQF.disc_smul` | none | in gate |
| Double roots are carried by invertible substitutions | `BQF.grad_zero_of_comp` | `a*d - b*c ≠ 0` | in gate |
| (1) Monomial branch values are only 0 and ∞ | `monomial_branch` | `(2:K) ≠ 0`, `IsMonomial G` | in gate |
| (1) Monomial maps ramify at 0 and ∞ | `monomial_ramif` | `IsMonomial G`, `q₁ * q₂ = 0` | in gate |
| (2) Vélu fibre discriminant and partials | `velu_fibre_disc`, `velu_fibre_dX`, `velu_fibre_dZ` | none | in gate |
| (3) Codomain cubic factorisation | `codomain_cubic_factor` | `x₀³ + a x₀ + b = 0` | in gate |
| (4) Core obstruction | `no_two_monomial_steps` | `(2:K) ≠ 0`, `(x₁ - x₀)² = 4v`, `v₁ ≠ 0`, `det Mᵢ ≠ 0` (i = 0, 1, 2) | in gate |
| **Main theorem** (cyclic Vélu chains) | `velu_chain_not_monomial` | `(2:K) ≠ 0`, `x₀` root of `x³+ax+b`, `3x₀²+a ≠ 0`, `x₁` root of the codomain cubic, `x₁ ≠ -2x₀`, `det Mᵢ ≠ 0` | in gate |
| The predicate is not vacuous | `squareMap_isMonomial` | none | in gate |

## Scope: what this does not prove

- **Vélu's theorem.** The pair of forms `velu x₀ v` is taken as the
  definition of the step. That it is the x-map of the 2-isogeny with kernel
  (x₀, 0) is Vélu's theorem, which is not in Mathlib. The study's harness
  checks it numerically (every pushed generator lies on the codomain).
- **Density 1 ⇒ monomial steps.** The link from "S1 density 1" (the cost
  measure in the study) to "every step is monomial in some coordinates" is
  argued in the protocol but not formalised. The Lean theorem is about
  monomial shape, not about Gröbner density.
- **ℓ ≥ 3.** The claim that no monomial step exists for ℓ ≥ 3 is not
  formalised here.
- **Single steps.** A single Vélu step can be made monomial: send its
  ramification points and branch values to 0 and ∞. The obstruction is
  specifically about two consecutive steps sharing a level.

## Verification record

Pending the `lean-proof-gate` CI run on the PR that adds this directory. That
record (run id, gate outcome, axiom audit) is appended here when it
completes.
