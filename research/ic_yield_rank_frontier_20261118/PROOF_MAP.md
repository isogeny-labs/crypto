# Yield/rank frontier proof map

Date: 2026-11-18

This map applies the Lean instance gate before the two proof modules are
edited.  The statements are deliberately generic.  No theorem below asserts
an elliptic-curve yield law or a random-matrix rank law.

## Yield, solver visibility and censorship

1. **Structure above.** `Fintype` domains and codomains, `Finset` fibres and
   predicates on a finite domain.  The semantic object is a map `φ : τ → G`;
   a solver is represented only by the predicate selecting the domain elements
   it exposes.
2. **Theorem used as stated.** `IcNegative.sum_yield` and
   `IcNegative.total_yield_eq` from `IcNegative.YieldConservation`; the standard
   `Finset.card_le_card` theorem supplies subset monotonicity.
3. **Objects as instances.** The generic visible fibre is a filtered fibre of
   `φ`.  Two explicit maps `Bool → Bool` instantiate the same finite semantic
   structure: the constant map and the identity map.
4. **Assembly probe.** For any acceptance predicate, the visible fibre is a
   subset of the mathematical fibre, hence its cardinality is at most the
   mathematical yield.  Under pointwise completeness the two fibres are equal.
   Both facts use only `Finset.filter_subset`/extensionality.  The endpoint then
   applies `sum_yield` to the two Boolean maps and computes only their fibres.
5. **Missing components only.** A definition of solver-visible yield; the
   subset and completeness lemmas; explicit Boolean countermodels for
   mean-versus-shape, rejection of a real witness, and success-only retention.
6. **Inheritance table.** Fibre enumeration: `Finset.univ.filter`; total yield:
   `IcNegative.sum_yield`; equality under completeness: `Finset.ext`; finite
   countermodel arithmetic: kernel `decide` on `Bool` and `Nat`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `visibleYieldAt_le_yieldAt` | law | `IcNegative.YieldVisibility` | finite map and decidable predicate | none |
| `visibleYieldAt_eq_of_complete` | law | same | finite map and predicate | pointwise completeness |
| `same_total_different_coverage` | endpoint/countermodel | same | two maps `Bool → Bool` | none |
| `reject_all_hides_real_witness` | endpoint/countermodel | same | identity map on `Bool` | none |
| `success_only_censors_failure` | endpoint/countermodel | same | complete Boolean population | none |

## Left/right-kernel scope

1. **Structure above.** Finite-dimensional function spaces over a field and
   the row-action linear map represented by `M : Fin n → Fin B → K`.
2. **Theorem used as stated.** `LeftKernel.exists_left_kernel` for `B < n`.
   No part of its finrank proof is copied.
3. **Object as an instance.** The zero matrix with `n = B + 1`, together with
   a coordinate vector `Pi.single j 1` in its right kernel.
4. **Assembly probe.** Apply `exists_left_kernel` to the zero matrix for the
   nonzero left-kernel vector.  Separately, direct simplification proves the
   coordinate vector is a nonzero right-kernel vector.  Thus row surplus gives
   left-kernel existence but does not imply reusable right uniqueness.
5. **Missing components only.** A predicate for the right kernel, an unused-
   column lemma, and the zero-matrix endpoint combining that lemma with the
   existing left-kernel theorem.
6. **Inheritance table.** Left kernel: `LeftKernel.exists_left_kernel`; right
   action: finite sum; nonzero coordinate: `Pi.single`; zero-matrix action:
   `zero_mul` and `Finset.sum_const_zero`.

| statement | kind | owner | quantified structure | hypotheses |
|---|---|---|---|---|
| `unused_column_gives_nonzero_right_kernel` | law | `KernelModeCountermodels` | finite matrix over a field | an unused column |
| `surplus_can_have_both_nonzero_kernels` | endpoint/countermodel | same | zero `(B+1) × B` matrix | `0 < B` |

There are no new syntax types, no copied algebraic theorem, no raised limits,
and no `native_decide`.  The countermodels refute implications between scopes;
they do not refute a separately stated probabilistic law for a specified row
ensemble or solver.
