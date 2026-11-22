# Experimental multiplication components in Rust

This change adds bounded executable components of the September 23, 2026
*Integer multiplication below n log n* construction to the existing
`isogeny_algos` crate. The feature is `experimental-subnlogn`; it has no external
dependencies and does not change the crate's default arithmetic.

The implemented integer multiplier is an **exact synthetic-transform reference**.
The complete published asymptotic algorithm is not implemented. Its requested
mode returns `Error::FullAlgorithmUnavailable` even on zero inputs. There is no
measured speed claim or claimed sub-`n log n` running time for this code.

## Component map

| Component | Executable behavior | Scope still missing |
| --- | --- | --- |
| `src/subnlogn/motif.rs`: `BitMotif` | The eight-row XOR schedule for one coordinate invocation; three invocations exchange banks and restore arbitrary auxiliary state | Full tensor-coordinate network, frame changes, rank-saving recursion and tape scheduling |
| `motif::PublishedAudit` | Checked `u128` evaluation of the manuscript's `h=100` bit and complex network counts, rank deficits and sufficient exponent inequality | Independent proof of the rank formulas, amplification and total-work bounds |
| `src/subnlogn/synthetic.rs`: `convolve` | Exact one-axis synthetic-root convolution over a valid odd-characteristic `Field` | Simultaneous tensor layers and the manuscript's complex approximation/error analysis |
| `subnlogn::multiply_exact` | Exact unsigned multiplication from little-endian `u64` limbs through bounded byte convolution and carry reconstruction | Full asymptotic integer assembly |
| `subnlogn::ExperimentalField` | Opt-in use through the existing `Field::conv_long` hook | Measured crossover selection and any claim of a production performance gain |
| Certified resampling | Not implemented | Gaussian resampling with explicit precision/error propagation |

These are source-level scope descriptions and native correctness observations,
not signed execution or formal-proof claims. [pcc:unverified]

## Run the checks

From the repository root:

```sh
cargo test --release --locked --manifest-path isogeny_algos/Cargo.toml --lib
cargo test --release --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --lib --test subnlogn --example subnlogn
cargo test --release --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --test polyfast --test powmod --test ntt
cargo clippy --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --lib --test subnlogn --example subnlogn \
  -- -D warnings
```

The example is a deterministic correctness demonstration; it performs no timing
measurement. Its test executes the same assertions as its command-line entry:

```sh
cargo run --release --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --example subnlogn
```

It prints JSON identifying the reference scope, actual convolution dispatch,
integer reconstruction, scalar motif checks and symbolic published counts.
Large counts are strings so JSON consumers do not round them to doubles.
Adding `-- --full` requests the unavailable complete algorithm and exits with an
error. The existing `isogeny-algos.yml` workflow now includes focused
feature-enabled lint and correctness steps on its existing triggers.

## Use the existing polynomial interface

```rust
use isogeny_algos::field::Zp;
use isogeny_algos::poly;
use isogeny_algos::subnlogn::{Config, ExperimentalField};

let field = ExperimentalField::new(Zp::new((1 << 31) - 1), Config::default());
let product = poly::mul_raw(&field, &vec![7; 33], &vec![11; 35]);
assert_eq!(product.len(), 67);
assert_eq!(field.stats().synthetic, 1);
```

The wrapper forwards the inner field's arithmetic, characteristic and field-size
semantics, thresholds, prepared-operand representation, cyclic and truncated
products, squaring, powering, square roots, batch inversion and dot operations.
Existing polynomial dispatch chooses whether to call `conv_long`; small products
can bypass it. `Config::default()` admits at least 32 coefficients in the shorter
operand and requests at most 128 transform points. The existing polynomial
threshold is also respected.

`conv_long` is opportunistic: below its minimum length, or after **any** synthetic
error, it delegates to the original field's `conv_long`, including that method's
possible `None` result. Its counters record attempts, successful synthetic calls,
and fallback calls. A failed attempt cannot increment the successful counter.
Clones share counters. Concurrent snapshots are diagnostic, not atomic combined
records; an empty successful strict call does no transform work.

For candidate-only work and full error diagnostics, use
`field.try_convolve(&a, &b)` or `synthetic::convolve(&field, &a, &b, max_points)`.
These calls never substitute another multiplication method. The strict wrapper
does not apply the minimum-length policy.

## Exact arithmetic and bounds

The synthetic reference embeds scalar coefficients as constant records in
`F[Y]/(Y^r + 1)`, with power-of-two transform length `t` and
`r = max(t/2, 2)`. It uses normalized `H0` butterflies, signed monomial
twiddles and bit-reversed frequency storage. Both transforms are normalized;
the pointwise ring product is divided by `r`. The opposite transform therefore
returns the desired convolution divided by `r*t^2`, which the final exact field
scaling restores. All divisions are by powers of two. The supplied `Field` must
satisfy its usual algebraic contract. [pcc:unverified]

Admission computes `one + one` in the field and checks its inverse. It does not
infer the characteristic from the potentially truncated `char()` accessor. Thus
the reference can work in an odd-characteristic field without long transform
roots in that base field. Characteristic two is rejected, including for empty
inputs; the optional field wrapper can still use its original fallback.

The record products are ordinary quadratic negacyclic products. The explicit
operation bound is `O(t*r^2 + t*r*log(t))`, and owned element storage is
`O(t*r)`. Since this reference chooses `r` proportional to `t`, that operation
upper bound is cubic in `t`. It is a small correctness reference for the paper's
algebra; substituting it for a production NTT does not inherit the paper's
asymptotic bound. [pcc:unverified]

Independent allocation limits are checked before the large vectors are created:

| Limit | Value and meaning |
| --- | --- |
| Transform points | Caller limit, default wrapper limit 128 |
| Owned element slots | `2*t*r + r + output_len <= 262144`; the second transform vector is released before the output is allocated |
| Owned vector payload bytes | At most 64 MiB, additionally enforced for large field-element types |
| Effective transform ceiling | At most 256 points under the slot cap, even if the caller requests more |
| Raw integer input | At most 256 `u64` limbs **per operand**, checked before trimming; the transform limits still apply and may reject much smaller operands |
| Executable scalar motif | `3 <= h <= 16`, with a maximum 1,048,576 counted 64-bit-equivalent payload words across indices, two banks and three scratch states |

The storage counts exclude caller-owned inputs and allocator metadata. Motif
shape checks and the published symbolic audit allocate no full tensor network.
All three exchange scratch-buffer lengths are checked before changing either
bank or any scratch state.

The integer wrapper converts the canonical input values into byte coefficients
over `p = 2^61 - 1`. Every coefficient is bounded by
`min(byte_lengths)*255^2`; the wrapper checks that this is below `p` before
recovering coefficients and carrying. Leading zero limbs are accepted within the
raw input cap. Output is canonical little-endian `u64` limbs, with zero represented
by an empty vector. [pcc:unverified]

This is public-data research code, with data-dependent work. No constant-time
contract is offered.

## Correctness evidence and provenance

The native checks compare against independent word multiplication and schoolbook
convolution; they exercise maximum-value carry chains, unequal lengths, zero and
padded inputs, endianness, dispatch and fallback, real P-224 characteristic
semantics, characteristic-two rejection and preserved optimized hooks. Internal
checks cover direct DFT identities, inverse scaling, signed wraparound, motif
basis vectors, arbitrary auxiliary restoration and allocation limits. The
example also requires actual candidate dispatch. [pcc:unverified]

`correctness.json` records commands, observed outcomes, source hashes and the
bounded evidence scope. Its PCC record is an unsigned attestation of native test
observations; neither local tests nor an unsigned record establish the complete
published theorem. No performance experiment or leaderboard update is part of
this change.

The native checks used local implementation commit
`a1a2a0b4e51554415caad43f7ba544d0de53bd7a`. Its tree
`b1ce7a122e04e7db88fdf1c644d8cb19f9e7d996` is identical to
[published implementation `83d9df79d1833d61adec8c41addaf30dacb753ec`](https://github.com/isogeny-labs/crypto/commit/83d9df79d1833d61adec8c41addaf30dacb753ec).
The commit identifiers differ because publication used the connected GitHub
integration. Native-observation record
`PCC-20261122-5140c7` pins the receipt; its standing is `unverified` and its
unsigned attestation check is `not_covered`. [pcc:unverified]

Primary source is the manuscript at `openai/math` commit
`4eaee87548250b2682affda7d05d947c05554bae`:

- [Section 3: scalar motifs and published count formulas](https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/03-motifs.tex), blob `cdb0ba527aa8df5d57c151183fd3893c1a7e21ab`.
- [Section 6: synthetic transforms](https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/06-transforms.tex), blob `de073e9292d58573f7065a3cd4f214e0d05f3444`.

The arithmetic implementation was written from these mathematical schedules.
The independent [Klakocik/below-nlogn](https://github.com/Klakocik/below-nlogn)
project was reviewed as a scope comparison; its source was not copied. The
knowledge-base MCP and repository wrapper were unavailable in this runtime;
targeted repository searches and primary-source reads supplied the fallback.

## Next implementation obligations

The next technical step is an executable rank-labelled tensor schedule with
explicit source/target frames, rank accounting and stream/tape movement costs.
That schedule must be checked independently before its arithmetic deficit is
used as a complexity claim. Certified resampling with interval/error budgets
and the complete integer assembly follow. The present bounded APIs and known
answers are test scaffolding for those steps, and their limits must not be
removed to imply that the enormous published construction is practical.
