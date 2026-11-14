# DRAFT Amendment R3 (proposed number 15): reserved RunPod CPU pods as an execution route

**Status: draft, revision 3 (2026-11-14), NOT frozen.** Revision 1 was reviewed by an independent red team
(`REVIEW_R3_R4.md`): "freeze with edits", blocking edits E3.1–E3.9. This revision applies them; §8 maps each edit to its place.

The owner's instructions of 2026-11-14 were: "unblock the fall-degree study and root solver. Run this on runpod", then "do the
isogeny arm at P1 first" (twice). R3 changes **where and on what host** registered jobs run. It does not change any hypothesis,
§6 rule, reading, cell, seed, draw, cap, budget, `--max-dense`, `max_entries`, binary or stage. Every number is `[pcc:unverified]`.

## 1. Why (cited)

- The EKS `cpu-spot` pool is Spot-only and capped at 16 vCPU / 128 GiB (`CAPACITY_R2.md`).
- Stage B was lost twice to Spot interruptions (`STAGE_B_LOST.md`). Cell `pfd-x-t13-k0-c6` was lost three times
  (`SPOT_LOSSES.md`).
- A RunPod pod created with `interruptible: false` is reserved: it is not reclaimed for capacity.

## 2. Jobs on this route, and their authority (E3.1)

| jobs | authority | status |
|:--|:--|:--|
| Amendment R4 (the 2-isogeny arm at `P1`) | owner, 2026-11-14: "do the isogeny arm at P1 first" | on this route once R3 and R4 are frozen |
| `pfd-x-t13-k0-c6` (would be attempt 4) | none quoted. Decision 7 returned the cell to the owner | **not on this route** until the owner answers |
| stage B `pfd-b` (would be attempt 3) | none quoted. Decision 8 chose the split (R2-G) | **not on this route** until the owner answers |
| `t = 14` cap 6, the `m = 4` closures and their nulls | the 2026-11-14 instruction names the study, not these cells | **not on this route** until the owner confirms |

**Cost ceilings.** Owner decision 4 set US$40 per job and US$400 for the study, for Spot compute. Applying the same values to
reserved compute needs the owner's confirmation. Until then, no job on this route starts.

## 3. The route

- **Pod.**
  - One reserved RunPod CPU pod per job: `computeType CPU`, `interruptible false`, flavor `cpu5m` or `cpu3m`
    (memory-optimised), 12 vCPU.
  - Image `python:3.12-slim@sha256:5e2bdb1b0d542d5039cffb8181c65930093949194a65c365dfb1b0fe13a4fa5b`, pinned by digest.
  - A 20 GB pod volume is mounted at `/workspace`.
- **Runner** (`runpod/server.py`; its SHA-256 is passed to the pod and recorded). Before anything runs, it refuses a job:
  - whose files do not match their pins: frozen binaries per `image/image.json`, R4 binaries per `iso/PINS.json`, and R4
    scripts per `iso/BUNDLE.sha256` at the job's commit;
  - with no time limit;
  - whose cost at the limit, plus one hour of idle billing, exceeds the ceiling, using the pod's live `costPerHr`;
  - whose container memory (cgroup `memory.max`, else `MemTotal`) is below the registered request;
  - that has fewer than 12 usable CPUs.
- **While running.**
  - `RAYON_NUM_THREADS` is fixed at 12.
  - The entry process group is killed at the job's registered time limit, recorded as exit `deadline`.
  - The cgroup `oom_kill` counter is read before and after. **OOM** means a stage exit of 137 together with an `oom_kill`
    increment. That is Amendment 14's `OOMKilled`.
- **Results** (E3.5).
  - The entry script's `aws` calls go to a shim that writes to `/workspace/results` on the pod volume, which survives a container
    restart.
  - The client fetches at every check-in: `/manifest`, the pod's own SHA-256 of each file, then the tarball, built on disk.
    Each file is checked against the manifest.
  - No job starts while S3 upload is unavailable. Fetched results go to S3 under `runs/<job>/runpod/<pod id>/` with their
    hashes, as the EKS route did.
- **Identity** (E3.8). Each receipt records the pod's `create.json` (including `machine`), the runner SHA-256, the image digest,
  `runner_request.json` (with verified file hashes and the commit) and `host.txt` (CPU model, `nproc`, cgroup `memory.max`).
- **Tested locally, partly on modified copies** (F-R3-1). On the 4-vCPU, 15 GB session machine (cgroup v1), 2026-11-14 about
  22:01Z. The logs are not committed; this is a description, not evidence.
  - The committed runner refused: cost at the limit US$50.96 (16 vCPU × 172,800 s, then with a 1-hour idle allowance) against
    US$40; memory below a 56 GiB floor; a request with no time limit.
  - The "4 CPUs < 12" refusal and the deadline kill (exit `deadline` after 3 s) were seen only on a scratch copy with the CPU
    floor lowered.
  - The guarded runner has **not yet run a job with every guard passing**. That first happens on a real pod, where its R4
    `gaux` stage is the check.
  - Since then: a cgroup-v1 fallback for the `oom_kill` counter (`memory.oom_control`) and the CPU quota (`cpu.cfs_*`), an
    idle allowance of 14,400 s, and a refusal when `RESULTS_DIR` is not empty.
- **Idle billing** (F-R3-2). The cost refusal includes `idle_seconds` (default 14,400 s), because a pod is billed until the next
  check-in deletes it. With 4 hours, `iso-a` (172,800 s) fits US$40 only at a live price of about US$0.769/h or less.
- **OOM rule scope** (F-R3-3). "Exit 137 plus an `oom_kill` increment" is used as Amendment 14's `OOMKilled` only in jobs with a
  single heavy stage: the `cap:`/`x:` jobs. In multi-stage jobs, a whole-job counter cannot say which stage was killed. There, an
  exit 137 with an increment is recorded as OOM of an unidentified stage, and the job's rows are checked one by one.
- **No mixing** (F-R3-4). A relaunch always uses a new pod. The runner refuses `/start` when `RESULTS_DIR` is not empty, so a
  container restarted on the same volume cannot write a second job into the first job's results.

## 4. Host dependence (E3.6)

Exact values of completed runs do not depend on the host. That is checked by G-AUX on every R4 job, and for Kummer jobs by the
E3.7 control. Censoring by a wall-clock budget does depend on the host (Amendment 9), and so does a G3 timeout. Every row records
the CPU model, `cpu.max` and its wall time. A budget-censored row is reported with its host and is not rerun on another host.

## 5. Positive control for Kummer jobs (E3.7)

Before the first Kummer job on a pod flavor, `cap:12:0:5:12800000000` and `cap:12:0:6:12800000000` are rerun and compared field
by field with the admitted X1/X3 rows (12 threads). Wall time is recorded against 1,824 s. A mismatch stops the route. These
rows are checks and enter no reading. R4 jobs use their own `gaux` stage instead.

## 6. Cost (E3.2)

- Before each pod is created, the projection is `(limit / 3600 + 1) × costPerHr`. If it exceeds the per-job ceiling, the job
  does not start and goes to the owner, as Amendment 12 does.
- A running study total is kept against the study ceiling and checked before each pod is created.
- The account balance: the RunPod REST API reachable from this session has no balance endpoint (only `/billing/pods`,
  `/billing/endpoints`, `/billing/networkvolumes`). Spend so far is read from `/billing/pods` before each pod is created and
  recorded. The owner confirms once, before the first pod, that the balance covers the projected spend.
- Pods are deleted at the first check-in after their job ends.
- There is no "censored-by-cost" reading. A job runs to its registered limit, or it does not start.

## 7. Losses (E3.9)

A reserved pod lost for any reason (host failure, a RunPod stop, a balance stop, a container restart) is recorded with its reason.
Its partial results on the pod volume are fetched if reachable; they are kept as evidence and enter no reading. Attempts are
counted per cell across routes and sizes, including the Spot attempts. The next attempt needs the authority named in §2.

R2-G (stage B as resumable units) is superseded for this route only if the owner chooses to run stage B here as one job (§2).
Otherwise it stays the owner's decision 8 (E3.10).

## 8. Review edits → where they are

| edit | where |
|:--|:--|
| E3.1 authority | §2; Kummer retries and ceilings wait for the owner |
| E3.2 deadline, cost | §3, §6; `server.py` (`seconds`, `cost_per_hr`, `max_usd`; process-group kill); `rp.sh start-iso` sends the pod's live `costPerHr` |
| E3.3 memory, OOM | §3; `server.py` (`memory_floor_gib`, cgroup `memory.max`, `oom_kill`); `rp.sh` flavors `cpu5m`, `cpu3m` |
| E3.4 threads, vCPU | §3; `server.py` (rayon 12 always; refuses fewer than 12 CPUs) |
| E3.5 durability | §3; pod volume `/workspace`, `RESULTS_DIR`; `/manifest`; tarball on disk; `rp.sh fetch` checks the manifest |
| E3.6 host dependence | §4 |
| E3.7 Kummer control | §5 |
| E3.8 identity | §3; image digest pin, `RUNNER_SHA256`, `create.json` with `machine` |
| E3.9 losses | §7 |
| E3.10 (optional) R2-G | §7 |
| F-R3-1 test statement, cgroup v1 | §3 "Tested locally"; `server.py` v1 fallback |
| F-R3-2 idle allowance 4 h | §3; `server.py` `idle_seconds` |
| F-R3-3 OOM rule scope | §3 |
| F-R3-4 new pod per relaunch, no mixing | §3; `server.py` refuses a non-empty `RESULTS_DIR` |
