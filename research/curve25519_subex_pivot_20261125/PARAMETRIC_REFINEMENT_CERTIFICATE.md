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

## Next measurable goal

The remaining local gap is now premise evidence, not coefficient arithmetic.
A useful next certificate must replace the generic Boolean tokens with typed
evidence for two independent claims:

1. the conditional-label variance theorem for an arbitrary admitted
   bipartite multigraph, including isolates and parallel ports; and
2. the factorization/precision bounds `nD<=6M` and `pu<=34MD` for the exact
   finite rearrangement consumed by the child interface.

The acceptance target is a verifier that consumes those proof objects at each
retained depth and emits the same parametric summary with
`generic_premises_remain_assumed=false`. Even that target would not verify the
external Theorem D.1 or imply a practical Curve25519 attack.
`[pcc:unverified]`
