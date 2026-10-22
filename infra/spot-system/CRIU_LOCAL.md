# Private legacy memory backups and recovery

Recurring private S3 memory exports are authorized and enabled, including future
images that may contain inherited credentials. No process-memory archive is in
GitHub or the software release. The original instance remains running and holds
`3.134.144.201`; it is never terminated by the managed idle controller.

## Protected experiments and backup schedule

The five original `factor_base_shape_ladder` processes (6656, 6657, 6665, 6696,
6704) all finished normally. Their complete terminal summaries were exported;
their `legacy-owner` completion receipt remains intact. A sixth, still-running
`diem_quartic_study gen` process, PID 400334, has a separate registry, S3 prefix
and ownership key. Its in-memory generation work has no native checkpoint.

| Group | Private source registry | S3 prefix | Ownership key | Timer |
| --- | --- | --- | --- | --- |
| Original five | `/var/lib/spot-criu-managed` | `checkpoints/criu/` | `legacy-owner` | `spot-legacy-backup.timer` |
| Diem generator | `/var/lib/spot-criu-diem` | `checkpoints/criu/diem/` | `legacy-owner-diem` | `spot-diem-backup.timer` |

Both timers use `OnCalendar=*:0/15`, with persistent catch-up. Each oneshot first
takes a leave-running snapshot, then exports it. The old separate snapshot/export
timers are disabled to avoid racing producers. If a run exceeds an interval,
systemd does not overlap it. The nominal interval is 15 minutes; dump/upload time,
failed backups and capacity gaps can increase the recovery window. Brief snapshot
freezes were authorized; the initial five measured 0.46–1.29 seconds.

`criu_local.py` binds an explicitly registered executable, PID start time, command
digest and cwd. It refuses unsupported external descriptors. The frozen post-dump
hook copies executable/library/output backing files, cwd and required directory
metadata, and records monotonic/boottime offsets. A complete, fsynced SHA-256
manifest commits the generation. Two local generations per PID remain.

`criu_export.py` copies a verified generation under the producer lock, then
compresses/uploads outside that lock. It requires a root-owned, private grant
bound to the bucket and exact identities. Every archive passes full S3 download
SHA-256, size, metadata and AES256 checks before index/pointer commit. Discovery
retains three index candidates for fallback. S3 archives and old versions remain.

Completion requires the registered process to have departed and the expected
terminal records to exist: exact per-instance factor summaries, or a new Diem
JSONL file with 52 records. The latter is an infrastructure completion signal,
not verification of the mathematical contents. Outputs are included in the
private committed index; no old image is replayed after a verified completion.

## Automatic recovery and fencing

`legacy_recovery.py` runs as root on replacements. Each group has an independent
namespace; a shared host ownership key co-locates all unfinished groups. Takeover
requires EC2-confirmed **termination** of the previous owner, a confirmed original
termination, and a conditional DynamoDB write. A stopped instance, missing
heartbeat, expired timestamp or uncertain API response cannot authorize takeover.
The controller durably records confirmed origin/owner deaths with an owner-bound
conditional write, so a long Spot gap does not depend on an expired EC2 record.
A missing EC2 record alone never proves termination. The source refuses to acquire
its own work. No existing process is killed to make
a restore PID available.

Before restore, the daemon verifies the full selected batch, all hashes, safe tar
members, registered identity, exact kernel, system-library bytes and CRIU CPU
compatibility. Unsupported/mismatched states fail closed. It restores private
local backing files, runs CRIU in a saved time namespace, and checks the restored
process identity. Monotonic downtime is excluded; UTC launcher deadlines remain
enforced outside the time namespace. An expired original deadline creates a
private terminal failure receipt instead of restarting work with a fresh budget.
The Diem generator had no declared deadline, so none is invented.

A replacement writes per-group readiness only after restore, enables that group's
15-minute backup timer and publishes new generations. The IP controller requires
all unfinished groups to be owned and ready on the same healthy worker. While
any legacy image remains unfinished, new placement is restricted to m8i; the
native runtime opens the six-family Intel/AMD pool after completion. CRIU images
are not assumed portable across CPU families or kernel upgrades.

## Executed validation

All five original images restored as copies on a separate compatible m8i. A later
automatic rehearsal used a disposable logical source, two successive recovery
workers and separate S3/DynamoDB namespaces. Each successor waited while its
predecessor was alive, restored automatically only after termination, advanced
CPU work, and produced a new encrypted backup. The second worker restored that
newer time-namespace image. All rehearsal hosts were then terminated. The original
source was untouched throughout.

A copy of the new Diem snapshot also restored on the idle managed standby: four
threads, CPU ticks 4 → 1506 over five seconds. Only that copy was stopped. See
[failover evidence](validation/automatic-failover.json),
[Diem copy restore](validation/diem-copy-restore.json), and
[latest backup receipts](validation/recurring-private-backups.json).

For checks on the source (metadata only):

```bash
sudo systemctl list-timers spot-legacy-backup.timer spot-diem-backup.timer
sudo python3 /opt/spot-criu/criu_local.py list --root /var/lib/spot-criu-managed
sudo python3 /opt/spot-criu-diem/criu_local.py list --root /var/lib/spot-criu-diem
```

New arbitrary processes require explicit enrollment and a tested restore contract;
this is not transparent recovery for every program, external socket, GPU or side
effect. New supported solver runs should use the native durable checkpoint
interface. File discovery alone never proves application recoverability.

## Tool and retained failures

CRIU 4.2.1 is pinned in `artifacts.json`; its software/source are public upstream
artifacts, not memory images. AL2023's CRIU 3.17.1 failed the disposable restore
with `sys_rseq -22`. Initial directory-permission and private namespace restore
failures remain in the earlier evidence archives; later direct compatible-host
restores and time-namespace failover passed. This is generic process/storage
orchestration; application algorithms/checkpoint serialization remain native Rust.
