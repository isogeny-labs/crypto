# Current-parent integration validation

This is development validation of the PR's base integration, not a scientific
pilot, confirmation, timing experiment or new parameter search.

## Source and resolution

- Actual parent source: `543bbd3e53b5f81f3875bafbb1678fce47d739e0`.
- Previously published PR head: `f12e8a2fed1488637e2948c1b73ffb82d7ca90f4`.
- Local merge revision: `adfb858227a5a31585a12afb2cd55114dbcd2c81`, with those
  two parents in PR-head/base order. No rebase or force-push is used.
- Copilot request [6442557426](https://github.com/isogeny-labs/crypto/pull/2684#issuecomment-6442557426)
  had no response or branch update after about one hour. The AGENTS.md fallback
  is recorded in [6443966551](https://github.com/isogeny-labs/crypto/pull/2684#issuecomment-6443966551).
- The only conflict is an add/add of `docs/session-logs/2026-10-27.md`.
  Both original entry bodies are retained byte-for-byte under a shared heading.
  The parent PR itself is not merged or otherwise changed.
- Git comparison at the merge revision confirms unchanged native example,
  protocol/allocation/CSV and the complete preflight, size-ladder and frozen
  replication study trees. The four `INPUTS.sha256` checks pass.

## Validation status

| Check | Observed status |
|---|---|
| `cargo check --release --lib --example conductor_gap_confirmation --offline` | Pass |
| Full `cargo test --release --lib --offline` | Incomplete: 5,036/5,037 terminal names, 13 initial failures; nine pass focused reruns with permitted local TCP access, four parent-code failures persist |
| Focused `cargo test --release --example conductor_gap_confirmation --offline` | 12 pass, zero failed/ignored/filtered |
| Release example and backlog build | Pass |
| Fresh `plan` compared with the retained allocation | Byte-identical: 1,011,591 bytes, SHA-256 `bf8ebc6255d660c4b9d34dc9843958eacc4182ee593f85f50e73284c9abe7ea5` |
| Native allocation audit | Pass; zero solver invocations |
| Native preflight | Expected exit 3, status blocked, execution not ready |
| Backlog validation | Pass; measurement card remains blocked |
| `python3 tools/artifacts/test_artifacts.py` | 18 tests pass |
| Current-parent validation-gate suite | 62 tests: 61 pass, one committed-tree test fails with 44 diagnostics |
| Site suite after sparse materialization | Blocked before the suite completes on the unhydrated backend-leaderboard artifact |
| Large-file check against actual parent | Pass |
| Artifact policy check against actual parent | Pass: no added/modified data files selected by policy |
| `rustfmt --check --edition 2021 examples/conductor_gap_confirmation.rs` | Pass |
| Whitespace check against actual parent | Pass |
| Repository-wide workflow artifact wiring | Exit 1; reports two unchanged parent workflows lacking artifact pulls |

Native validation is complete. The full library command still has no final
result; publication does not claim a passing full suite or exact-head CI.

See [raw development traces](traces.md), [native allocation audit](allocation-audit.json),
[blocked preflight](readiness.json), [merge-preservation receipt](merge-review.json),
[complete gate diagnostics](gate-errors.json), [current-verdict hydration provenance](current-verdict-hydration.json)
and [site-input classification](site-input-plan.json). These are additional
validation records; the original checks are preserved.

## Full-library execution and focused diagnosis

At 14:46 UTC the full run has 4,901 passing, 13 failed and 122 repository-ignored
terminal test names; the single remaining case is
`cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
No test was manually disabled or skipped, and the full command's exit remains
unconfirmed. See the [progress receipt](library-progress.json).

Nine failures are local-server tests: the initial sandbox denies TCP bind with
`Operation not permitted`. All nine pass focused reruns with permitted network
access for their public loopback fixtures. The other four still fail: the
weighted comparison's `zero.legacy_comparison_id.is_none()` assertion and
three hyperelliptic cases rejecting `branches`. The library and Cargo inputs
are identical to the actual parent. See [complete focused traces](library-focused-failures.md)
and [focused exit receipts](library-focused-results.json). These focused checks
do not replace or imply completion of the full suite.

The active full capture remains at
`/workspace/conductor-2684-validation-20261027/release-lib.log`. Its retained
publication snapshot is 431,637 bytes, SHA-256
`a99f4c179ac774cf84bbd0ac117dfc672a9c5d32a38fec6352e7bfec4a189528`.
It is local only, not durably uploaded or placed in Git contrary to artifact
policy. The existing process is tracked for the authoring follow-up; no full
rerun is launched. A future timeout must remain failed/incomplete and cannot
be treated as a passing or inapplicable check.

## Parent evidence and setup limits

The gate's 44 messages comprise 13 leaderboard, seven benchmark and 24 archive
diagnostics; 20 explicitly name a missing file. None names this conductor-gap
study. The gate scripts, indexes, workflows and artifact tooling match the
actual parent; this PR does not patch around those failures.

The current benchmark-verdict manifest pins 135,403 bytes, SHA-256
`da0ba5185aaf8baaea5bd8e676c79e5ff33aa294f9b6e28ebde5dbe6d9d3ef02`.
Those exact bytes were recovered from accessible Git revision
`1ad880ad00d2ad27f64e7d8a86302657c396ca1e` and checked against the current
manifest before local hydration. This is not S3 read-back. The older 25d9230d
input and its row-schema error are not substituted for this current input.

Twenty tracked site inputs excluded by sparse checkout were materialized with
the repository's sparse-checkout tool. The remaining backend leaderboard is
8,451,778 bytes, SHA-256
`91d30a0f5d72dd191b5361f5c5a2699d8b0e6ac074b2f21957d8155daa75cab9`.
Its bytes remain unavailable locally; the history lookup supplied no usable
commit. Runtime credentials/identity configuration are unchanged and remote
artifact read-back remains unconfirmed.

`python3 tools/artifacts/wire_workflows.py --check` reports missing pulls in:

- `.github/workflows/conductor-gap-replication.yml` for
  `research/conductor_gap_replication_20261025`;
- `.github/workflows/conductor-gap-size-certificates.yml` for
  `research/conductor_gap_size_ladder_20261026`.

Both workflows are byte-identical to the parent. An attempted bulk prefetch of
existing Git manifest metadata failed with `bad revision 005df686...`; the
unmodified wiring check subsequently finished with the two diagnostics above.
No authentication configuration, workflow, evidence record or gate was changed.

## Scientific and publication limits

The prospective allocation and all scientific requirements are unchanged.
The planner/auditor/preflight do not execute solvers. Repository unit tests are
development checks, not observations for the proposed study. The 32,768-solve
pilot and provisional 1,024,000-solve confirmation remain unlaunched, with full
replays and controls additional work. Crater equivalence, complete CM costs
and the prime-field algebraic oracle remain open. No hardness or full-cost
speed conclusion follows from these checks.

The PR remains open pending all applicable exact-head CI, reviews and repository
rules. The existing published-head Actions jobs failed before startup under
the account payment/spending restriction; they do not cover this local merge.
