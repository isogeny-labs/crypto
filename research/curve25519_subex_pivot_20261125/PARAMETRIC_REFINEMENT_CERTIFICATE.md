# Parametric one-step refinement certificate

The schema-2 parametric-refinement certificate checks the algebraic implication
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
and the next call rechecks the guards or takes exact fallback.
```

The verifier derives `nD<=6M` instead of accepting it as a premise: packet
admission gives `KL<=3M`, while the admitted power-of-two coordinate satisfies
`2^(i+1)D=2L`, hence `nD=2KL<=6M`. `[pcc:unverified]`

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

The threshold transcript is 10,202 bytes and reports
`conditional_parametric_implication`. Six focused tests cover the accepted
five-depth composition, missing/reordered schedule data, source/shrink-rule
constants, tuple/cost-ledger constants, missing premises/conclusions, and every
unsupported promotion. Its summary reports
`mode_dimension_bound_derived=true`; the wire object no longer contains a
caller-supplied mode-bound Boolean. The checker imports the
composition-boundary verifier, not graph, packing, factorization, contraction,
or producer code.
`[pcc:unverified]`

## Lean kernel replay

`lean/ParametricRefinement.lean` gives a second, typed proof of the downstream
real-inequality chain. It kernel-checks the single-class cap
`[pcc:unverified]` from the scaled variance and source guard, padded capacity,
both child-shrink bounds, degree closure,
the exact power-of-two identity `2^(i+1)*2^(a-i)=2*2^a`, the resulting
`nD<=6M` bound from packet admission `K*2^a<=3M`, the `6*34=204`
mode/precision product after cancelling positive `D`, and the exact
cost-ledger constants. The proof makes `M>=0` explicit in both shrink lemmas;
Lean rejected the first draft without it, exposing a genuine missing domain
condition in that standalone formulation.

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
nonincreasing.

`lean/LabelSequence.lean` composes those steps over the complete native row
order followed by the complete column order. `lean/ObjectiveBoundary.lean`
then identifies common-endpoint ordered pairs with squared degree fibers,
bounds the initially unknown objective, identifies the fully labeled objective
with the `q^2` encoded class collision sum, and combines the degree cap and
source guard into `c_gamma D<=2M` for every class. The statements admit
zero-degree isolates and distinct parallel edges directly. `[pcc:unverified]`

`lean/PrecisionAbsorption.lean` imports the repository's existing exact
`childPrecision` model and finite-history theorem without changing their
sources. It derives `8^d<h` from the root bound, eightfold shrink history and
active leaf cutoff; derives the square bound and positivity from the exact
leaf definition `h=floor(sqrt(lambda))`; derives `u>=32,h<=512u` from that
depth bound, the exact source quotient and threshold; kernel-checks both precision
absorptions; and proves the composed
`p_next*u<=34*M*2^(A+2u-i)` conclusion. The imported source files retain
SHA-256 values `92e4300c...951989a` and `0a235579...cd4b4`; their declarations
and the new bridge are audited together. This theorem still requires a native
path witness for the exact precision recurrence and shrink history, root and
active-cutoff facts, exact source quotient, and exact square-root definition.
`[pcc:unverified]`

The project pins Lean `v4.35.0` and mathlib commit
`56a005855ea17e0b0958f73ec801ae140b0a1679`. Reproduce it with:

```text
cd research/curve25519_subex_pivot_20261125/lean
lake update
lake build
lake env lean AxiomAudit.lean
lake env lean PrecisionAxiomAudit.lean
```

The build completes 9,059 jobs. `#print axioms` reports only `propext`,
`Classical.choice`, and `Quot.sound` for each exported theorem; there are no
project axioms or admitted proofs. There are 83 explicitly audited declarations,
including four reused precision declarations, fifteen new precision/schedule
bridges, the natural-number mode/precision cancellation theorem, and ten
generic finite mode-to-group factorization identities plus ten nilpotent,
geometric-remainder, tile-shift, and exact Cauchy-series lemmas, plus ten
extension-ring decoding, reduction, nilpotence, and Cauchy-instantiation lemmas.
The frozen pre-module precision-history source emits six Lean
module-compatibility warnings when imported into this project; its bytes are
unchanged.

The aggregate downstream theorem still consumes generic inequalities as
hypotheses, but the separate natural-number chain derives the mode inequality
from packet capacity, derives the normalized precision inequality from explicit
history and schedule facts, and composes them through positive power-of-two
cancellation to `n*p*u<=204*M^2`. The replay does not yet bind arbitrary native
endpoint/label traces or recursion histories to those theorem inputs, or
formalize the external contraction theorem. `[pcc:unverified]`

Separately, the native `CauchySourceWitness` now exposes and independently
reconstructs every stored scalar factor valuation/inverse, the base and
numerator unit, all packet weights, and emitted nonzero mode coefficients.
Together with accepted packing-provenance packet pairs it reconstructs the
complete mode list and every source zero-suppression in exact loop order. This
supplies replayable native evidence for the scalar and mode code-to-formula
boundary; instantiating the Lean theorem and binding complete child objects
remain separate obligations. `[pcc:unverified]`

## Next measurable goal

The remaining local gap is now premise evidence, not downstream coefficient
arithmetic.
A useful next certificate must replace the generic Boolean tokens with typed
evidence for two independent claims:

1. a typed code-to-model binding, or replayable proof object, showing that the
   native endpoint arrays and first-minimum traces instantiate the formal
   graph theorem at each active call; and
2. a replayable native path witness instantiating the checked precision theorem:
   initial precision, exact `childPrecision` recurrence, eightfold shrink,
   active leaf cutoff, exact source quotient, and the exact root square-root
   definition. The leaf positivity and square inequality, depth bound `8^d<h`,
   schedule facts `u>=32,h<=512u`,
   companion `nD<=6M` inequality, scalar `pu<=34MD` implication, and combined
   `npu<=204M^2` product are now kernel-derived, so none remains an independent
   algebraic premise.

The acceptance target is a verifier that consumes those proof objects at each
retained depth and emits the same parametric summary with
`generic_premises_remain_assumed=false`. Even that target would not verify the
external Theorem D.1 or imply a practical Curve25519 attack.
`[pcc:unverified]`
