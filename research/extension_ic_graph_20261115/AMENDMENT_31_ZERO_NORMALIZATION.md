# Amendment 31: measure zero normalization on an immutable matrix

Trial36 replays the exact candidate35raw input, SHA256
`11ac30e4e22202b12dd79fe9d30085b15979b89a47809e7e876f9017a875cfe3`.
This is a recorded affine-matrix component experiment, not a new DLP sample
or a repricing/composition of35. Both modes receive the same rows, modulus,
column count and target column; the recorded answer is used only afterward
as a reference. [pcc:unverified]

The source already skips zero pivot coefficients during forward updates
and uses compact nonzero support during backward updates. Preserve those
optimizations. The candidate differs only by skipping zero coefficients
and zero RHS values in pivot normalization. Preserve Shoup/u128 reduction
routes, the Euclidean inverse, column choice, row-update order and the
target stopping rule. Require the baseline's7897311products/rank387 and
the same rows used/scalar; original raw data remain untouched.
[pcc:unverified]

The copied Rust qualifier independently reduces every inserted prefix
using a column-keyed basis and straightforward u128 modular operations,
without the benchmark's Shoup helper or compact support updates. Require
both full pivot-state traces to match that oracle. Count zero normalization
entries independently before each oracle normalization; the product and
normalization-product savings must equal that count. Additions, inversions,
factor constructions and update-scan profiles must remain equal. A separate
batch solve checks the full recorded affine system. [pcc:unverified]

Charge one word event for every candidate zero test. Other native loop,
container, quotient setup and Euclidean costs remain incomplete. Baseline
zero observation and both per-insert state hashes are diagnostic harness
work, outside arithmetic subtotals and inside process wall. No wall speed,
complete cost, band, ratio, pipeline improvement or rho-parity claim.
[pcc:unverified]

Use the standalone Rust analysis crate before another library rewrite.
Native controls cover hand counts, Shoup/wide reduction, every prefix of
known-solution sparse streams in small/wide fields, inconsistent inputs,
forged counts/state hashes and false completeness. Freeze source/binaries
and protocol before one immutable Runpod dispatch. One paired replay,
zero warmups/retries,60s worker/180s whole/900s runner,CPU4vCPU8GiB at most
0.40USD/hour, first allowed CPU and pinned official Rust image. The native
capture lifetime now accepts900..7200s with its existing2700s default.
Collect/hash-check originals before terminating the owned runner.
[pcc:unverified]

Sources checked(KB): recent exact/negative RREF queries unavailable as
recorded in the journal. Read actual `residual_walk::RelationSystem`,
`FixedFactor`, `inv_mod`, counted adapter and old recorded-matrix trial30.
This is an existing arithmetic optimization candidate, without a novelty
or general-curve claim. [pcc:unverified]
