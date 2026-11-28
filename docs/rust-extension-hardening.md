# Extending the Rust library and tracking its security guarantees

Import the library as `crypto_lib` (the Cargo package is named `crypto`).
Existing free functions remain available. This change extends the existing
stage traits rather than requiring a new application framework.

For a Git dependency, add the following to `Cargo.toml`, replacing the revision
placeholder with the reviewed commit:

```toml
[dependencies]
crypto_lib = { package = "crypto", git = "https://github.com/aburan28/crypto", rev = "<reviewed-commit>" }
```

The default crate still includes the existing research/database/TLS dependency
graph; a small `no_std` or primitives-only feature set is not provided here.
The existing [`crypto-analysis-core`](../crates/crypto-analysis-core/Cargo.toml)
crate already exposes avalanche and statistical analysis with a much smaller
dependency surface. It reuses the main repository's implementations directly;
consumers needing only those utilities should reuse it rather than importing
the root crate. This change preserves that crate and extends the root's IC,
entropy and cipher-provider interfaces.

## Caller-owned index-calculus stages

[`run_pipeline_with_stages`](../src/cryptanalysis/ic_framework/mod.rs) accepts
[`PipelineStages`](../src/cryptanalysis/ic_framework/stages.rs): a borrowed
`FactorBaseBuilder<G>`, mutable `DecompositionOracle<G>`, and
`RelationSolverFactory`. The matrix factory constructs a fresh matrix after
the factor base determines its column count, and supplies its completion and
base-column verification policy. No matrix name needs to be added to a match
in this crate. Its returned solver implements
`cryptanalysis::ic_boundary::RelationSolver`, the trait used by the collector.
It is also exported as `stages::RelationMatrix`. The older metadata-bearing
`stages::RelationSolver` is a separate interface.

Use `expected: None` for a public target whose logarithm is unknown: the runner
verifies the recovered value against the actual group equation. `Some(k)` adds
a known-answer check. Existing `run_pipeline` and
`run_pipeline_with_progress` preserve their known-answer semantics. Live
observers are optional, borrow actual witnesses and report the same final
verification status as the returned report.

[`PluginRegistry<'a, T>`](../src/cryptanalysis/ic_framework/registry.rs) is an
optional typed factory registry. Each application controls its registrations,
lifetimes, discovery and naming. Factories may capture borrowed curve data;
they construct fresh values instead of sharing mutable oracle state across
runs. `create_named` accepts `Params` directly; `create` accepts
`name:key=value,...`. Duplicate registrations and duplicate parameter keys
return errors instead of silently replacing an implementation or setting.
Replacement is explicit through `remove` followed by `register`.

The generic output type permits separate registries for factor-base builders,
oracles, matrix factories and `SystemSolver` implementations. Direct trait
injection needs no registry. Nothing is installed process-wide, and no executable
or shared-library loader is introduced. These are trusted, in-process Rust
plug-ins; their arithmetic, relation validity and accounting remain their
implementer's responsibility. The runner rejects inconsistent base indexes
and invalid basic subgroup/cofactor bounds before collecting relations.

The existing ecbench `BinaryPlugins` startup hook remains compatible and
continues serving its binary-linked adapters and frozen replays. The new
registry is optional application-local composition, and does not replace that
hook or automatically add entries to the CLI's built-in `--list` output.

[`examples/ic_custom_pipeline.rs`](../examples/ic_custom_pipeline.rs) is a
complete downstream-style consumer. It registers a builder that borrows its
curve, configures an oracle, injects a matrix borrowing an application counter,
requests full-rank completion and checks every base column. Run it with:

```sh
cargo +1.98.0 run --locked --release --example ic_custom_pipeline
cargo +1.98.0 test --locked --release --example ic_custom_pipeline
```

The built-in target policies remain the `Targets` enum. Arbitrary target-stream
injection and feature separation of the large research dependency graph are
not implemented by this change. Applications can still use the lower-level
collector APIs. This runner retains its existing word-sized subgroup parameters;
the wide Koblitz collectors keep their separate APIs. IC routines, operation
counters and live traces are variable-time
research tools for public fixtures; they are not secret-key backends.

## Entropy and diagnostics without application policy

`utils::random` exposes fallible `try_random_bytes`, `try_random_scalar` and
their `*_with_rng` variants. Caller-owned sources implement the standard
`rand` 0.8's `RngCore + CryptoRng` traits; source failures propagate with no
fallback, and failed byte buffers are zeroized. Invalid scalar bounds consume
no entropy. Uniform scalar generation uses rejection sampling rather than
modular reduction. `CryptoRng` is a provider contract, not an audit, and callers
are responsible for seeding their source securely. The existing convenience
functions keep their OS-backed defaults and panic behavior on entropy failure.

`EccKeyPair::try_generate_with_rng` uses the supplied source and existing
supported-curve secret-scalar backend. Scalar rejection and `BigUint`
representations are variable-time; this API does not turn key generation into
a constant-time operation. RSA and PQC generation still use their existing
OS-backed entry points.

Private-key `Debug` output is redacted for ECC, RSA, Kyber and McEliece, as is
`MontgomeryContext`. `KeyBundle::write_summary` writes public metadata to any
`std::io::Write` sink and propagates I/O errors. `summary` retains its stdout
convenience behavior and no longer exports the ECC private scalar. Explicit
private export APIs remain explicit. Public secret fields and explicit export
can still leak if callers log them. Owned Kyber and McEliece secret buffers
now use optimization-resistant `zeroize` calls on drop; BigUint-backed secret
keys still have the zeroization limitation described in `SECURITY.md`.

## Fixed-width arithmetic hardening

The reusable [`symmetric::gf128`](../src/symmetric/gf128.rs) module provides
`double_be`, `halve_be` and `double_le` under the standard
`x^128 + x^7 + x^2 + x + 1` polynomial. CMAC, SIV, OCB3, PMAC and XTS now share
these helpers instead of duplicating branches on key-derived carry bits.
Reduction uses `subtle` selection with fixed-width loops. The tests include
NIST SP 800-38B subkeys and every polynomial basis element in both endian
conventions. GHASH's reflected bit convention is deliberately distinguished.
PMAC's table bound also uses the message block count's native `usize` width,
removing the truncating `u32` cast and the newer compiler's bit-width lint.

`MontgomeryContext::new` previously computed `R mod n` and `R² mod n` with
variable-time `BigUint` division. RSA's CRT path rebuilds these contexts using
secret primes during private operations; this was a real secret-dependent
setup path, not an unavoidable hardware limitation. It now computes the
constants with exactly `128 * LIMBS` modular doublings, using only fixed-width
add/subtract and `subtle` selection. Valid moduli follow a common source-level
arithmetic schedule. Width and odd/nonzero validity are public preconditions;
invalid moduli return `None`, including zero-width contexts.

`Uint<N>` also implements the standard `subtle::ConstantTimeEq`,
`subtle::ConditionallySelectable` and `zeroize::Zeroize` traits. These allow
downstream wrappers to reuse the existing arithmetic. `Uint` is still `Copy`:
zeroizing one instance cannot erase previous copies, compiler temporaries,
registers or `BigUint` allocations. `Zeroizing<Uint<N>>` scrubs on drop but its
debug representation is not a secrecy boundary; use a redacted application
wrapper when formatting is possible.

This removes one source-level timing dependency. Tests of arithmetic equality
are not timing proofs. End-to-end resistance across all secret-key APIs, compiler
versions, CPUs and microarchitectural channels remains **not established**.
The crate remains educational and unaudited. The precise remaining obligation
is recorded in [the secret API timing coverage problem](../problems/rust-secret-api-timing-coverage.md).

## Runtime-selected block-cipher providers

The existing `symmetric::modes::cipher::BlockCipher<N>` and `BlockCipher128`
traits remain the provider contracts. Their generic modes now accept unsized
implementations. CMAC, PMAC, GCM, CTR, EAX, CCM, ECB, CFB, OFB and key wrap can
consume borrowed trait objects; GCM-SIV accepts a trait-object master and a
factory returning a boxed provider. This adds runtime composition without
requiring dynamic allocation for callers who use concrete cipher types.

```rust
use crypto_lib::symmetric::{aes::AesKey, cmac::cmac};
use crypto_lib::symmetric::modes::cipher::BlockCipher128;

// Public example fixture. Application providers may wrap a different backend.
let provider: Box<dyn BlockCipher128> = Box::new(AesKey::new(&[0; 16]).unwrap());
let tag = cmac(provider.as_ref(), b"message");
```

Forwarding implementations for `Box<T>` and `&T` let factories and borrowed
providers compose with those contracts. Downstream crates that previously
implemented this crate's trait specifically for `Box<LocalCipher>` or
`&LocalCipher`, or supplied an unsized `BlockCipher128` marker impl, must remove
that now-redundant implementation to
avoid overlapping impls. Providers control their own secret storage, error
handling and timing behavior; the trait alone does not certify their security.

## Requirement and evidence record

| Requested requirement | Status | Evidence / remaining scope |
| --- | --- | --- |
| Reuse existing modular work | Verified for this change | Existing stage contracts and built-in wrappers retained; existing lightweight analysis crate passes 10 tests. |
| Downstream plug-ins without editing this crate | Verified for the documented runner | Typed registries, borrowed factories, matrix injection and two consumer-example tests pass. Arbitrary target policies and wider subgroup parameters remain a gap. |
| Less opinionated application I/O and entropy | Verified for the added APIs | Entropy failure/rejection tests, CSPRNG compile rejection, public summary and I/O failure checks pass. RSA/PQC RNG injection is not added. |
| Runtime-selectable cryptographic providers | Verified for the added interfaces | Downstream NIST CMAC/GCM vectors and GCM-SIV boxed factory comparison pass; existing mode tests also pass in the library suite. |
| Side-channel hardening | Verified arithmetic and source changes; timing resistance partial | Montgomery cross-checks, GF basis/vectors, redaction and zeroization integration pass. No end-to-end timing proof is established. |
| Entire codebase is “ironclad” / end-to-end side-channel resistant | Partial; not established | Variable-time boundaries, legacy fallbacks, BigUint allocations, target-specific timing and independent audit remain open. |

The follow-up integrated release library suite passes **4,393 tests, with
0 failures and the existing 117 ignored**. The IC, progress, extension and
embedding tests pass **49/49**; four N83 example tests bring the selected
consumer suite to **53/53**. Full maintained-target strict Clippy passes on
Rust 1.98.0 and 1.99.1, all workspace targets type-check on 1.99.1, and pinned
formatting passes. Native catalog refreshes and a calibrated-cost regression
repair the earlier four library failures; the two lint sites are repaired
without exceptions. See the [follow-up results and raw logs](validation/rust-regression-repairs-20261017/README.md).
The [original results](validation/rust-extensions-20261017/README.md) remain
historical evidence. Shared research metadata and frozen-workflow failures
still block the [required final-head gates](../problems/current-main-research-regressions.md).

The PR remains open until the integrated required checks and review gates pass.
No algorithmic speedup, new cryptanalytic result or whole-crate security
guarantee is claimed by this change.
