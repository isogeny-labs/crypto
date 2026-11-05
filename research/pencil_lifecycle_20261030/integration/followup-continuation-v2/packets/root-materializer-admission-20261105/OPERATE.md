# Exact materializer preparation admission, pending command/config review

T-1656 binds materializer-v2 source64afb6b895 and independent source review0977b50f8e. preparation.json contains concrete unique paths, pinned revision, existing input-manifest hashes, candidate/tool/cache paths, current lease identity, source publication commit and campaign balance. Source-reviewed=true refers to the independent source delta and six mock controls; prospective launch/config review and parent release remain required. No actual invocation occurred.

The exact launch argv and properties are in launch.json. The materializer worker retains its880-second work limit. Unit RuntimeMax885s plus5s stop grace, payload GNU timeout895s plus5s kill grace and the parent's900-second total supervision bound are distinct. One payload only,2CPU/4GiB/no swap/512tasks. No dollar values are invented. Budget.json reserves900 preparation +1800 existing full-library validation from43200 remaining, leaving40500; earlier1200 and current norm2400 reservation remain charged.

## Before launching

1. Parent confirms current norm payload is terminal with owned cleanup, exact config/command/source/review commits are published on GitHub, the prospective command/config review passes, and the T1656 lease still matches. Renew the same lease; if it expired, do not fill a new identity silently—publish the exact renewed admission before execution. Copy publication readback and prospective review receipt into durable control records.
2. Recheck unit LoadState=not-found and workspace/payload output/control directory absence. Create only the private control directory `/home/ec2-user/pencil-validation-receipts/t1656-materializer-20261105-1800` (mode0700); materializer itself creates its worker subdirectory and workspace. Refuse existing attempt output. Store parent monotonic start, absolute deadline, boot ID, launch argv, source/config hashes and current budget before launch.
3. Invoke exactly launch.json argv once through the parent's existing command interface. No new supervisor program is introduced. Capture command exit plus stdout/stderr; systemd appends native output directly into the durable control directory. Launch-client exit0 is not payload success.

## Existing ownership/termination procedure

Use the existing nonbranch-replay-continuation-v3 launch.py procedure, whose hash is bound in procedure-source-bindings.json, as the operational source. Within five seconds of launch, record live InvocationID, ControlGroup and MainPID with systemctl show, boot ID, and /sys/fs/cgroup/<captured group>/cgroup.procs/cgroup.events. Save live CPU/memory/swap/tasks readback. The native worker independently checks its cgroup identity and limits; record its receipt path. Observe within the same900-second total budget.

Only the exact captured invocation can receive systemctl stop --no-block. A different active invocation or missing initial identity leaves custody unresolved and stops further work. Never stop a unit merely because its name matches. The captured cgroup must become empty or disappear; a process-client exit alone is insufficient.

If manager garbage collection clears the terminal InvocationID, preserve privileged journal JSON and use the existing v3 checks: same boot, PID1, UID0, init.scope, exact UNIT and INVOCATION_ID; a start-done record MESSAGE_ID39f53479d3a045ac8e11786248231fbf followed by a success record MESSAGE_ID7ad2d189f7e94e70a38c781354912448, together with current unit/cgroup absence. An empty InvocationID alone never establishes success. Missing journal permission or records means unresolved custody, not success.

At timeout or failed preparation, retain all partial workspace, logs and receipt, charge the bounded attempt, and launch nothing else until ownership/cgroup cleanup is resolved. No automatic retry. The900-second total includes capture, wait, stop and sealing; begin cleanup with sufficient reserve.

## Acceptance and next command

Require native worker status passed, final materialized-inputs.json digest match, no errors, source/candidate hashes unchanged, owned native terminal proof, and empty captured cgroup. Preserve readbacks and SHA256/bytes for all native output files in a durable terminal manifest. A missing receipt or incomplete input closure remains a failure or unknown. Fresh archive/cache/tool hashes are only claimed after the materializer records them.

Then bind the produced input manifest and exact new paths into the existing durable root validation recipe. The1800-second full validation remains separately reserved: default-feature offline locked compilation, four exact ordered controls, full unfiltered library suite, immutable-input postchecks and owned cleanup. Its input-manifest hash cannot legitimately be filled before preparation completes. No compiler is started by this packet.
