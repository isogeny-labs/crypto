# PR4525 publication closeout

PR [4525](https://github.com/isogeny-labs/crypto/pull/4525) merged on
2026-11-17 at 15:09:49 UTC as `4bf2d9dae80ea1c7d6f70478492d95dd0ddb2dac`,
from head `9c66263eb62f1e91685f2ac60eef92ba8c1ef4c0`.
GitHub names `aburan28` as the merging actor. This session did not perform
that merge. The bounded follow-up was already disabled when resumed on
November 23 and remains disabled. This closes publication monitoring only.

## Recorded checks

| Check on the final PR head | Actual outcome | Completion UTC | Relation to merge |
| --- | --- | --- | --- |
| [gate](https://github.com/isogeny-labs/crypto/actions/runs/44031468303/job/133864208555) | failure | Nov 17 06:33:41 | Before merge |
| [size](https://github.com/isogeny-labs/crypto/actions/runs/44111144725/job/134136763616) | success | Nov 17 15:15:47 | After merge |
| [artifacts](https://github.com/isogeny-labs/crypto/actions/runs/44111144941/job/134136760536) | failure | Nov 17 15:15:57 | After merge |

Earlier draft size/artifact jobs were skipped. Their later outcomes cannot
establish passing checks at merge. Reviews, discussion comments and inline
threads returned empty lists in this inspection. No protection bypass or
merge procedure is inferred from those facts. The recorded outcome is not
clean CI. The gate failed its committed-tree test, as documented in the
earlier follow-up. The retained artifact log identifies the nine existing
PNG/log policy files. No bot migration occurred on the final PR head.

## Preserved evidence and remaining obligations

The study tree `speed-20261117` has the same Git object
`8f1ee2e1e4c366c8880da97947feead0e959a1ae` at the final head, merge and
inspected current main `816f9c3a942efed569049618c056d12a0c56d693`.
The data claim file is also unchanged. Both merge parents' complete
whitespace-normalized level-two journal blocks, including multiplicities
and file preambles, are retained. The first parent's PCC ledger is an exact
byte prefix of the merged ledger. See [merge-audit.json](evidence/merge-audit.json).
These are producer metadata audits [pcc:unverified].

The official standing query, after materializing its exact tracked claim,
ledger and keyring inputs, reports data claim `PCC-20261117-dcce99` as
`unverified` with no verdicts; see [data-standing.json](evidence/data-standing.json).
The full general degree argument remains unverified. The original
[RESULTS](../speed-20261117/RESULTS.md),
[REQUIREMENTS](../speed-20261117/REQUIREMENTS.md) and
[REGISTRATION_ERRATA](../speed-20261117/REGISTRATION_ERRATA.md) retain the
failed canonical seed qualification and below-L2 clock scope. No scientific
claim or qualification is upgraded by this closeout.

The study's benchmark record was OPEN on inspected main. The historical
archive still has SHA-256
`cceddc4549b0a35a9acb5ad5001d4dd335ab9d82a70c7cdc945927cfa600f501`;
its first command lacks a successful integer exit. However, current main
already carries an exact-hash comparison exclusion for that historical
record. The unchanged archive bytes alone therefore did not establish the
old blocker. After hydrating the exact tracked metadata and our original
study files, the official helper closed **only our record as censored** at
2026-11-23T06:50:07Z and regenerated progress.

[record-close-audit.json](evidence/record-close-audit.json) retains the successful
helper exit, zero scoped record findings, all 1,301 unchanged raw entries,
unchanged commands and metrics, and the exact five changed metadata fields.
The record names the original report, counts/timing SVG and PNG figures and
validation receipt; its summary states below-L2 clocks and failed canonical
seed registration. The global progress files were stale on main. Regeneration
also reflects main's existing records; the series are identical with our
record open or censored, and our closure only moves one status count from
open to censored. No Vieta measured series or protected row was added.
The global bench-record gate still reports unavailable inputs outside this
hydrated scope; it is not claimed to pass. No unrelated historical record,
exclusion, protocol, raw datum, claim, source, study report or protected index
was edited. No benchmark, CI rerun, paid/cloud experiment or local S3 operation
occurred. These administrative checks remain producer assertions [pcc:unverified].

## Administrative outcome

The official helper recorded autonomy episode `E-784b9d24e07e` as partial,
merged and not green at merge, with lesson outcome `O-8beb72961cf3`.
This is not an unsupervised success. The incident schema offers only SELF
or HUMAN recovery; unresolved failures are retained in the check evidence,
without inventing a recovery classification. Complete original episode
intervention timestamps are unavailable in this continuation, so no zero-
intervention conclusion is claimed. The 14-day revert window has not elapsed.
Practice records remain producer assertions [pcc:unverified].

Sources checked: current AGENTS, current ACTIVE and November 23 journal;
the study handoff and pinned results/requirements/errata/record receipts;
the workspace report-evidence skill; GitHub's PR metadata, review/thread
lists and final-head Actions jobs. This is a publication metadata closeout,
with no new research result or literature claim. Existing study report PDFs
and graphs remain unchanged; only the official progress renders regenerate.
Conductor is unavailable (`command not found`);
the closeout uses an isolated worktree and no shared original worktree edits.

The GitHub connector supplies normalized metadata but omits job timestamps;
the retained read-only `gh pr view --json ...` snapshot and paginated
check-runs API selection supply the recorded times. Local checks cover
append-only metadata, prose, scoped record closure, artifact policy and exact
Git byte comparisons. The PCC ledger check passes after hydrating main's
existing schema-rejection manifest and claim inputs; no historical payload
or rejection entry was changed.
No Rust source changed, and no new full-root test pass is claimed.

The initial local artifact warning also came from incomplete sparse inputs:
`.artifacts/config.json` was off disk, so the tool used its default policy.
After exact tracked hydration, the current unchanged policy keeps all PDFs
and `benchmarks/progress/progress.json`; the actual local artifact check
passes. The official migration job on `24aae5d188` likewise reported no
changed data files and made no commit. No migration or S3 operation is needed
for this closeout. See [artifact-policy-correction.json](evidence/artifact-policy-correction.json).
The earlier `local-checks.json` and `pending-artifact-migration.json` are
retained as superseded sparse-input diagnostics, not current policy standing.

The authorized stop condition is met: PR4525 is merged, its actual outcomes
are recorded, and its follow-up stays off. PR4535 and its separate cohorts
are outside this closeout.

## First bounded administrative check — November 23

PR4987 head `7d1b8408a0` remains open and mergeable, with MERGE auto-merge
enabled and no reviews, comments or unresolved threads. Required size passed;
validation-gate and artifacts failed. Validation stopped on Git-only recovery
of the unchanged packed-filter archived facts, before reaching its gate tests.
Artifacts stopped on four inherited tool-test failures, including an undefined
`paths` in `wire_workflows.py`. A focused same-source metadata test reproduced
its failure. The local artifact **policy** check still passes; it is narrower
than that failed CI job [pcc:unverified].

Other observed failures concern unchanged archived planner inputs, frozen
backend report availability, old curve naming, PCC source/lock mismatch and
PCC claim/test failures. The exact compared scopes match current main.
See [CI diagnosis](evidence/ci-diagnosis-20261123T0805Z.json) and
[failing-input equality](evidence/failing-input-equality-20261123T0805Z.json).
No unrelated infrastructure, historical claim, record, registry or exclusion
was changed. No CI rerun, benchmark, replay or local S3 operation occurred
[pcc:unverified].

One new journal paragraph lacked an explicit unverified citation. That
qualification is added, and its original full block and digest are retained
in [the edit receipt](evidence/journal-qualification-20261123T0805Z.json).
No original statement was removed. All earlier main journal bytes remain an
exact prefix; all other old journal blocks are untouched. Global prose lint
now reports the same 17 findings as main, with zero new PR-owned paragraph
findings. See [the comparison](evidence/prose-qualification-result-20261123T0805Z.json).
The change does not confer independent scientific standing [pcc:unverified].

The remaining bounded checks follow actual head changes and required results.
No merge is authorized with failed or pending required checks, and no
numerical goal or measurement is reactivated by this administrative follow-up.

## Second bounded check — upstream enforcement review

At head `e73c899630`, required size passed, while artifact-tool tests and
validation input recovery still failed. The earlier PR-owned prose issue is
resolved: PCC CI now reports the same 17 journal findings as the base. No
reviews, comments or unresolved threads appeared [pcc:unverified].

Current main `143a0b91fb` now includes [PR5001](https://github.com/isogeny-labs/crypto/pull/5001),
which repairs the artifact generator's undefined names and the PCC test/
subgroup-verifier issues. Reviewing the complete upstream changes also found
`continue-on-error` added to artifact recovery, a unit expectation changed to
accept that tolerance, and archive matching broadened beyond exact removed-row
text equality. The relevant source excerpts and hashes are retained in the
[upstream enforcement review](evidence/upstream-enforcement-review-20261123T1200Z.json).
The older equality receipt against main `816f9c3a94` is historical and does not
describe these newly changed tools [pcc:unverified].

The proposed main merge was conditional on preserving the checks. The combined
bundle does not satisfy this task's explicit no-weakening constraint, so it
was not merged or selectively patched here. MERGE auto-merge is paused while
that conflict remains. This is a review of the named enforcement changes,
not an assertion that every later validation step would pass or fail open.
No workflow, policy, verifier, historical claim or protected evidence was
edited by this follow-up [pcc:unverified].

Only the final already-scheduled administrative check remains. It may assess
a compatible upstream repair and actual required results; the three-occurrence
schedule will not be extended. Both scientific studies and the completed local
numerical goal retain their earlier scope and standing.
