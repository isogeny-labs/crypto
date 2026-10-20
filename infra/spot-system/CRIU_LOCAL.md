# Local backups for the existing factor-base processes

The five original processes were started from an executable without native
checkpoint support. The user explicitly authorized brief CRIU snapshot pauses.
CRIU 4.2.1 produced leave-running dumps of all five; the first measured freezes
were 0.46–1.29 seconds. All sources continued. This does not yet establish
automatic legacy failover. On 2026-10-19, copies of all five images restored on
a separate `m8i.8xlarge` with matching kernel and library hashes. Both threads
of each copy ran and its CPU ticks advanced during a 10-second check; only the
copies were stopped. See [the cross-host evidence](validation/legacy-criu-cross-host-restore.json).

`criu_local.py` is a generic infrastructure producer for those explicitly
registered PIDs. It checks the executable, process start ticks, command digest
and cwd before each snapshot. It supports the known regular stdout/stderr files
and mapped executable/system libraries; other external descriptors are refused.
It never reads process environment values through procfs and never starts,
restarts, restores or terminates an experiment.

The `post-dump` action copies backing files while the process is still frozen.
It preserves directory mode/ownership as well as file bytes and metadata. State
files are fsynced and checksummed before committing `manifest.json`. The root
directory and metadata are private; two committed generations per PID remain.
Partial attempts are retained as failure evidence. The 15-minute timer starts
its next interval after a run finishes; dump, hash and fsync time add to that
interval. Larger states and storage failures can increase the lost-work window.

On the original host:

```bash
sudo python3 /opt/spot-criu/criu_local.py list --root /var/lib/spot-criu-managed
sudo python3 /opt/spot-criu/criu_local.py discover \
  --root /var/lib/spot-criu-managed --pid 6656
sudo systemctl status spot-legacy-checkpoint.timer
```

Discovery checks each candidate's complete file inventory, SHA-256/size, safe
paths and registered process identity, selecting the newest valid generation
with fallback. It reports `application_restore_verified: false`: file integrity
alone is not a successful process restore. The first local restore probe exposed
directory-permission mismatches, which the producer now fixes. Later private
PID/mount/network-namespace probes stalled at CRIU's restoration barrier. Their
namespaces were cleaned up; original processes were never stopped or replaced.
[The remaining recovery obligation](../../problems/legacy-spot-process-recovery.md)
tracks clocks, ownership, automatic recovery and cutover. Subsequent direct
restores on the separate compatible worker passed; the earlier namespace
failures remain recorded and their cause is not established.

Local snapshots remain on the original retained EBS volume. A new host does not
automatically attach that volume. The user explicitly authorized export of the
five current memory images, including possible inherited credentials, to the
private bucket `spot-experiments-runtimebucket-g8acwaz3kewm`. All five archives
were uploaded under `checkpoints/criu/20261019T011736Z-7bad660db8bb/`, with AES256
server-side encryption, SHA-256 metadata and full download/hash verification.
The 6,923,934,633-byte batch commits its index after verifying every archive.
`checkpoints/criu/latest.json` points to the verified index. See
[the export receipt](validation/private-criu-export.json).

`criu_export.py` takes a private copy of the newest fully verified generations
under the producer lock, then releases that lock before compression and upload.
It requires a root-owned private authorization file bound to the bucket and
registered process identities. A one-time grant also binds exact generation
names; future generations require separate recurring authorization. An index
and discovery pointer are published only after every archive passes encryption,
size, metadata and full-download SHA-256 checks. Failed staging stays private
for diagnosis. Completed staging is removed; S3 archives are retained.

Automatic approval review initially blocked memory export; the user's explicit
grant resolved the one-time export. Review separately rejected enabling future
scheduled exports because it interpreted that grant as covering only the five
current images. Recurring permission has been requested. The export service
and timer are provided for review but are not installed or enabled. The existing
15-minute local snapshot timer continues. No memory image is in GitHub or the
software deployment bundle; credentials and snapshots must never enter public Git.

## Reproduce installation

Use the pinned CRIU 4.2.1 tool/source from the software artifact manifest (or
the [official release](https://github.com/checkpoint-restore/criu/releases/tag/v4.2.1)).
AL2023's packaged 3.17.1 failed the disposable restore with `sys_rseq -22`.
Build dependencies used for 4.2.1 were `gcc`, `make`, `libnet-devel`,
`libnl3-devel`, `libcap-devel`, `libuuid-devel`, `protobuf-c-devel`,
`protobuf-c-compiler`, `protobuf-devel`, `protobuf-compiler`, `python3-protobuf`
and `libaio-devel`; build with `make -j4`. The source artifact includes the
upstream license. The tool is dynamically linked against AL2023 libraries.

Install the reviewed producer and tool root-owned under `/opt/spot-criu`, and
the two `spot-legacy-checkpoint.*` units under `/etc/systemd/system`. Register
only the currently authorized PIDs into a new private root, take the first
snapshot and verify it, then enable the timer. Registration is explicit and
cannot overwrite an existing registry. The original five-PID deployment is
already installed; do not register a duplicate producer for it.

This producer is infrastructure orchestration and file integrity checking. The
actual mathematical checkpoint implementation and tests remain native Rust.
