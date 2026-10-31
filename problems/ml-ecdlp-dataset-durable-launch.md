# ECDLP dataset worker publication and cumulative launch admission

- **Status:** open
- **Kind:** access limit; resource admission gap; implementation gap
- **Affected work:** [dataset design](../docs/ml-dataset/DESIGN.md), [operating guide](../docs/ml-dataset/OPERATIONS.md), [coverage obligations](../docs/ml-dataset/REMAINING-COVERAGE.md), [PR 2756](https://github.com/isogeny-labs/crypto/pull/2756)
- **Next action owner:** dataset producer task T-684; scoped obstruction record T-713; human approval is required for the prepared shared-role grant
- **Last updated:** 2026-10-28

## Problem

The native producer has not established a completed cloud cell with durable S3
publication and fresh independent replay: the actual worker cannot write the
approved bucket, and the next wave has not been admitted against the existing
cumulative campaign budget.

The original target remains Koblitz, general binary and prime curves, both
twists, n=23..31 and 47..61 including p=2^61-1, m=2..15, many generators/keys,
and replayable solver intermediates. The frozen [720-cell pilot](../research/ml_dataset_20261028/protocol.json)
is an initial correctness panel. A functional CLI, a plan with 165,150,720 method
cells, local unit tests or a successful synthetic target construction does not
establish that cloud corpus.

## Evidence and uncertainty

- **Observed:** [pilot 002 terminal evidence](../research/ml_dataset_20261028/pilot-wave-002-terminal.json)
  records S3 HeadObject HTTP 403, exit 1 and the exact Job/Pod identities.
  Worker role `isolab-job` has the legacy-bucket managed policy; its inline-policy
  list was empty at the 07:27 UTC readback. The exact [additive scoped policy](../research/ml_dataset_20261028/worker-s3-policy-proposed.json)
  is prepared and unapplied, with the human approval still pending.
- **Observed:** pilot 001's original stderr was lost during cleanup. A
  same-image local input-read reproduction found unreadable input modes. That
  reproduction is not a retained runtime explanation for the original failure.
- **Observed:** source `f0863aefd6e1a416929f8acc02283d46f9c69936` has
  [software validation and artifact readback receipts](../research/ml_dataset_20261028/validation-f0863ae/validation.json).
  Fresh S3 GET/hash checks covered 44 validation references. Those transfers used
  the controller's credentials and contain software evidence, not verified
  native cloud dataset cells or proof of worker permission.
- **Observed:** merged [PR 2751](https://github.com/isogeny-labs/crypto/pull/2751)
  records a USD 200 total ceiling through its stated horizon, with conservative
  reservations of USD 188.3888. These are estimates, not verified billing.
  The proposed worker reservation uses us-east-1 rates; this EKS worker is in
  us-east-2. Prior pilot allocation/cost and a new actual worker shape remain
  unreconciled. The 07:52 UTC lab overview showed no active dataset job; an EC2
  query for pilot 002's former node private DNS returned no instance.
- **Missing capability:** campaign-wide durable admission/accounting for the
  proposed 20 GiB unique output, 100,000 PUT and 200,000 GET limits. Per-cell
  byte limits do not enforce shared cumulative limits or account for retries.
- **Not established:** exact worker failure cause from HTTP 403 alone, actual
  billed campaign spend, successful native publication/restoration, full solver
  coverage, trained-model efficacy or a first/last-fall-degree result. No
  mathematical obstruction follows from these infrastructure gaps.

## What would unblock it

- [ ] Obtain the human answer to the exact scoped shared-role proposal; if
  approved, apply only those List/Get/Put prefixes and verify IAM readback.
- [ ] Using actual worker credentials, verify a fresh-key conditional PUT,
  metadata read, fresh GET and complete SHA-256/length match in the approved
  dated prefix before starting scientific generation.
- [ ] Reconcile existing jobs and prior wave charges, admit the actual
  us-east-2 worker and original deadline under the shared USD 200 ceiling,
  and enforce cumulative storage/request reservations across retries/resumes.
- [ ] Run the frozen finite wave with a tested source, binary and digest-pinned
  image; retain each outcome and receipt, then perform fresh restore/public
  replay and the separately implemented native consumer's arithmetic checks.
- [ ] Demonstrate interruption/resume without duplicate cells, preserve
  incomplete outcomes, and publish a source-linked actual coverage report.

These conditions establish the first durable, admitted workflow. They do not
complete the enormous corpus, the full 720-cell standard export, missing native
backend/trace capabilities, model training or mathematical conjecture proofs.
Ordinary completed-cell resumption does not establish CRIU recovery.

## Next discriminating step

- **Action:** resolve the prepared permission decision and cumulative admission,
  then run the worker publication preflight before the already specified finite
  generation wave. [Readiness 003](../research/ml_dataset_20261028/pilot-wave-003-readiness.json)
  is a historical prepared record, not a submitted job or current-source claim.
- **Inputs and controls:** frozen pilot plan/protocol, exact approved bucket and
  prefixes, unique preflight object, digest-pinned tested binary, fresh downloaded
  hash check, and explicit terminal outcomes for denied access or mismatched data.
- **Execution limits:** existing pilot maximum 36 cells, 30 seconds/cell,
  2,048 MiB per worker, 64 MiB trace/cell, EKS 2 CPU/4 GiB/3,600 seconds and
  zero retries, subject to the reconciled campaign ceiling. This record creates
  no new allowance and does not authorize submission before the gates above.
- **Outcomes:** a permission/preflight failure retains evidence and starts no
  generation; a successful preflight permits only the admitted finite wave;
  replay/hash failure rejects publication; verified cells permit accounting and
  coverage review before any further allocation.
- **Recheck trigger:** human role-policy decision, new accounting enforcement,
  or a new source-bound worker preflight receipt.

## Resolution and history

Unresolved. Recorded 2026-10-28 after both failed pilots and the merged-source
software review. The publication code has scoped fixes and regression tests in
progress; neither fixes nor this record grant worker access. Preserve the failed
attempts and their original source/image identities.

## 2026-10-28 budget implementation and launch-adapter review

Source `23b3c0e527d621aebf6532db61744fab77867f48` implements native
cumulative admission and S3/STS attempt accounting, protected finalization,
actual-owner-bound grants and retained worker journals. Focused software tests,
strict lint, CLI and container preflight passed; final full-library validation
is pending. This closes the earlier absence of an accounting implementation,
but no real campaign ledger/grant or new worker wave has been issued.

The [launch-adapter contract](../docs/ml-dataset/LAUNCH-ADAPTER.md) records the
remaining integration: reserve startup before provisioning, gate work until
actual Job/Pod identities are reconciled, deliver the designated grant and retain
authority state, and bind actual hardware to an independently established finite
resource lifetime. Job deadline and tracked empty-node consolidation do not
establish an EC2 billing bound. Generic MCP run lacks the lower-level launcher's
instance-type option and has no grant startup handoff. These are concrete
implementation gaps, not a mathematical obstruction or a reason to relabel
planned cells as generated data.

Preflight also needs an additive canonical plan digest output for preparing
grants; its existing file digest is different. That correction is temporarily
waiting on the active T-755 file reservation. The operator guide explicitly
prohibits substituting the file hash. The pending human answer for the scoped
shared-role policy remains unresolved, and no IAM mutation has occurred.

Next: land the metadata correction when its scope is available; finish and
archive exact-source software checks; implement/review the finite launch
handshake; reconcile prior obligations and resource evidence; then perform
worker-role conditional PUT/GET verification and the frozen wave only after
its existing admission and permission gates are satisfied. Preserve the
earlier failed attempts and zero verified cloud cells.

### Completed software validation and coordination

The final unchanged-source23b3 release library suite completed with5,105 passed,
zero failed and123 ignored, exit0 observed. Source hashes still matched after
the run. The existing initial validation record remains unchanged; the final
result is retained separately in the source23b3 archive.

The human authorized coordination with T-755. Its owner could not be addressed
through the app's available task routing, so the exact minimal metadata request
was posted on the owning delivery PR: [coordination comment](https://github.com/isogeny-labs/crypto/pull/2790#issuecomment-6475281809).
No reserved source was changed. The worker IAM decision remains separate and
unanswered; no new cloud wave or real budget grant was issued.
