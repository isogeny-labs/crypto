# ECDLP dataset worker publication and cumulative launch admission

- **Status:** open
- **Kind:** access limit; resource admission gap; implementation gap
- **Affected work:** [dataset design](../docs/ml-dataset/DESIGN.md), [operating guide](../docs/ml-dataset/OPERATIONS.md), [coverage obligations](../docs/ml-dataset/REMAINING-COVERAGE.md), [PR 2756](https://github.com/isogeny-labs/crypto/pull/2756)
- **Next action owner:** dataset producer task T-684; scoped obstruction record T-713; human approval is required for the prepared shared-role grant
- **Related implementation:** T-716, [partial artifact capture and cumulative S3 admission](../research/ml_dataset_live_capture_20261028/DESIGN.md)
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
- **Implemented, deployment unverified:** source
  `c87174cddc02a47da0fdac7312e955db599bd1b2` adds disjoint campaign allocations,
  cumulative per-command S3 reservations, and bounded partial artifact
  snapshots. Failed requests stay charged; terminal snapshots have a reserved
  allowance. The [source checkpoint](../research/ml_dataset_live_capture_20261028/source-checkpoint.json)
  passed fresh S3 readback and Git bundle verification. Later persistence runtime
  `7d2aad8baa5fa5851e3f2ac16b6024cd3999210a` passed 5,130 full-library tests,
  zero failures and 123 ignored, including the controller-lock contention fix.
  Policy runtime `d71ec2ef5ab9b7f2a038f61c941af10297a775ad` passed 5,169
  full-library tests, zero failures and 123 ignored. Their source-bound
  [validation archives](../research/ml_dataset_policies_20261028/VALIDATION.md)
  were freshly downloaded and verified. No actual worker publication is
  established by these software tests.
- **Missing deployment capability:** that quota authority requires durable
  mounted storage shared with replacement hosts and protected from rollback
  with worker snapshots. Ordinary lab `/work` storage is temporary. This change
  installs no mount and does not provide an S3 conditional-update authority.
  The proposed 20 GiB/100,000 PUT/200,000 GET campaign limits have not been
  deployed. Object-body/request caps are not account-wide dollar accounting.
- **Observed after migration, 14:40–14:42 UTC:** the registered MCP kubeconfig
  still provides the existing job-submitter route. Permitted reads found zero
  current Pods and five retained Jobs, all with `emptyDir` volumes. Pilot002
  remains failed with UID `641ed2d6-ce1b-4e86-a113-3874ccf176c7`. Storage reads
  for PVCs, PVs, StorageClasses and CSI drivers are explicitly unauthorized.
  No eligible existing mount was identified; this does not establish that none
  exists. The separate stale kubeconfig's denied launcher-role assumption does
  not invalidate the working scoped route. No role or access was changed.
- **Observed source boundary:** ordinary submission exposes temporary storage;
  the recovery controller's gp3 `/work` is restored with the worker snapshot
  and cannot be its independent quota authority. Workspace EFS alone is not
  proof of an EKS mount. A mounted filesystem is the current implementation's
  dependency; the user requirement is durable cumulative admission. Any new
  backend must first preserve that contract under duplicates, ambiguous
  requests, interruption and stale restoration. An S3 create-once claim alone
  does not bound failed claim requests from duplicate worker starts.
- **Durable operational evidence:** the filtered EKS receipts, authorization
  responses, source assessment and parent checkout-failure diagnosis are at
  `s3://setec-astronomy-artifacts/ml-dataset/20261028/operations/6c410d7c724bab6d13668fa1506707b0eb4bc4b36b57fd53e268ada2707fe312/operations.tar.gz`,
  19,567 bytes, SHA-256
  `6c410d7c724bab6d13668fa1506707b0eb4bc4b36b57fd53e268ada2707fe312`.
  Conditional PUT, fresh ETag-bound GET, byte comparison and contained hashes
  passed at `2026-10-28T14:51:12Z`. This archive records an operational
  assessment, not a successful cloud dataset.
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
- [ ] Mount and verify the same durable quota authority for original and
  replacement workers; demonstrate that stale/missing journals cannot reopen
  spent allowance, while failed or interrupted requests remain charged.
- [ ] Run the frozen finite wave with a tested source, binary and digest-pinned
  image; retain each outcome and receipt, then perform fresh restore/public
  replay and the separately implemented native consumer's arithmetic checks.
- [ ] Demonstrate interruption/resume without duplicate cells, preserve
  incomplete outcomes, and publish a source-linked actual coverage report.

These conditions establish the first durable, admitted workflow. They do not
complete the enormous corpus, the full 720-cell standard export, missing native
backend/trace capabilities, model training or mathematical conjecture proofs.
Ordinary completed-cell resumption does not establish CRIU recovery.
Partial artifact restoration also does not resume opaque solver state or turn
an unfinished attempt into a completed cell.

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
software review; updated after migration, completed source validation and
read-only EKS reconciliation. The publication code's scoped fixes and regression
tests passed locally; neither those passes nor this record grant worker access.
The separate consumer companion audited 72 sealed software fixtures and checked
22 positive certificates, with its own non-equivalent static training view;
it has not validated an actual restored worker dataset. Preserve failed attempts
and their original source/image identities. No new job, E12 retry, administrative
grant, budget increase or deadline extension was made.
