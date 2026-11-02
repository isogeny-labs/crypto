# Lean: Vélu x-chains admit no monomial coordinates (ℓ = 2 and odd ℓ ≥ 3)

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

## Readable proof, odd ℓ ≥ 3 (`VeluObstructionOdd.lean`)

**Orders of zeros.** Restrict a binary form F to the line t ↦ p + tq.
- F vanishes to order ≥ 2 at p when the derivative at t = 0 is 0 in every
  direction q.
- F has a simple zero at p when the value at t = 0 is 0 and the derivative
  is nonzero in some direction.

A linear substitution moves lines to lines, so it carries these orders
across exactly (`lineR_subst`).

1. **The Vélu map's fibre over ∞.** The map is (N : Z·ψ²). Its fibre over ∞
   is −Z·ψ².
   - It vanishes to order ≥ 2 at every root (r : 1) of ψ.
   - It has a simple zero at (1 : 0), because ψ(1, 0) = 1 for monic ψ.
2. **Monomial fibres.** The fibre of a monomial map over any target is a
   binomial αX^ℓ + βZ^ℓ.
3. **A binomial can't have both kinds of zero.** Suppose it has a zero of
   order ≥ 2 at P ≠ 0. The directions (1, 0) and (0, 1) give
   ℓαP₀^{ℓ−1} = 0 and ℓβP₁^{ℓ−1} = 0. So α = 0 or β = 0, and say α = 0.
   - Then at any zero R the derivative ℓβR₁^{ℓ−1}q₁ is nonzero only when
     βR₁ ≠ 0.
   - But then the value βR₁^ℓ is nonzero, so R is not a zero after all.
   - So a binomial with a zero of order ≥ 2 has no simple zero.
4. **Contradiction.** If M′∘f∘M⁻¹ were monomial, the fibre over M′(∞) would
   be a binomial. It would equal (up to det M′) the fibre of f over ∞ after
   substitution by adj M.
   - By (1) and the transport lemma, it would then have both a zero of order
     ≥ 2 and a simple zero.
   - (3) rules that out.

## Proof map

| Report statement | Lean declaration | Hypotheses (exact) | Status |
|---|---|---|---|
| Fibres of a conjugated map are pulled-back fibres | `fibre_conj` | none (any field) | checked |
| Discriminant under substitution and scaling | `BQF.disc_comp`, `BQF.disc_smul` | none | checked |
| Double roots are carried by invertible substitutions | `BQF.grad_zero_of_comp` | `a*d - b*c ≠ 0` | checked |
| (1) Monomial branch values are only 0 and ∞ | `monomial_branch` | `(2:K) ≠ 0`, `IsMonomial G` | checked |
| (1) Monomial maps ramify at 0 and ∞ | `monomial_ramif` | `IsMonomial G`, `q₁ * q₂ = 0` | checked |
| (2) Vélu fibre discriminant and partials | `velu_fibre_disc`, `velu_fibre_dX`, `velu_fibre_dZ` | none | checked |
| (3) Codomain cubic factorisation | `codomain_cubic_factor` | `x₀³ + a x₀ + b = 0` | checked |
| (4) Core obstruction | `no_two_monomial_steps` | `(2:K) ≠ 0`, `(x₁ - x₀)² = 4v`, `v₁ ≠ 0`, `det Mᵢ ≠ 0` (i = 0, 1, 2) | checked |
| **Main theorem** (cyclic Vélu chains) | `velu_chain_not_monomial` | `(2:K) ≠ 0`, `x₀` root of `x³+ax+b`, `3x₀²+a ≠ 0`, `x₁` root of the codomain cubic, `x₁ ≠ -2x₀`, `det Mᵢ ≠ 0` | checked |
| The predicate is not vacuous | `squareMap_isMonomial` | none | checked |
| **ℓ ≥ 3, any field with ℓ a unit:** line restriction commutes with substitution | `lineR_subst`, `eval0_lineR` (`VeluObstructionOdd.lean`) | none | checked |
| ℓ ≥ 3: fibres of a conjugated map | `fibreL_conj` | none | checked |
| ℓ ≥ 3: monomial fibres are binomials αX^ℓ + βZ^ℓ | `monomial_fibre_binom` | `IsMonomialL ℓ G` | checked |
| ℓ ≥ 3: a binomial has no double zero together with a simple zero | `binom_contra` | `2 ≤ ℓ`, `(ℓ:K) ≠ 0`, `P ≠ 0` | checked |
| ℓ ≥ 3: fibre over ∞ of N/ψ² (double at roots of ψ, simple at ∞) | `fibInf_double`, `fibInf_simple_val`, `fibInf_simple_slope` | `ψ(r,1) = 0`; `ψ(1,0) ≠ 0` | checked |
| ℓ ≥ 3: core theorem (single step) | `not_monomial_of_sq_denominator` | `2 ≤ ℓ`, `(ℓ:K) ≠ 0`, `ψ(1,0) ≠ 0`, `ψ(r,1) = 0`, `det M, det M' ≠ 0` | checked |
| **Main theorem, odd ℓ** (Vélu–Kohel form N/ψ², ψ = ∏ (x − x_Q)) | `velu_odd_not_monomial` | `3 ≤ ℓ`, `(ℓ:K) ≠ 0`, `r ∈ S` (a kernel abscissa in K), `det M, det M' ≠ 0`; `N` arbitrary | checked |
| ℓ ≥ 3: the predicate is not vacuous | `power_isMonomial` | none | checked |

## Scope: what this does not prove

- **Vélu's theorem.** The pair of forms `velu x₀ v` is taken as the
  definition of the step. That it is the x-map of the 2-isogeny with kernel
  (x₀, 0) is Vélu's theorem, which is not in Mathlib. The study's harness
  checks it numerically (every pushed generator lies on the codomain).
- **Density 1 ⇒ monomial steps.** The link from "S1 density 1" (the cost
  measure in the study) to "every step is monomial in some coordinates" is
  argued in the protocol but not formalised. The Lean theorem is about
  monomial shape, not about Gröbner density.
- **ℓ ≥ 3** is formalised in `VeluObstructionOdd.lean`. For odd ℓ the
  obstruction is stronger: no Möbius coordinates make even one step
  monomial. The modelling input is again Vélu's theorem, in Kohel's form:
  the x-map is N(x)/ψ(x)², where ψ is the monic kernel polynomial. Only
  the shape of the denominator is used; N is arbitrary.
  - **Kernel point in K.** The theorem assumes one kernel abscissa is in K.
    Over K̄ this always holds, and a Möbius map over K is also one over K̄,
    so the statement over K̄ covers every K.
  - **ℓ a unit in K.** The theorem needs (ℓ : K) ≠ 0, i.e. ℓ ≠ char K.
- **Single steps.** A single Vélu step can be made monomial: send its
  ramification points and branch values to 0 and ∞. The obstruction is
  specifically about two consecutive steps sharing a level.

## Verification record

Both runs used the repository gate, `lean-proof-gate.yml`, dispatched by hand
on branch `claude/velu-obstruction-lean-20261102`. That gate is not a
pull-request check. The project is gate target 23,
`research/isogeny_vs_mu_factor_base_20261102/lean`. The gate compiles every
module and audits each theorem's transitive axioms; only `propext`,
`Classical.choice` and `Quot.sound` are accepted. It found no `sorry`, no
`native_decide` and no user axioms.

| Commit | Modules | Run / job | Gate line |
|---|---|---|---|
| `e2808ec` | `VeluObstruction` (ℓ = 2) | 41484877701 / 125412732227 | `PASS 42 thm 0 trusted` |
| `9c6b4cf` | + `VeluObstructionOdd` | 41488716595 / 125426076496 | `FAIL_build`: parse error at `VeluObstructionOdd.lean:238`. The identifier `hψ∞` is invalid because `∞` is not an identifier character. The run was cancelled. |
| `faea6a9` | `VeluObstruction` + `VeluObstructionOdd` | 41489297063 / 125428825698 | **`PASS 67 thm 0 trusted`** |

In run 41489297063, targets 13 and 126 failed on unrelated projects. They
failed before building anything: the elan installer download returned
`curl: (22) … 500` from GitHub. That is an infrastructure failure, not
evidence about any proof.

Every row of the proof map above is kernel-checked at `faea6a9`.
