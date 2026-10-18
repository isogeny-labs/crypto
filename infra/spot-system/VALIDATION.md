# Publication validation

This change adds native application-state checkpointing and deploys the managed
Spot runtime. It makes no scientific or performance claim. Public tiny-field
fixtures below validate durability and replay, not the research scorer's verdicts.
Full test/build/disposable-probe logs are preserved byte-for-byte in
`validation/raw-evidence.tar.gz`, with member hashes in `raw-evidence.json`.
The adjacent text files are summaries. The archive contains no live memory images.

| Requirement | Status and evidence |
| --- | --- |
| Publish complete deployable system to S3 | Uploaded; immutable object locations, SHA-256 and sizes in `artifacts.json` |
| Publish source to aburan28/crypto | This directory contains controllers, worker services, client tooling, tests and the staged template |
| Preserve private credentials and keys | Only explicitly allowlisted source/artifact files were packaged; no credential/key directories, PEM/key files or database state included |
| Standalone infrastructure tests | 48 passed, including local CRIU identity/integrity/path/fallback cases; `validation/system-tests.txt` |
| Native checkpoint discovery | 20 tests passed, including actual supervisor kill/restore with corrupt-generation fallback; `validation/checkpoint-tests.txt` |
| Live discovery | Read-only timer installed and run successfully; five workers identified without a checkpoint interface; `validation/current-host-discovery.json` |
| Python compilation and CloudFormation schema | Passed; exact source hashes/tool version in `validation/results.json` |
| Repository release library gate | `cargo test --release --lib`: 4,399 passed, 0 failed, 118 ignored; `validation/cargo-library-tests.txt` |
| AWS configuration | Applied; independent wake/idle lifecycle enabled, original IP protected; `validation/aws-deployment.json` |
| Native solver state | Exact pivot-column round-trips, actual SIGKILL/restart and corruption rejection passed; full library log |
| Touched example | 2 scoring replay tests passed; built-in selftest passed 104 checks; `validation/experiment-selftest.txt` |
| Exact packaged recovery gates | Generic and actual sparse-solver kill/restore gates passed on AMD; `validation/final-native-cold-boot.json` |
| Managed save/replay | Actual supervisor job wrote native generations, reused them on a second attempt, and scored one logical target; `validation/managed-native-save-replay.json` |
| Applied Spot quota | 96 vCPUs; the separate minimum-64 request can remain open; `validation/aws-deployment.json` |
| Managed worker launch/replacement | AMD and Intel cold starts passed; real AWS rebalance replaced a worker; activity evidence retained |
| Cross-worker native recovery | Disposable solver was SIGKILLed, its host terminated, and its automatic replacement restored column 1 and matched the reference; `validation/native-cross-host-restore.json` |
| Real idle shutdown | Replacement idle around 22:00, controller scaled to zero at 22:30:32 UTC; `validation/managed-rebalance-activities.json` |
| Existing process snapshots | Five leave-running dumps and scheduled local generations; `validation/live-criu-snapshots.json`, `local-criu-producer.json` |
| Actual legacy process restore | Unverified: private local namespace probes stalled; see `problems/legacy-spot-process-recovery.md` |
| S3 process-memory export | Not performed; automatic approval review requires explicit destination authorization because memory may contain credentials |
| Fixed IP and original host | IP still attached to the original host; no original process was restarted, terminated or migrated |

The S3 objects were checked for matching lengths, stored SHA-256 metadata and
AES256 server-side encryption. The deployment bundle's local whole-file hash
and allowlisted archive paths were independently checked before upload. A full
download from S3 matched its SHA-256 in `artifacts.json`.

The original quota-blocked record is preserved in
`validation/aws-deployment-before-quota-increase.json`. Initial remote-build
dependency failures and the sandbox-only socket failures remain as evidence.
The full suite passed after enabling local sockets. The later managed tests
also preserved PostgreSQL cluster identity across a real worker replacement.

The live discovery inventory is explicitly partial: the historical archive
exceeds the traversal limit, some same-user processes deny procfs access, and
the original host has no managed jobs manifest. Durable state and live working
directories are prioritized; this report is not evidence that every checkpoint
on the entire disk was examined. The five current factor-base workers still
require validated legacy restore and ownership handoff before replacement/
cutover; taking a dump is insufficient.
