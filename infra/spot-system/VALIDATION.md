# Publication validation

This imports the already built AWS infrastructure system. It adds no
cryptographic implementation, scientific result or performance claim.

| Requirement | Status and evidence |
| --- | --- |
| Publish complete deployable system to S3 | Uploaded; immutable object locations, SHA-256 and sizes in `artifacts.json` |
| Publish source to aburan28/crypto | This directory contains controllers, worker services, client tooling, tests and the staged template |
| Preserve private credentials and keys | Only explicitly allowlisted source/artifact files were packaged; no credential/key directories, PEM/key files or database state included |
| Standalone infrastructure tests | 42 passed; `validation/system-tests.txt` |
| Native checkpoint discovery | 20 tests passed, including actual supervisor kill/restore with corrupt-generation fallback; `validation/checkpoint-tests.txt` |
| Live discovery | Read-only timer installed and run successfully; five workers identified without a checkpoint interface; `validation/current-host-discovery.json` |
| Python compilation and CloudFormation schema | Passed; exact source hashes/tool version in `validation/results.json` |
| Repository release library gate | `cargo test --release --lib`: 4,396 passed, 0 failed, 118 ignored; `validation/cargo-library-summary.txt` |
| AWS configuration | Applied; independent wake/idle lifecycle enabled, original IP protected; `validation/aws-deployment.json` |
| Exact packaged bootstrap recovery gate | Passed on mounted EFS on the original host; `validation/efs-bootstrap-selftest.txt` |
| Managed worker launch | Wake accepted; EC2 launch rejected by Standard Spot quota; cold boot remains unverified |
| Active experiment recovery and automatic cutover | Remain pending per `README.md`; original experiments remain running |

The S3 objects were checked for matching lengths, stored SHA-256 metadata and
AES256 server-side encryption. The deployment bundle's local whole-file hash
and allowlisted archive paths were independently checked before upload. A full
download from S3 matched its SHA-256 in `artifacts.json`.

The deployment's earlier live tests included a real EFS mount, checkpoint
recovery tests using EFS, IAM-invoked staged wake/release, a real DynamoDB
activity report and node service restart with PostgreSQL cluster identity
preserved. Those tests do not establish a successful new-worker cold boot or
live experiment failover. No live idle termination was performed.

The live discovery inventory is explicitly partial: the historical archive
exceeds the traversal limit, some same-user processes deny procfs access, and
the original host has no managed jobs manifest. Durable state and live working
directories are prioritized; this report is not evidence that every checkpoint
on the entire disk was examined. The five current factor-base workers still
require application checkpoint support before any replacement/cutover.
