# Exact GitHub-runner execution envelope

**Protocol:** `EXP-NORM-BRANCH-CENSUS-GHA-20261118-v1`

**Parent mathematical protocol:** `EXP-NORM-BRANCH-CENSUS-20261113-v1`

**Parent protocol SHA-256:**
`8c3a7e5cbf0647e077c8fd5b1a1b8f19c82f31adab1e4b63eb3ffddd9ed8c0d6`

This is a separately preregistered execution envelope for the already-frozen
norm-branch census. It does not amend, complete, or substitute for the blocked
Iso Lab EKS v1 execution. The EKS result remains `INCOMPLETE_EXECUTION` because
that protocol forbids a backend change after freeze.

No held-out `k=12,14` artifact existed when this envelope was frozen. The
GitHub-hosted run is exact and non-timing; runner speed, host load, and wall
time are not scientific endpoints.

## Requested scope and evidence contract

| Requirement | Frozen evidence |
| --- | --- |
| Test the same mathematical family | The producer receives the byte-verified parent protocol digest, not this runner digest. |
| Preserve the same instances and controls | Parent `protocol.json`, canonical Rust IR, seeds, fields, ladder, split search, arms, mutation controls, and checker are in `SOURCE_CLOSURE.sha256`. |
| Preserve the decision rule | Only `ADVANCE_B2`, `KILL_MEASURED_FAMILY`, `INCONCLUSIVE`, or `INCOMPLETE_EXECUTION` may be reported; the parent rule is inherited without change. |
| Keep the EKS attempt distinct | Both protocols and both digests are retained; the execution receipt names `github-actions-hosted-x86_64`, while the EKS v1 status stays incomplete. |
| Permit SAT only on evidence | No SAT/proof-size job is part of this workflow. It may be preregistered later only if the checked result is exactly `ADVANCE_B2`. |
| Preserve failures | The workflow uploads checkpoint, build, exit, stdout/stderr, partial output, and hash receipts under `if: always()`. |

## Inherited mathematical protocol

The following parent fields are incorporated by exact digest and are unchanged:

- question, claims, hypotheses, predictions, and inadmissible claims;
- fields `8191` and `131071`, four support seeds, and `k=6,8,10,12,14`;
- typed canonical `BranchInstance` IR and all exact representation rewrites;
- natural, support-matched random-coefficient, random-ordinate,
  second-Vandermonde, and separable arms;
- exhaustive balanced split search through `k=10` and the frozen eight splits
  at `k=12,14`;
- semantic, mutation, family-completeness, product-kernel, pivot-minor, and
  rank-one positive controls;
- all measured fields, the 40-cell census, and the complete frozen decision
  rule.

The producer writes the parent digest into `artifact.json`. This runner's
digest binds only backend, source closure, resource caps, transport, and
receipts. Any discrepancy between the narrative above and the parent bytes is
resolved in favor of the byte-verified parent protocol.

The parent EKS-only placement, transport, stopping clauses, and rule that a
backend change invalidates EKS v1 are not inherited as runner execution rules.
They remain fully effective for EKS v1. This separately identified protocol
has the runner-specific stopping and invalidation rules below; semantic and
mathematical control failures retain their parent meanings.

## Frozen source and build identity

The workflow exists only on `main` and accepts one full 40-hex
`source_revision`. It fails unless that revision equals both the workflow
dispatch SHA and the checked-out `HEAD`. It also verifies
`SOURCE_CLOSURE.sha256` before building.

The build uses Rust `1.99.1`, the committed lockfile, two Cargo build jobs,
`-C target-cpu=x86-64`, and target `x86_64-unknown-linux-musl`. Both binaries
must be static x86-64 ELF files. Unit and mutation/control tests run before the
scientific invocation; they do not call the 40-cell producer.

A transport-only wrapper test uses fake shell producer/checker fixtures. It
must pass replay, manifest, and create-new checks and prove that an existing
output directory is rejected before a second fake producer call. Its synthetic
40-element JSON array is not a curve computation or scientific artifact.

`build-info.json` binds the source revision, workflow revision, source-closure
manifest, both protocols, and both binaries by SHA-256. Compilation may fetch
the lockfile's public crates. The scientific producer and checker receive no
AWS credentials and the wrapper performs no network operation.

## Frozen execution

- Backend: GitHub-hosted `ubuntu-24.04`, x86-64 only.
- Scientific worker: the single-threaded native producer followed by the
  native independent checker and one deterministic checker replay.
- Memory: `ulimit -v 4194304` KiB (4 GiB virtual-memory ceiling).
- Deadline: one outer `timeout` of 1,800 seconds for wrapper, producer, and both
  checker invocations, with a ten-second termination grace period.
- Output: at most 67,108,864 bytes across retained runner files.
- Attempts: one original workflow attempt; at most one separately retained
  rerun of the identical workflow run, only after a definite GitHub
  infrastructure failure. Scientific failure, timeout, OOM, control failure,
  or threshold miss does not authorize a retry.
- Budget: at most USD 2 total. Do not use a paid self-hosted runner. Stop before
  execution if the GitHub-hosted allocation cannot be kept within this ceiling.
- Timing: invalid as evidence. No runtime comparison or speed claim is allowed.

The standard repository checkpoint gate must upload the exact source and this
recovery brief to private S3 through GitHub OIDC before the scientific wrapper
starts. A missing or unverifiable checkpoint is a hard stop.

## Exact invocation and validation

The wrapper creates a new output directory, copies both protocols and build
identity, verifies every recorded digest, and invokes exactly:

```text
produce artifact.json SOURCE_REVISION PARENT_PROTOCOL_SHA256
check artifact.json checker.json
check artifact.json checker-replay.json
```

Producer stdout is separate from its create-new artifact path. Both checker
receipts and stdout streams must be byte-identical. The artifact must have
schema `norm.branch-census/v1`, exactly 40 cells, the exact source revision,
and the parent digest. Both checker receipts must have schema
`norm.branch-census-check/v1`, `pass=true`, and zero errors. Producer and
checker verdicts must agree and belong to the frozen vocabulary.

The wrapper emits `execution.json` with schema `experiment.export/v1`, binary
and artifact hashes, exact command arguments, backend identity, run ID and
attempt, and the checked verdict. It emits `job-exit.json` and `SHA256SUMS` on
both success and ordinary failure. An outer hard kill may leave only the
workflow-level exit receipt and partial directory; that is retained and is
`INCOMPLETE_EXECUTION`.

## Frozen outcome handling

1. Any source, protocol, closure, checkpoint, build, schema, hash, output-cap,
   control, replay, or checker mismatch is `INCOMPLETE_EXECUTION` or
   `INCONCLUSIVE` as required by the parent rule; it is never a mathematical
   negative.
2. A checker-clean artifact is interpreted only by the parent's frozen
   `analyze_decision` implementation.
3. `KILL_MEASURED_FAMILY` closes only the registered linear-message
   compression on the frozen family. It does not close nonlinear tensor
   methods, SAT, or index calculus generally.
4. `ADVANCE_B2` authorizes drafting and freezing a separate SAT/proof-size
   experiment. It does not itself establish a SAT or ECDLP improvement.
5. `INCONCLUSIVE` preserves the measured mixed or threshold-missing outcome.
6. Every result report must separate exact observations from interpretation
   and account for setup, relation collection, linear algebra, and target
   recovery before any end-to-end index-calculus claim.

## Reporting and recovery

Retain the raw artifact, both checker receipts, all stdout/stderr, build and
protocol files, source-closure manifest, checkpoint receipt, workflow exit,
`experiment.export/v1`, and `SHA256SUMS`. The result PR must independently
download and verify the workflow artifact, rerun the checker from the pinned
source, and publish source-linked Markdown, SVG, and PDF evidence. Failed or
interrupted attempts remain alongside any allowed retry.

The controlling files are `protocol.json`, `PROTOCOL.sha256`,
`SOURCE_CLOSURE.sha256`, `RESUME.md`, `run.sh`, `test-run.sh`, and
`.github/workflows/norm-branch-exact-runner.yml`.
