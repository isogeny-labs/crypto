# Selected PKM map primitive-price protocol

Frozen before launching a primitive-price series. This is an amendment path
for the native selected-tower counters introduced by source commit
`a1c7d2699bd01690416274300da4b1ace3fa92e6`; it does not alter the v4
cold cohort, binary, controls, targets, or append-only store. The six held-out
primes are exactly `2081,2113,2273,16417,16481,16673`, in manifest order
from `z2xz8_holdout/manifest.json` (SHA-256
`99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8`).
The selected-field replay archive pins release binary SHA-256
`2b06842604a3afc2f75931d25e5c6d472a6f8a822f7dd8478e7a5e6842ead025`
and six first-target maps. No native operation count is a time or complete
field-work charge by itself.

## Arithmetic classes and source identity

Calibrate separately, using byte-pinned source and executable receipts:

1. Selector direct multiplication: `prime_tower::mul`, a `u128` product and
   `u128` remainder. The existing `fp_mul_explicit.rs` series measures this
   arithmetic expression; apply its price only to this class after qualifying
   both directions on the held-out primes.
2. RHS, finite curve-membership checks, and counted square roots:
   `ic_boundary::mulmod`, which takes its reduced, `p < 2^32` word-product
   branch on this cohort. Preserve their native operation counts as distinct
   buckets even when a shared measured price is justified by identical source
   and operand preconditions.
3. Selector and Vélu inversion powers: `residual_walk::mul_mod` inside
   `pow_mod`, whose small-product branch uses word remainder here. Verify
   the source path and operand bounds before sharing a price with class 2.
4. Target Legendre/Jacobi checks: `ic_boundary::jacobi` uses variable-count
   shifts, tests, swaps, and integer remainders. Count calls and source-level
   loop/remainder work, but do not multiply calls by an invented constant.
5. Direct additions, subtractions, reductions, rejected auxiliary search,
   map and Vélu control flow, allocation, and memory traffic: retain as
   positive unpriced work until measured or subsumed by an independently
   timed whole-constructor comparison. In particular, do not price an entire
   Vélu evaluation as one multiplication.

The stand-alone calibration must copy exact arithmetic expressions from the
specified source, record their source-file SHA-256 hashes, Rust compiler and
flags, host architecture, and binary SHA-256, and include independent
reference arithmetic checks for each sample. No optimization may remove the
operation: use dependent reduced operands, a consumed checksum, and a
documented operation count. The compiler output or a disassembly spot check
must establish that the intended remainder path was executed. Record the
input distribution; a microbenchmark on different operands is diagnostic.

## Admission and samples

The existing development-price watcher remains the sole live calibration
series until it terminates. Start no held-out or new primitive series beside
it. On the same host as the cold panel, run six primes forward and then
reverse in separate quiet windows. Use the admission and censor rules of
`PROTOCOL_Z2XZ8_HOLDOUT_FP_PRICE_20261128.md`: reject a pre-call host load
above logical CPU count or competing compute process without launching a
child; reject a contaminated post-call observation; retain every attempt in
an append-only store; cap each child at 30 seconds and 2 GiB; at most 20
attempts per prime and direction. Each qualifying cell has seven fresh
process samples, child CPU, wall, peak RSS, raw stdout/stderr hashes, and
seven independent arithmetic checks. A forward/reverse pair qualifies only
when both medians agree within 10%; use their midpoint, retaining spread.
Do not pool hosts or silently replace an unpriced class with another price.

For Jacobi, record actual call operands or a lossless source-pinned replay of
the exact deterministic map search, including rejected draws and cosets.
Calibrate that workload as a whole or charge the measured time of every
source-equivalent call class with its observed loop/remainder distribution.
A call-count-only estimate is insufficient. Compare isolated constructor
timing with the sum of calibrated primitives as a consistency check; retain
any positive residual as uncharged work, not zero.

## Independent replay and cost gate

Replay must verify manifest identity, source/build/binary hashes, arithmetic
reference results, raw content hashes, sample counts, median rule, both
direction and quiet gates, all rejected/censored attempts, and exact match
of charged operation classes to the native map and oracle ledgers. Freeze a
new cold amendment and store only after the selected source is independently
replayed. The six-curve, two-rung, 16-target-per-curve cold panel still needs
complete process wall/CPU/RSS, failed relation trials, joins, rank,
elimination, scalar verification, and matched same-target rho. The primitive
prices are an accounting cross-check; a complete-cost comparison uses the
measured end-to-end cold processes and retains all censored arms.
