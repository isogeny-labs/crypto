# Rust verification pieces

This dependency-free crate implements arithmetic on the selected curve
`y²=x³−3x+40` over `p=2^61−1`, plus verification and linear-algebra recovery
from supplied signed relations. It **does not find relations**.

`Point` is either infinity or an affine coordinate pair. Public fixed-curve
functions `is_on_curve`, `add`, `mul`, `sum_terms`, `selected_root_factor_base`,
and `recover_log` use the selected curve. The factor-base constructor returns
the 2,447 canonical point lifts of the 4,950th-root x-subgroup; relation signs
are supplied separately. Multiplication uses `u128` intermediates: every field product
is below `p²<2^122`, and every field sum is below `2p<2^62`. The selected
prime-order subgroup has order `R=329406144237565159` and cofactor 7.

`recover_log` checks that each supplied row sums to a point killed by the
cofactor, that the generator and target decompositions agree after cofactor
projection, and that the relation matrix has rank `B−1` modulo `R`. It then
uses the one-dimensional nullspace to compute a candidate log and verifies
`[d]G=Q` exactly. Factor-base points may have full-group cofactor components.
The rank routine is dense Gaussian elimination, suitable for the small fixture;
its cubic field-operation cost does not meet the research goal at `B≈2447`.

Tests cover the selected M61 arithmetic and factor-base count, a projected-only
M61 relation, and a fixed `F_8191` toy fixture with 20 points and 19 independent rows. The toy
fixture also checks projection-only acceptance and corrupt-row rejection.

Run from this directory:

```sh
cargo test --offline
cargo run --release --offline --example torsion_translate
```

`Curve` parameters are private; only the selected instance is exported. A toy
instance is constructed internally in tests. This avoids accepting arbitrary
composite or oversized moduli with Fermat-based inversion.
