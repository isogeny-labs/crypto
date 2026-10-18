# Publication validation

This imports the already built AWS infrastructure system. It adds no
cryptographic implementation, scientific result or performance claim.

| Requirement | Status and evidence |
| --- | --- |
| Publish complete deployable system to S3 | Uploaded; immutable object locations, SHA-256 and sizes in `artifacts.json` |
| Publish source to aburan28/crypto | This directory contains controllers, worker services, client tooling, tests and the staged template |
| Preserve private credentials and keys | Only explicitly allowlisted source/artifact files were packaged; no credential/key directories, PEM/key files or database state included |
| Standalone infrastructure tests | 37 passed; `validation/system-tests.txt` |
| Python compilation and CloudFormation schema | Passed; exact source hashes/tool version in `validation/results.json` |
| Repository release library gate | `cargo test --release --lib`: 4,396 passed, 0 failed, 118 ignored; `validation/cargo-library-summary.txt` |
| Active experiment recovery and automatic cutover | Remain pending per `README.md`; publication changes no running experiments or live scaling flags |

The S3 objects were checked for matching lengths, stored SHA-256 metadata and
AES256 server-side encryption. The deployment bundle's local whole-file hash
and allowlisted archive paths were independently checked before upload. A full
download from S3 matched its SHA-256 in `artifacts.json`.

The deployment's earlier live tests included a real EFS mount, checkpoint
recovery tests using EFS, IAM-invoked staged wake/release, a real DynamoDB
activity report and node service restart with PostgreSQL cluster identity
preserved. Those tests do not establish a successful new-worker cold boot or
live experiment failover. No live idle termination was performed.
