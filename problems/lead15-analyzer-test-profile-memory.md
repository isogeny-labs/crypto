# LEAD-15 analyzer tests exceed the current 16-GiB build envelope

- **Status:** `open`
- **Kind:** resource limit
- **Affected work:** [LEAD-15 draft protocol](../research/lead15_diem_quartic_20261101/PROTOCOL.md), [2026-11-01 verification log](../docs/session-logs/2026-11-01.md)
- **Next action owner:** LEAD-15 independent validation runner
- **Last updated:** 2026-11-01

## Problem

The eight exact analyzer tests cannot yet be executed from the root
example test profile in the current 16-GiB cgroup because compiling the main
crate consumes the remaining memory before any test body starts. Their absence
blocks the pre-freeze validation gate for `LEAD15-DT-E5-20261101`; it does not
block static review, the example's clean `cargo check`, or the dependency-free
graph-certificate regressions.

The original requirement remains to execute exactly these non-target tests:

- `tests::result_schema_rejects_unknown_fields`
- `tests::deep_validator_contains_malformed_cell_failure`
- `tests::common_cover_replay_rejects_tampered_certificate`
- `tests::canonical_flow_keeps_unattempted_plans_out_of_runs`
- `tests::k4_bigint_cross_product_handles_admissible_denominators`
- `tests::panic_message_truncates_utf8_on_a_character_boundary`
- `tests::harvest_count_partition_rejects_balanced_impossibilities`
- `tests::partial_attempt_authentication_rejects_wrong_identity`

No frozen target-size generator, example driver, ignored test, freeze,
checkpoint, or launch is part of this validation step.

## Evidence and uncertainty

- **Observed:** Host stable `rustc 1.99.0 (b940084d7 2026-09-28)` and `cargo
  1.99.0 (5f94df478 2026-08-27)` passed
  `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/lead15-final-check-86BGRK cargo
  check --locked --example lead15_diem_quartic`. The ordinary exact-test build
  approached the 17,179,869,184-byte cgroup ceiling and was interrupted before
  a test body. A fresh retry with `CARGO_BUILD_JOBS=1
  CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0` reached
  16,493,719,552 cgroup bytes while the main-crate rustc process had
  5,245,144 KiB RSS; it too was interrupted before a test body. The cgroup
  counters did not increase during that retry (`oom=4`, `oom_kill=1`). Exact
  details and cleaned target sizes are in the linked session log. That clean
  check binds the earlier repaired snapshot only: subsequent deadline/lineage,
  schema, row-weight, and workflow-audit repairs in the current tree have not
  been Cargo-checked locally.
- **Observed:** A dependency-free harness compiled directly from
  `src/cryptanalysis/diem_quartic.rs` at SHA-256
  `4e416ecd728aa99b2bacce7f0dfd1431bfc209e40dc9de282642b31ce76fd3c2`.
  Its three exact graph-certificate regressions passed 3/3. This verifies the
  repaired graph algorithm at its stated small-graph scope only.
- **Observed:** Fresh independent no-file/no-Cargo enumeration of the current
  library bytes (SHA-256
  `3df082c5616c263b9cf495d7aa72143294c79fc82101b855ae20190986f31bf5`)
  found the late-sink certificate yield/support `1/[0,1,2]`, exactly 1,928
  admissible simple-graph orders and 53,814 admissible multigraph streams of
  length at most six, with zero materialized-dimension mismatches. The current
  Rust regressions admit every contiguous-ID mask (`001`, `011`, `111`) and
  assert those exact censuses. This current-byte model replay is still narrower
  than compiling and executing the eight example tests.
- **Derived or proved:** No minimum sufficient memory is established. Both
  attempted Cargo profiles exceed the safe headroom of this particular
  concurrent 16-GiB environment.
- **Suspected explanation:** Root example tests monomorphize and link the full
  library plus the AWS/client dependency graph, producing a larger compiler
  live set than `cargo check`. A memory trace or successful larger-memory build
  is needed to distinguish this from a compiler regression or avoidable test
  harness coupling.
- **Missing capability:** A source-bound builder with enough memory to compile
  and run the eight exact tests, or a reviewed narrower native test target that
  exercises the same analyzer code without compiling the unrelated root
  dependency graph.
- **What this evidence does not establish:** Neither interrupted build is a
  test failure, and the 3/3 standalone graph result does not validate the eight
  analyzer/schema behaviors. No target-size scientific result was produced or
  observed.

## What would unblock it

- [ ] On a fresh source-bound builder with adequate memory, run the eight exact
  test names above under `cargo test --locked --example lead15_diem_quartic
  <name> -- --exact --nocapture --test-threads=1`, retaining toolchain, command,
  source commit, peak memory,
  exit status, and output for each test.
- [ ] Alternatively, factor the analyzer into a dependency-minimal native
  module, prove the example calls that exact module, and run equivalent positive
  and adversarial tests with independent schema/certificate fixtures.

A clean execution of the eight current exact tests is sufficient for this local
pre-freeze test gap. It does not replace the independent full source/protocol/
schema audit, the frozen-manifest gate, or any post-build lineage checks.

## Next discriminating step

- **Action or proof obligation:** First independently review and push the
  source-only commit without the workflow. Then bind
  `.github/workflows/lead15-hosted-validation.yml` to a one-shot push on
  `codex/lead15`: its job must require both `github.event.before` and `HEAD^`
  to equal that exact reviewed source parent, and the workflow-only commit must
  be the sole pushed change. It runs the locked example check followed by the
  existing eight exact tests unchanged at that parent-plus-workflow revision.
  There is no `workflow_dispatch` bypass. The hosted runner is a validation
  venue, not an assertion that its memory envelope is sufficient.
- **Inputs and controls:** Use `Cargo.lock`, the recorded stable toolchain, the
  eight exact test filters, and an empty target directory. Confirm the commands
  cannot dispatch the target-size driver. Retain the actual cgroup memory/swap/
  CPU limits, runner disk size/availability, OS/kernel, Rust toolchain, and
  per-command `/usr/bin/time -v` logs. These are validation provenance only;
  the workflow makes no scientific, wall-time, or performance claim.
- **Execution limits and stop condition:** One build job; stop before host OOM,
  on a compiler/test failure, or after all eight exact tests terminate. Do not
  freeze or launch a study from this validation job.
- **Outcomes and next actions:** Eight passes close this resource-only blocker;
  a test assertion failure becomes a correctness defect to repair; another
  memory stop sizes the required builder but remains infrastructure evidence;
  a compiler failure is preserved separately from test semantics.
- **Recheck trigger:** Independent approval of the immutable hosted workflow
  and source bytes, availability of an adequate builder if the hosted attempt
  is also infrastructure-limited, or a reviewed dependency-minimal analyzer
  test target.

## Resolution and history

2026-11-01: Recorded the clean example check, both pre-test memory
interruptions, the standalone 3/3 graph result, and exact cleanup. Unresolved.

2026-11-01: Replaced the offline high-memory route with the review-only
`.github/workflows/lead15-hosted-validation.yml` on `ubuntu-latest`. It retains
draft/no-target admission, exact one-test-match checks, source/toolchain/host
receipts, explicit no-scientific/no-performance assertions, and exact
temporary-target cleanup. It has not been dispatched; the current post-check
  repairs remain uncompiled pending independent source audit, source-only
  commit review, and an independently audited one-shot workflow-only push.
