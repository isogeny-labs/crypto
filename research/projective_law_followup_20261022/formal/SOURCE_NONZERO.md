# Nonvanishing of the common 2+2 source function

**Scope:** The new Lean declarations prove a conditional nonvanishing property on the actual weighted-Proj curve. They do not add a DLP experiment, a target solve, or a cost row. The prior 1,260-target and matched-rho data remain in [the four-followup packet](../../four_followups_review_20261022/REPORT.md).

For a field `K`, the affine chart is `K[x,y]/(y²-f(x))`. Under the nested polynomial equivalence, its equation has degree two in `y`. A polynomial depending only on `x` has degree zero in `y`; therefore `y` cannot equal such a polynomial in the quotient. The checked normalized lemma says that `p(x)-B*y` is nonzero for every nonzero `B`.

The common 2+2 numerator `N` is a polynomial in `x`. The new source-specific theorem applies the normalized lemma to `N`, then uses the previously checked injection from the actual affine chart into the actual curve's function field. Thus `N/B-y` is nonzero when `B != 0` and the quintic is separable. The affine lemmas require neither separability nor a characteristic restriction. The field embedding requires separability because that is the hypothesis of the existing integrality theorem. No assumption that the auxiliary coefficient `C` is nonzero is needed.

The pinned Lean 4.34.1 compiler receipt, direct axiom audits, source files, [status CSV](source-nonzero.csv), [dependency graph](source-nonzero.svg), and [PDF](SOURCE_NONZERO.pdf) are retained together. The four new declarations' axiom audits list only `propext`, `Classical.choice`, and `Quot.sound`; no `sorryAx` appears.

This supplies a prerequisite for the previously checked `Scheme.ord` wrapper. It does not prove that a named closed support has coheight one, calculate orders at all finite supports or infinity, construct a principal divisor, derive a `Pic⁰` class equation, or connect every compiled Rust branch to the proof-side source expression. The CSV keeps those gates explicit.

**Prior sources checked:** [Common22BranchDomain.lean](Common22BranchDomain.lean) contains the exact nested-degree argument used as a template; [WeightedProjSourceFunctionField.lean](WeightedProjSourceFunctionField.lean) defines the actual embedding and denominator condition. [Stacks Project Lemma 10.119.7](https://stacks.math.columbia.edu/tag/00PD) describes the downstream DVR valuation step; it does not supply the nonvanishing result or the missing global divisor theorem.
