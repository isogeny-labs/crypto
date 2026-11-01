# Recovery of running legacy Spot processes

- **Status:** implemented and validated for the enrolled workload; laptop activation remains
- **Kind:** bounded process recovery and deployment
- **Affected work:** [Spot runtime](../infra/spot-system/README.md)
- **Last updated:** 2026-10-19

The five original factor-base processes predate native checkpoints. Their images
were exported to authorized private encrypted S3, and all five restored as copies
on a compatible separate m8i. They subsequently finished normally. Recurring
future memory exports, including inherited credentials, are now explicitly
authorized and enabled at 15-minute boundaries. A sixth running Diem generator
has its own registry/prefix/ownership key and a successful four-thread copy restore.

Automatic recovery is deployed. EC2-confirmed predecessor/source termination and
conditional DynamoDB ownership fence restoration; heartbeat expiry never permits
stealing. All unfinished groups share one replacement owner, and each must be
ready before guarded IP handoff. Kernel, libraries, CPU, archive integrity,
backing files, clocks and original deadlines are verified/enforced.

A disposable-source/first-owner/second-owner rehearsal passed automatic recovery,
new-owner encrypted backups and a second restore from the newer time-namespace
image. The actual source and IP were untouched. Raw failures and earlier manual
copy evidence remain retained; later success does not remove that history.

- [x] Explicit recurring sensitive-image authorization and verified private S3 export.
- [x] Compatible-host restore of all five original images and the Diem image.
- [x] Automatic ownership transfer after confirmed death; no duplicate live-owner restore.
- [x] Saved monotonic/boottime recovery across two hops; UTC deadlines retained.
- [x] Completion receipts prevent replay; expired deadlines produce failure receipts.
- [x] Applied guarded controller and worker services; source IP remains unchanged.
- [ ] Execute laptop installer and review/trust Codex hooks; no laptop access is available.

Evidence: [automatic failover](../infra/spot-system/validation/automatic-failover.json),
[Diem copy restore](../infra/spot-system/validation/diem-copy-restore.json),
[recurring backups](../infra/spot-system/validation/recurring-private-backups.json),
and [full requirement status](../infra/spot-system/VALIDATION.md).

This resolves the enrolled workload's automatic recovery gap. It does not establish
transparent recovery of arbitrary programs, CPU-family/kernel portability of RAM
images, exactly-once external side effects or partition-safe database failover.
New arbitrary processes require explicit enrollment and a tested restore contract.
