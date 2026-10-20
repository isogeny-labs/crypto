# Automatic CI queue policy

The owner confirmed on 2026-10-20 that existing pull-request checks and their
documented timeout exceptions must be retained. This supersedes this PR's
earlier proposal to cap automatic jobs at 15 minutes and move nine validation
workflows to schedules. Queue admission time and job execution time remain
separate; a shorter timeout does not establish faster successful validation.

## Routing and budgets

- Existing pull-request and merge-group triggers are retained, including
  CodeQL, curve-cover certificates, ecbench, IC autolab, the Koblitz relation
  bridge and SOTA reproduction, performance history, polynomial reuse, and
  release packaging.
- Existing job timeout declarations are retained for every PR workflow. The
  regression policy remains the 25-minute default with named exceptions in
  `scripts/check-pr-budgets.py`. This includes CodeQL's analysis/preparation
  exceptions and the 45-minute ecbench and archived-producer exceptions added
  by PR #2189. Their longer manual/research budgets are unchanged.
- The release workflow retains main-push publication and PR packaging checks.
  Performance history retains its existing main-push behavior. This patch does
  not replace either with daily publication.
- 65 long research workflows formerly triggered by pushes to experiment
  branches now require `workflow_dispatch`. Their inputs and execution steps
  remain available on the intended research branch.
- Stateless CI uses separate concurrency groups per workflow, event, and PR/ref
  to cancel obsolete automatic runs. Existing scientific and publishing
  serialization, scientific matrix limits, and the single-writer autofix queue
  are retained.

The retained checkout/cache improvements narrow the trusted reviewer checkout,
use a depth-two complete tree for curve names, sparse-check out publication-test
scripts, and cache both external isogeny dependency targets. Every original
build/test variant and publication-integrity check remains present. These are
configuration changes; hosted timing improvement has not been measured.

## Frozen replay and validation repairs

Historical workflow/source seals are verified against immutable revisions before
replay in isolated copies. The native-m3 replay uses its exact historical source
view. The compact shared-log replay retains its original lockfile and evidence.
Source, manifest, or digest mismatches fail instead of rewriting historical
receipts. Lean auditing accepts both supported CLI forms and validates its
working-directory boundary. Performance-counter checks reject missing or
uncounted IPC after a successful probe and preserve command failures and raw
output. Formatting/lint repairs preserve the validated native behavior.

## Live workflow state

The earlier queue cleanup disabled nine validation workflows while proposing
scheduled-only routing. The owner's subsequent instruction retains those PR
gates, so those nine must be active for their applicable checks to run. Source
restoration and live Actions enablement are separate operations: a disabled
check or an absent run is not a passing result. Current provider state and
current-head results must be checked before merge.

The historical cleanup also included six separately held workflows. This PR
does not resolve their ownership or evidence migration:

| Workflow | Existing boundary |
| --- | --- |
| `compact-s3-batch.yml` | Already disabled; reserved by T-4279 |
| `compact-s3-prefilter.yml` | Already disabled; reserved by T-4279 |
| `compact-swap-quotient.yml` | Already disabled; reserved by T-4279 |
| `weak-curve-database.yml` | Reserved by T-4258 |
| `weak-curve-lean.yml` | Workflow bytes belong to a historical evidence seal |
| `weak-curve-conjectures.yml` | Workflow bytes belong to a historical evidence seal |

Three branch-only workflows were also disabled during that cleanup:
`boolean-single-write-runtime.yml`, `koblitz-modular-carriers.yml`, and
`n83-padd-rank.yml`. Their branch contents were not changed. The separate
`validation-gate.yml` had a 10-minute budget and remained active. None of these
branch-only or separately held workflows is silently enabled by a source edit.

## Historical queue evidence

The initial 2026-10-19 snapshot found 354 queued workflow runs and 15 in progress,
including 48 ecbench, 42 CodeQL, 40 Rust lint, 37 all-targets, and 21 Release
runs in the queue. These are workflow counts, not individual job counts or a
measurement of occupied hosted-runner capacity.

The cleanup selected 152 queued automatic runs, re-read each state, preserved
eight that had left the queue, and cancelled 144. Those 144 were then verified
completed/cancelled. All 18 workflows in the historical disablement inventory
were verified disabled at that time: 15 disabled in that operation and three
already disabled. Queue snapshots changed from 356 to 215 during cleanup;
concurrent arrivals/completions prevent attributing that difference solely to
the cancellations. Existing artifacts and run records were retained.

The source audit covered 431 main-tree workflow files and four observed
branch-only workflows, from 524 registered entries including historical and
GitHub-managed workflows. It did not cover every inactive branch revision.
Successful sampled jobs included autolab controls at 22.5 minutes, cover
certificates at 22.0, polynomial correctness at 23.3, SOTA reproduction at 38.0,
and Windows isogeny packaging at 18.7. These observations support retaining
adequate correctness-check budgets; they are not repository-wide percentiles.
The immutable [latency evidence](ci-latency-evidence.json) and qualified
[latency analysis](CI_LATENCY_ANALYSIS.md) retain the original measurement
boundary and links. No new queue or deployment speedup is claimed.

## Validation

The final native validation before restoring workflow policy passed 4,418
library tests with zero failures and 118 existing ignored tests. Focused
isogeny/eigen tests, historical archive replays, Lean audit checks, and mocked
performance-counter regressions also passed. Restoring event/timeout metadata
does not change those native sources or their frozen inputs.

The policy-restoration review compares every existing PR workflow's event and
timeout declarations against current main, including #2189's exceptions.
Actionlint, the budget checker and its 15 tests, CI-scope/frozen-source helper
tests, publication tests, and performance-counter regressions are rerun on the
final source. Applicable current-head hosted checks remain necessary before
merge; local validation does not substitute for platform-specific CI.
