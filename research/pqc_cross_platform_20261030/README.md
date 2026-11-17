# ML-KEM and ML-DSA: portability, performance and timing review

Source baseline: `fc9af16fa1ce0654ee9df3ce63101c487ed215fb`.
Publication base: `b7f830121417997d8863189f435bcb66f6466ca2`. The tested PQC,
hash and RNG sources are unchanged between these bases; candidate source
hashes match after the refresh.
This is a source assessment and a first portable hardening change, not a
completed side-channel audit or a new performance measurement.

## Requested scope

Assess and improve ML-KEM and ML-DSA performance and side-channel resistance
across platforms. Retain that scope; a native x86 test or a cross-compilation
check does not discharge hardware validation on another platform.

| Requirement | Status | Evidence / remaining obligation |
| --- | --- | --- |
| Locate reference and optimized implementations | Verified | `src/pqc/{ml_kem,ml_dsa}.rs`, `src/pqc/fast/{ml_kem,ml_dsa}.rs` |
| Establish current platform paths | Source review | ML-KEM runtime AVX2 arithmetic and AVX2/AVX-512 Keccak; scalar fallbacks elsewhere. ML-DSA-65 uses scalar arithmetic and streaming Keccak. |
| Correctness and cross-compilation | Partial | 88 focused native tests pass; five cross-target test compilations pass. Full-library tests are blocked by an unchanged syntax error. Other hardware was not executed. |
| Remove reference ML-KEM encoder's coefficient-bit branch | Implemented and functionally checked | FIPS 203 bit layout, all 12 widths, existing ACVP and fast/reference tests pass. Compiled timing behavior remains unverified. |
| Speed improvements | Not established | Requires frozen baseline/candidate, profiling, matched external reference and admitted measurements. Existing figures below are historical, not this revision. |
| End-to-end timing/cache resistance | Not established | Secret-flow audit, target assembly inspection, dynamic leakage tests and compiler/CPU-specific evidence remain. |
| Power/EM and fault resistance | Not attempted | Requires a defined device and physical evaluation; source-level branch removal is insufficient. |

## Literature and prior work checked before selecting a change

- [FIPS 203](https://doi.org/10.6028/NIST.FIPS.203), section 4.2.1,
  Algorithms 5–6 and K-PKE.Decrypt: ByteEncode assigns every bit and is used
  on the recovered message as well as secret-key polynomials. Writing zero
  bits unconditionally preserves that specified mapping.
- [FIPS 204](https://doi.org/10.6028/NIST.FIPS.204), sections 3.6.1–3.6.3
  and ML-DSA.Sign_internal: preserve the rejection algorithm and distinguish
  its aggregate rejection decision from extra coefficient/position-dependent
  execution. Hedging and destroying sensitive intermediate data are separate
  requirements; neither is provided by a constant-time equality check.
- [`docs/pqc-speed.md`](../../docs/pqc-speed.md) and
  [`pqc_ml_kem_profile_20261005`](../pqc_ml_kem_profile_20261005/README.md):
  historical profiles identify packing, comparison and arithmetic costs.
  They do not establish the performance of the source baseline above.
- [`pqc_ml_kem_reference_20261005`](../pqc_ml_kem_reference_20261005/README.md)
  and [`pqc_ml_kem_system_level_20261005`](../pqc_ml_kem_system_level_20261005/README.md):
  compare the same key preparation contract; cached-key results are not
  interchangeable with stateless API results. The reference study did not
  independently check the external implementation's outputs.
- [pq-crystals/kyber `ref/poly.c`](https://github.com/pq-crystals/kyber/blob/main/ref/poly.c),
  `poly_tobytes`: unconditional stores and shifts provide an established
  encoding approach. No novelty or fastest-implementation claim is made.
- [`verification/pqc-assay/README.md`](../../verification/pqc-assay/README.md):
  existing native proofs cover selected ML-DSA scalar reductions under
  specified preconditions. They do not establish timing behavior or cover
  the ML-KEM encoder. The proof-bound source files are unchanged here.

## Findings and implementation boundaries

1. **Reference ML-KEM encoding.** `byte_encode` branches and conditionally
   writes on each coefficient bit. K-PKE decryption calls it on the recovered
   message, so this is secret-derived computation even when ciphertext bytes
   are public. The first change makes the OR/write unconditional, using the
   bit as data. This removes a source-level secret-dependent branch; it is
   not a demonstrated exploit or a compiler-independent timing proof.
2. **Optimized ML-KEM rejection.** `decaps_tail` always derives the rejection
   key, re-encrypts, compares with `subtle::ConstantTimeEq`, and selects with
   `ConditionallySelectable`. Keep this invariant. Its comparison cost is an
   optimization candidate, subject to preserving constant-time semantics.
   Matrix rejection sampling uses public `rho`; its variable loop count alone
   is not evidence of a secret leak. Audit secret-derived compression and
   arithmetic in the emitted code for each target/compiler.
3. **ML-DSA norm checks.** Optimized `chknorm` returns at the first failing
   coefficient; the signing loop also stops at the first failing polynomial.
   Centering and low-bit checks contain further data-dependent source control
   flow. Audit these within the complete rejection procedure. A full scan can
   increase rejected-candidate cost; no speed or security result is inferred
   from rewriting the helper in isolation.
4. **Secret lifetime.** `MlDsaSecretKey::drop` writes ordinary zeroes, which
   does not ensure the optimizer retains the wipe. `MlKemDecapsKey` is a
   `Vec` wrapper without a zeroizing destructor; prepared ML-KEM decapsulation
   keys already have explicit zeroization. Review intermediate buffers too.
   Prefer the existing `zeroize` dependency for retained wipes, and validate
   compatibility with the proof source bindings when changing ML-DSA.
5. **API/coverage.** ML-KEM implements 512/768/1024. ML-DSA currently implements
   65, and its signing tests explicitly use the FIPS internal interface with
   raw messages. Do not describe this as a completed external FIPS interface
   with context framing, or as support for all ML-DSA parameter sets.
   `std`, heap allocation and OS RNG dependencies also preclude inferring
   general bare-metal or browser support from portable arithmetic.

## Validation and execution status

Native development tests are permitted by AGENTS section 9a. No experiment
has been launched. Raw logs and commands are in [evidence/README.md](evidence/README.md).

| Check | Baseline | Candidate | Scope |
| --- | --- | --- | --- |
| Native Linux x86-64, Rust 1.99.1 | 86 pass, 0 fail | 88 pass, 0 fail | Full production PQC modules, their real dependencies, ACVP and differential tests; two new encoding regressions |
| ARM64 Linux test compilation | Pass | Pass | `aarch64-unknown-linux-gnu`; not runtime execution |
| ARM64 macOS test compilation | Pass | Pass | `aarch64-apple-darwin`; not runtime execution |
| Windows x86-64 test compilation | Pass | Pass | `x86_64-pc-windows-gnu`; not runtime execution |
| RISC-V 64 test compilation | Pass | Pass | `riscv64gc-unknown-linux-gnu`; not runtime execution |
| 32-bit x86 Linux test compilation | Pass | Pass | `i686-unknown-linux-gnu`; not runtime execution |
| Focused Rust formatting / authored-file whitespace | — | Pass | `cargo fmt` for the focused package; whitespace check excludes verbatim raw logs/diff, whose original blank/context whitespace is preserved |
| Existing PQC-Assay source bindings | — | All five match | Its proof-bound production files are unchanged; no new proof execution is claimed |
| Root `cargo test --release --lib` | Compile failure, exit 101 | Compile failure, exit 101 | Original base: unchanged `ecbench/methods.rs:8824`. Refreshed base: unchanged `binary_semaev_chain_sat.rs:1703`. Each failure was reproduced with its original encoder restored. |

The native host is an Intel Xeon Platinum 8370C Linux x86-64 executor. These
are correctness/build checks, not isolated performance measurements. No
speedup, timing-leakage pass, EKS receipt or all-platform runtime result is
claimed. See the [open validation problem](../../problems/pqc-cross-platform-validation.md).

The selected executor initially lacked Rust and Conductor. Rust 1.99.1 was
installed. `conductor check` returned 127; `docs/CONDUCTOR.md` documents that
missing binaries do not enforce reservations. No shared reservation is claimed.
The root locked check refused an already inconsistent `Cargo.lock`. Running
without `--locked` normalizes three dependency entries, then reaches the
unchanged syntax error. The first full-library attempt started before the
encoder edit and compiled the root after that edit, so it is recorded as the
candidate attempt. A second run with the exact original encoder restored
reproduces the same failure. The candidate file was then restored byte for
byte, and the root lockfile was restored to its original bytes. The generated
lock diff is evidence only; no root dependency change is part of this patch.
After refreshing to the publication base, the first parser error instead
names `src/cryptanalysis/binary_semaev_chain_sat.rs:1703`; separate candidate
and original-encoder runs both reproduce that error. These unrelated files
were not edited to get a passing check.

The documented GitHub Iso Lab build route currently builds the `ecbench`
executable for `ecbench.spec/v1`. No PQC benchmark spec/image or authenticated
custom-command EKS client has been established in this session. Do not dispatch
the default elliptic-curve validation fixture as an ML-KEM/ML-DSA benchmark.

## Next validation obligations

1. Run native ACVP/differential tests on each target, including baseline x86
   without vector extensions and each available vector backend. Cross-check
   test compilation for ARM64 Linux/macOS, Windows, RISC-V and 32-bit x86.
2. Define and qualify a native PQC benchmark image/submission route, then
   freeze workloads, source identities, A/A and interleaved A/B protocols,
   external-reference revisions and output equality before optimization.
3. Inspect compiler output for secret-dependent branches, memory addresses
   and operand-dependent instructions. Run target-native timing/cache tests
   with positive controls, including chosen ciphertexts and signing behavior.
4. Land separately validated secret-lifetime and ML-DSA control-flow fixes;
   retain signing bytes and rejection semantics. Repeat affected proof bindings.
5. Add architecture-specific kernels only after profiling that platform, with
   runtime dispatch and portable fallbacks. Report gains and regressions per
   operation, parameter set, compiler and actual hardware.
