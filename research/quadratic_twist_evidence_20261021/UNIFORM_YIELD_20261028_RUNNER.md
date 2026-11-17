# Uniform native runner: implemented, scientific execution pending

2026-10-28. This is ordinary implementation work for the existing frozen
uniform-yield protocol. It adds thin shell build/run glue, without changing
the C++ producer, independent Rust arithmetic, populations, sampler, solver,
controls, source reductions, historical receipts or scientific outputs.

## Authorization and remaining access

The user approved the previously presented checkpoint-only scope: start,
finish and plan-change checkpoints for this same PR branch under
`s3://setec-astronomy-artifacts/checkpoints/crypto/codex__quadratic-twist-evidence-import/`.
This approval persists; do not ask again for routine checkpoints in that scope.
It does not authorize arbitrary result assets, another branch, AWS credential
reuse, paid compute or a replacement uploader.

Both secure-token presence checks are false in this session: neither
`CHECKPOINT_UPLOAD_TOKEN` nor the documented token file is configured. No
token value was read. The official gate has not been invoked and no upload
has occurred. Scientific and proof execution remain blocked on actual access,
not on a missing repeat approval. The detached worktree must first be bound to
the approved branch, and a fresh complete RESUME/preview reviewed for that exact
head. Do not reuse an earlier worktree's preview or create a detached prefix.

The reviewed main checkpoint blob at `6187036c049b6de2b1521b014f575952e4f1705c`
is `6a2a37967047b1a46ed62a4123f4c69f08a55d04`, newer than the old cached gate.
Its changes include GNU/BSD stat handling and failure checks. This update
neither replaces the repository gate nor invokes the stale cached copy.

## Runner contract

After fetching current main, secure access, current-instruction review and a
complete fresh RESUME:

```sh
timeout 3600 bash research/quadratic_twist_evidence_20261021/run-uniform-yield.sh FRESH_SCRATCH_DIRECTORY
```

An optional second argument names a later frozen expected record containing
`cases/`, `audit.json` and `INPUTS.sha256`; canonical cases, receipt and source
bytes must then compare exactly. No such uniform scientific record exists yet.
The initial invocation records an attempt; it cannot claim historical replay.

The runner refuses a different/detached branch, repository, bucket/prefix,
missing secure token, dirty tracked sources, absent tracked inputs or an
existing/checkout-contained output directory. It invokes the unmodified
repository checkpoint gate before builds/native commands, and refuses a gate
that differs from fetched main. It does not copy or patch a global gate.
Token presence is
not gate success; malformed/expired token handling remains in the official
script, which selects a nonempty token before any AWS fallback. The runner
never invokes an AWS credential check or a skip path.

The documented outer timeout bounds the complete command at3600 seconds;
each external build/native command is also capped at600 seconds and at the
remaining command budget. Cargo has two build workers. A fresh Cargo target directory
prevents accidental use of an arbitrary cached executable. The C++ program
receives ROOT/FRESH_CASES; the Rust program receives audit ROOT/CASES/RECEIPT.
No scalar labels are added to either interface. Native implementations still
enforce the96 arms/896 source targets/210 comparisons and exact case membership.

All13 existing corruption modes are attempted on separate copied payloads.
A non-null mutation must change a payload, then reject with101, the expected
uniform-data diagnostic and no receipt. Only missing-failed-query may be null;
that classification must agree with the independent receipt and leave the
case bytes unchanged. No timeout or unrelated crash counts as rejection.
Builds, native backend tests and exact-byte comparisons retain commands,
logs, statuses and input/output hashes, including failures. No result upload
or new workflow trigger is added. The caller must update RESUME from the actual
outcome and run the unmodified gate for its finish/failure checkpoint; the
script does not falsely label a stale RESUME as a fresh completion checkpoint.

## Actual validation and scope

This implementation step ran only shell syntax and non-executing CLI checks.
The runner's native paths, gate call, corruption controls
and backend tests are not executed. No uniform yield, degree, expected-call,
state, scalar, theorem, identity or performance result is supplied by this file.
The current strict registry assertion and incomplete full-library result stay
open; the runner does not bypass or replace either.

The faithful actual-state/iid probability bridge and Lean elaboration remain
pending. Native recurrence replay alone will not establish that theorem, a
stationary twist advantage, intrinsic regularity or complete calibrated work.
The frozen October25 protocol and October26 compile-only records remain
unchanged. References checked: those two contracts, the auditor's exact CLI
and corruption outcomes, the prior checked-root runner, and current official
checkpoint scope/transport/gate functions. No new ECDLP mechanism is proposed.
