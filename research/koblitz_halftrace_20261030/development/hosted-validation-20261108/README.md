# Supported-host native library validation

The local 5,937-test parent attempt remains incomplete. Two resource-publication
tests independently reproduced the missing Linux time-namespace interfaces on
that host. Repeating the same local suite would not address that dependency.

The existing registered `rust-all-targets.yml` workflow now has an explicit
`full_library` manual option and an immutable `source_revision` input. Ordinary
compile jobs and normal triggers are unchanged. The optional job uses standard
GitHub-hosted Ubuntu24.04 development CI, checks the actual required interfaces
before checkout/build/test, and fails closed if they cannot be read.

This is correctness validation, not a scientific experiment or a substitute
benchmark. It uses no new secret, AWS identity, artifact-store credential or
permission beyond `contents: read`. It never runs ecbench measurement commands,
the study's scientific workload or checkpoint preparation. The frozen protocol,
spec and scientific source remain unchanged.

The job pins Rust1.99.1, builds the complete library test executable, records its
hash, and runs `cargo +1.99.1 test --release --lib --locked` with no test filter.
Existing ignored tests retain their normal policy. Cargo build jobs are limited
to2, harness threads4 and Rayon threads1 for development resource use. The job
deadline is90minutes; checkout/build/test steps are bounded to5/25/55minutes.
One initial dispatch is intended; no automatic retry is configured.

Request/workflow/source/run identities, host admission, compiler output, full
parent stdout/stderr, Cargo and log-writer exit codes, post-run source checks
and log digest are retained as an Actions artifact for14days. A timed-out or
interrupted step without its terminal receipt stays incomplete. A nested test
summary never substitutes for a complete parent result. An Actions artifact is
temporary development evidence; scientific S3/canonical-export gates remain.

After publishing this change, the exact invocation is:

```sh
gh workflow run rust-all-targets.yml --repo isogeny-labs/crypto \
  --ref codex/koblitz-halftrace-native \
  -f full_library=true -f source_revision=EXACT_PUBLISHED_HEAD_SHA
```

Retain the actual dispatch identity before checking it at the AGENTS cadence.
Reconcile an ambiguous submit before another call. Collect the optional full
library job separately from the existing all-target compile job; neither is the
required `gate`/`size`/`artifacts` merge set or a resolution of the performance P1.

Sources reviewed: the existing Rust compile/lint, release, ecbench and research
workflows; saved local failure/closure receipts; AGENTS development/CI and merge
rules; repository `ci-scope.mjs` and artifact policy. No native algorithm source
or historical evidence was edited to create this route.

Preparation validation: actionlint1.7.12 passed after correcting a job-level
runner-context reference to a step-level GITHUB_ENV binding. The initial lint
failure remains in actionlint-initial.txt. Native source hashes still match the
retained507829726f manifest. This preparation is not a completed hosted test run.

Submitted one development run at22:17UTC:
[42529489383](https://github.com/isogeny-labs/crypto/actions/runs/42529489383),
with both requested source and workflow-run head bound to
cda1e5810f41550a2b929cbdb6fe4ff242865617. Its initial observation was queued;
host admission, the optional job and parent result are pending. Exact command,
publication identity and dispatch result are in dispatch-receipt.json. Do not
submit another attempt or infer a full-suite pass from the accepted dispatch.

At the23:24UTC check, job128880307287 passed actual host-interface preflight,
immutable checkout/toolchain binding and the complete release test-binary build.
The full parent test step is still in progress; no terminal receipt or artifact
has been published. Separate compile job128880359995 passed workspace/all-target
and wide-vars checks plus the core crate's tests. Those are distinct from the
still-pending full library result. See progress-20261108T2324Z.json; the original
queued dispatch receipt remains unchanged. No additional attempt was submitted.
