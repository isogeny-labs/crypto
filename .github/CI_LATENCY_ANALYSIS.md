# PR and release latency analysis — 2026-10-19

The owner requested a deep analysis of why PRs take too long to deploy, ways to
speed up the jobs, and appropriate concurrency increases. This analysis extends
[PR #2303](https://github.com/aburan28/crypto/pull/2303) and the
[queue policy](CI_QUEUE_POLICY.md). The principal observed problem is admission
delay. There is also a concrete Windows release failure, expensive repeated
checkout/build work, and a release-cadence tradeoff in the earlier patch.

## Evidence and measurement boundary

[ci-latency-evidence.json](ci-latency-evidence.json) records sampled job/step
timestamps, run links, cache statistics, and the release failure. Source baseline:
`a1468cd5ffe10505bbe944e31cd658dd6bcb3c50`; fetched main:
`d67bb0dac6e8edf907765f6527291508c9e94325`.

The fresh queue snapshot contained **296 queued workflow runs**, including 42
Rust lint, 37 Rust all-targets, 28 curve-name checks, 13 workflow syntax, and 11
each of workflow parsing, status pages, and LLM review. PR #2303 had 87 queued
checks. Its broad workflow changes explain much of its own fan-out; that count
is not a claim about a typical PR. An in-progress workflow can itself contain
queued jobs; workflow counts do not establish occupied runner slots or the
account's capacity. The API reads are not an atomic snapshot.

Twelve completed jobs were sampled from the latest 100 successful runs, taking
up to two per named lightweight workflow. Their median initial admission delay
was **95.1 minutes**, median job execution **65 seconds**. Initial waiting was
**98.8% of the sum of wait plus execution** in that selected sample. These are
sample statistics, not repository-wide percentiles or a before/after speedup.
Skipped compilation/analysis jobs are not measured builds or scans.

| Completed sample | Initial wait | Job execution | Checkout within execution |
| --- | ---: | ---: | ---: |
| [Workflow syntax](https://github.com/aburan28/crypto/actions/runs/39320320032) | 96m 15s | 12s | under 15s |
| [All-targets scope classification](https://github.com/aburan28/crypto/actions/runs/39320319770) | 92m 25s | 26s | under 15s |
| [Curve names](https://github.com/aburan28/crypto/actions/runs/39322366822) | 83m 30s | 142s | 131s |
| [LLM review](https://github.com/aburan28/crypto/actions/runs/39323410632) | 80m 23s | 123s | 118s |
| [Status-pages PR test](https://github.com/aburan28/crypto/actions/runs/39315712603) | 117m 01s | 107s | 90s |

Elapsed time is the sum of initial queueing, execution, dependency-stage waits,
and any release cadence delay. A 15-minute per-job timeout bounds only one of
those terms. It does not promise a 15-minute PR or deployment.
Repeatedly retrying a cold build that exceeds the cap can increase demand;
such a job needs a smaller validated scope, an appropriate split, or scheduled
routing, rather than treating a timeout change as an optimization.

## What is consuming time

**Repeated queue admission.** Rust lint first queues a scope job, then queues
its matrix; both clippy matrix entries allocate runners even when all their
substantive steps are inapplicable. All-targets has another scope job. CodeQL's
old automatic path queues scope, database building, cache warming, and analysis
separately. In [run 39289916896](https://github.com/aburan28/crypto/actions/runs/39289916896),
scope ended at 18:11:52 UTC; database building started at 19:59:05 and ended at
20:16:38; warming did not start until 22:01:12. The two intermediate gaps alone
were 107m 13s and 104m 34s. Increasing matrix parallelism cannot remove these
dependency waits. The earlier scheduled routing removes this path from PRs
once it lands; existing runs still use their original definition.

**Checkout.** The audited tree has 115,127 tracked files with 7.87 GB of
uncompressed blob contents. This is not a network-transfer measurement. There
are 98 full-history checkout declarations across all workflows, including
manual research that may require ancestry. The reviewer needs one 8,868-byte
script; the release asset tests need two scripts totaling 5,210 bytes. Their
sampled full checkouts took 118s and 89s respectively. Curve-name checking needs
the complete current tree and merge parents, but not all history. Frozen
replay workflows with ancestry checks need a separate audit before reducing
their checkout depth.

**Cache coverage.** The cache list contained 69 entries totaling 30.36 GB:
64 PR-scoped entries totaling 25.03 GB and only three main-branch entries
totaling 0.15 GB. Nine Lean entries consumed 23.31 GB, about 77% of this sample.
A separate usage read reported 64 entries and 30.00 GB; concurrent updates
explain why those readings must not be treated as one atomic inventory.
The configured storage capacity was not retrieved (lookup returned 404), so
this evidence does not prove eviction or that the account has a 10 GB limit.

[GitHub's cache rules](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)
allow PRs to restore applicable base-branch caches; merge-ref caches do not
warm unrelated PRs. Rust lint and all-targets also use different job identities,
toolchains and flags. Release root binaries pin 1.90.0, isogeny pins 1.93.1,
lint pins 1.98, and all-targets follows stable. A shared key alone cannot make
incompatible compiler outputs reusable. The cache action normally excludes
workspace crates and incremental artifacts; a cache hit does not imply that
the large root library was already compiled.

**Compilation and release packaging.** In
[release run 39294211255](https://github.com/aburan28/crypto/actions/runs/39294211255),
root release builds took 364s Linux, 642s macOS, and 603s Windows. The Windows
isogeny job took 1,124s: 122s checkout, 588s build/test, and 365s rebuilding and
testing the portable hash backend. Both backend checks serve a correctness
purpose. Its configuration explicitly disabled target caching despite two
separate external target directories. Caching those dependencies addresses
avoidable rebuilding without removing either check. It will not eliminate
compilation of source-bound executables.

**Publication failure.** The newest failed release in the sampled main history,
[run 39289773329, Windows job 118235694291](https://github.com/aburan28/crypto/actions/runs/39289773329/job/118235694291),
failed because `checkpoint.rs:12` imports `std::os::unix::fs::OpenOptionsExt`
and line 85 calls `.mode(0o600)` without platform gating. The final publisher
was skipped. The same unguarded code exists in the fetched main baseline.
The latest published release observed was
[v0.1.2954](https://github.com/aburan28/crypto/releases/tag/v0.1.2954), published
2026-10-18 at 20:03:23 UTC. The compilation failure needs a portability fix
that preserves the checkpoint's private-file contract; dropping the Windows
package or weakening the contract would not fix it.

**Successful review status without a review.** The sampled LLM reviewer log
reports that no provider is configured. It spent 118 of its 123 execution
seconds checking out the repository, then warned and exited successfully.
That check result establishes neither a completed model review nor review
coverage. The sparse checkout removes most of its unnecessary payload;
configure a provider if this check is intended to supply actual review.

**Cadence.** PR #2303 currently changes binary publication to daily at 11:47 UTC,
with manual dispatch retained. That reduces background load but adds up to
24 hours of schedule delay, before queueing/building. It is containment, not
the desired fast-deployment design. Status Pages already has its own push and
scheduled publisher and is not controlled by the binary release schedule.

## Changes implemented in this follow-up

- Reviewer checkout now materializes only its trusted default-branch script,
  with blob filtering. PR content remains inert API data.
- Curve-name checkout uses depth two. The complete current-tree audit and
  exact synthetic-merge-parent comparison remain intact.
- The release asset publication tests use a sparse checkout of their two scripts.
- Isogeny dependency caching maps both real external target directories using
  platform-relative paths, includes the target triple in the key, and saves
  only on main. Workspace-crate caching remains off; both build/test variants,
  source provenance, installed-tree tests, and byte-verification remain.

These are implemented configurations, not measured hosted improvements. Cold
cache runs still compile dependencies. A hosted run on each platform is needed
to confirm cache hit rates, cleanup and restoration, and actual time saved.

## Ordered next changes

| Priority | Change | Acceptance evidence |
| --- | --- | --- |
| P0 | Fix the Windows checkpoint compilation failure while preserving private-file behavior. Add a fast platform compilation check before packaging. | Windows build passes, checkpoint contract tests pass, publisher succeeds with every intended asset. |
| P0 | Land #2303, then re-enable the nine scheduled workflows only after their new triggers are on main. Keep the named exceptions disabled pending migration. | Inspect main workflow definitions and workflow states; measure arrival rate and oldest queue age afterward. |
| P1 | Consolidate lightweight checks into one fast PR job and run source classification once; use a dynamic set of applicable build jobs. Move no-op conditions to job admission. | Compare selected checks against the existing dependency-aware classifier, including missing-base/failure behavior; preserve a stable aggregate required check. |
| P1 | Give routine PR validation and publication capacity separate from scheduled research, and verify account-wide slot usage. Request a hosted concurrency increase if saturation remains. | PR p95 admission below two minutes during measured load; research scheduling cannot consume the entire reserved PR pool. |
| P1 | Seed and retain compatible main-branch Rust caches; share keys only for matching toolchain, target, profile, features and flags. Reduce duplicate large Lean caches after measuring their hit/transfer cost. | Record restore hit, restore/save seconds, compile seconds and cache size across at least 20 comparable runs. |
| P1 | Replace nightly-only publication with coalesced main-branch release preparation plus a serialized final publisher. Keep full research replay scheduled. | Latest eligible main reaches publication within the agreed budget; provenance identifies the exact built commit; cancelled preparation cannot interrupt or roll back publication. |
| P2 | Consolidate same-configuration Rust compilation and reuse same-run binaries/artifacts for related tests. Profile and split the large crate only where compile timings justify it. | Compare total runner-minutes and complete PR elapsed time, retaining all applicable tests and artifact/source identity. |

The main-branch release design needs an explicit product decision about whether
every commit must publish or only the latest eligible commit. A latest-only
queue is appropriate for rolling releases but is a semantic change. This
follow-up leaves the earlier scheduled trigger in place rather than silently
restoring long automatic builds before that design is validated.

[GitHub concurrency limits](https://docs.github.com/en/actions/reference/limits)
are account/plan capacity, not a numeric setting in workflow concurrency groups.
The audited repository reported no self-hosted runners, and the busy routine
workflows had no restrictive `max-parallel`. The actual account limit and
other-repository occupancy remain unverified. Larger or isolated ephemeral
runners may help, but need a measured capacity/cost choice; no paid capacity
was provisioned in this change.

Proposed service objectives are PR p95 queue wait below two minutes, routine
PR validation below ten minutes end to end, and publication below fifteen
minutes after an eligible merge when release preparation is warm. These are
targets, not current measurements or guaranteed speedups. Collect first-job
queue delay, inter-job waits, execution, retries, total runner-minutes, and
merge-to-publication latency separately. Failure and cancellation rates must
accompany timing statistics so faster failures do not look like improvement.

## Validation

Actionlint passed for all 431 workflows. Exact sparse-checkout fixtures ran
the reviewer self-test and release publication tests successfully. The existing
curve-name audit found zero problems. The cache path generator was exercised
against Unix and Windows path semantics. A depth-two synthetic-merge fixture
preserved the exact PR parent, diff, and complete current tree. Existing budget
tests (15), scope/frozen-workflow tests (25), and workflow parse tests (4) passed.
The fresh `cargo test --release --lib` run passed: 4,399 passed, zero failed,
118 existing ignored tests, 483.89 seconds of test execution. It reused
`/Volumes/SSD990-2/crypto-actions-queue-target` (build preparation 0.29s) and ran
with localhost socket permissions. This local timing is not a hosted benchmark.
No hosted speedup is claimed before these changes land and execute.
