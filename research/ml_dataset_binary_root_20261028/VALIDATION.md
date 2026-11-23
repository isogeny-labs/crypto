# Binary root producer validation

Recorded 2026-10-28. This supersedes the pending checks in the original handoff
without replacing its historical evidence. Runtime source is
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`, based on policy runtime
`d71ec2ef5ab9b7f2a038f61c941af10297a775ad`. The later merge of policy delivery
head `8736ed3eeaf66b86a9833d318eb824cb10273210` changed documentation and
evidence only: an exact Git comparison of `src`, Cargo manifests/lockfile,
`build.rs`, tests, examples and benches was empty.

## Implemented behavior

The binary/Koblitz root adapter at m=2 now uses bounded native quadratic and
Artin--Schreier operations. It retains selected field operations, odd-degree
half-trace states, even-degree matrix reductions, root reports and finite-domain
lifts through the live artifact sink. Its certificate binds the full canonical
public instance, factor base and CaseConfig, explicit field/modulus, fixed-x
query ordering and reconstructed S3 coefficients. Work, event and byte caps
apply across calls. A sink failure preserves only acknowledged output and does
not become a successful completed call.

Separate portable arithmetic checks the distinct root set using
`gcd(f, X^(2^n)+X)`. This is distinct-root completeness for a supplied polynomial,
not multiplicity, whole-search completeness, an UNSAT certificate, scalar
recovery or a first/last-fall-degree result. Selected field/matrix/lift charges
are mixed admission units; execution-completeness and total-operation-count
claims remain false. Trace and certificate payloads remain restricted labels
or provenance, outside static training features.

## Native checks

Rust 1.98.0 release checks on the recorded source:

| Check | Result |
| --- | --- |
| Full library | 5,183 passed, 0 failed, 123 ignored; exit 0 |
| Binary root module | 9 passed |
| Dataset module | 158 passed, 0 failed, 1 existing ignored endpoint sweep |
| CLI integration | 8 passed |
| Strict Clippy: library, crypto binary and CLI integration | Passed with `-D warnings` |
| Scoped formatting and whitespace | Passed |

The full library invocation used the immutable copied test executable with
`RAYON_NUM_THREADS=16` and `--test-threads=1`. The log reports 1191.22 seconds;
this is validation elapsed time, not a benchmark. The executable SHA-256 is
`aee1ba0b4916b6f55af6511eb3957a0f1b2c36b75d7d07336e367b7445956830`.
The source-bound CLI SHA-256 is
`c01e024156067799356f590a0c3ac749da509305bd569e26c9bf084d5402d04a`.

Native tests include constructed roots at all requested degrees 23..31 and
47..61, exhaustive small fields, degenerate polynomials, inverse accounting,
event/work/sink boundaries, and certificate tampering. Adapter tests cover both
families, twists, odd/even fields, aggregate caps, exact object hashes, outcome
binding and durable prefixes. These are software checks; they do not establish
scientific corpus coverage at all those parameters.

## Independent consumer

The separate native consumer at runtime `25e863fe6075d7bac795ee62680b72c123ba0f33`
and tested source `a39594756923822f3705b4e5a916bb9321dacba6` reports 95 passing
standalone tests and strict Clippy. It audited 51 retained software fixture
cells, including 30 completed polynomial root sets. Both binary families,
odd/even degree, both twists and prime legacy/v2 records are covered. Rehashed
root omission and input substitution are rejected. Five setup-error rows and
two unsuccessful coverage checks remain in the packet. This does not turn all
51 cells into completed solver results.

Consumer archive:
`s3://setec-astronomy-artifacts/research/ml-dataset/validation/20261028/consumer-root-certificates/40d88d8d0bdfbe57ab8c201970d4ced5c49792c86eea552067e44726921c4809/validation.tar.gz`
(12,750,951 bytes). The implementation owner verified conditional PUT, ETag-bound
fresh GET, SHA-256, byte equality, all 1,738 contained hashes and the restored
51-cell/30-root-set audit. A separate source reviewer found no actionable issue;
that review is explicitly uncompiled and separate from runtime validation.
Prime v1 lacks full-object hashes; its mathematical binding does not certify
uncommitted annotations. Intermediate event identity and full costs remain
outside the independent consumer's proved scope.

## Durable evidence

- Final gate: `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/binary-root-final-gate/8d1bac39e3909edd281b7feea3f5a17a6f4a7ac181b90d87dc04f2ac4f82d238/validation.tar.gz`
  (114,652 bytes). Conditional PUT, fresh ETag-bound GET, byte comparison and all
  contained hashes passed at 15:30:42 UTC. Includes the full log, source/binary
  digests, independent source review and the proposed next trace design.
- Earlier source/binaries: `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/binary-root/432c4e3141166bb7ac7df4b6713c979e630b2281727c58528a4878dd92d4dd1b/validation.tar.gz`
  (49,959,713 bytes), also freshly downloaded and verified. Its historical
  full-suite-pending status remains unchanged.
- Source checkpoint: `s3://setec-astronomy-artifacts/checkpoints/crypto/codex__ml-binary-root-trace-20261028/20261028T144615Z-d79f658f7a0a/checkpoint.tar.gz`.
  Bundle verification and all eight scoped restored source hashes passed.
  Broad historical fsck encountered pre-existing partial-clone missing blobs;
  the failure log is retained and full-history connectivity is not claimed.

The JSON receipts beside this file record exact archive and source identities.
No dataset campaign worker or E12 retry was launched by this increment. The
original USD100–200 total allowance, maximum USD200, is unchanged. Actual EKS
worker S3 authorization and a durable cumulative quota authority remain pending;
these software archives do not establish cloud-produced dataset cells.

## Remaining delivery and scope

The producer PR is stacked on policy PR2777. Current-head hosted checks and
bottom-up main integration remain required. Persistence PR2769 was externally
merged into its temporary review-base branch; it is not a main delivery and
requires a later integration PR preserving its source lineage.

Higher-m root elimination, complete instruction traces, other requested native
solver intermediates and the broad scientific corpus remain unfinished. The
next approved software increment is bounded large-prime row-operation capture
with an independent source-combination checker. Its design was preserved before
implementation; no experimental result follows from that plan.
