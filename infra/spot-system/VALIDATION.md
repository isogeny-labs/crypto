# Validation and requirement status

This infrastructure change makes no research, mathematical or performance claim.
The original evidence archives remain immutable. `raw-evidence.tar.gz` and
`private-export-evidence.tar.gz` preserve earlier tests and failures. The later
`failover-evidence.tar.gz` preserves full validation logs and exact executed
metadata/probe scripts; none contains process-memory images or credentials.

| Requirement | Observed status and evidence |
| --- | --- |
| Spot quota at least 64 vCPUs | Verified: applied 96; `aws-deployment.json` |
| Spot-only similar capacity | Deployed 32 vCPU/128 GiB, price-capacity optimized, three AZs; temporarily m8i while legacy CRIU work remains |
| Preserve running experiments | Original source/IP remain; five factor jobs finished naturally, Diem PID 400334 continues with leave-running backups |
| Recurring private S3 backups | Enabled at :00/:15/:30/:45, including authorized future sensitive images; full download SHA-256 and AES256 checks; `recurring-private-backups.json` |
| Checkpoint discovery | Native inventory/generation validation, corrupt fallback and supervisor restore; 20 native tests; earlier live inventory is explicitly partial |
| Harness-independent restart | Deployed `experiment.run/v1`, JSON Schema and `experiment-runner`; native kill/restore under different synthetic launcher sessions; actual worker-IAM resume restored progress 3 and finished at 12; `run-records.json` |
| Native application checkpoints | Actual solver SIGKILL/restore, cross-worker reference equality, AMD gate and managed replay; earlier native evidence retained |
| Legacy copy restore | All five factor copies passed on compatible m8i; Diem four-thread copy also passed; `legacy-criu-cross-host-restore.json`, `diem-copy-restore.json` |
| Automatic legacy failover | Passed disposable-source/first-owner/second-owner rehearsal, including newer backups and time namespace recovery; `automatic-failover.json` |
| Fenced ownership | EC2 termination plus conditional owner transfer; no timeout-based stealing; all unfinished groups must share ready owner before IP |
| Fixed IP | Deployed/armed with source guard; live response protects original; no forced original-host handoff was attempted |
| Cairn/Conductor | Deployed on source and managed workspace; health/restart identity checks and managed cold boot passed |
| Idle shutdown | Real managed worker scaled to zero after 30 minutes; earlier activity record retained; original protected host excluded |
| Laptop hooks and SSH wake | Installer/tests ready and AWS invocation/public-key checks passed; actual laptop installation/event remains blocked by missing laptop access |
| Infrastructure tests | 83 passed; compilation and cfn-lint passed; `results.json`, full logs in `run-record-evidence.tar.gz` |
| Repository release library gate | 4,399 passed, 0 failed, 118 ignored, 168.66 s for portable-record change; full log in `run-record-evidence.tar.gz` |
| Touched example gate | 2 tests passed; prior built-in 104 checks retained |
| S3/GitHub publication | Content-addressed private release and source PR; `artifacts.json` |

## Recovery rehearsal scope

Only disposable validation hosts were terminated. The original running source
and real experiments were never killed/restarted. A separate test prefix and
DynamoDB key modeled predecessor death. The first recovery waited for source
termination, restored a real coordinate image, then made its own encrypted
backup. The second waited for the first's termination and restored its newer
image. Thread health and CPU progress passed after both hops, and elapsed
monotonic time excluded the downtime. All test hosts were cleaned up.

This proves the exercised legacy recovery path, not transparent migration of
arbitrary processes, arbitrary CPU/kernel portability, exactly-once external
side effects, or a real original-source failure that has not occurred. Source
completion after its most recent checkpoint but before an abrupt loss may be
recomputed from that checkpoint; completed verified receipts are never replayed.

The new Diem namespace uses the same fenced state machine and separate production
backup contract. Its copy restore passed on the managed standby after verifying
kernel, library and image integrity. Its terminal record count is an infrastructure
completion signal; no research output was independently verified here.

## Retained failures and limits

Earlier quota, dependency, sandbox-socket, CRIU 3.17.1 and namespace restore
failures remain in frozen archives. Later successful tests do not erase them.
The live native inventory cannot exhaust the entire historical archive and some
procfs entries are inaccessible. Discovery is partial, not universal protection.
Only explicitly enrolled and restore-tested legacy processes have memory recovery;
future managed workloads must implement/test a native checkpoint contract.

Laptop execution access is the remaining activation gap. The package merges
existing client settings and configures strict SSH verification. Codex's `/hooks`
trust step is deliberately left to the user. No client trust store is bypassed.

## Follow-up platform compatibility

The previous PR #2232 had already merged. A subsequent release check exposed an
unconditional Unix import and file-mode setter in the checkpoint module. The
follow-up guards both with `cfg(unix)`, retaining Linux mode 0600. The release
library, example and 72 infrastructure tests passed again. The deployed Linux
native binary remains the pinned, previously validated artifact; no algorithm
or serialized state format changed. Full Windows cross-checking on this host
is blocked by the absent MinGW C compiler needed for bundled SQLite; Windows
release CI remains a merge gate, not a claimed local pass. Exact failures and
new test logs are retained in `portability-evidence.tar.gz` and its JSON receipt.

## Portable execution records

Managed runs atomically emit private `state/JOB/run.json` records. The stable CLI
validates durable identity and current checkpoints before admitting a resume-only
request; missing state cannot silently start a fresh run. It reconstructs a
missing jobs manifest and lets the background service own execution. Region,
admission-table and managed-wake settings persist with the workspace, so the
resuming agent does not need the original launcher's environment.

The deployed role invoked managed wake and resumed a disposable native worker
after its process was killed and its original job list removed. Progress resumed
at 3 and completed at 12. The native integration suite separately exercised
fallback from corrupt newer generations under changed synthetic Claude/Codex
environment values. These are CLI and process recovery proofs; interactive
Claude/Codex/OpenCode applications were not automated. No inherited session or
secret values were serialized. Existing saved-run descriptors were backfilled
without launching them; the original source process and fixed IP were preserved.

The new archive retains the inline-template size rejection, an SSM test-script
permission failure, and a test against a replacement's older runtime, followed
by corrected successful deployment/test evidence. The replacement was upgraded
only after confirming its main job queue was empty. See
[`RUN_RECORDS.md`](RUN_RECORDS.md) for the format, commands and compatibility limits.
