# Native factor-base checkpoints

The `factor_base_shape_ladder` example now produces and restores application
checkpoints when launched by the managed supervisor with
`checkpoint_contract: "manifest-sha256-v1"`. This covers its G1/G2/G3/G5
bounded-Macaulay measurements on the all-sparse path, including F5 row building.
It does not retrofit the five processes started from the older executable.

The state includes every sparse row, ordered pivot buckets, pivot flags,
the next column, elimination statistics, the monomial columns, completed
lower-degree profiles, and completed target profiles. Restore continues inside
the current elimination. Equal-weight pivot tie breaks keep their original
order. Equations, variable count, degree bounds and F5 mode bind the state to
the deterministic replayed target; the manifest also binds the job specification.

New runs replay their deterministic target enumeration. Completed target solves
come from the catalog; the active solve continues at its saved column. RNG state
is reconstructed by the unchanged seed/arguments and target enumeration, not a
memory image. Preserve the executable and arguments for a job. Changing the
specification requires a new job id. A different active target is an error.

Each generation contains `catalog.json` and, while eliminating, `matrix.bin`.
The matrix uses an explicit little-endian format with 32-bit column indices
and 64-bit sizes. Files are streamed through SHA-256, fsynced, and committed
by an atomic manifest rename followed by directory fsync. Three committed
generations are retained so discovery can fall back from a corrupt newer one.
Partial generations remain as failure evidence and cannot be restored.

The default interval is 300 seconds, configurable with
`SPOT_CHECKPOINT_SECONDS` (positive integer). The program writes an initial
matrix checkpoint and a completed-target checkpoint as well. SIGTERM sets an
allocation-free flag; the next complete-column boundary saves and exits. A
hard kill restores the latest committed generation. There is no fixed maximum
loss bound: a long column, matrix build, storage outage, or checkpoint write
can exceed the requested interval and the supervisor's 75-second grace period.
Matrix construction itself is not checkpointed. It may need rebuilding after
loss before its first commit. Dense finish is explicitly refused when native
checkpoints are requested; it has no resumable state implementation yet.

Checkpoint writes, restore and deterministic replay add overhead. Checkpointed
outcomes contain `checkpoint_enabled`; reused target outcomes also set
`checkpoint_replayed`. Appended stdout may repeat target/summary records after
replay. `score` deduplicates deterministic native target and summary records
within each input file and rejects conflicting mathematics/counters. Historical
logs retain their original scoring behavior. Neither replayed nor resumed wall times are fresh cold measurements;
do not promote them as performance results. The checkpoint catalog preserves
the mathematical profiles, not an exactly-once external output transaction.

The supervisor verifies up to 64 GiB of state with a 30-minute discovery
timeout. The background inventory runs 15 minutes after its previous run
finishes, avoiding a full matrix hash every minute. The CLI's smaller default
limit remains available for exploratory scans. Exceeding either budget refuses
admission; adjust explicit limits after inspecting the inventory. EFS storage
and throughput costs continue while compute is off.

## Deployment

Build a portable executable in a separate target directory. Keep the running
legacy binary untouched. Install the resulting file under a versioned path in
the managed workspace, with its digest recorded alongside it, then register
new work in `jobs.json`:

```json
[
  {
    "id": "shape-new-run",
    "cwd": "work",
    "checkpoint_contract": "manifest-sha256-v1",
    "argv": ["/srv/experiments/spot-worker/bin/IMMUTABLE_DIGEST/factor_base_shape_ladder",
             "g1", "--cell", "13:5:6", "--shapes", "random",
             "--instances", "2", "--targets", "2", "--seed", "20261015"]
  }
]
```

The manifest is an example, not permission to duplicate the running legacy
experiments. New worker jobs use their own `state/JOB/checkpoint` tree on EFS.
Other experiment programs need their own tested checkpoint integration; a
checkpoint declaration alone cannot protect arbitrary processes.

## CRIU assessment

On the original kernel `6.18.54-121.166.amzn2023.x86_64`, the packaged CRIU
3.17.1 passed a kernel check and dumped a disposable counter, but restore
failed with `sys_rseq ... -22`. CRIU 4.2.1, built from the official release,
successfully dumped and restored that same type of disposable process after
its original was killed. The restored counter advanced. This is a same-host
probe, not validation of the live Rust experiments or cross-family failover.

CRIU freezes processes during a dump even with `--leave-running`. It also
requires compatible kernel/CPU features and matching backing files; it does
not snapshot those files. After explicit approval for brief snapshot pauses,
all five original factor-base processes were dumped with `--leave-running`.
CRIU measured frozen intervals of 0.46–1.29 seconds; each source continued.
Images and copied backing files are root-private on the retained original EBS
volume. `criu_local.py` refreshes the explicitly registered PIDs via
`spot-legacy-checkpoint.timer`, 15 minutes after a run completes, retaining two
committed generations per PID. Start ticks and command hashes prevent PID reuse.
The producer never restores, restarts, terminates or exports a process.

The live-image restore probe remains incomplete: a private same-host PID/mount/
network namespace copy first exposed missing directory permissions (fixed by
preserving directory metadata), then stalled at CRIU's restore barrier. Only
the private probe namespaces were stopped; original processes remained running.
This is not evidence that the legacy images can yet recover on another host.
See [the specific remaining gap](../../problems/legacy-spot-process-recovery.md).

Automatic approval review rejected S3 export of full memory images because they
can contain inherited credentials. Explicit authorization for that destination
has been requested; no memory images have been uploaded or included in the
software release. Native checkpoints are preferred for future runs because
they serialize mathematical application state rather than process memory.

CRIU memory images can contain secrets. Keep them private, encrypted and
access controlled; never include arbitrary process images in the public Git
repository or source release. The disposable probe used an empty environment.

References: [CRIU incremental dumps](https://criu.org/Incremental_dumps),
[CRIU limitations](https://criu.org/What_cannot_be_checkpointed),
[official CRIU release](https://github.com/checkpoint-restore/criu/releases/tag/v4.2.1).
