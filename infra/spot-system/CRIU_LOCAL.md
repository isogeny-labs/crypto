# Local backups for the existing factor-base processes

The five original processes were started from an executable without native
checkpoint support. The user explicitly authorized brief CRIU snapshot pauses.
CRIU 4.2.1 produced leave-running dumps of all five; the first measured freezes
were 0.46–1.29 seconds. All sources continued. This does not yet establish a
working legacy restore or automatic legacy failover.

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
tracks compatible-worker validation, clocks, ownership and cutover.

Snapshots are on the original retained EBS volume, not on a replacement's
disposable root. A new host does not automatically attach this volume. Cross-AZ
recovery needs a disk snapshot/copy or explicitly authorized off-host export.
No memory image is in GitHub or the software deployment bundle.

Automatic approval review blocked S3 memory export because full process images
can include inherited credentials. Explicit permission for the private bucket
has been requested. Until resolved, the producer has no export code or enabled
export path. Do not treat general permission to upload software as permission
to publish process memory. Credentials and snapshots must never enter public Git.

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
