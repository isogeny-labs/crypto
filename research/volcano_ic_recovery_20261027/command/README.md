# Exact-input command packaging

This directory prepares the original collector for the controller's command
mode. The public source instance, radius 12, 48 setup attempts, one-hour
scientific deadline and table representation stay byte-identical to the
October 27 protocol. These are build and development sources, not a deployed
image or a live restore receipt. [pcc:unverified]

## Resource admission

The original `runner plan` reports 244,140,625 left entries, 536,870,912 slots
and 9,566,497,092 payload bytes. This is a static layout calculation; it does
not allocate the table or measure resident memory. The previous controller
allowed only 4Gi and 8Gi, both below that payload. This change adds explicit
14Gi source admission, matching the pilot's previously recorded request and
leaving room above its 12GiB application cap. The default 4Gi and explicit
8Gi serialization and request identity are retained. [pcc:unverified]

No pool quota, instance type, four-CPU request, retry allowance, account budget
or deployed controller is changed. A future command request must explicitly
select `memory: "14Gi"`, use the original deadline and include the actual
image/binary digests. Do not submit 8Gi or reduce the radius to make it fit.

For CRIU storage, use a separately admitted 64Gi computation PVC: up to three
resident-memory images can coexist during generation commitment, before older
live-PVC generations are pruned. Three 14Gi envelopes leave 22Gi of that PVC
for outputs and other state. These are capacity bounds, not observed dump
sizes. The old 20GB development disk estimate does not cover this case. A
changed storage request and all retained snapshot costs need budget accounting
before execution; no storage is allocated here.

## Build context

After committing the image sources, from the repository root:

```sh
bash research/volcano_ic_recovery_20261027/command/prepare-context.sh /absolute/new/public-context
```

The helper exports only the enumerated public files with `git show HEAD:<path>`,
rejects changed frozen inputs/source or changed packaging, requires a new
directory outside the checkout, and records the actual source revision. It
does not read a private journal, credential file or process environment into
the context. Preparing a context does not establish remote publication or
authorize a build. Run the existing `check-experiment-git` publication gate
and current source-checkpoint gate before an IsoLab build/run.

The Dockerfile takes two mandatory immutable base references, `BUILD_IMAGE`
and `RUNTIME_IMAGE`, each specified with `@sha256:<digest>`. The build base
must supply C++20 g++, static libc/libstdc++ and coreutils; the runtime base
must supply bash and coreutils. It performs no package installation and accepts
no credential build arguments. Pin and retain both base digests, compiler,
final OCI digest and actual binary hashes; no base digest is invented here.

The image installs the committed source, static runner and separate verifier
under `/opt/volcano`. Its default command is `plan`, which does not allocate
the production table. The launcher rechecks the original source manifest and
the built binary manifest. Actual command mode uses:

```text
argv[0] = /opt/volcano/launch.sh
argv[1] = run
outputs = /work/compute/out
```

Bind the launcher's hash as `build.ecbench_sha256` and both native binary hashes
under `command.binaries`. This legacy field name does not turn the collector
into ecbench. Keep the original input/config in the image; do not mount witness
files or import a historical generic answer. The supervisor supplies
`ISOLAB_CRIU=1` and the canonical output directory. The launcher refuses an
ordinary local production run and previously used output. It retains native
failure status; on native success it calls the separate verifier and retains
its exit code/output. A controller terminal state still requires inspecting
these exit codes.

Raw draws and relations stay in the private output tree. The wrapper marks its
computation clocks invalid for benchmark admission; CRIU-spanning timing and a
raw collector timestamp are not an uninterrupted performance measurement.

## Development and remaining qualification

```sh
bash research/volcano_ic_recovery_20261027/test.sh /absolute/new/native-receipt
bash research/volcano_ic_recovery_20261027/command/test.sh /absolute/new/native-receipt /absolute/new/command-receipt
```

The command checks run the exact original static plan, then reject missing
supervisor admission, an outside-PVC path, changed input bytes, changed binary
bytes and reuse of the context directory. They never execute the original
full-width `run`. The native collector's small known-answer controls remain
separate from image execution and actual worker-loss restore.

Live command mode is still unqualified by the controller's own documentation.
Before the frozen pilot, publish the image built from tested committed source,
obtain working execution access and a known budget balance, qualify the actual
collector process tree and file descriptors with CRIU/EBS replacement, and
independently check the retained continuation. No default fixture, local
production run or fresh restart may stand in for that restore evidence.
