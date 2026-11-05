# Dataset custody transport: migrated-state handoff (2026-11-01)

This is a read-only reconciliation for Conductor T-1275. It records the exact
state needed to resume the larger dataset campaign without repeating completed
work or silently spending the historical balance. No raw archive, full-run
bundle, dataset cell, IAM policy, or Kubernetes Job was created or changed by
this handoff.

The approved migration handoff at
`/srv/experiments/agent-migration/handoff-receipt.json` was read through
scoped noninteractive access and has SHA-256
`a16037f61139810dcb13bea313349a2f9d4f1a43471c7ffc04ea3e1828dac0c1`.
Its workspace snapshot was reconciled before the live checks below.

## Retained local and remote evidence

| Artifact | Verified state |
| --- | --- |
| n47 compatibility archive | `/home/ec2-user/ml-dataset-local-compat-n47-multigen-20261101/raw-private.tar.zst`, SHA-256 `1af312699ac99df5fe462972a5fb5f14be3a7d5d44ce0a582da68a590f8cc875`, 20,632,534 bytes; 756 assigned cells, 161 verified known answers and 595 timeouts. |
| n61 compatibility archive | `/home/ec2-user/ml-dataset-local-compat-n61-multigen-20261101/raw-private.tar.zst`, SHA-256 `e16877578756b0b024112a640eae1c31cf9ab479f1a22515b95cafb4d2aece3e`, 21,936,868 bytes; 756 assigned cells, 170 verified known answers and 586 timeouts. |
| Sealed cohort summary | `/home/ec2-user/ecdlp/local-n47-n61-sealed-custody-20261101.json`, SHA-256 `182d9aae6605a6be6768dd8330bdd3afabfd2c53662c606bbca3844bfee4ba70`. The existing S3 CAS object `s3://setec-astronomy-artifacts/store/sha256/182d9aae6605a6be6768dd8330bdd3afabfd2c53662c606bbca3844bfee4ba70` has a 1,144-byte AES256 HEAD receipt. |
| n47 native full-run bundle | `/home/ec2-user/ecdlp/n47-full-custody-20261101/manifest.json`, SHA-256 `1dca633ab1ab5311110fd0a5b8233e07108164ff09675471f2b2fb9fa877a325`; 21,249 members, 163,668,716 member bytes, 4 MiB chunks. Creation replayed known answers; local verification checked member hashes. The matching S3 CAS manifest key returned HTTP 404 on November 1. |
| n47 independent restore | `/home/ec2-user/ecdlp/n47-full-custody-restore-receipt-20261101.json`, SHA-256 `f868c3433f22d638fc8fd73c70fdad637cc4ae0c78e45c0b284439a33d161eda`. The terminal successful native restore verified 21,249 members and replayed 756 assigned cells (161 known-answer verified, 595 timeout). Its existing S3 CAS object has a 708-byte AES256 HEAD; a fresh GET matched the local SHA-256 and bytes. |

The two restricted archives remain local. An S3 listing under
`ml-dataset/20261028/` found no keys containing `n47`, `n61`, or
`full-custody`. This listing is a namespace check, while the exact n47
manifest CAS HEAD above is the direct absence check. Neither result proves
that no differently named copy exists elsewhere in the account.

## Current admission state

The original shared USD 200 budget record in
[PR #2751](https://github.com/isogeny-labs/crypto/pull/2751) reserved USD
188.3888 and showed USD 11.6112 unallocated on October 28. Its horizon has
expired. The [October 31 admission audit](../ml_dataset_cloud_admission_audit_20261031/README.md)
found no current allocator policy, original ledger/authority marker, or
grant, and deliberately did not call `reserve-launch --dry-run` with invented
inputs. The November 1 scoped search found no newer authority in the
dataset research/custody paths. The old remainder is not launch authority.

[PR #3037](https://github.com/isogeny-labs/crypto/pull/3037) merged a
review-only worker policy and ConfigMap issuer proposal. A live November 1
IAM read found zero inline policies on `isolab-job`; its sole attached
`isolab-job-artifacts` v1 policy grants access only to the separate
`isolab-results-104186811093-us-east-2` bucket's `inputs/*` and `runs/*`.
It grants no `setec-astronomy-artifacts/ml-dataset/20261028/*` access.
The restored host's current AWS caller is the Spot experiments WorkerRole;
controller-side S3 readback does not prove worker-side Get/Put permission.

The Iso Lab overview on November 1 listed 14 existing Jobs: six failed,
seven succeeded, and one pending/suspended SAT qualification Job. It listed
zero dataset-named Jobs. No new Job was submitted. Research EKS access remains
limited to the existing submitter role and namespace.

## Exact next transport gate

Keep the original USD 200 shared ceiling and reconcile every prior obligation
and pending grant in the original durable ledger and authority marker. The
owner must supply or approve a current horizon, regional price/resource caps,
finite node billing lifetime, exact worker identity and scoped policy/issuer
review. Pin the current source, binary/image, complete protocol file and
native parsed-plan hashes, exact shard/cell IDs, Job name, and original
deadline. Only then run native `dataset budget reserve-launch --dry-run` on
those real inputs and inspect its `admitted` field; an admitted dry run is
still not a launch reservation or permission to submit a Job.

T-1257 is the separate source task for grant-bound full-run S3 transport.
Its implementation should charge each attempted request and byte against the
same cumulative native budget, use immutable content-addressed chunk and
manifest keys with conditional create, and retain ambiguous/failed writes.
After each successful write, compare a fresh HEAD and GET SHA-256 with the
local object. Save incomplete-run checkpoints before promotion, then restore
from remote objects into a fresh directory and run native member-hash and
known-answer replay checks. Bind the final manifest to its complete chunk
set, worker grant, Job/Pod UIDs, source and binary hashes. Do not promote a
summary receipt as a full-run remote bundle.

This handoff does not allocate money, apply IAM/RBAC, upload private data, or
establish a remote full-run replay. Its checks distinguish the completed local
custody and published small receipts from the still-unadmitted raw transport.
