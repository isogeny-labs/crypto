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
the law on every divisor or a performance comparison. In particular, the
wrapper's equality branch has not yet been included in the full 1,260-target
index-calculus collector, and no speed or rho ratio is claimed. The
published complete inversion-free law remains a separate implementation
path. The observed upstream failure is recorded without modifying upstream
source.

Reproduce from the repository root:

```sh
cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin genus2-external-gate > /tmp/genus2-external-receipt.json
cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin cohort > /tmp/genus2-external-cohort.json
cmp /tmp/genus2-external-receipt.json research/genus2_external_gate_20261021/receipt.json
cmp /tmp/genus2-external-cohort.json research/genus2_external_gate_20261021/cohort.json
```

The next discriminating experiment is a same-host operation-count and
isolated wall comparison of the wrapped external backend, the existing
projective chart, and complete affine Cantor on the 21 curves, followed by
the original 1,260-target cohort if it improves a stage. Every exceptional
input and wrapper dispatch must be charged.
