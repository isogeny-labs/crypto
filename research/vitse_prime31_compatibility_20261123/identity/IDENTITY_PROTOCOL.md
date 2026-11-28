# Frozen protocol: S3 coordinate-negation identity certificate

Date: 2026-11-23. Parent: crypto PR #5016, `lane/vitse-prime31-compatibility`.
Baseline: `7d61e3b45de6e93d235345a0061dae08db1f8b44`.
This protocol is written before the build and native certificate calls. Its
SHA-256 is frozen in `../evidence/identity-01/source-build.json` before building.
No compatibility qualification is repeated by this additional identity layer.

## Claim and limits

The statement is the identity in `MATHEMATICAL_REVIEW.md`, section 3:

`S3(-X,-Y,t) - S3(X,Y,t) = 4*(X+Y)*(t*(X*Y+a)+2*b)`.

Use exactly the normalization of `pkm_smooth_tower.rs::s3`:

`S3(X,Y,t) = (X-Y)^2*t^2 - 2*((X+Y)*(X*Y+a)+2*b)*t
             + (X*Y-a)^2 - 4*b*(X+Y)`.

The ordered variables are `X,Y,t,a,b`; all five are variables. The syntactic
degree bound is **4**, not 3. The expression tree constructs the two S3
substitutions separately; it does not replace the left side with its claimed
factorization. The certificate is a same-team, deterministic finite-evaluation
record over the production certificate field `2^61-1`. It is not a formal proof
over the integers or over the Prime31 target field `2147483647`, and it does
not establish a symmetry of any selector, curve, relation set, or solver chart.
There is no formal proof pointer, independent-checker claim, performance
measurement, or scalar-recovery claim. The production API's reported numerical
false-accept expression is retained as an API output, not promoted to a
rigorous failure-probability claim for these fixed deterministic seeds.

## Sources and bindings

Compile the entire unchanged production `src/cryptanalysis/identity_certificate.rs`
(Git blob `114f1f4a3f2a1be027a9b046ef166259df49dd45`) and the entire unchanged
`src/hash/sha256.rs` (Git blob `9566e3de448e21ff98811cfc4c6992ff3448962e`).
Rust module bindings provide `crate::hash::sha256::sha256` without modifying
either source. The exact baseline S3 item and a pre-certificate mathematical
review snapshot are saved as source witnesses. The review snapshot SHA-256 is
`30bbfda953e0ec1cecf044a96f849ef823f6e8831f963b4b79b717b884d5c159`.

Use a standalone Cargo workspace. Pin `serde=1.0.229`, `serde_json=1.0.151`,
and `hex=0.4.3`, with their transitive package records copied unchanged from
the baseline Cargo.lock. Copy only this dependency closure's cached archives,
source directories and sparse-index metadata from the parent's cache into a
new task-specific CARGO_HOME. Verify each archive against the locked registry
checksum and every archived source file against the copied source. No Cargo
source replacement, path patch, alternate arithmetic, or network fetch is
permitted. Record all ancestor Cargo configuration files and refuse source
overrides or compiler-wrapper configuration. Build with `--frozen --offline
--locked`, explicit compiler paths, and a task-local target directory.

The accepted prior review is the parent mathematical review and exact source
normalization. KB was unavailable in the parent source audit; this subtask adds
the repository's existing certificate machinery and makes no novelty claim.

## Calls and expected controls

1. Build separate `issue_identity` and `check_identity` executables using the
   supplied Rust 1.100.0 toolchain. Total build timeout: 180 seconds. No test
   harness or compatibility executable is run.
2. The issuer validates syntactic degree 4, issues with seed **20261123** and
   **32** points, and writes the certificate once. A negative RHS-sign
   statement must return `IssueError::False`; a deliberately declared degree
   3 must return `Malformed::DegreeBound` before evaluation. Keep the actual
   wrong-sign counterexample in raw output.
3. A separate checker process reads the written certificate, checks its
   expected statement and protocol fields, replays its 32 points and checks
   **32 extra points** using seed **20261124**. It must accept. The wrong RHS
   sign, with recomputed statement ID so that validation reaches evaluation,
   must return `Verdict::Reject`. Mutating one hex digit of only the valid
   certificate's value digest must return `Verdict::DigestMismatch`.
4. Each native call has a 60-second timeout and its own pre-call and post-call
   JSON manifests, raw stdout and raw stderr. Preserve failures; do not repeat
   an unchanged call. Exit zero only when all declared controls match.

## Artifact integrity and interpretation

The metadata-only Python runner records immutable source/build manifests,
exact production Git blob IDs, all source SHA-256 digests, complete dependency
archive/source/index pins, Cargo.lock, compiler and Cargo executable pins and
versions. Frozen executables are separate from Cargo's mutable build outputs.
Their hashes and all source pins must match before and after each call.
Certificate bytes must be unchanged across the checker call.

All evidence goes to the new `../evidence/identity-01/` directory; every record,
raw output and certificate is opened with exclusive creation. The certificate
is `identity.certificate/v1` at `identity.certificate.json` beside this protocol.
The source/build record, two call records and certificate jointly support
only the bounded finite checks stated above. This layer does not change the
status of the mathematical derivation to kernel-verified or independently
replayed. The parent will cite the certificate beside the reviewed identity.
