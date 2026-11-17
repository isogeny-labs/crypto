# Frozen bounded gate software build and native helper plan

T-1419, 2026-11-03. Before any build/test execution, freeze PLAN.md, controls.c and run.py hashes in frozen-plan.json. One attempt only. Exact reviewed gate.c SHA2566548d8d41a37975bb30c1a225583113ea6c25056753b3d455d90f1608d8e65bc is read from preserved runtime-private-qualifier. Do not change that source.

Build static linux/amd64 candidate with /usr/bin/gcc -static -O2 -Wall -Wextra -Werror, at most60seconds and1GiB virtual memory. Record compiler realpath/hash/version/target and source/binary hashes. Inspect ELF with readelf: x86-64, no INTERP or NEEDED. Never execute the candidate or whole renamed gate main.

Compile native controls that include exact gate source with main renamed. Only fstat is interposed: real kernel metadata is read; uid/gid may be explicitly mapped to0 for the success fixtures because testuser is1000. Other mode/type/linkcount/size values stay real. Negative uid/gid cases use explicit overrides. Test release_token over absent, every partial length0..36, complete37byte token, wrong nonce/newline, extra byte, wrong mode, wrongowner, symlink, FIFO, directory and hardlink states. Test immediate valid await_release and persistent malformed token with its real15second timeout. No fork, container, namespaces, mounts, privileges or daemon; no whole gate execution.

Entire validation at most180seconds and1GiB; helper process at most25seconds. Preserve all failed commands/output and stop after failure without automatic retry. No performance claim. A candidate ELF and helper checks do not constitute PID1/container gate qualification. Production binary promotion, image and runtime admission remain false. Native gate controls involving true rootownership/cgroup/PID1/signals remain pending.

Publish binaries using official content-addressed artifact tool and verify remote readback. This small artifact transport is authorized; no cloud build/experiment is launched. Source/recipe independent review before scoped local commit; root owns GitHub publication.

Pre-execution refinement: each command owns a new process group and kills/reaps that group on timeout/error; command argv and partial output remain recorded on failure. Original frozen-plan.initial.json is preserved, no attempt ran under it.

Pre-run reviewer amendment: outer command is `timeout --signal=TERM --kill-after=5s 175s bash -c 'ulimit -v 1048576; exec python3 -B run.py'`. Signal handlers raise into command cleanup; signals are blocked across Popen until PID/PGID is recorded. Repeated signal does not interrupt initial cleanup. Receipt records uncertain launch/reap and cleanup error. This does not promise termination of kernel-uninterruptible tasks. The pre-signal freeze remains preserved, unexecuted.
