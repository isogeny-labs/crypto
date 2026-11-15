# Pod hardening (guard, msolve gate, attach)  [pcc:unverified]

Scope: `pod/{guard.sh (new), job.sh, launch.sh, podserver.py}`; `rp.sh` unchanged. Nothing committed. All results below are from
real smoke pods (cpu5c, 2 vCPU, 4 GB cgroup, 20 GB disk, 0.07 USD/h) and local runs on 2026-11-14.

## Key finding: `kill 1` does NOT stop billing
On pod `1qkvz0foygfo9t` (smoke-hard-c, first run) guard.sh fired at deadline+grace (`guard: STOP COMPUTE`), `kill -TERM 1` ended
`python3 /podserver.py` (PID 1; a SIGTERM handler is needed for that, see podserver below), and RunPod then **restarted the container**:
`rp.sh info` stayed `RUNNING`, `rp.sh ls` failed once (23:45:31) and then answered again (23:45:43), `/work` survived, PID 1 was a new podserver.
A pod billed 0.07 USD/h here; a production pod bills 2.08 USD/h at 32 vCPU while idle-restarted.
Fix: RunPod injects a pod-scoped `RUNPOD_API_KEY` and `RUNPOD_POD_ID` into the pod and the image ships `/usr/bin/runpodctl` (1.14.15).
`runpodctl stop pod "$RUNPOD_POD_ID"` run inside the pod moved it to `EXITED` (tested, pod `1qkvz0foygfo9t`, 23:46:55) and `rp.sh delete` still returned http 204 afterwards.
guard.sh now calls that first (3 tries, 20 s apart, output scrubbed of long tokens, key value never read/printed/written), and only then falls back
to the spec'd `kill -TERM 1`, 30 s, `kill -KILL 1`, `pkill podserver`, `kill -KILL -1`. It also touches `WORKDIR/guard.stopped`; a restarted podserver sees it and
retries `runpodctl stop pod` every 60 s (so a restart cannot un-stop the decision). Note: from inside a PID namespace `kill -KILL 1` is ignored by the kernel unless
init has a handler, so the fallback is weak by itself.

## guard.sh (new, 51 lines)
`guard.sh DEADLINE_EPOCH GRACE_S WORKDIR`; re-launches itself with `setsid nohup ... &` (stdio redirected, flock single instance) and returns at once, so
the `rp.sh exec` that started it can end. Verified alive after the exec ended: `ps` on pod `pmea9r6hj9jm7r`: `bash /work/guard.sh ...` pid 71, ppid 1, own session.
Poll 60 s (env `GUARD_POLL_S`, tests only; it never sleeps past the next deadline). Events:
* at DEADLINE: creates `WORKDIR/run/STOP` (job.sh hook below), logs it.
* stop compute when (a)/(c) `run/out/done.json` AND `fetched.flag`; (b) now >= DEADLINE+GRACE; extra (d) `fetched.flag` and now >= DEADLINE (partial fetch at the hard deadline, no done.json).
* the reason is written to `WORKDIR/guard.log` before stopping. `GUARD_DRY=1` logs "would kill" and signals nothing (used for local tests and the `--dry-run` emulation).
Local tests (GUARD_DRY, poll 2 s): (a) no stop with done.json only, stop after fetched.flag; (b) STOP created at deadline, stop at deadline+grace; (d) fetched.flag after deadline stops at once. All as designed.
Real kill test: pod `tryc3c71jx53wn` (deadline +72 s, grace 60 s, poll 15 s, launch.sh SIGKILLed after the job exec): status RUNNING until 23:52:02, `EXITED` at 23:52:11
(deadline+grace = 23:52:10). `launch.sh --attach` on it said "pod is not running ... nothing can be fetched", `partial.tgz` could not be requested, `ERROR: nothing fetched`, pod deleted (http 204), result PARTIAL.
Not exercised on a real pod: the `kill -TERM 1` fallback after a failed runpodctl (only the restart behaviour of a bare `kill 1` was observed).

## job.sh (+17 lines)
1. **msolve identity gate** (after the install step and env capture, before any row). Before: an install failure only set `INSTALL_RC` (recorded in `done.json` as `msolve_install_rc`);
   `dpkg -i debs/*.deb` was the only fallback; the plan ran with whatever `/usr/bin/msolve` existed, or a missing one gave rc 127 rows. Now, when not `--dry-run` and
   `sha256(/usr/bin/msolve) != MSOLVE_SHA` (or missing), job.sh writes `done.json` = `{"schema":"pkm.m8-panel.done/v2","aborted":"msolve_identity","rows_planned":0,"rows_recorded":0,"msolve_sha256":..,"msolve_sha_expected":..,"msolve_install_rc":..,"pod_id":..,"utc":..}` and exits 5.
   `--dry-run` skips the gate (it uses the local msolve by design). Tested with a copy of job.sh whose expected sha was altered: rc 5, valid JSON, 0 rows. launch.sh treats an `aborted` done.json as PARTIAL, logs `JOB ABORTED`, fetches `partial.tgz`, deletes the pod.
2. **STOP hook** at the top of the row loop: if `$W/STOP` exists (W = `/work/run`, guard.sh creates it) it logs, appends one `not_run` line per remaining row to `runs.jsonl`
   (`censored":"not_run","reason":"deadline"`, `wall_s/user_s/sys_s` 0 so the existing `done` helper works unchanged) and `break`s; job.sh then closes normally (results.tgz, done.json).
   The row in flight finishes (bounded by its own timeout, max 4 h + 1 h verify, below the 6 h grace). Local test: STOP pre-created: both rows `not_run`, done.json `by_status {"not_run":2}`.
   Also: two header comment lines.

## podserver.py (+8/-1)
`signal.SIGTERM` handler `os._exit(0)` (PID 1 ignores unhandled signals sent from inside) and the `guard.stopped` re-stop thread described above. Applied to pods created after this change (rp.sh embeds the file).

## launch.sh (+123/-22)
* guard.sh is `put` with job.sh (before the job starts) and started right after the job exec: `DEADLINE_EPOCH = created_epoch + deadline_h*3600`, `GRACE_S = 21600`; confirms `guard.log` "started" (else prints a WARNING). `put` retried 3 times (one transient 404 from the proxy right after `wait` was seen; the trap deleted that pod correctly).
* `fetched.flag`: `rp.sh exec NAME 'touch /work/fetched.flag'` after the sha256-verified results.tgz and after a partial fetch, before delete (non-fatal if it fails).
* `launch.json` is written right after creation (not after the job exec) with `created_epoch, deadline_epoch, pod_name, pod_id, usd_per_hr, flavor, vcpu, deadline_h, grace_s` (`start_epoch` kept = created_epoch).
* Creation also copies the state file to `$RESUME_DIR/pod_state/NAME.json` (default `/home/user/crypto/.resume`; contains the pod's podserver token, not the RunPod key) and adds one line under `## Pod in flight` in `RESUME.md` (created if missing, inserted at the end of that section otherwise): name, id, flavor, $/h, created/deadline UTC, exact attach command, out dir, state copy. On delete the line is prefixed `[ENDED <utc>, pod deleted ...]`.
* `--attach NAME --out DIR [--deadline-h H] [--poll-s S]`: no bundle, no flavor gate, no create/put/job exec; state file searched in `$RP_STATE`, `DIR/state`, `$RESUME_DIR/pod_state`, `pod/state` (copied into `DIR/state`); created/deadline from `DIR/launch.json` (missing fields are written back; `--deadline-h` recomputes from created); the trap deletes the pod. Without a job exec id it looks for `job.sh` on the pod (`pgrep`, pattern tested locally) and gives up after two misses with no done.json. If the pod is EXITED/STOPPED/TERMINATED it says so, skips polling, cannot fetch, and deletes.
* poll loop (both modes): pod status check each poll; two consecutive `EXITED/...` ends polling (PARTIAL, then delete). `None`/empty (API blips, seen) are ignored.
* `--dry-run`: no RunPod call; guard.sh is emulated locally with `GUARD_DRY=1` (logs "would kill"), fetched.flag is a local file, RESUME is not touched. Tested.
* test-only override: `LAUNCH_TEST_NAME=pkm-m8-smoke-* [LAUNCH_TEST_FLAVOR LAUNCH_TEST_VCPU LAUNCH_TEST_DISK LAUNCH_TEST_GRACE_S]` bypasses the 32 vCPU flavor gate; refused (rc 4) unless the name starts with `pkm-m8-smoke-` and not with `--replace`. Verified refusal with `pkm-m8-panelA-x`.

## Tests on real pods (all deleted; `rp.sh pods` shows no `pkm-m8-smoke-` pod at the end)
Env for all: `LAUNCH_TEST_FLAVOR=cpu5c LAUNCH_TEST_VCPU=2 LAUNCH_TEST_DISK=20 RESUME_DIR=<scratch>` (so the real `.resume` was not touched), mini bundle = plan rows `timer_floor` + `t2r9-m4 d025159 gb_check`, no hidden.json.
| pod | what | result |
|---|---|---|
| `pmea9r6hj9jm7r` (smoke-hard-a) | `launch.sh --bundle mini.tgz --poll-s 30 --deadline-h 1`, normal path | apt `msolve=0.6.5-1build2` installed on `runpod/base:1.0.2-ubuntu2404`; done.json: msolve_sha256 `c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa` (= expected), msolve_install_rc 0, by_status ok 2, pod_id set; cgroup v2 memory.max 3999997952 (limit_kb 3906248), cpu.max `max 100000`, cpuset `35,99`. results.tgz sha ok, fetched.flag set, guard still waiting (poll 60 s) when launch.sh deleted the pod (http 204). 0.0206 h. |
| `rj6t4j18iw0lcb` (smoke-hard-b) | launch.sh SIGKILLed right after the job exec; `launch.sh --attach` from `env -i` fresh shell | attach found state in `DIR/state`, done.json present, results.tgz sha ok, fetched.flag set, pod deleted. done.json same sha/rc 0, ok 2. |
| `1qkvz0foygfo9t` (smoke-hard-c, manual) | guard.sh with deadline +60 s, grace 60 s; then the restart finding; then `runpodctl stop pod` | STOP file at +60 s, "STOP COMPUTE" at +120 s, status stayed RUNNING (container restarted), then EXITED after runpodctl stop; `rp.sh delete` 204. |
| `btk135jgf6v9m0` | first launch attempt for the next test | `put` got a proxy 404, trap deleted the pod (cleanup path verified); led to the put retry. |
| `tryc3c71jx53wn` (smoke-hard-c) | launch killed, guard deadline+grace with the runpodctl stop, then `--attach` on the EXITED pod | RUNNING -> EXITED at 23:52:11; attach said so, deleted (204), PARTIAL. |
| `hoezifcj6misxf` (smoke-hard-d) | 61-row mini plan, launch killed after job exec, `--attach` while the job ran (poll 20 s) | progress 3 -> 20 -> 37 -> 56 rows during polls, done.json, results sha ok (`ok` 61), flag, delete 204. |
Also: a stale `pod/state/pkm-m8-smoke-233009.json` from an earlier session exists (no such pod listed); left alone.

## Open points
* `RUNPOD_API_KEY` inside the pod is RunPod's injected pod key; if a future image/entrypoint stops injecting it, guard.sh logs "falling back to killing PID 1" and the pod would restart-loop and keep billing; launch.sh's own deadline delete is then the only bound.
* Attach `job_state` printing `up` was not observed explicitly (the attach run just kept polling correctly); the pgrep pattern was tested locally.
* A pod stopped (EXITED) may lose its container disk: at deadline+grace unfetched results are lost by design.

## Production commands (not run)
```
bash pod/launch.sh --bundle pod/bundle.tgz --out results/<stamp> --deadline-h 72
bash pod/launch.sh --attach pkm-m8-panelA-<stamp> --out results/<stamp> --deadline-h 72     # exact line is also in .resume/RESUME.md under "## Pod in flight"
```
