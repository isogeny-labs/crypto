# Parametric one-step refinement certificate

The schema-1 parametric-refinement certificate checks the algebraic implication
from explicit generic node premises to packing capacity, group load, strict
shrink, degree preservation, and the symbolic cost ledger. It consumes the
strict-root/history composition boundary but never identifies the normalized
history exemplar with the root. Generic graph premises, graph realization,
the external contraction theorem, and total subexponential work remain
separate. `[pcc:unverified]`

## Explicit premises

Each accepted transcript states that the following inputs are available at an
active node:

```text
sum_gamma (c_gamma-M/D)^2 <= 2M delta,
M >= 2D^2 delta,
M >= LD,
2L <= delta,
maximum old-vertex degree <= delta,
pu <= 34MD,
nD <= 6M,
and the next call rechecks the guards or takes exact fallback.
```

All size and counting quantities are nonnegative, and `D>0` where the
mode/precision proof cancels it. The explicit nonnegativity premise matters:
without `M>=0`, `64 child<=6M` would not imply `8 child<=M` over the reals.

The checker proves implications from these premises. It does not treat their
Boolean tokens as evidence that an arbitrary graph realizes them, and its
summary reports `generic_premises_remain_assumed=true`. `[pcc:unverified]`

## Independently reconstructed implication

The verifier rebuilds every coefficient rather than trusting the transcript:

```text
variance factor                         2
class cap                     cD <= 2M
padded capacity               KL <= (2+1)M = 3M
derived one-side load                   2M/64
history one-side bound                  3M/64
derived child numerator                 2*2 = 4
history child numerator                 2*3 = 6
strict check                             8*6 <= 64
packet degree                            2L <= delta
mode/precision product                   6*34 = 204
sqrt ceiling                             14^2 < 204 <= 15^2
tuple constant                           5*15+5 = 80
local ledger                             80+1+4+5+6 = 96
root-u ledger                            96*3+1+1 = 290
root-log ledger                          290*512 = 148480.
```

It also reconstructs all five retained threshold depths from eightfold shrink
and the `u,a` schedule, then requires the constants to match the independently
replayed history summary. This closes the scalar one-step and cost-ledger
interface conditional on the listed premises. `[pcc:unverified]`

## Native replay and mutation controls

```text
cargo run --quiet --locked \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin parametric-refinement-certificate-emit |
cargo run --quiet --locked \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin parametric-refinement-certificate-check
```

The threshold transcript is 10,204 bytes and reports
`conditional_parametric_implication`. Six focused tests cover the accepted
five-depth composition, missing/reordered schedule data, source/shrink-rule
constants, tuple/cost-ledger constants, missing premises/conclusions, and every
unsupported promotion. The checker imports the composition-boundary verifier,
not graph, packing, factorization, contraction, or producer code.
`[pcc:unverified]`

## Lean kernel replay

`lean/ParametricRefinement.lean` gives a second, typed proof of the downstream
real-inequality chain. It kernel-checks the single-class cap
`[pcc:unverified]` from the scaled variance and source guard, padded capacity,
both child-shrink bounds, degree closure,
the `6*34=204` mode/precision product after cancelling positive `D`, and the
exact cost-ledger constants. The proof makes `M>=0` explicit in both shrink
lemmas; Lean rejected the first draft without it, exposing a genuine missing
domain condition in that standalone formulation.

`lean/ConditionalVariance.lean` moves one premise boundary upstream. It
kernel-checks the bipartite degree-square budget
`sum_left deg^2+sum_right deg^2<=2M delta`, with zero-degree isolates and
parallel-port multiplicity represented directly, the exact centered class
variance identity, division of the scaled collision-objective inequality by
positive `D`, and the resulting `c_gamma D<=2M` cap.

`lean/ConditionalObjective.lean` models the same scaled ordered-edge-pair
objective as the native quadratic reference. For either a row or column
vertex, it shows that summing the objective over all `q` candidate labels is
exactly `q` times the preceding partial objective. Hence some candidate is
nonincreasing, and any minimum candidate—including the native first minimum—is
nonincreasing. The finite vertex-order composition and the initial
endpoint-charge/final class-count identities for this exact model remain open.
`[pcc:unverified]`

The project pins Lean `v4.35.0` and mathlib commit
`56a005855ea17e0b0958f73ec801ae140b0a1679`. Reproduce it with:

```text
cd research/curve25519_subex_pivot_20261125/lean
lake update
lake build
lake env lean AxiomAudit.lean
```

The build completes 9,051 jobs. `#print axioms` reports only `propext`,
`Classical.choice`, and `Quot.sound` for each exported theorem; there are no
project axioms or admitted proofs. This formal replay still takes the variance,
source guard, precision, and mode inequalities as hypotheses. It neither
proves that an arbitrary admitted multigraph realizes them nor formalizes the
external contraction theorem. `[pcc:unverified]`

## Next measurable goal

The remaining local gap is now premise evidence, not downstream coefficient
arithmetic.
A useful next certificate must replace the generic Boolean tokens with typed
evidence for two independent claims:

1. the remaining finite composition of the checked first-minimum steps,
   including the initial endpoint-charge and final class-count identities; and
2. the factorization/precision bounds `nD<=6M` and `pu<=34MD` for the exact
   finite rearrangement consumed by the child interface.

The acceptance target is a verifier that consumes those proof objects at each
retained depth and emits the same parametric summary with
`generic_premises_remain_assumed=false`. Even that target would not verify the
external Theorem D.1 or imply a practical Curve25519 attack.
`[pcc:unverified]`
