# Exact exceptional-branch action and chart-basis correction

The [native producer](src/exceptional_ideal.rs) and a separate
[native verifier](src/validate_exceptional_ideal.rs) checked the five
recorded rational two-torsion actions on each of the 21 frozen Rosenhain
models. For every one of 105 model/root pairs, all six univariate
cross-products on the `U=u(r)=0` branch are zero polynomials over the exact
recorded prime field. The same checker reconstructs the `O -> T_r` and
`T_r -> O` vectors and verifies all six projective cross-products for
each direction. Both distinct chart matrices have nonzero determinants
on every recorded model. The [raw rows](results/exceptional-ideal.json),
[standardized CSV](results/exceptional-ideal.csv), [graph](exceptional-ideal-graph.svg),
and [PDF](EXCEPTIONAL_IDEAL.pdf) preserve these exact checks.

## Corrected chart matrices

The earlier [chart note](CHART_NONVANISHING.md) said that the degree-two
direct chart and degree-one candidate chart use the same conversion
matrix. That statement is **retracted**. Let `c=(mu,nu*lambda,nu,mu*lambda)`,
`d2=(lambda+nu,1+mu,lambda+mu,1+nu)`, and
`d1=(1+mu,lambda+nu,1+nu,lambda+mu)`, with theta scaling `s_i`.
The degree-two row is `C2[i]=s_i*(c_i*d2_i,-c_i,d2_i,-1)`.
The source degree-one `conversion_matrix` row is
`C1[i]=s_i*(c_i*d2_i,-c_i,d1_i,-1)`. Exact finite-field determinants
are nonzero for both on all 21 models and unequal for every model.
The earlier N16 `general_conversion_determinant` was for `C1` only.
That determinant also proves degree-two nonvanishing after the exact basis
change `C2(1,-a1,-a0,q3)=C1(1,-a1,a0,q3+sigma*a0)`, where
`sigma=1+lambda+mu+nu=-f4`. In particular, the existing N15 `general()`
inverts the correct alternate degree-two vector. The corrected `C2`
determinant gives the direct check: its input vector starts with 1. The `C1` degree-one input vector
`(0,1,x,x²)` has second coordinate 1, so it is nonzero for every x.

The [retained wrong-basis diagnostic](results/exceptional-ideal-wrong-basis.json)
runs the initial, erroneous `C1=C2` assumption **without transforming the
degree-one input vector**. All 105 model/root
pairs have six nonzero exceptional cross-products, and 84 of 105 pairs
have mismatching `O`/torsion checks. This is a failed diagnostic, not a
counterexample to the corrected action. The original 27,470 finite
comparisons used the source `C1`; their results are unchanged.

## Reduced-form branch proof and scope

For a valid reduced degree-two Mumford class with `U=u(r)=0`, the curve
equation implies `v(r)²=f(r)=0`, hence `v(r)=0`. Factor
`u=(x-r)(x-s)`. The branch root is simple on every recorded Rosenhain
curve (`f'(r)!=0`). If `s=r`, then `v²` has a double zero at `r`, while
`f` has a simple zero, contradicting `u | (v²-f)`. Therefore `s!=r`.
Write `v=b1(x-r)`; validity at `s` gives
`(s-r)²*b1²=f(s)`. Translating the divisor
`(r,0)+(s,y)-2∞` by `T_r=(r,0)-∞` leaves `(s,y)-∞`, because
`2T_r=div(x-r)` as a degree-zero class.

The producer substitutes `a0=r*s`, `a1=-(r+s)`, and the squared-slope
identity into the degree-two direct chart. It clears `(s-r)²`, applies
the recorded action matrix, and compares it with the **source**
degree-one chart `C1*(0,1,s,s²)`. All six resulting polynomials in `s`
are identically zero in all 105 rows. The independent checker reconstructs
`f`, `C1`, `C2`, action matrices, chart polynomials, branch-root
derivatives, both determinants, and all cross-products from the frozen
inputs. The determinant proof makes both charts nonzero everywhere in
their stated domains.

The reverse action from a degree-one class at `s!=r` follows by applying
the same recorded matrix again: `M_r²=alpha_r I` with `alpha_r!=0`
by the earlier exact matrix certificate. Since the two chart vectors
and `M_r` are nonzero/invertible, the projective equality reverses.
For `s=r`, the class is `T_r` and its translate is `O`; both directions
have exact six-cross-product checks in every row. Together with the
[generic `U!=0` certificate](GENERIC_IDEAL.md), these are action
identities for the **declared charts on every valid reduced divisor
class** of the 21 recorded models.

The remaining distinction is geometric: a source-independent proof that
these declared formulas are the intended Kummer embedding is not in this
packet. Nor has an all-state signed, relation-label-preserving collector
using the action been run. Its cold cost and matched-rho ratio remain
`null`; the exact chart law alone does not provide such a ratio.

## Reproduction

From the repository root, with source artifacts hydrated:

```sh
N16_WRONG_BASIS=1 CARGO_TARGET_DIR=/dev/shm/n16-ideal-target TMPDIR=/dev/shm \
  cargo run --release --offline --locked \
  --manifest-path research/kummer_differential_collector_20261022/n16-action-certificate/Cargo.toml \
  --bin exceptional-ideal
CARGO_TARGET_DIR=/dev/shm/n16-ideal-target TMPDIR=/dev/shm \
  cargo run --release --offline --locked \
  --manifest-path research/kummer_differential_collector_20261022/n16-action-certificate/Cargo.toml \
  --bin exceptional-ideal
CARGO_TARGET_DIR=/dev/shm/n16-ideal-target TMPDIR=/dev/shm \
  cargo run --release --offline --locked \
  --manifest-path research/kummer_differential_collector_20261022/n16-action-certificate/Cargo.toml \
  --bin validate-exceptional-ideal
```

The `SHA256SUMS` file pins both raw outputs and the presentation packet.
