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

## Live 1,260-target collector

The [live collector](src/bin/collector.rs) uses the same fixed 21 exact-order
source models, 60 targets per model, algorithm seed and trial cap as the frozen
projective and negation-rho arms: 1,260 targets at 16–40 source-order bits.
It constructs the destination model cold for each target, charges the
normalization to `y² = x⁵ + f₃x³ + f₂x² + f₁x + f₀`, transfers both source
divisors, and uses the wrapped external law in jump initialization, scalar
multiplication and every live walk step. It retains the two-choice relation
policy and signed graph elimination. Both prospective additions and all valid
relations are processed. A fresh source-curve multiplication checks every
recovered scalar against the target and planted answer, and its arithmetic
is charged. All **1,260 recovered scalars verified**; failures and
exhaustions: **0**. The [paired analyzer](analyze.py) requires exact workload
hashes, seeds, caps, source-addition units and planted answers, and
independently reproduces the frozen rho and projective control totals.
It reads those controls directly from the committed
`research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`
archive with `tar --zstd`; no extracted session directory is required.

Complete cold one-target cost at `M + 0.5S + 10I` and **0.1 GAE per native
logical operation** is:

| Arm, 1,260 targets | GAE | Relative to rho |
| --- | ---: | ---: |
| Wrapped external collector | 2,666,291,893.905 | **2.448830×** |
| Frozen projective collector | 3,137,323,980.305 | 2.881445× |
| Frozen negation rho | 1,088,802,370.333 | 1× |

The external law improves the projective collector to **0.849862×** its
cost. It still loses to rho in **all 36 pooled pricing cells**: external/rho
spans **1.345253–13.126486×** across `S/M = 0, 0.5, 1`, `I/M = 1, 10,
100, 1000`, and native logical prices `0, 0.1, 1`. It beats projective in
all 36 cells (ratio **0.813129–0.970009×**). At 40 bits, the representative
ratio is **2.464040× rho** and **0.859296× projective**. At 16 bits, 9 of
36 pricing cells beat rho (all with native price 1 and `I/M ≥ 10`, by at
most 2.46%), but its representative ratio is **1.177905×**;
the other six size slices have no rho-winning cells. None of the 21 curves
beats rho at the representative price.

The external walk processed **219,581,773** candidate divisors and made
**220,368,293** wrapped additions, including **481,051** equal-input
dispatches. At representative pricing its field arithmetic contributes
1,242,998,567.405 GAE, field additions 1,090,500,768.800 GAE, and other
native logical work 332,792,557.700 GAE. The per-target split is in the
[raw records](results/); the complete 36-cell and per-size/per-curve prices
are in [price-grid.csv](results/price-grid.csv) and
[summary.json](results/summary.json). Field additions, subtraction's two
additions, equality dispatches, graph work, transfer, and source verification
are included in the declared pricing. These are **operation-count prices**;
the shared host's wall time is not used to claim speed.

Holding this run's field-addition and other logical counts fixed, making
every charged multiplication, square and inversion free still leaves
**1,423,293,326.500 GAE, or 1.307210× rho** at the representative logical
price. This is a floor for the measured counts, not for a redesigned solver
that changes addition or relation counts. A total-cost reduction of 59.16%
would be needed to tie rho at this price.

The verdict is a real collector improvement over the existing projective
backend, but **no end-to-end rho advantage** at this scale or within the
declared pricing band. The published complete inversion-free law is still a
different, untested implementation path.

Reproduce from the repository root:

```sh
cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin genus2-external-gate > /tmp/genus2-external-receipt.json
cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin cohort > /tmp/genus2-external-cohort.json
cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin cost > /tmp/genus2-external-cost.json
cmp /tmp/genus2-external-receipt.json research/genus2_external_gate_20261021/receipt.json
cmp /tmp/genus2-external-cohort.json research/genus2_external_gate_20261021/cohort.json
cmp /tmp/genus2-external-cost.json research/genus2_external_gate_20261021/cost.json
for bits in 16 20 24 28 32 36 40; do
  cargo run --release --locked --manifest-path research/genus2_external_gate_20261021/Cargo.toml --bin collector -- "$bits" > "/tmp/genus2-external-replay-${bits}.jsonl"
  cmp "/tmp/genus2-external-replay-${bits}.jsonl" "research/genus2_external_gate_20261021/results/order-genus-${bits}.jsonl"
done
python3 research/genus2_external_gate_20261021/analyze.py research/genus2_external_gate_20261021/results
```

Raw per-model counts and zero-mismatch attestations are in
[cost.json](cost.json). All seven live record files were independently
replayed byte for byte; [replay.json](results/replay.json) freezes their
digests and 1,260/1,260 replay result. The analyzer reconstructs the
reported prices from the retained records and frozen controls.
