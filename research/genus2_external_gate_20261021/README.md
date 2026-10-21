# Independent genus-2 arithmetic backend: correctness gate

This gate pins Sebastian Lindner's
[`divisor-arithmetic`](https://github.com/salindne/divisor-arithmetic)
to commit `87e2ebc1b59e5313b67bf81d11e42ec094251518`. Its
`g2::ramified::not_char2` API uses an optimized affine law with an inversion
and separate `add` and `double` functions. It is an independent backend to
compare with the repository's Cantor arithmetic; it is **not** an
implementation of the complete inversion-free Ozbay Gürler–Hisil law.

The first [receipt](receipt.json) records a decisive equal-input check on
`y²=x⁵+2x³+3x²+4x+5` over `F₃₁`. The valid point `(0,6)` is not 2-torsion,
but upstream `add(D,D)` returns the identity while upstream `double(D)`
returns a degree-two divisor. A second degree-two example also disagrees.
Thus the unwrapped `add` API cannot be used as a complete group operation
without an equal-input dispatch.

The [cohort gate](src/bin/cohort.rs) reads the **21 original exact-order
Scholten models** from the existing checked coordinate-normalization
receipt. On each model it builds a deterministic pool of the identity,
rational point divisors, their negatives and pair sums, then compares every
ordered pair with the repository's complete Cantor law. The wrapper calls
upstream `double` when its two inputs are equal and upstream `add`
otherwise. Results in [cohort.json](cohort.json):

| Check | Result |
| --- | ---: |
| Curves | 21 / 21 original exact-order models |
| Distinct pool divisors | 1,342 across curves |
| Ordered pair additions | 85,764 |
| Unwrapped equal-input wrong answers | 1,311 |
| Unwrapped equal-input panics | 1 |
| Wrapped mismatches vs complete Cantor | **0** |
| Wrapped panics | **0** |

This is a bounded correctness gate on deterministic pools, not a proof of
the law on every divisor. The observed upstream failure is recorded without
modifying upstream source.

## Same-model arithmetic gate

The independent [cost harness](src/bin/cost.rs) repeats all 85,764 ordered
pair additions on the same 21 normalized exact-order models. A counted field
wrapper records the external law's actual branch-dependent field operations,
including `double` for all 1,342 equal-input calls. Every output is compared
with complete native Cantor; mismatches: **0**. Both sides exclude pool
construction and count one field inversion as `I`; subtraction counts as two
field additions to match native `Field::sub`. Equality checks and dispatch are
logical operations, not field operations.

| Across 85,764 calls | M | S | I | field additions |
| --- | ---: | ---: | ---: | ---: |
| Wrapped external law | 1,296,196 | 192,984 | 79,324 | 2,716,897 |
| Complete native Cantor | 7,213,146 | 112,768 | 270,321 | 12,719,421 |

At `M + 0.5S + 10I`, the wrapped external law costs 2,185,928 versus
9,972,740 field-price units, a **0.219190×** stage ratio. Across all 12
arithmetic pricing cells (`S/M = 0, 0.5, 1`; `I/M = 1, 10, 100, 1000`),
its ratio is **0.183808–0.291065×**. These pools weight branch types
differently from a live collector; this is **not** an end-to-end cost or a
ratio against rho. The inversion-free published law remains a separate path.

The stage result warrants an end-to-end collector test. That test must move
the transferred divisors into the checked normalized model once, charge the
coordinate change, use the wrapped law for every walk and initialization
addition, retain the complete signed relation solver, and run all 1,260
original targets before claiming a rho comparison. The equality dispatch and
all exceptional branches must remain charged.

Reproduce from the repository root:

```sh
cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin genus2-external-gate > /tmp/genus2-external-receipt.json
cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin cohort > /tmp/genus2-external-cohort.json
cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin cost > /tmp/genus2-external-cost.json
cmp /tmp/genus2-external-receipt.json research/genus2_external_gate_20261021/receipt.json
cmp /tmp/genus2-external-cohort.json research/genus2_external_gate_20261021/cohort.json
cmp /tmp/genus2-external-cost.json research/genus2_external_gate_20261021/cost.json
```

Raw per-model counts and zero-mismatch attestations are in
[cost.json](cost.json). Wall time is not used for ranking on this shared host.
