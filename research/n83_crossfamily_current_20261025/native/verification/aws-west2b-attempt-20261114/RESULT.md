# N83 native west-2b launch attempts — 2026-11-14 UTC

## Result

The approved two-instance `g5g.4xlarge` Spot allocation reached the EC2
launch boundary after the source and presigned-transfer canary were stored as
versioned S3 objects. AWS rejected the corrected `RunInstances` request with
`Client.Blocked` for account verification. Both controller receipts have zero
instance IDs, and fresh `DescribeInstances` reads found no N83-tagged instance
in `us-west-2` or `us-east-1`. No factor-base measurement began; the frozen
selector's `selected_best_total_runtime` remains null for this panel.

The owner approved the amended `us-west-2b` route on 2026-11-14 for the same
single two-instance allocation: account `590183823895`, zone `us-west-2b`,
security group `sg-031c140ef9d078b3d`, maximum bid $0.31 per instance-hour,
21,600-second guards and a $4.15 projection under the $4.20 cap. The exact
read-only plan for the corrected source was recorded at 19:02:12 UTC, with
placement score 5, quote $0.290300 per instance-hour, and provider preflight
success. That preflight did not establish account launchability.

| Attempt | Exact source commit | Outcome | S3 source archive version | S3 manifest version | Canary version |
| --- | --- | --- | --- | --- | --- |
| 18:55 UTC | `4af4cd77fe53a7b4acf80621c3356354cad33d8e` | Installed AWS CLI rejected `--min-count` and `--max-count` before an EC2 API call; corrected to `--count 1`. | `Eee5nBu5WTAjpYbNhtkCyVm6ORmvu3GZ` | `GBir2PrWdl3O9rsEc_RHCvKpE3vdYph7` | `ogr4sbu1ZmBgOzxl4feRWpl94QS5onNK` |
| 19:02 UTC | `fc9e351d188a032f3b4d0e69e2111c5c47689bbd` | `RunInstances` returned `Blocked`; CloudTrail records `Client.Blocked` at 19:03:02 UTC, request `779e10bb-6481-4edd-bd5a-7149dffb80d4`. | `h1RPlzEHNCAKMx5wR2eRehIWlKrhKIf_` | `w8DbkR6LslPamxhO_0DBs0a7hTw_DCzj` | `X_1sn9S5QZarkz7c4tP.skgxJsuA97rZ` |

All listed versions are in bucket
`aburan28-n83-tier1-20261016-590183823895` (`us-east-1`). The first
attempt's key prefix is
`n83-highk-current/4af4cd77fe53a7b4acf80621c3356354cad33d8e/20261114T185556Ze137a47e/`;
the second's is
`n83-highk-current/fc9e351d188a032f3b4d0e69e2111c5c47689bbd/20261114T190254Zf9fe8aa0/`.
Object suffixes are `source.tar.gz`, `manifest.json` and
`control/presign-canary.txt`.

For the corrected attempt, CloudTrail records `Client.DryRunOperation` at
19:02:55 UTC, request `993f14db-b7cb-407a-9e96-1a420b959d4e`. The dry run
validated the launch argument shape and authorization, then the actual request
reported the account block. The controller's S3 source object has content
length 31,517,166 bytes and SHA-256
`2eee8dbf87b262e9b722e6d78702d41763d71a551ae6fe5e013a497947fdb4fc`;
the manifest object has content length 228,433 bytes and SHA-256
`5ba7093cc799b91b1cb453896ec4f72a4073baf750522b8eca85a83fc28187b7`.
Fresh version-specific S3 `HeadObject` reads matched these versions, sizes
and uploader-supplied metadata hashes. The canary version has 23 bytes.

The local raw receipts are retained under
`/Volumes/SSD990-2/tmp/n83-native-aws-20261114/allocation-west2b-4af4cd77/`
and
`/Volumes/SSD990-2/tmp/n83-native-aws-20261114/allocation-west2b-fc9e351d/`.
The first directory includes `attempt-diagnosis.json`; the second includes
`attempt-failure.json` and `provider-evidence.json`. The independent local
deletion guards remain armed for their original deadlines; their run IDs
are distinct and neither receipt contains an EC2 instance ID.

The next allocation gate requires AWS account verification to be resolved,
fresh source and provider checks, score at least 3, quote at most the $0.31
bid, the $4.20 cap, the exact approved security group, and confirmation that
no prior N83 pair is active. The separate `a=0` N83 solver panel retains its
own protocol and resource authorization boundary.
