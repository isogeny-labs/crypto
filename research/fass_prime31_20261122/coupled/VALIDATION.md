# Coupled partial-norm validation status

**Whole-repository validation remains open.** The focused native checks and separate-role replays passed at their stated scope. Both bounded whole-library attempts ended before tests executed; the cached attempt additionally produced an explicit allocation failure while compiling `aws-sdk-s3`. No whole-library pass is claimed. All execution records here are unsigned development evidence, not signed PCC standing. [pcc:unverified]

## Commands and outcomes

Paths in commands below are relative to the stated working directory.

| Gate | Working directory and exact command | Outcome | Evidence and scope |
| --- | --- | --- | --- |
| Focused rank qualifier | `coupled/native`: `timeout 60s target/immutable/qualifier-attempt2` | Exit 0; 40 rank cells, 2,690 scalar-product checks, 270 Sylvester checks, 269 cusp coefficient columns, 20 cusp determinant checks, 40 leading-check invocations, and the explicit nonsingular rank exception | [Producer raw output](evidence/attempt2/qualifier.stdout.jsonl); [separate-role replay](evidence/independent/receipt.json). Native module correctness, not relation yield or solver cost |
| Focused native unit tests | `coupled/native`: `timeout 60s target/immutable/tests-attempt2 --test-threads=1` | Exit 0; 9 passed, 0 failed, 0 ignored | [Replay log](evidence/independent/tests.stdout.log); three new qualifier tests plus six unchanged imported-module tests. Not the whole repository's test suite |
| Fixed-d6 identity evaluation | `coupled/native/certificate`: `timeout 60s target/immutable/certificate-attempt1 check ../../evidence/certificate-attempt1/cusp-d6.identity.json 17665607814493179865` | Exit 0; 32 certificate points plus 32 fresh checker-seeded points; `proof:null` | [Certificate review](evidence/independent/CERTIFICATE_REVIEW.md), [receipt](evidence/independent/certificate.receipt.json). Fixed d=6 expression over 2^61-1 only; no Lean or signed execution proof |
| Rank qualifier formatting | Repository root: `cargo fmt --manifest-path research/fass_prime31_20261122/coupled/native/Cargo.toml -- --check` | Parent observed exit 0 and empty stdout at approximately 2026-11-23 02:00 UTC; no source mutation | Parent tool observation supplied to this reviewer; no separately archived raw stdout. Formatting only |
| Certificate driver formatting | Repository root: `cargo fmt --manifest-path research/fass_prime31_20261122/coupled/native/certificate/Cargo.toml -- --check` | Parent observed exit 0 and empty stdout at approximately 2026-11-23 02:00 UTC; no source mutation | Parent tool observation supplied to this reviewer; no separately archived raw stdout. Formatting only |
| Whole-library attempt 1 | Repository root: `cargo test --release --lib --locked --offline`, wrapped in `timeout 60s`, with `ulimit -v 2097152` and `CARGO_BUILD_JOBS=2` | Exit 124 while compiling dependencies; no test executed | [Pre-call manifest](evidence/integration/manifest.pre-call.txt), [stderr](evidence/integration/root-tests.stderr.log), [exit status](evidence/integration/root-tests.exit-status) |
| Whole-library cached attempt 2 | Repository root: `timeout 300s cargo test --release --lib --locked --offline`, with `ulimit -v 2097152`, `CARGO_BUILD_JOBS=2`, `RUST_TEST_THREADS=2` | Exit 124 at the fixed deadline after an `aws-sdk-s3` allocation error; no test executed | [Pre-call manifest](evidence/integration/attempt2/manifest.pre-call.txt), [stderr](evidence/integration/attempt2/root-tests.stderr.log), [exit status](evidence/integration/attempt2/root-tests.exit-status), [receipt](evidence/integration/attempt2/receipt.json) |
| Lean/Lake availability | Repository root: `command -v lean`; `command -v lake` | Neither executable found on PATH; no Lean compilation or axiom audit attempted | [Availability record](evidence/integration/tool-availability.txt) |

The focused replay invocations also used `ulimit -v 2097152`. This is a 2 GiB **per-process virtual-address-space ceiling**, not measured RSS or an aggregate job-memory measurement. The limits are validation bounds, not benchmark conditions.

## Concrete integration blocker

The one authorized cached retry ran from 2026-11-23 01:56:05 UTC to 02:01:05 UTC. It progressed through published local path dependencies and then logged:

```text
error: failed to create encoded metadata from file: Cannot allocate memory (os error 12)
error: could not compile `aws-sdk-s3` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

The timeout wrapper subsequently returned 124 while Cargo was waiting for remaining work. Its stdout is empty and there is no root unit-test execution or result. This is an observed allocation failure under the imposed resource limits, followed by the fixed deadline; it is not evidence of a defect in the new algebraic component, nor a passing integration gate. No further build, resource-limit increase, source repair, benchmark, EKS invocation, or GitHub action was performed by this reviewer. No missing sparse-include diagnostic was encountered, so no additional source materialization was needed.

The cached attempt used Cargo 1.100.0 and rustc 1.100.0 on x86_64-unknown-linux-gnu. Root `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/cryptanalysis/mod.rs`, and the imported `semaev_prime_poly.rs` were hashed before and after; those bindings remained unchanged. The pre-call record names HEAD `ae5de9b066e936671a2da66667c50fdbd7727273` and its tree. No tracked source edits were made for this validation.

The remaining gate is a completed unchanged-source, locked whole-library build and test run in an execution envelope that can finish it. The current receipts do not supply that outcome. The parent FASS lane continues to track its unfinished circuit, extraction, and recovery work; these scoped component artifacts do not close those obligations.

## Evidence interpretation

The replayed project-curve rank observations cover exactly d=1,2,3,6,7,9,14,18,21,31,42,63. They do not establish fixed-curve rank at d=1302. The singular control is an algebraic specialization, not a cryptographic curve fixture. Generic separation-rank reasoning remains a written argument pending formal proof.

Coefficient rank is not independent relation rank; neither the rank qualifier nor the fixed-d6 evaluation performs signed relation recovery or a complete DLP solve. The separate-role checks use the same host and producing team, with frozen binaries and fresh source/hash checks rather than an independent rebuild. They remain unsigned. See [source review](evidence/independent/REVIEW.md) for the precise claims and exclusions. [pcc:unverified]
