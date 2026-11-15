# DRAFT Amendment R2-H (proposed number 18): the capped runs at t = 14 and the further t = 13 draw 0 attempt on reserved RunPod CPU pods

**Status: draft revision 2, NOT frozen.** Revised 2026-11-15 after the independent review of revision 1 (commit
`59e487150f`; blocking items M1-M7, advice S1-S14). When frozen it is appended to
`research/prime_fall_degree_20261027/PREREGISTRATION.md` as Amendment 18, before any job named below is started.
Every number is `[pcc:unverified]`.

- **Numbering.** Amendment 15 is R2-G (stage B unit jobs) and Amendment 16 is stage B on one reserved RunPod pod (both
  in the preregistration). Amendment 17 is claimed by another session's open PR #4173 (rebuilt binaries admitted by
  exact reproduction, for stage B), so this is 18, appended after 17. If PR #4173 closes unmerged, this amendment is
  frozen as 17 instead, and the constant `AMENDMENT_HEADING` in `common.sh` changes with it (that file's hash is
  recorded at the freeze).
- **Scope.** It changes the host of the registered capped jobs below, how their results come back, and how a run on
  that host is classified and accounted. It changes no hypothesis, §6 rule, reading, cell, seed, draw, cap,
  `--budget`, `--max-dense`, binary or command line.
- **Authority.** Owner decision 12 (2026-11-14, "Use runpod"): command jobs run on RunPod on-demand pods in the
  reserved tier, not interruptible, under dated amendments frozen before each run. Decision 10: t = 14, and the
  further t = 13 draw 0 attempt, run on non-Spot capacity. Decision 4: at most US$40 per job and US$400 for the study.
  Decision 3 (one relaunch after a loss) is applied here as the main session's reading (section 4.3).
- **Reuse.** The pod runs Amendment 16's runner `runpod/server.py` unchanged (SHA-256 pinned). Its client is a pinned,
  unchanged copy of main's `runpod/rp.sh` at `d2a16c723d` (`runpod_capped/rp_pinned.sh`), so a later edit of
  `runpod/rp.sh` (PR #4173 edits it) does not reach this route. The pod's start command is new (section 2.4).

**Disclosure (what the authors had seen).** When revision 1 was written and when this revision was made, the main
session had seen every t = 12 and t = 13 row, and the partial F4 log of the lost third t = 13 draw 0 attempt
(`results/Y6/0b275a39-5cad-40de-b26e-634672d97955/runs/X-m2-t13-k0-c6.log`, 10,564 bytes, git-excluded; its step
counters through step 30 equal those of the completed draw 1 log, `SPOT_LOSSES.md`). Nothing from t = 14 exists or has
been seen. The subagent that wrote this revision read the first line of the t = 13 draw 1 cap 6 row file (Y4) to find
the row's `oversize` and `timed_out` field names, and the development smoke row at t = 8; it read no other row. None of
the rules below depends on any of these.

## 1. Jobs, names and order

Each job runs one `cap:` stage. The stage equals `command[1]` of its request, and `run.sh` refuses any other. Only the
names below can be launched (decision D2), each at most once, in this order: cell `t14-k0-c6` (with its relaunch and
`-m112` run if they become due), then cell `t14-k1-c6` (likewise), then cell `t13-k0-c6`.

| name | when it may run | request | stage |
|:--|:--|:--|:--|
| `pkm-x-t14-k0-c6` | first | `requests_r2i/pfd-x-t14-k0-c6.json` (frozen) | `cap:14:0:6:12800000000` |
| `pkm-x-t14-k0-c6-r1` | the base is LOST or ABORTED | same | same |
| `pkm-x-t14-k0-c6-m112` | the cell's 56Gi attempt is OOM, or ADMITTED with `oversize true`, and was not itself counted as the `-m112` attempt (section 4.2, memory rule) | `runpod_capped/requests_m112/pfd-x-t14-k0-c6-m112.json` | `cap:14:0:6:27900000000` |
| `pkm-x-t14-k0-c6-m112-r1` | the `-m112` attempt (the `-m112` run, or a 56Gi run counted as the `-m112` attempt) is LOST or ABORTED and the cell's relaunch is unused | same | same |
| `pkm-x-t14-k1-c6`, `-r1`, `-m112`, `-m112-r1` | after cell t14-k0 is resolved; same rules | `requests_r2i/pfd-x-t14-k1-c6.json`, `requests_m112/pfd-x-t14-k1-c6-m112.json` | `cap:14:1:6:…` |
| `pkm-x-t13-k0-c6-r3` (the further attempt; decision 10) | after both t = 14 cells are resolved | `requests_r2i/pfd-x-t13-k0-c6.json` (frozen) | `cap:13:0:6:12800000000` |
| `pkm-x-t13-k0-c6-m112` | the `-r3` run is OOM, or ADMITTED with `oversize true`, and was not itself counted as the `-m112` attempt | `requests_m112/pfd-x-t13-k0-c6-m112.json` | `cap:13:0:6:27900000000` |

- Every stage is Kummer m = 2, P1, cap 6, `--budget 86400` (Amendment 7 B), `RAYON_NUM_THREADS = 12` (Amendment 9).
  The 56Gi stages use `--max-dense 12800000000` and the `-m112` stages `--max-dense 27900000000` (Amendment 14).
- **`-m112` requests (decision D5).** `make_requests_m112.sh` derives each from its frozen `requests_r2i` request exactly
  as Amendment 14 says: memory 112Gi, `--max-dense 27900000000`, job-name suffix `-m112`, a sentence appended to the
  provenance note; every other field is copied, including `provenance.protocol_sha256`. Its output
  (`make_requests_m112_output.txt`) lists exactly those four changed paths for each file. `run.sh` re-derives the
  request and refuses if the file differs. The three requests pass the repository's request validator, the one the
  `requests_r2i` requests passed (`validate_command_request.sh`; output in `requests_m112_validate_output.txt`). A first
  run failed them because the appended note made `provenance.note` 2,088 characters, over the validator's limit of
  2,048; the sentence was shortened and the requests regenerated. The t = 13 draw 0 cell is covered because Amendment 14 says a further
  attempt of it "runs under it".
- **A cell's row is its earliest admissible attempt.** With the order above, a cell has at most one admissible attempt
  per size, and an `-m112` row replaces the 56Gi attempt only as Amendment 14 says.
- **The next bracket step needs its own request and amendment**, in either direction (Amendment 7 B, `c* = 7` at
  t = 14): cap 7 if cap 6 does not refute, cap 5 if it refutes. Neither is generated or covered here.

## 2. What changes

1. **Host: one pinned flavor, no fallback (decision D8).** Every study run uses `cpu5m` at 16 vCPU and 128 GB
   (US$1.04/h, read 2026-11-14), in the reserved tier, not interruptible, image `python:3.12-slim`, 20 GB disk. If
   `cpu5m` at that size is out of stock, `run.sh` exits 10 without creating anything, and no attempt is charged; the
   session waits for stock. The literal request fields are:

   ```
   computeType: "CPU"  cloudType: "SECURE"  interruptible: false  cpuFlavorIds: ["cpu5m"]  vcpuCount: 16
   ```

   - Before the runner starts, `run.sh` checks the create response (`pod-create.json`): `machine.secureCloud` true, a
     data center id, flavor `cpu5m`, 16 vCPU, at least 61 GB (121 GB for `-m112`), and `costPerHr` equal to the
     on-demand price of the reserved tier read just before create. The API pod record returns `interruptible: null`,
     so the not-interruptible fact comes from the request record `pod-request.json` (`interruptible: false`). A
     record with `interruptible: true` is refused. Any failed check deletes the pod (exit 4).
   - Worst case per run: 26 h × the price read before create (US$27.04 at US$1.04/h).
2. **Threads (S13).** The thread count stays 12 (`rayon 12`). On EKS the container saw `nproc 12`; here 12 threads run
   on a pod whose affinity gives `nproc 16`. `muladds` does not depend on this, because the F4 chunk size is set from
   the rayon thread count (`src/cryptanalysis/f4_fp_tower.rs:1694` at `f13a690df0`); wall time can.
3. **Entry script file.** The jobs invoke `/usr/local/bin/entry_r2.sh STAGE` where the frozen requests name
   `/usr/local/bin/entry.sh`, because Amendment 16's runner runs only `entry_r2.sh` (SHA-256 `25b01841…`).
   - For a `cap:` stage the commands, file names and `progress.txt` lines are the same: `entry_r2.sh` differs from
     `entry.sh` only in its header comment and the added `m4n:` branch. Its preamble, `cap:*)` branch and tail are
     byte-identical to `entry.sh`'s (checked with `cmp` on the three extracted regions, and by the review).
4. **Pod start command (decision D4).** The start command is `bash -c` with a short script (recorded per run as
   `boot-template.sh`, and its SHA-256 in `pod-request.json`) that:
   - copies the cgroup memory files into `/results/cgroup/` every 30 s and once more within 5 s after
     `runner_state.json` appears, then stops: cgroup v2 `memory.max`, `memory.peak`, `memory.events`, or, if the pod
     has cgroup v1, `memory.limit_in_bytes`, `memory.max_usage_in_bytes`, `memory.oom_control`, `memory.failcnt`; it
     also records `/proc/self/cgroup` and the cgroup file-system type;
   - then `exec`s the unchanged runner, exactly as Amendment 16's start command did.
   Whether a RunPod CPU pod exposes the v2 or the v1 files is **not known**: the smoke pod did not read them
   (`SMOKE.md`). The loop tolerates either and records which. If neither exists, there is no OOM evidence and an exit
   137 counts as LOST (section 4.2).
5. **Results path and durability (S2).**
   - `entry_r2.sh`'s `aws s3` calls go to the runner's shim, which copies into the pod's `/results` (Amendment 16).
   - `fetch.sh` downloads `/results.tar.gz` twice, extracts both, and requires byte-identical per-file SHA-256
     manifests. It keeps one tree in `runpod_capped/results/<name>/` (git-ignored by `runpod_capped/.gitignore`) with
     its manifest `local-sha256.txt` in the receipts, and never overwrites an existing tree.
   - Before a completed run's pod is deleted, the session adds the kept tree to the job's `bench.record/v1`
     (`tools/benchrec.py add-raw`, then `upload`); `fetch.sh NAME --delete RECORD` deletes the pod only when the
     record lists every file of the manifest as uploaded. If the upload fails before the run deadline, the pod is
     kept and the upload retried. At the run deadline the pod is deleted regardless (the tree is local).
6. **Time limit and the backstop (decision D3).**
   - The pilot's own `--budget 86400` is unchanged.
   - The registered job limit of 90,000 s becomes the **run deadline**: the runner's recorded start (`start.json`)
     + 90,000 s. Before `/start`, `run.sh` writes `pod-state.json` with the creation time, the **hard deadline**
     (creation + 26 h) and a provisional run deadline (just before start + 90,000 s). It starts the runner only within
     55 minutes of creation, so the run deadline is at most creation + 25 h 55 min.
   - At or after the run deadline, or on `--abort`, `fetch.sh` deletes the pod even if the transfer fails, after
     recording the failure (the run is then classified by section 4).
   - Check-ins: one about 4 h after start, then at most every 4 h, and one at the run deadline.
   - **Backstop: `reap.sh`.** It lists the account's pods and, for every pod whose name starts with `pkm-x-` and that is
     older than 26 h since creation, past its run deadline, or without a runner start 70 minutes after creation (a
     launcher that died before start), runs `fetch.sh` for it (collect what can be collected,
     delete) and deletes it directly if it is still listed. It is idempotent, touches no other pod, and appends one
     JSON line per `pkm-x-` pod to `reaper/reap-YYYYMMDD.jsonl`. The main session schedules it as a scheduled check-in
     at most every 4 h while any study pod exists.
   - **The US$40 ceiling relies on the session or the scheduled reaper.** Arithmetic at US$1.04/h: the nominal pod
     life of 26 h costs US$27.04. With the reaper every 4 h, a pod is deleted by creation + 25 h 55 min + 4 h, at most
     US$31.11. The ceiling holds while the pod is deleted within 38.4 h of creation (40 / 1.04), that is within about
     12.5 h after its run deadline. If both the session and the scheduled reaper stop, nothing on RunPod's side
     deletes the pod; the owner does.
7. **Host lines and memory.** `host.txt` is the frozen script's, so it has no RunPod pod id.
   - The pod id, flavor, `costPerHr`, data center, machine id and CPU model are recorded in the receipts and with the
     row.
   - On RunPod `nproc` reads the pod's affinity (16), while `MemTotal` reads the host's memory (263,400,384 kB on the
     smoke pod). The API's `memoryInGb` is RunPod's figure for the pod, not a measurement. The memory limit a run had
     is recorded from the cgroup copy (`memory.max`, or the v1 limit). The memory peak used by the memory rule of
     section 4.2 is the pilot's own `VmHWM` field, else the cgroup's (`memory.peak`, or v1 `memory.max_usage_in_bytes`).

## 3. Route files

All in `research/pkm_falldegree_unblock_20261107/runpod_capped/`, shell only (bash, jq, curl, awk, coreutils; no
Python): `common.sh` (settings, pins, allow-list, order, ledger), `run.sh` (launch), `fetch.sh` (check-in, collection,
the single outcome classifier, deletion), `reap.sh` (backstop), `rp_pinned.sh` (client copy), `make_requests_m112.sh`
and `requests_m112/`, `ledger.tsv`, `tests/` (offline tests). `SMOKE.md` records the development check.

- **`run.sh NAME STAGE`** refuses, before any API call (exit 3):
  - a name that is not on the allow-list (or `pkm-x-dev-<word>` with t ≤ 10, receipts in `receipts_dev/`);
  - an existing receipts directory or results tree for the name, or two earlier pre-start failures of the name;
  - a study name that is not the one the order allows now (`next_allowed`: a relaunch only after a LOST or ABORTED
    base, an `-m112` run only after an OOM or an `oversize true` row, nothing while an earlier run has no outcome);
  - a stage that differs from the request, or an `-m112` request that differs from its derivation;
  - for a study name, a preregistration without `## Amendment 18` (S1); it logs the preregistration's SHA-256 and the
    git HEAD for every launch;
  - `SHA_FILE` set; a client or runner whose SHA-256 differs from the pin (decision D6);
  - any of the 4 binaries, `entry_r2.sh` or `run.sh` differing from `image/binaries.sha256`, `build-info.json`
    differing from `image/image.json`; for a study name, a missing `RPC_LAYER` or a layer whose SHA-256 is not
    `image.json`'s `layer.digest` (`a0f7e28e…`) or whose files differ from `RPC_BIN`'s (S12; the check is logged);
  - a ledger that is missing or malformed, or, for a study name, whose `complete_as_of` line is older than 24 h;
  - `RPC_STATE` inside the git worktree (S5).
  Then, with the API: refuse if any `pkm-x-` pod is live (serial rule); read the `cpu5m` stock and price (exit 10 if
  out of stock); refuse if price × 26 h > US$40 or the ledger sum plus this worst case > US$400. Then it creates the
  receipts directory, sets the cleanup trap **before** the create call, creates the pod, checks it (section 2.1),
  waits for the runner, uploads the 7 files with `rp_pinned.sh` and compares each pod-side SHA-256, writes
  `pod-state.json`, and starts the runner (which re-checks every SHA-256 against `image/image.json`). Any failure before
  the runner starts deletes the pod: by id, or by name when the create call failed or timed out; confirms by listing;
  writes `cost.json` and an outcome `NOT_AN_ATTEMPT`; and moves the receipts to `<name>.prestart-<k>` (or
  `<name>.nocreate-<time>` if no pod existed), so the name stays usable (exit 4 or 5; 11 if the deletion could not be
  confirmed).
- **`fetch.sh NAME [--abort | --delete RECORD]`** takes one check-in (runner status through the proxy, the REST pod
  record, list membership; appended to `checkins.jsonl`), decides with the classifier whether to collect, collects,
  writes `collect.json` and `outcome.json` once, waits for the raw upload when the run completed, and deletes. It
  refuses (exit 9) when `cost.json` exists, and never removes an existing results tree. A delete that could not be
  confirmed is resumed by the next call (exit 7 until then). Exit codes are listed in the script.
- **Manual recovery if `fetch.sh` stops mid-collection.** If `fetch.sh` dies after keeping the tree in
  `results/<name>/` but before writing `collect.json`, later calls refuse (exit 9: the tree exists), and `reap.sh`
  deletes the pod directly once it is due. The session then: (1) changes no receipt; (2) moves the orphan tree to
  `results/<name>.orphan-<UTC time>/`, where it is kept but never used as a row; (3) writes one line in `run.log`
  saying so; (4) runs `fetch.sh <name>` again. If the pod is still there, `fetch.sh` takes a new check-in and two new
  fetches and classifies as usual; if it is gone, the classifier sees the loss in the receipts (P3, or P4/P6 at the
  deadline). Either way the class comes only from the receipts and the newly kept tree; the orphan tree enters no
  rule. A completed run's tree is the same in both fetches, so the move cannot change a class.
- **Tokens and keys (S4).** The API key is in a mode-0600 header file in `RPC_STATE`, removed on exit; it is never in
  argv or receipts. The pod token is written to a 0600 file and passed to the create call inside a 0600 body file, so
  it is not in this side's argv for create. Two exposures remain, both accepted: (1) `rp_pinned.sh` (unchanged, pinned)
  sends it as `-H "X-Token: …"` in curl's argv, visible only to processes of this single-user session container; (2) it
  sits in the pod's environment, readable by anyone holding the shared RunPod account key. Before `/start` such a
  holder could upload files, but `/start` refuses any hash other than the frozen ones; after `/start` the runner
  refuses every upload and every second start, so the token gives read access only. Anyone with the account key can
  already delete or create pods.
- **Not built:** a pod-side job script; the runner already runs the frozen script unchanged.

## 4. Outcome classes, relaunches and limits

### 4.1 One outcome table (decision D1)

`fetch.sh` holds the only classifier. It reads receipts only: `start.json` (the runner's recorded start `s0`), the
REST `lastStartedAt` recorded right after start (`pod-record-start.json`), every check-in in `checkins.jsonl`, the
pre-fetch check-in and reason in `collect.json`, and the kept tree (`runner_state.json`, `progress.txt`,
`cgroup/`, the row). The run deadline is `s0` + 90,000 s. **Loss evidence** is any of: a check-in where the runner is
`idle` after its start, or reports a start time other than `s0`; a REST record whose `lastStartedAt` changed, or whose
`desiredStatus` is not `RUNNING`; the pod missing from a successful list; the runner unreachable through the proxy at
check-ins spanning at least 1 h while the REST record is not a healthy `RUNNING` pod with unchanged `lastStartedAt`
(S11: a proxy outage alone does not make a run lost); more than one `start` line for the cell in `progress.txt`; a
`runner_state.json` start other than `s0`. A **completion is secured** when the collection reason is `done`, the two
fetches agree, the pre-fetch check-in shows the runner `done` under `s0`, no loss evidence is dated at or before that
check-in, the tree's `runner_state.json` agrees (state, start, exit) and has one start line.

Precedence, first match wins:

| step | condition | class |
|:--|:--|:--|
| P1 | no receipt shows a runner start | NOT_AN_ATTEMPT |
| P2a | completion secured, runner ended after the run deadline | LIMIT |
| P2m1 | completion secured, a 56Gi attempt with memory peak (pilot `VmHWM` first, else the cgroup; section 4.2) > 56 GiB (counted as the `-m112` attempt), and peak > 112 GiB or exit 137 with `oom_kill` > 0 | OOM (at `-m112`) |
| P2m2 | as P2m1's first part, exit 137 without oom evidence | LOST (the `-m112` attempt) |
| P2m3 | as P2m1's first part, admission rules 1-7 hold | ADMITTED (the cell's `-m112` row) |
| P2m4 | as P2m1's first part, otherwise | NOT_ADMITTED |
| P2b | completion secured, an `end` line of the cell has exit 137, cgroup `oom_kill` > 0 | OOM |
| P2c | completion secured, an `end` line has exit 137, `oom_kill` 0 or no cgroup file | LOST |
| P2p | completion secured, an `-m112` attempt whose memory peak exceeds 112 GiB | OOM |
| P2u | completion secured, no memory peak observable | NOT_ADMITTED |
| P2d | completion secured, admission rules 1-7 hold (section 6) | ADMITTED |
| P2e | completion secured, otherwise | NOT_ADMITTED |
| P3 | loss evidence dated before the run deadline | LOST |
| P4 | a check-in at or after the run deadline shows the runner running, or done after the deadline, under `s0` | LIMIT |
| P5 | collected on `--abort` before the run deadline | ABORTED |
| P6 | collected or deleted otherwise (completion not secured, nothing positive at the deadline) | LOST |
| P7 | not collected yet | (pending) |

What this settles, case by case (each is an offline test, `tests/README.md`):
- A restart seen before the deadline is LOST; one first seen at or after the deadline is LOST unless a check-in at or
  after the deadline saw the runner still running (then LIMIT). LIMIT needs positive evidence.
- One unreachable reading at the deadline: nothing can be fetched; the pod is deleted; LOST (P6).
- Completed, then restarted: classified by the completion only if its receipts were fetched first (the pre-fetch
  check-in and both fetches before any loss evidence); otherwise LOST.
- `--abort` on a run that is already done, lost or past its deadline does not change its class; only an abort of a
  running run before its deadline is ABORTED.
- A runner that finishes after 90,000 s is LIMIT, never ADMITTED.
- A completed run that fails an admission rule (for example the pilot exits 1) is NOT_ADMITTED: no row, not a loss,
  no relaunch; the cell goes to the owner. This class is added to decision D1's list for the case the list did not
  cover.

### 4.2 What each class means

- **ADMITTED.** The run's row is the cell's row at that size; `oversize` and `timed_out` in the row then act as
  Amendments 7 B and 14 say (a budget stop is censored and not re-run; `oversize true` sends the cell to `-m112`).
- **LIMIT** mirrors an EKS job killed at its time limit. The preregistration and Amendments 7, 9, 12 and 14 give no
  separate rule for that case. On EKS, `seconds` is the Job's `activeDeadlineSeconds` (R2-C §8); Kubernetes kills the
  pod, and the row file is written only when the pilot ends, so no row exists. Such a run is "stopped by a limit" in
  Amendment 7 B's sense: censored, reported with its host, not re-run (Amendment 14's re-run triggers are only
  `oversize true` and `OOMKilled`), and it uses no relaunch. LIMIT here is exactly that; a partial tree fetched at the
  deadline is kept as a receipt and enters no rule.
- **OOM** is Amendment 14's `OOMKilled`, kept exactly: censored-by-limit, not the cell's row, and the cell is re-run
  once at the `-m112` size under the `-m112` name; that run's row is the cell's row and it uses no relaunch. An `-m112`
  run that is OOM, or ADMITTED with `oversize true`, is censored (Amendment 7 B). Exit 137 is classified by the cgroup
  evidence, never by the exit code alone: `oom_kill` > 0 is OOM; `oom_kill` 0, or no cgroup file, is LOST.
- **Memory rule (main session decision of 2026-11-15, fixed before any data).** Every run here has a 128 GB pod, so a
  56Gi run could finish here with more memory than its registered request; on EKS it would have been `OOMKilled`.
  - The run's memory peak is, first, the pilot's own field: the largest `memory R MB (peak P MB)` value in
    `runs/<cell>.log`, which is the process's `VmHWM` in MiB (`examples/pkm_tower_pilot.rs`, `memory_mb`, at
    `f13a690df0`), converted to bytes as MiB × 1,048,576. That is the source of the 20,618 MB peak of the t = 13 draw 1
    cap 6 run (`RESULTS_T13_BRACKET.md` §2b; the largest such value in that log is 20,618). Only when the log has no
    such line is the cgroup figure used as a fallback: v2 `memory.peak`, else v1 `memory.max_usage_in_bytes` (bytes).
  - Why the pilot first: the cgroup figure includes reclaimable file cache and the runner process, whereas an EKS
    `OOMKilled` depends on memory that cannot be reclaimed. The process high-water mark is the closer analogue, and the
    cgroup figure could censor a decisive t = 14 run for page cache alone.
  - An exit 137 with cgroup `oom_kill` > 0 is OOM whatever either peak says (P2b, or P2m1 when the pilot peak is above
    56 GiB).
  - A 56Gi attempt whose peak exceeds 56 GiB (60,129,542,144 bytes) counts as `OOMKilled` under Amendment 14, and the
    run itself is classified as the cell's `-m112` attempt (`outcome.json` `memory.as_m112` true). It is ADMITTED, as
    the cell's `-m112` row, when the peak is at most 112 GiB (120,259,084,288 bytes) and every other rule holds; it is
    OOM at `-m112` (censored) above 112 GiB or with exit 137 and `oom_kill` > 0; LOST with exit 137 and no oom evidence.
    It takes the `-m112` slot and Amendment 14's relaunch accounting: no `-m112` run follows it; if it is LOST or
    ABORTED, `-m112-r1` follows when the cell's relaunch is unused. It keeps its own stage (`--max-dense 12800000000`);
    if its row has `oversize true`, it is censored as an `-m112` run stopped on its limit.
  - An `-m112` attempt whose peak exceeds 112 GiB is OOM (P2p), by the same reading at the larger size and with the
    same source priority. This step is the main session's extension of the memory rule (section 9, point 4).
  - If no peak is observable (no pilot field and no cgroup peak file), the run is NOT_ADMITTED (P2u) and goes to the
    owner.
  - Disclosure: the pilot's `VmHWM` covers only the pilot process (not `pfd_grid_zeros`, the runner or the shell),
    and the pilot prints it only when an F4 step ends, so a final rise after its last printed line is missed. The
    cgroup figure covers the whole container, file cache included. Both are recorded in `outcome.json`'s `memory`
    object (`pilot_peak_bytes`, `cgroup_peak_bytes`, `cgroup_peak_source`), with the one used in `source_used` and
    `peak_bytes_used`.
- **LOST** is the analogue of a Spot loss: nothing from the run is admitted or analysed, even if a partial tree was
  fetched. Each loss is recorded with its reason in `RUNPOD_LOSSES.md`, as in `SPOT_LOSSES.md`.
- **ABORTED** counts as an attempt and is charged exactly like LOST for relaunch accounting, so an abort cannot buy a
  different host beyond what a loss would.
- **NOT_AN_ATTEMPT** (a failure before the runner starts) uses no relaunch. At most two per name; a third goes to the
  owner. Before the start the session can see the pod's data center and the host's thread count and memory (not its
  CPU model), so a deliberate pre-start abort could re-draw the host; the limit of two and the committed
  `<name>.prestart-<k>` receipts bound and expose that.

### 4.3 Relaunches (S9: the main session's reading, for the owner to confirm)

Decision 3 is about Spot reclaims. The R3 draft (`AMENDMENT_R3_RUNPOD.md` §4) said it does not apply to a reserved pod,
and allowed one relaunch for a pod lost for any other reason. **Applying one relaunch per cell to a LOST or ABORTED run
on a reserved pod is the main session's reading of decision 3, for the owner to confirm before the freeze.** Under it:
- t = 14 cells: one relaunch per cell, across both sizes (as Amendment 14 counts it): `-r1` after a LOST or ABORTED base,
  or `-m112-r1` after a LOST or ABORTED `-m112` run if `-r1` was not used. Both attempts are kept.
- `pkm-x-t13-k0-c6-r3` is the further attempt that decision 10 allows. Neither it nor its `-m112` run is relaunched; if
  either is LOST or ABORTED, the cell goes back to the owner.

## 5. Cost guard

- **Price** (read 2026-11-14 23:24:47Z, RunPod GraphQL `cpuFlavors.specifics.securePrice`, `cpu5m-16-128`): US$1.04/h;
  the smoke pod's `costPerHr` was 1.04. `run.sh` reads it again before every create and records it in `price.jsonl`.
- **Per run.** Refused before create if price × 26 h > US$40; the pod is deleted before start if `costPerHr` differs
  from that price. Worst case today US$27.04 (section 2.6 for the arithmetic of the backstop).
- **Study (S10).** `runpod_capped/ledger.tsv` is the study-wide ledger: one row per job on any route (the EKS jobs,
  `pfd-b-rp` under Amendment 16, any Amendment 17 work, this route), with the actual cost when closed and the worst case
  when open. The main session keeps it complete and dates that with its `complete_as_of` line; `run.sh` refuses a
  study launch if that line is older than 24 h, or if the ledger sum plus the new run's worst case exceeds US$400.
  `run.sh` adds this route's row (worst case) at create, and `fetch.sh` closes it with the derived cost.
- **Bounds.** Three runs: at most US$81.12. Every allowed run (3 per t = 14 cell, 2 for t = 13): at most 8 × US$27.04
  = US$216.32, plus about 1.5 h of pod time per pre-start failure (at most two per name).
- **Recorded.** `price.jsonl`, `pod-create.json`, `pod-record.json`, `cost.json` (costPerHr × pod lifetime, derived).

## 6. Admission and receipts

A run is ADMITTED only through P2d or P2m3, that is when its completion is secured, a memory peak is observable and
within the limit of section 4.2, and all of these hold (the first four are the EKS verify-card pairs):

1. `binary.sha256` has the four lines of `image/binaries.sha256` at `/usr/local/bin/…`.
2. `host.txt` has `argv: <stage>` and `rayon: 12`.
3. `progress.txt` has exactly one `start X-m2-tT-kK-c6` line, `end X-m2-tT-kK-c6 exit 0` and
   `end X-m2-tT-kK-c6.gridzeros exit 0`.
4. `runner_state.json` has `state done`, `exit 0` and the start recorded in `start.json`.
5. `runner_request.json`'s `verified_sha256` equals `image/image.json` for the files the runner checks: its `BIN`
   list (the 4 binaries and `entry_r2.sh`) and `run.sh` (S7; `build-info.json` is uploaded and recorded, not checked).
6. The two fetches' manifests are identical, and `local-sha256.txt` equals the kept tree when classified.
7. `pod-create.json` shows `machine.secureCloud` true, `cpu5m`, 16 vCPU, at least the memory floor, and `costPerHr`
   equal to the price in `price.jsonl`; `pod-request.json` shows `interruptible: false`, the reserved tier and
   `cpuFlavorIds ["cpu5m"]` (M6).
8. Before the row is used: `cost.json` with `confirmed_gone: true`, the `deleted and no longer listed` line in
   `fetch.log`, and a 204 in `delete.http` (S7).

**Receipts of each run**, in `runpod_capped/receipts/<name>/` (development runs in `receipts_dev/`), never
overwritten: `run.log`, `price.jsonl`, `pod-request.json`, `boot-template.sh`, `pod-create.json`, `pod-record.json`,
`status-ready.json`, `upload.jsonl`, `pod-state.json`, `start.json`, `pod-record-start.json`; per check-in a line in
`checkins.jsonl` with `status-<stamp>.json` and `rest-<stamp>.json`; `fetch.log`, `collect.json`, `outcome.json`,
`local-sha256.txt`, `tarball.sha256`, `admission-lines.txt`, `upload-check.txt`; `pod-record-final-<stamp>.json`,
`delete.http`, `delete-<stamp>-<n>.response`, `pods-after-delete-<stamp>.jsonl`, `cost.json`. The account identifier
`consumerUserId` and the pod environment are removed from every saved API record (S14). The results tree
`results/<name>/` goes into the job's `bench.record/v1` with S3 locations by SHA-256. Records kept as on EKS: the resume
checkpoint before launch, the director's launch and verify cards, and their receipts.

## 7. Host dependence (M7)

- **The bracket readings depend on the host.** They use degrees, `oversize` and `timed_out`, and `--budget` is
  wall-clock, so whether a run reaches its budget depends on the CPU. At t = 14 this can decide the reading: the cap 6
  wall time went from 1,762.8 s at t = 12 (Xeon 8488C) to 11,570 s at t = 13 (Xeon 6975P-C), a factor of 6.56 across
  different hosts, and the same factor gives about 75,900 s at t = 14, within 12% of 86,400 s (derived, from the
  review).
- That is why one flavor is pinned for every study run, with no fallback (section 2.1). RunPod may still place a
  flavor on different CPU models. The CPU model of each run is recorded from `host.txt`; if the two t = 14 draws ran on
  different CPU models, the report says so next to the bracket reading. No rule changes because of it.
- A censored row is reported with its host, never dropped. `muladds` are compared only between rows with the same
  thread count (Amendment 9); the thread count here is 12.
- **Memory request versus pod memory.** Every run here has a 128 GB pod, whereas the registered request of the 56Gi
  runs is 56Gi. `--max-dense` stops a run at the same point as on EKS. A 56Gi run whose peak lies between 56 GiB and
  the pod's limit would complete here where EKS would have killed it; the memory rule of section 4.2 classifies such a
  run as the cell's `-m112` attempt.

## 8. Unchanged

- Binaries (static musl builds of `f13a690df016d0a457bd40579fbb9d054f486adf`, by SHA-256), commands, stages, cells,
  draws, caps; `--budget 86400`; `--max-dense` per Amendment 14; 12 threads.
- Hypotheses and §6 rules; Amendments 7 B, 9 and 14 (the OOM branch included).
- The US$40 per job and US$400 study ceilings.
- The requests in `requests_r2i/`, which are not edited. Their `provenance.protocol_sha256` (`69af89b7…`) predates
  Amendments 16-18; the protocol in force at each launch is logged by `run.sh` (SHA-256 of the preregistration and
  the git HEAD).

## 9. Points for the reviewer before freezing

1. The relaunch reading of section 4.3 needs the owner's confirmation.
2. NOT_ADMITTED is a class added for completed runs that fail an admission rule (section 4.1); accepted by the main
   session on 2026-11-15.
3. The ceiling relies on the session or the scheduled `reap.sh` (section 2.6).
4. The memory rule (section 4.2) was decided by the main session on 2026-11-15, with the pilot's `VmHWM` as the
   primary peak source and the cgroup figure as the fallback. P2p (the 112 GiB step for `-m112` attempts) is the main
   session's extension of the owner-facing memory rule, which names only the 56Gi attempts; it is kept, and it is for
   the owner to confirm with the rest of that rule.
5. Checks that need a pod (one more development smoke, `pkm-x-dev-*`, t ≤ 10):
   - the new start command (`bash -c`, cgroup copy loop, then the runner) boots and the runner answers;
   - which cgroup files a RunPod CPU pod has, and what `memory.max` reads;
   - whether `lastStartedAt` changes on a container restart (it is one of several loss detectors);
   - the create response still carries `machine.secureCloud` and `costPerHr` as in the smoke;
   - the end-to-end path with `--delete RECORD` after a `benchrec` upload, and `reap.sh` listing the pod.
6. Offline tests: `tests/README.md` (every refusal path, the classifier on synthetic receipt sets for each class and
   for the review's conflicting cases, the order, the reaper).

## 10. Script hashes at this revision

Recorded again at the freeze, after the development smoke (any edit made after it changes these).

| file | SHA-256 |
|:--|:--|
| `common.sh` | `d36c08529db4dd0cdc539e3fa374ed789088a44507067bef29350495425a80d6` |
| `run.sh` | `294c017735c5d6815c582ffe2e2f66f261dfb039c2192d00b57e840a1d0983ed` |
| `fetch.sh` | `6a46fc93c19cfcd18b1fa1cd4f451c68d5ce78b349197efeaffb2021de3700ea` |
| `reap.sh` | `165d6ccc5ed67bece87c8cd7a5665fe609267f042144fb0c977d1ea4485eb202` |
| `make_requests_m112.sh` | `1459e3d3ee821133e936cc40671946b94b6304413325d4d247e29c234e16d58b` |
| `rp_pinned.sh` | `b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d` (= main's `runpod/rp.sh` at `d2a16c723d`) |
| `runpod/server.py` (pinned, not copied) | `f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550` |
