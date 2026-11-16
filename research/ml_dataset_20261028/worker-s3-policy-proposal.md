# EKS worker access needed for the dataset campaign

Status: prepared, not applied. The failed pilot002 retained stderr reports a
403 from S3 HeadObject. Read-only inspection on2026-10-28 found that role
`arn:aws:iam::104186811093:role/isolab-job` has only the attached managed
policy `isolab-job-artifacts`, default version v1. Its read/write permissions
name only `isolab-results-104186811093-us-east-2`. The approved artifact bucket
has no bucket policy and uses SSE-S3 encryption.

The proposed additional inline policy in `worker-s3-policy-proposed.json`
allows object get/put and prefix-limited listing only under the new campaign's
`s3://setec-astronomy-artifacts/ml-dataset/20261028/` and dated lab receipt
prefixes. It grants no deletion, IAM, trust-policy, bucket-configuration or
access to other dataset prefixes. The existing managed policy is preserved.

After approval, the exact additive action is:

```sh
aws iam put-role-policy --role-name isolab-job \
  --policy-name ml-ecdlp-dataset-20261028 \
  --policy-document file://research/ml_dataset_20261028/worker-s3-policy-proposed.json
```

Read the policy back, compare canonical JSON, then test S3 head/put/get/hash
from the actual EKS workload. Local-role success and IAM simulation alone are
not end-to-end worker verification. The next immutable launch record must use
`--remote s3://setec-astronomy-artifacts/ml-dataset/20261028/store` and retain
the explicit dated receipt destination. Reconcile prior Job identities first.

The policy is removable by its unique inline-policy name after the campaign,
without touching existing permissions or deleting any artifacts. No automatic
removal is scheduled because future replay needs read access.
