# Experimental sub-nlogn construction in Rust

The `experimental-subnlogn` feature now contains an executable reconstruction of
the September 23, 2026 *Integer multiplication below n log n* construction:
rank-labelled recursive layers and address interchanges, certified scalar and
Gaussian arithmetic, synthetic transforms, CRT transport, and whole-integer
assembly. It has no external crate dependencies. The existing field wrapper
continues to expose its separate bounded finite-field reference. [pcc:unverified]

**The native implementation with the claimed sub-`n log n` bound remains
unfinished.** The mathematical stages are present, but the segmented filesystem
interpreter performs host metadata work beyond the represented fixed-tape
recurrence. Its logical movement receipts do not establish that runtime bound.
The complete published `h=100` network has not been executed end to end.
[CONSTRUCTION.md](CONSTRUCTION.md) maps the source to the implementation, derives
the amended arithmetic costs, and identifies the remaining obligation.
[pcc:unverified]

The paper's stated target is

\[
O\!\left(n(\lg n)^{1-\kappa}\right),\qquad \kappa=2^{-182},
\]

on its fixed-alphabet, fixed-tape model. This is a literature statement, not a
native benchmark or a verified theorem about this Rust program. The source is
pinned below. No production crossover, speedup, or constant-time contract is
claimed. [pcc:unverified]

## Public entry points and actual routes

```rust
use isogeny_algos::subnlogn::{multiply_published, PublishedExecution};

let product = multiply_published(&[u64::MAX; 2], &[u64::MAX; 2])?;
assert_eq!(product.limbs, [1, 0, u64::MAX - 1, u64::MAX]);
match product.execution {
    PublishedExecution::BaseCase { reason, prerequisite } => {
        // These describe the actual conventional prerequisite work.
        println!("{reason:?}; {prerequisite:?}");
    }
    PublishedExecution::Transform { parameters, execution, precision } => {
        // These describe the admitted construction, completed stages,
        // layer/swap receipts, and finite coefficient-error bound.
        println!("{parameters:?}; {execution:?}; {precision:?}");
    }
}
# Ok::<(), isogeny_algos::subnlogn::Error>(())
```

Inputs and outputs are canonicalized little-endian `u64` limbs; zero is an empty
output vector. `multiply_exact(a, b, Algorithm::PublishedSubNlogN, max_points)`
uses the same selector and returns a concise `IntegerRoute`. Its `max_points`
argument applies only to `Algorithm::SyntheticReference`.

The literal prescription retains `epsilon=2^-75` and `c=2^-56`. Every input
length representable by the native `usize` interface derives dimension `d=1`
and takes an explicitly reported base case. The concrete Fermat-ring multiplier
handles that base case and supplies the scalar/ring arithmetic prerequisite.
An error in an admitted transform is returned; it does not trigger a substitute
multiplication path. [pcc:unverified]

For a finite diagnostic that actually executes the complete numerical pipeline,
use `assembly::Plan` with explicit parameters:

```rust
use isogeny_algos::bigint::Big;
use isogeny_algos::subnlogn::assembly::{Parameters, Plan};

let mut plan = Plan::new(Parameters {
    radix_bits: 8,
    precision: 128,
    source_lengths: vec![5, 7],
    target_lengths: vec![8, 16],
    alpha: 4,
    chunk_bits: 1,
})?;
let product = plan.multiply(&Big::from_u128(u128::MAX),
                            &Big::from_u128(u128::MAX))?;
assert_eq!(product.value.0, [1, 0, u64::MAX - 1, u64::MAX]);
# Ok::<(), isogeny_algos::subnlogn::assembly::Error>(())
```

These declared diagnostic parameters do not satisfy the literal asymptotic
prescription. They exercise CRT, Gaussian expansion/compression, chirp
convolutions, retained-layout transforms, pointwise products, both source-volume
scales, coefficient rounding and carries. Their layer and swap receipts expose
which recursive branches ran; a call to a layer API is not evidence that its
astronomical rank-saving branch executed. [pcc:unverified]

## Implementation map

Paths below are relative to `isogeny_algos/src/subnlogn/`.

| Source | Responsibility |
| --- | --- |
| `parameters.rs` | Exact literal powers, target geometry, distinct prime selection, eventual-condition admission |
| `assembly.rs` | Whole numerical flow, explicit disk/error margins, both source-volume factors, integer recovery |
| `certified.rs` | Signed dyadics, directed intervals, Machin binary splitting, bit-burst exponentials and phases |
| `resampling.rs` | Gaussian expansion, truncated correction series, compression and finite line-error bounds |
| `crt.rs` | Padded CRT layout, triangular rotations and inverse transport, including widened signed payloads |
| `transforms.rs` | Named-slot layout, forward/opposite transforms, exact monomial rotations, simultaneous rounds |
| `tensor.rs` | Published residual compiler, binary frame programs, recursive complex layers, exceptional repair |
| `swap.rs` | Rational rank factorization, admissible-prime compiler, odd-radix padding and recursive interchanges |
| `tape.rs` | Segmented sequential interpreter, role reuse, active-parent parking, separate movement/metadata counters |
| `ring.rs` | Signed Kronecker packing, four exact integer products and normalized negacyclic recovery |
| `fastint.rs`, `fastdiv.rs` | Concrete Schönhage–Strassen-style Fermat recursion and exact Newton division prerequisites |
| `synthetic.rs`, `motif.rs` | Earlier bounded finite-field and scalar-motif references, plus symbolic published counts |

This is a source-level reconstruction map. It is not a certificate for the
unexecuted full fixed network. [pcc:unverified]

## Run the checks and demonstrations

From the repository root:

```sh
cargo test --release --locked --manifest-path isogeny_algos/Cargo.toml --lib
cargo test --release --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --lib --test subnlogn \
  --example subnlogn --example subnlogn_full_flow
cargo test --release --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --test polyfast --test powmod --test ntt
cargo clippy --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --lib --test subnlogn \
  --example subnlogn --example subnlogn_full_flow -- -D warnings
```

The examples perform deterministic correctness checks, with no timing or
performance measurement:

```sh
# Earlier bounded references and field-wrapper dispatch.
cargo run --release --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --example subnlogn

# Literal prescription: reports its native base-case route explicitly.
cargo run --release --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --example subnlogn -- --full

# Entire numerical flow at the explicit diagnostic parameters above.
cargo run --release --locked --manifest-path isogeny_algos/Cargo.toml \
  --features experimental-subnlogn --example subnlogn_full_flow
```

The existing CI feature job includes both examples. This workflow runs on its
existing main-push/manual triggers, not as a new required PR check.

## Existing polynomial wrapper

`ExperimentalField<F>` remains an opt-in adapter for the bounded
`SyntheticReference` route through `Field::conv_long`. It forwards the existing
field's other arithmetic and prepared-product semantics. Its default admission
requires at least 32 coefficients in the shorter operand and at most 128
transform points. A rejected synthetic attempt delegates to the inner field and
increments a separate fallback counter. `try_convolve` is strict and returns
errors. This wrapper does not silently substitute the new complex construction.

The synthetic integer reference retains its raw 256-limb-per-operand cap and
independent transform/storage limits. Its byte-convolution reconstruction uses
the prime `2^61-1`. Those bounded-reference limits are not limits or asymptotic
claims for `multiply_published`. [pcc:unverified]

## Evidence and open completion obligation

Native correctness observations must be read at their recorded scope. The
historical `correctness.json`, its test logs and PCC record
`PCC-20261122-5140c7` describe only the earlier references at their pinned source
commit. They are preserved unchanged and do not attest to the new construction.
Their standing is unverified, with an unsigned attestation rather than a signed
execution verdict. [pcc:unverified]

The new [construction-correctness.json](construction-correctness.json) pins all
compiled crate sources, commands, logs and demonstration output. Native producer
commit `db270b54ac9cf05bcd9e39e024e855ff65808229` and
[published implementation `ab0a3fee4ad85036897593b4a790597e5dddf3a7`](https://github.com/isogeny-labs/crypto/commit/ab0a3fee4ad85036897593b4a790597e5dddf3a7)
have the identical tree `91ca547dda751394e959b7ae7f3fe4ddcb6613f8`.
The native-observation record is `PCC-20261122-9e6381`; the separate source/cost
interpretation is `PCC-20261122-48c64c`. Both have unverified standing and
unsigned `not_covered` attestations. [pcc:unverified]

The recorded Rust 1.100.0 checks observed the following outcomes. These are
producer observations, not independently signed execution results.
[pcc:unverified]

| Check | Recorded outcome |
| --- | --- |
| Default-feature library | 53 passed, one pre-existing ignored test |
| Feature-enabled library | 117 passed, the same pre-existing ignored test |
| Feature integration | 10 passed |
| Both example test harnesses | Three passed |
| Existing NTT/polynomial/powmod regressions | 10 passed |
| Feature clippy with warnings denied | Exit zero |
| Touched-source rustfmt check | Exit zero |
| Literal dispatch and full numerical-flow command-line examples | Exit zero; declared known answers matched |

The full-flow JSON reports three source transforms, both source-volume factors,
45 Gaussian expansions, 63 Gaussian compressions, 24 normalized polynomial
products and 27 layer calls. It also reports **zero recursive layer nodes and
zero recursive transform-swap nodes**. Thus the recorded diagnostic checks the
whole numerical composition at its declared parameters while using the small
branches inside those layer/swap calls. [pcc:unverified]

Native known-answer checks, finite error admissions and independent source
review do not prove the full fixed table or the asymptotic theorem. Read the
receipt for exactly which source, commands and branches were checked. No claim
is made that the workflow's separate Rust 1.98 environment ran these local
commands. [pcc:unverified]

The remaining native-bound task is to replace or rigorously lower the tape
interpreter so that repeated cursor serialization and segment addressing cannot
add a growing per-bit factor, then account for that implementation together with
scalar arithmetic, data motion and setup. Full-table verification is a separate
large finite obligation. The task remains open and its PR remains draft while
these obligations are unresolved. [pcc:unverified]

## Primary sources

- [Pinned September 23 manuscript](https://github.com/openai/math/tree/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026), `openai/math` commit `4eaee87548250b2682affda7d05d947c05554bae`; all sections 0–11 were read for this reconstruction. Exact section mappings are in [CONSTRUCTION.md](CONSTRUCTION.md).
- [Brent and Zimmermann, Modern Computer Arithmetic](https://members.loria.fr/PZimmermann/mca/mca-cup-0.5.9.pdf), §2.3.3, Algorithm 2.4, for the independent Fermat-ring prerequisite.
- Harvey and van der Hoeven, *Integer multiplication in time O(n log n)* (2021), especially the analytic and Gaussian lemmas imported by the September manuscript. Their imported routines are replaced here by concrete directed-integer algorithms and explicitly accounted for in the construction note.

The shared knowledge-base MCP and repository wrapper were unavailable in this
runtime. Pinned primary sources and the existing repository evidence provided
the fallback. No external implementation source was copied.
