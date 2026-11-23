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

The authorized stop condition is met: PR4525 is merged, its actual outcomes
are recorded, and its follow-up stays off. PR4535 and its separate cohorts
are outside this closeout.
