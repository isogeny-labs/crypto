# Prospective private-container backstop qualification

State: REVIEW REQUIRED. No build, Docker command, binary execution, or watchdog invocation has been performed by this task. The native fixture and unchanged source are proposal files only. Scope is the local mechanism, not a production runtime image or scientific run.

## Exact source and invocation boundary

Copied controller src/deadline.rs SHA256 b7eaf3b0249a7bb630c9a486eb613c48e3ec779c486796c9634c2fdfc61644cb unchanged. The module says never invoke its namespace helper in host/unit-test processes. This proposal preserves that boundary: fixture invocation is permitted only as root PID1 inside the reviewed new Docker private PID namespace. The helper also requires its exact controller assertion string; the fixture adds a separate explicit qualification environment marker. No test module is invoked and no host fixture command appears in the procedure.

The source samples a single monotonic admission anchor before UTC; watchdog checks retain that Deadline, sleep10ms, signal kill(-1,SIGKILL), wait100ms and exit125. This qualification exercises the exact anchor/cutoff path under ordinary clocks. It does not change host/container clocks and does not experimentally establish backwards-clock behavior; that behavior is covered separately by source/pure fixtures. Linux excludes caller/PID1 from kill(-1); child termination must be observed before PID1 exit to distinguish this mechanism from namespace teardown.

## Proposed bounded software preparation after root review

1. Reconcile concurrent native builds and exact installed musl toolchain/cache. Create a new private source snapshot/target directory. Generate/record a private offline lockfile for the exact pinned dependencies, using existing cache only, then build locked/offline with Rust1.99.1, x86_64-unknown-linux-musl, release, two jobs, external120s cap. No installs. Retain any lock/build failure. Never run resulting binary on host, including --help.
2. Verify ELF interpreter/NEEDED absence using readelf, not execution. Rehash unchanged deadline source. Copy only the exact fixture binary and this scratch Dockerfile into a fresh two-file context. Record source, lockfile, executable and Dockerfile hashes.
3. Build a unique local image from scratch with network disabled; no pulls, RUN steps or credentials. Cap administrative build time120s. Image creation is local qualification only, not image publication or production assembly.

## Container creation and inspection, before start

Create exactly one uniquely named container with a unique ownership label and recorded image ID. Require `--network none --read-only --cap-drop ALL --security-opt no-new-privileges --memory 64m --memory-swap 64m --cpus 0.25 --pids-limit 16 --user 0:0 --env ISOLAB_PRIVATE_QUALIFICATION=reviewed-container-only/v1`, bounded local log rotation (1MiB, one file), stop timeout1s. No `--init`, mounts, volumes, devices, privileged flag, PID/IPC/UTS sharing, host networking, restart policy, or Docker socket access. No host filesystem is exposed to the fixture. Use a literal dedicated name; do not derive deletion targets from untrusted output.

Inspect the CREATED container before start and fail closed unless: HostConfig.PidMode empty/default-private (never host/container), IpcMode private, UTSMode empty, Privileged false, NetworkMode none, ReadonlyRootfs true, all capabilities dropped/no adds, no devices/mounts/binds/volumes, no-new-privileges set, init false/null, restart policy no, exact resource caps, exact image ID, expected entrypoint/user/environment/ownership label. Parent host/container PID namespaces must not be shared. Docker daemon isolation is a trusted boundary; fixture PID1/euid checks alone do not prove it.

## One bounded observation and exact-owner cleanup

Start only that inspected container. Record monotonic host start time. External wait cap10s covers the approximately2–3s cutoff and shutdown tail; on wait timeout, classify incomplete, inspect/log, kill only the exact owned container ID, then bounded wait. Every Docker CLI call has its own external timeout. Never invoke kill(-1) from orchestration. No concurrent second fixture, no retries without a separate review of failure.

Before removing anything, preserve inspect output, logs, wait result and image/container IDs. Expected normal observed result: installed marker; detached child PID>1; child explicitly reaped with signal9 before exit; container ExitCode125, OOMKilledfalse, no wait timeout. The detached child creates its own session via setsid, while PID1's main thread blocks in pause. The observer does not enforce the cutoff. Missing child-reap observation is INCOMPLETE even if exit125; it must not be inferred from empty container state. Record exit time as operational observation, not a comparative benchmark.

Cleanup only the exact labelled container after verifying ID/name/image/ownership label. Bound cleanup30s; record terminal state and successful removal or concrete retained-resource failure. Do not prune Docker or touch any prior image/container. Preserve the new image unless root authorizes removing that exact ID; no background containers may remain unreported.

## Pass scope and remaining dependencies

A pass supports this unchanged helper stopping an owned detached child and blocked-main PID1 within one inspected private container under its captured cutoff. Production caller provenance, Kubernetes admission/custody, CRIU interactions, image packaging and physical infrastructure deallocation are separate. A source hash mismatch, missing toolchain, guard failure, missing reap, OOM, timeout or cleanup failure is retained with its exact result. No scientific trial, cloud job, IAM change, runtime publication or campaign budget extension is part of this plan.
