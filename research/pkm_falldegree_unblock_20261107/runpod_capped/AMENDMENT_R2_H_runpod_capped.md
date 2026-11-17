# DRAFT Amendment R2-H (proposed number 18): the capped runs at t = 14 and the further t = 13 draw 0 attempt on reserved RunPod CPU pods

**Superseded, not to be launched (2026-11-15T07:50Z).** At 04:16-04:23Z another session committed its own Amendments 18-20 on PR #4173 ("t=13 draw 0 and t=14 capped cells on reserved RunPod pods", the m = 4 cells, and a US$500 study ceiling, each marked "owner instruction") and launched pods `pfd-x-t13-k0-c6-rp` (04:16Z), `pfd-x-t14-k0-c6-m112-rp` and `pfd-x-t14-k1-c6-m112-rp` (06:50Z) for the same cells. Running this route as well would run the decisive cells twice, so this session does not launch it, whatever owner decision 16 said; the conflict is with the owner. The route, its three reviews and the validation stay as a reviewed record. `run.sh` cannot launch a study name from this file: it requires its own script hashes in the preregistration and a ledger `complete_as_of` date, neither of which exists. Every number here is `[pcc:unverified]`.

**Status: draft revision 4, NOT frozen.** Revised 2026-11-15 after the third independent review (round 3, of commit
`a1c9a4b79c`; verdict "freeze with corrections"; corrections R1-R3 and the reaper lock, S1). Revision 3 answered the
second review (`7305d2ee8d`, N1-N6), revision 2 the first (`59e487150f`, M1-M7). When
frozen it is appended to `research/prime_fall_degree_20261027/PREREGISTRATION.md` as Amendment 18, with the SHA-256 of
the four route scripts (section 10), before any job named below is started. Every number is `[pcc:unverified]`. The
decisions this text relies on, with their dates and reasons, are listed in `runpod_capped/DECISIONS_ROUTE.md`.

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
  Owner decisions 13, 14 and 15 (2026-11-15): relaunches (section 4.3), enforcement by the session and the reaper
  (section 2.6), and the 112 GiB step for `-m112` attempts (section 4.2). All are in
  `research/open_leads_execution_20261108/DECISIONS.md`.
- **Reuse.** The pod runs Amendment 16's runner `runpod/server.py` unchanged (SHA-256 pinned). Its client is a pinned,
  unchanged copy of main's `runpod/rp.sh` at `d2a16c723d` (`runpod_capped/rp_pinned.sh`), so a later edit of
  `runpod/rp.sh` (PR #4173 edits it) does not reach this route. The pod's start command is new (section 2.4).

**Disclosure (what the authors had seen).** When revisions 1-3 were written, the main session had seen every t = 12
and t = 13 row, and the partial F4 log of the lost third t = 13 draw 0 attempt
(`results/Y6/0b275a39-5cad-40de-b26e-634672d97955/runs/X-m2-t13-k0-c6.log`, 10,564 bytes, git-excluded; its step
counters through step 30 equal those of the completed draw 1 log, `SPOT_LOSSES.md`). Nothing from t = 14 exists or has
been seen. The subagent that wrote revisions 2 and 3 read the first line of the t = 13 draw 1 cap 6 row file (Y4) to
find the row's `oversize` and `timed_out` field names, the largest `peak` value of that run's log (20,618, already
reported in `RESULTS_T13_BRACKET.md`), and the development smoke row at t = 8; it read no other row. None of the rules
below depends on any of these.

## 1. Jobs, names and order

Each job runs one `cap:` stage. The stage equals `command[1]` of its request, and `run.sh` refuses any other. Only the
names below can be launched (decision D2), each at most once, in this order: cell `t14-k0-c6` (with its relaunch and
`-m112` run if they become due), then cell `t14-k1-c6` (likewise), then cell `t13-k0-c6`.

| name | when it may run | request | stage |
|:--|:--|:--|:--|
| `pkm-x-t14-k0-c6` | first | `requests_r2i/pfd-x-t14-k0-c6.json` (frozen) | `cap:14:0:6:12800000000` |
| `pkm-x-t14-k0-c6-r1` | the base is LOST with relaunch `auto` (section 4.3) | same | same |
| `pkm-x-t14-k0-c6-m112` | the cell's 56Gi attempt is OOM, or ADMITTED with `oversize true`, and was not itself counted as the `-m112` attempt (section 4.2, memory rule) | `runpod_capped/requests_m112/pfd-x-t14-k0-c6-m112.json` | `cap:14:0:6:27900000000` |
| `pkm-x-t14-k0-c6-m112-r1` | the `-m112` attempt (the `-m112` run, or a 56Gi run counted as the `-m112` attempt) is LOST with relaunch `auto`, and the cell's relaunch is unused | same | same |
| `pkm-x-t14-k1-c6`, `-r1`, `-m112`, `-m112-r1` | after cell t14-k0 is resolved; same rules | `requests_r2i/pfd-x-t14-k1-c6.json`, `requests_m112/pfd-x-t14-k1-c6-m112.json` | `cap:14:1:6:…` |
| `pkm-x-t13-k0-c6-r3` (the further attempt; decision 10) | after both t = 14 cells are resolved | `requests_r2i/pfd-x-t13-k0-c6.json` (frozen) | `cap:13:0:6:12800000000` |
| `pkm-x-t13-k0-c6-m112` | the `-r3` run is OOM, or ADMITTED with `oversize true`, and was not itself counted as the `-m112` attempt | `requests_m112/pfd-x-t13-k0-c6-m112.json` | `cap:13:0:6:27900000000` |

- Every stage is Kummer m = 2, P1, cap 6, `--budget 86400` (Amendment 7 B), `RAYON_NUM_THREADS = 12` (Amendment 9).
  The 56Gi stages use `--max-dense 12800000000` and the `-m112` stages `--max-dense 27900000000` (Amendment 14).
- **A name is launched at most once, ever.** `run.sh` refuses a name that already has a receipts directory, a results
  directory, or any row in `ledger.tsv` (main-session decision N6). So a run whose receipts were lost cannot be
  launched again under its name; any further attempt of the cell needs the owner.
- **`-m112` requests (decision D5).** `make_requests_m112.sh` derives each from its frozen `requests_r2i` request exactly
  as Amendment 14 says: memory 112Gi, `--max-dense 27900000000`, job-name suffix `-m112`, a sentence appended to the
  provenance note; every other field is copied, including `provenance.protocol_sha256`. Its output
  (`make_requests_m112_output.txt`) lists exactly those four changed paths for each file. `run.sh` re-derives the
  request and refuses if the file differs. The three requests pass the repository's request validator, the one the
  `requests_r2i` requests passed (`validate_command_request.sh`; output in `requests_m112_validate_output.txt`). A first
  run failed them because the appended note made `provenance.note` 2,088 characters, over the validator's limit of
  2,048; the sentence was shortened and the requests regenerated. The copied part of the note still names the 56Gi
  values; the appended sentence states the 112Gi ones. The t = 13 draw 0 cell is covered because Amendment 14 says a
  further attempt of it "runs under it".
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
   - **The pod's cgroup memory limit is read before start (S4).** The start command's copy loop (item 4) writes the
     cgroup files into `/results/cgroup/` before anything else exists there; `run.sh` fetches that tree once before
     `/start` and records the limit in `cgroup-prestart.json`. It must be `max` or at least 112 GiB
     (120,259,084,288 bytes), so that an OOM kill here means what Amendment 14's `OOMKilled` means at the `-m112` size.
     Otherwise, or if no limit file exists, the pod is deleted before start and the case goes to the owner.
2. **Threads (S13).** The thread count stays 12 (`rayon 12`). On EKS the container saw `nproc 12`; here 12 threads run
   on a pod whose affinity gives `nproc 16`. `muladds` does not depend on this, because the F4 chunk size is set from
   the rayon thread count (`src/cryptanalysis/f4_fp_tower.rs:1694` at `f13a690df0`); wall time can.
3. **Entry script file.** The jobs invoke `/usr/local/bin/entry_r2.sh STAGE` where the frozen requests name
   `/usr/local/bin/entry.sh`, because Amendment 16's runner runs only `entry_r2.sh` (SHA-256 `25b01841…`).
   - For a `cap:` stage the commands, file names and `progress.txt` lines are the same: `entry_r2.sh` differs from
     `entry.sh` only in its header comment and the added `m4n:` branch. Its preamble, `cap:*)` branch and tail are
     byte-identical to `entry.sh`'s (checked with `cmp` on the three extracted regions, and by both reviews).
4. **Pod start command (decision D4).** The start command is `bash -c` with a short script (recorded per run as
   `boot-template.sh`, and its SHA-256 in `pod-request.json`) that:
   - copies the cgroup memory files into `/results/cgroup/` at once, every 30 s, and once more within 5 s after
     `runner_state.json` appears, then stops: cgroup v2 `memory.max`, `memory.peak`, `memory.events`, or, if the pod
     has cgroup v1, `memory.limit_in_bytes`, `memory.max_usage_in_bytes`, `memory.oom_control`, `memory.failcnt`; it
     also records `/proc/self/cgroup` and the cgroup file-system type;
   - then `exec`s the unchanged runner, exactly as Amendment 16's start command did.
   The second development smoke (`receipts_dev/pkm-x-dev-smoke2/`) shows cgroup v2 on a RunPod CPU pod: file-system
   type `cgroup2fs`, `memory.max` 128,000,000,000 bytes before start (`cgroup-prestart.json`) and in the results, and
   `memory.events` (`oom_kill 0`) and `memory.peak` copied (`admission-lines-1.txt`). The loop still tolerates v1.
5. **Results path and durability (S2, S8).**
   - `entry_r2.sh`'s `aws s3` calls go to the runner's shim, which copies into the pod's `/results` (Amendment 16).
   - Each collection attempt `k` first writes an intent record `collect-started-<k>.json` (time, reason, the pre-fetch
     check-in). `fetch.sh` then downloads `/results.tar.gz` twice, extracts both, and compares the per-file SHA-256
     manifests. It writes `transfer-<k>.json`, `local-sha256-<k>.txt` and `tarball-<k>.sha256`, keeps one tree in
     `runpod_capped/results/<name>/collect-<k>/` (git-ignored), then writes `admission-lines-<k>.txt` and
     `collect-<k>.json`. Nothing is overwritten or removed; a later attempt uses `k + 1`.
   - Before a completed run's pod is deleted, the session adds the kept tree to the job's `bench.record/v1`
     (`tools/benchrec.py add-raw`, then `upload`); `fetch.sh NAME --delete RECORD` deletes the pod only when the
     record lists every file of the manifest as uploaded. The pod waits for that at most 2 h after the collection
     (S8), and never past the hard deadline; then it is deleted regardless (the tree is local).
6. **Time limit and the backstop (decisions D3, owner decision 14).**
   - The pilot's own `--budget 86400` is unchanged.
   - The registered job limit of 90,000 s becomes the **run deadline**: the runner's recorded start (`start.json`)
     + 90,000 s. Before `/start`, `run.sh` writes `pod-state.json` with the creation time, the **hard deadline**
     (creation + 26 h) and a provisional run deadline (just before start + 90,000 s). It starts the runner only within
     55 minutes of creation, so the run deadline is at most creation + 25 h 55 min.
   - **No information is never a loss (main-session decision N2).** When the runner is unreachable, the session's
     network is down, or the transfer check fails, `fetch.sh` changes nothing and retries at the next check-in, also
     after the run deadline, until the hard deadline. A healthy `RUNNING` REST record with unchanged `lastStartedAt`
     always means "wait". At the hard deadline the pod is deleted for cost after one more collection attempt; with no
     evidence the class is NO_EVIDENCE (section 4).
   - Check-ins: one about 4 h after start, then at intervals of at most 4 h, one at the run deadline, and at least one
     between the run deadline and the hard deadline (an interval of 5 to 60 minutes, depending on when the runner
     started).
   - **Backstop: `reap.sh`.** It lists the account's pods and deletes a `pkm-x-` pod only when it is past its hard
     deadline (from its receipts' `pod-state.json`, or creation + 26 h when none was recorded); never earlier, whatever
     `fetch.sh` reports (main-session decision N3c). For such a pod it first runs `fetch.sh` (collect what can be
     collected, classify, delete) and deletes directly if the pod is still listed. A pod whose deadline cannot be read
     is not deleted but reported (exit 1). It is idempotent, touches no other pod, logs every delete with its reason
     before the API call, and appends one JSON line per `pkm-x-` pod to `reaper/reap-YYYYMMDD.jsonl`. The main session
     schedules it as a scheduled check-in at intervals of at most 4 h while any study pod exists. It takes the same
     per-name lock as `run.sh` and `fetch.sh`; a pod whose lock is held is skipped for that pass, and the skip is
     recorded (review round 3, S1). A pass that skipped a due pod for a held lock still exits 0 (the pod is not
     counted as left over); the skip is visible only on stderr and as a "skipped: lock held" line in the reaper's JSONL.
     The session therefore reads that JSONL after every reaper pass and runs the pass again once the lock is free; a due
     pod skipped this way is never left until the next scheduled pass (validator observation, revision 4).
   - **One worst-case pod life, used everywhere (S7).** A pod lives at most until its hard deadline (26 h) plus one
     reaper interval (4 h): **30 h, US$31.20 at US$1.04/h.** The per-run guard (price × 30 h ≤ US$40), the open ledger
     rows and the US$400 check all use these 30 h.
   - **The deadline and the US$40 ceiling are enforced by session check-ins and the scheduled reaper; the owner
     accepted that there is no enforcement independent of Claude sessions (owner decision 14).** If both stop, nothing
     on RunPod's side deletes the pod. Whether a pod could delete itself (`RUNPOD_POD_ID` and a pod-scoped key) is
     recorded at the next development smoke (section 9), but it is not required.
7. **Host lines and memory.** `host.txt` is the frozen script's, so it has no RunPod pod id.
   - The pod id, flavor, `costPerHr`, data center, machine id and CPU model are recorded in the receipts and with the
     row. **Pinning `cpu5m` does not pin the CPU model:** the two development smokes ran on two models, an AMD Ryzen
     Threadripper 7960X (smoke 1, `SMOKE.md`) and an AMD EPYC 9575F 64-Core Processor (smoke 2, `host.txt` in
     `receipts_dev/pkm-x-dev-smoke2/admission-lines-1.txt`).
   - On RunPod `nproc` reads the pod's affinity (16), while `MemTotal` reads the host's memory (263,400,384 kB on the
     first smoke host, 527,605,596 kB on the second). The API's `memoryInGb` is RunPod's figure for the pod, not a measurement. The memory limit is read
     before start (item 1) and copied again with the results. The memory peak used by the memory rule of section 4.2 is
     the pilot's own `VmHWM` field, else the cgroup's (`memory.peak`, or v1 `memory.max_usage_in_bytes`).

## 3. Route files and procedure

All in `research/pkm_falldegree_unblock_20261107/runpod_capped/`, shell only (bash, jq, curl, awk, coreutils; no
Python): `common.sh` (settings, pins, allow-list, order, ledger), `run.sh` (launch), `fetch.sh` (check-in, collection,
the single outcome classifier, deletion), `reap.sh` (backstop), `rp_pinned.sh` (client copy), `make_requests_m112.sh`
and `requests_m112/`, `ledger.tsv`, `tests/` (offline tests), `DECISIONS_ROUTE.md`. `SMOKE.md` records the
development check. `run.sh`, `fetch.sh` and `reap.sh` take one lock per name in `RPC_STATE`, so they never act on the
same run at once.

- **`run.sh NAME STAGE`** refuses, before any API call (exit 3):
  - a name that is not on the allow-list (or `pkm-x-dev-<word>` with t ≤ 10, receipts in `receipts_dev/`);
  - an existing receipts directory or results tree for the name, a row for the name in `ledger.tsv`, or two earlier
    pre-start failures of the name;
  - a study name that is not the one the order allows now (`next_allowed`, section 4.3);
  - a stage that differs from the request, or an `-m112` request that differs from its derivation;
  - for a study name, a preregistration without `## Amendment 18`, or one that does not list the SHA-256 of
    `common.sh`, `run.sh`, `fetch.sh` and `reap.sh` (S1, S3); it logs the preregistration's SHA-256, the git HEAD and
    the route hashes for every launch;
  - `SHA_FILE` set; a client or runner whose SHA-256 differs from the pin (decision D6);
  - `image/binaries.sha256` or `image/image.json` disagreeing with the binary hashes and the layer digest pinned in
    `common.sh` (S3); any of the 4 binaries, `entry_r2.sh` or `run.sh` differing from those pins; `build-info.json`
    differing from `image.json`; for a study name, a missing `RPC_LAYER` or a layer whose SHA-256 is not
    `a0f7e28e…` or whose files differ from `RPC_BIN`'s (S12; the check is logged);
  - a ledger that is missing or malformed, or, for a study name, whose `complete_as_of` line is older than 24 h;
  - `RPC_STATE` inside the git worktree (S5).
  Then, with the API: refuse if any `pkm-x-` pod is live (serial rule); read the `cpu5m` stock and price (exit 10 if
  out of stock); refuse if price × 30 h > US$40 or the ledger sum plus this worst case > US$400. Then it creates the
  receipts directory, sets the cleanup trap **before** the create call, creates the pod, checks it (section 2.1),
  waits for the runner, uploads the 7 files with `rp_pinned.sh` and compares each pod-side SHA-256, reads the cgroup
  limit, writes `pod-state.json`, and starts the runner (which re-checks every SHA-256 against `image/image.json`). Any
  failure before the runner starts deletes the pod (logged first): by id, or by name when the create call failed or
  timed out; confirms by listing; writes `cost.json` and an outcome `NOT_AN_ATTEMPT` (with the exit code and whether a
  signal ended the launcher, S11); and moves the receipts to `<name>.prestart-<k>` (or `<name>.nocreate-<time>` if no
  pod existed). A pre-start failure that created a pod leaves a ledger row, so that name is not launched again (exit 4
  or 5; 11 if the deletion could not be confirmed, in which case the ledger row stays open at the worst case).
- **`fetch.sh NAME [--abort | --delete RECORD]`** takes one check-in (runner status through the proxy, the REST pod
  record with `lastStartedAt` and `lastStatusChange`, list membership; one line appended to `checkins.jsonl`), decides
  with the classifier whether to collect, collects (section 2.5), writes `outcome.json` once, waits for the raw upload
  when the run completed, and deletes (logged first, with the reason). It refuses (exit 9) when `cost.json` exists.
  A delete that could not be confirmed is resumed by the next call (exit 7 until then); a collection whose
  `outcome.json` was never written is classified by the next call (S9). If the local pod token is missing, it is
  recovered from the REST pod record's `env` into `RPC_STATE` only, never into receipts, and the recovery is logged
  (main-session decision N6). For a study name it refuses unless its own and `common.sh`'s SHA-256 are in the frozen
  amendment (S3). Exit codes are listed in the script.
- **Recovery when `fetch.sh` stops mid-collection (main-session decision N3b).** Nothing is discarded or overwritten.
  If `fetch.sh` dies after keeping a tree but before writing `collect-<k>.json`, the next call finds the intent record
  and the tree. It writes `collect-<k>.json` from the intent record, marked `recovered`, with `transfer_ok` true only if
  `transfer-<k>.json` says so and the tree still equals `local-sha256-<k>.txt`. If that collection was made with the
  runner done and checks out, it is the collection that is classified; otherwise the next collection from the pod
  uses `k + 1`, and the orphan tree is kept. When two collections made with the runner done exist, their trees must
  agree file by file, or the run is NO_EVIDENCE and goes to the owner (P0b). The class always comes from the receipts
  and the kept trees only.
- **After every launch and every `fetch.sh` call, the session commits and pushes the run's receipts and `ledger.tsv`
  (main-session decision N6).** The scripts print a one-line reminder with the paths; the session does the git part.
  So a new session can continue from the repository if the launching session is lost.
- **The session makes no `status` or `fetch` call with a pod token outside `run.sh` and `fetch.sh`, and no delete,
  stop or restart outside `run.sh`, `fetch.sh` and `reap.sh`, which log each delete with its reason before the call.**
  RunPod keeps no independent record of who ended a pod (unlike EKS's audit log), so these logs and the recorded
  `lastStatusChange` are the only attribution there is (section 4.3).
- **Tokens and keys (S4).** The API key is in a mode-0600 header file in `RPC_STATE`, removed on exit; it is never in
  argv or receipts. The pod token is written to a 0600 file and passed to the create call inside a 0600 body file, so
  it is not in this side's argv for create. Exposures that remain: (1) `rp_pinned.sh` (unchanged, pinned) sends it as
  `-H "X-Token: …"` in curl's argv, visible only to processes of this single-user session container; (2) it sits in
  the pod's environment, readable by anyone holding the shared RunPod account key, through the REST record. With it,
  anyone can read `/status` and `/results.tar.gz` at any time. Before `/start` such a holder could upload files, and the
  runner compares the uploads only with the hashes sent in the `/start` request (review round 3, S7); the protection is
  that `run.sh` calls `/start` right after its own upload check, and admission rules 1 and 5 check the pinned hashes.
  After `/start` the runner refuses every upload and every second start. Anyone with the account key can already delete or create pods.
- **Not built:** a pod-side job script; the runner already runs the frozen script unchanged.

## 4. Outcome classes, relaunches and limits

### 4.1 One outcome table (decision D1; classifier version `a18-r4`)

`fetch.sh` holds the only classifier. It reads receipts only: `start.json` (the runner's recorded start `s0`; a
zero-byte file means the start call got no response and is read as absent), the REST record right after start
(`pod-record-start.json`: `lastStartedAt`, `lastStatusChange`), every check-in in `checkins.jsonl`, the session's
logged actions (`session-actions.jsonl`), the collection records and the kept trees. Every `outcome.json` records the
classifier version and the SHA-256 of `fetch.sh` and `common.sh` (S3). An unparsable receipt fails closed (P0, S2).

**Positive evidence of loss** (main-session decision N2) is any of: a reachable runner that reports `idle` after its
start, or a start other than `s0`; a REST record (HTTP 200) whose `desiredStatus` is not `RUNNING`; the pod missing
from a successful list **and** its GET returning 404 (S6); more than one `start` line for the cell, or a
`runner_state.json` start other than `s0`, in the tree. A changed `lastStartedAt` alone is recorded but is not evidence
(S1): a runner that answers `running` under `s0` keeps its state in process memory, so its container has not
restarted. An unreachable runner, an unreadable REST record or a failed transfer check is no information, never
evidence.

A **completion is secured** when the collection reason is `done`, the two fetches agree, the pre-fetch check-in shows
the runner `done` under `s0`, no loss evidence is dated at or before that check-in, the tree's `runner_state.json`
agrees (state, start, exit) and has one start line.

Precedence, first match wins:

| step | condition | class |
|:--|:--|:--|
| P0 | a receipt the classifier reads is unparsable | NO_EVIDENCE |
| P0b | two collections made with the runner done have different trees | NO_EVIDENCE |
| P1 | no receipt shows a runner start | NOT_AN_ATTEMPT |
| P2a | completion secured, runner ended after the run deadline | LIMIT |
| P2m1 | completion secured, a 56Gi attempt with memory peak > 56 GiB and row `oversize false` (counted as the `-m112` attempt), and peak > 112 GiB or exit 137 with `oom_kill` > 0 | OOM (at `-m112`) |
| P2m2 | as P2m1's first part, exit 137 without oom evidence | LOST (the `-m112` attempt) |
| P2m3 | as P2m1's first part, admission rules 1-7 hold | ADMITTED (the cell's `-m112` row) |
| P2m4 | as P2m1's first part, otherwise | NOT_ADMITTED |
| P2b | completion secured, an `end` line of the cell has exit 137, cgroup `oom_kill` > 0 | OOM |
| P2c | completion secured, an `end` line has exit 137, `oom_kill` 0 or no cgroup file | LOST |
| P2p | completion secured, an `-m112` attempt whose memory peak exceeds 112 GiB | OOM |
| P2u | completion secured, no memory peak observable | NOT_ADMITTED |
| P2d | completion secured, admission rules 1-7 hold (section 6) | ADMITTED |
| P2e | completion secured, otherwise | NOT_ADMITTED |
| P3 | positive evidence of loss dated before the run deadline | LOST |
| P4 | a check-in at or after the run deadline shows the runner running, or done after the deadline, under `s0` | LIMIT |
| P5 | collected on `--abort` | ABORTED |
| P6 | positive evidence of loss dated at or after the run deadline | LOST |
| P7 | collected (at the hard deadline) with none of the above | NO_EVIDENCE |
| P8 | not collected yet | (pending) |

What this settles, case by case (each is an offline test, `tests/README.md`):
- A restart seen before the deadline is LOST; one first seen at or after the deadline is LOST unless a check-in at or
  after the deadline saw the runner still running (then LIMIT). LIMIT needs positive evidence.
- An unreachable runner at or after the run deadline: wait and retry until the hard deadline; then NO_EVIDENCE.
- A session-side network outage, however long: no information; the pod is kept (until the hard deadline).
- Completed, then restarted: classified by the completion only if its receipts were fetched first (the pre-fetch
  check-in and both fetches before any loss evidence); otherwise LOST.
- `--abort` on a run that is already done, lost or past its deadline does not change its class; an abort of a run
  that is still running is ABORTED, which uses the attempt.
- A runner that finishes after 90,000 s is LIMIT, never ADMITTED.
- A completed run that fails an admission rule (for example the pilot exits 1) is NOT_ADMITTED: no row, not a loss,
  no relaunch; the cell goes to the owner. This class was added to decision D1's list and accepted by the main
  session on 2026-11-15.

### 4.2 What each class means

- **ADMITTED.** The run's row is the cell's row at that size; `oversize` and `timed_out` in the row then act as
  Amendments 7 B and 14 say (a budget stop is censored and not re-run; `oversize true` sends the cell to `-m112`).
- **LIMIT** mirrors an EKS job killed at its time limit. The preregistration and Amendments 7, 9, 12 and 14 give no
  separate rule for that case. On EKS, `seconds` is the Job's `activeDeadlineSeconds` (R2-C §8); Kubernetes kills the
  pod, and the row file is written only when the pilot ends, so no row exists. Such a run is "stopped by a limit" in
  Amendment 7 B's sense: censored, reported with its host, not re-run (Amendment 14's re-run triggers are only
  `oversize true` and `OOMKilled`), and it uses no relaunch. LIMIT here is exactly that; a partial tree fetched at the
  deadline is kept as a receipt and enters no rule. One difference is disclosed (S10): EKS counts
  `activeDeadlineSeconds` from the Job's start, pending time included; here the run deadline counts from the runner's
  start, which comes after the pod's boot and the upload (at most 55 minutes after creation).
- **OOM** is Amendment 14's `OOMKilled`, kept exactly: censored-by-limit, not the cell's row, and the cell is re-run
  once at the `-m112` size under the `-m112` name; that run's row is the cell's row and it uses no relaunch. An `-m112`
  run that is OOM, or ADMITTED with `oversize true`, is censored (Amendment 7 B). Exit 137 is classified by the cgroup
  evidence, never by the exit code alone: `oom_kill` > 0 is OOM; `oom_kill` 0, or no cgroup file, is LOST.
- **Memory rule (main-session decisions of 2026-11-15, and N4; fixed before any data).** Every run here has a 128 GB
  pod, so a 56Gi run could finish here with more memory than its registered request; on EKS it would have been
  `OOMKilled`.
  - The run's memory peak is, first, the pilot's own field: the largest `memory R MB (peak P MB)` value in
    `runs/<cell>.log`, which is the process's `VmHWM` in MiB (`examples/pkm_tower_pilot.rs`, `memory_mb`, at
    `f13a690df0`), converted to bytes as MiB × 1,048,576. That is the source of the 20,618 MB peak of the t = 13 draw 1
    cap 6 run (`RESULTS_T13_BRACKET.md` §2b; the largest such value in that log is 20,618). Only when the log has no
    such line is the cgroup figure used as a fallback: v2 `memory.peak`, else v1 `memory.max_usage_in_bytes` (bytes).
    The largest pilot peak line is copied into `admission-lines-<k>.txt`, so the decision can be checked from git (S5).
  - Why the pilot first: the cgroup figure includes reclaimable file cache and the runner process, whereas an EKS
    `OOMKilled` depends on memory that cannot be reclaimed. The process high-water mark is the closer analogue, and the
    cgroup figure could censor a decisive t = 14 run for page cache alone.
  - An exit 137 with cgroup `oom_kill` > 0 is OOM whatever either peak says (P2b, or P2m1 when the pilot peak is above
    56 GiB and the row has `oversize false`).
  - A 56Gi attempt whose peak exceeds 56 GiB (60,129,542,144 bytes) **and whose row has `oversize false`** counts as
    `OOMKilled` under Amendment 14, and the run itself is classified as the cell's `-m112` attempt (`outcome.json`
    `memory.as_m112` true): its row is the row the `-m112` run would compute, since `--max-dense` did not stop it. It
    is ADMITTED, as the cell's `-m112` row, when the peak is at most 112 GiB (120,259,084,288 bytes) and every other
    rule holds; OOM at `-m112` (censored) above 112 GiB or with exit 137 and `oom_kill` > 0; LOST with exit 137 and no
    oom evidence. It takes the `-m112` slot and Amendment 14's relaunch accounting: no `-m112` run follows it; if it
    is LOST with relaunch `auto`, `-m112-r1` follows when the cell's relaunch is unused.
  - **If the row has `oversize true`, Amendment 14 applies unchanged (main-session decision N4):** the 56Gi run is an
    oversize row (censored-by-limit, not the cell's row) whatever its peak, and the cell gets its `-m112` run at
    `--max-dense 27900000000`.
  - An `-m112` attempt whose peak exceeds 112 GiB is OOM (P2p), with the same source priority. **Owner-confirmed
    (owner decision 15):** a 112 GiB attempt whose own process peak exceeds 112 GiB is OOM, as the cluster would have
    killed it.
  - If no peak is observable (no pilot field and no cgroup peak file), the run is NOT_ADMITTED (P2u) and goes to the
    owner.
  - **Disclosure: the direction of the `VmHWM` bias (S5).** `VmHWM` covers only the pilot process. It leaves out what
    also counted against EKS's 56Gi cgroup limit (requests equal limits there): the 300-s `aws s3 sync` process and
    the shells, page tables and kernel memory that cannot be reclaimed. And the pilot prints it only when an F4 step
    ends, so a final rise after its last printed line is missed. Both make the pilot figure lower than what the EKS
    limit saw, so the rule leans toward admitting: a run whose pilot peak lies just below 56 GiB (within roughly the
    last 0.5 GiB, the reviewer's estimate) might have been `OOMKilled` on EKS and is admitted here. The cgroup figure
    covers the whole container, file cache included, and is recorded next to it. Both are in `outcome.json`'s
    `memory` object (`pilot_peak_bytes`, `cgroup_peak_bytes`, `cgroup_peak_source`), with the one used in
    `source_used` and `peak_bytes_used`.
- **LOST** is the analogue of a Spot loss: nothing from the run is admitted or analysed, even if a partial tree was
  fetched. Each loss is recorded with its reason in `RUNPOD_LOSSES.md`, as in `SPOT_LOSSES.md`. A LOST carries
  `relaunch: auto` or `relaunch: owner` (section 4.3). Partial trees of a LOST run are never read for any rule, its
  memory peak included (review round 3, S8): a LOST 56Gi attempt does not become an `-m112` attempt.
- **ABORTED** (a session abort of a running run) uses the attempt and grants no relaunch; any further attempt of the
  cell needs a new owner decision (main-session decision N1, owner decision 13). `run.sh` refuses the `-r1` after an
  abort.
- **NO_EVIDENCE** (P0, P0b, P7): no row and no automatic relaunch; the case goes to the owner (decision N2).
- **NOT_AN_ATTEMPT** (a failure before the runner starts) uses no relaunch. Before the start the session can see the
  pod's data center and, from the runner's ready status, the host's thread count and memory, which tell the observed
  hosts apart (48 threads and 263 GB on the first smoke host, 128 threads and 527 GB on the second;
  `status-ready.json`). A deliberate pre-start abort could therefore aim at a host class (S11). The lever stays closed:
  an abort after start uses the attempt (N1), and a pre-start failure leaves a ledger row so the name is not launched
  again (N6). Since a pre-start failure that created a pod leaves a ledger row, the name cannot be
  launched again (N6): any such failure sends the cell to the owner. Only a create that left no pod
  (`<name>.nocreate-*`) can be retried under the same name.

### 4.3 Relaunches (owner decision 13, 2026-11-15)

**One relaunch per t = 14 cell, on positive host-side evidence of loss only; an abort, a NO_EVIDENCE outcome or a loss
without host-side evidence goes to the owner; t = 13 gets none (owner decision 13).** In the route:
- **Host-side evidence** takes two forms, as the owner's question named them: (1) a RunPod-side status change recorded
  by a check-in, that is a REST `lastStatusChange` that differs from its value right after start and does not name the
  user ("by User"); (2) the pod gone from a successful list with its GET returning 404. Either counts only when recorded
  before any action the session logged for the pod, and only when tied to the loss (review round 3, R1): from a
  check-in after the last check-in that saw the runner alive under `s0`, or from the check-in that carries the first
  loss evidence, and at or before that first loss. If the status at the first-loss check-in names the user, the
  relaunch goes to the owner. If any check-in before the first loss saw the runner `done` under `s0`, the relaunch goes
  to the owner too, before or after the run deadline (R2): the session had seen the completion before the loss. Both
  forms rest on the caveat the owner accepted: RunPod keeps no
  independent record of who ended a pod, so the route relies on the session logging every delete, stop or restart
  (in `session-actions.jsonl`, by `run.sh`, `fetch.sh` and `reap.sh`) before the API call (section 3). A LOST with such
  a record has `relaunch: auto`; every other LOST has `relaunch: owner`. The "by User" test is a deny-list built on the
  one status text observed so far, "Rented by User: …" (both smokes); a session stop whose text does not contain it
  would read as RunPod-side (review round 3, S4).
- t = 14 cells: one relaunch per cell, across both sizes (as Amendment 14 counts it): `-r1` after a base LOST with
  `relaunch: auto`, or `-m112-r1` after an `-m112` attempt LOST with `relaunch: auto` if `-r1` was not used. Both
  attempts are kept.
- `pkm-x-t13-k0-c6-r3` is the further attempt that decision 10 allows. Neither it nor its `-m112` run is relaunched;
  any loss sends the cell back to the owner.

## 5. Cost guard

- **Price** (read 2026-11-14 23:24:47Z, RunPod GraphQL `cpuFlavors.specifics.securePrice`, `cpu5m-16-128`): US$1.04/h;
  the smoke pod's `costPerHr` was 1.04. `run.sh` reads it again before every create and records it in `price.jsonl`.
- **Per run.** Refused before create if price × 30 h > US$40 (section 2.6); the pod is deleted before start if
  `costPerHr` differs from that price. Worst case today US$31.20.
- **Study (S10).** `runpod_capped/ledger.tsv` is the study-wide ledger: one row per job on any route (the EKS jobs,
  `pfd-b-rp` under Amendment 16, any Amendment 17 work, this route), with the actual cost when closed and the worst case
  (30 h) when open. The main session keeps it complete and dates that with its `complete_as_of` line; `run.sh` refuses a
  study launch if that line is older than 24 h, or if the ledger sum plus the new run's worst case exceeds US$400.
  `run.sh` adds this route's row at create, and `fetch.sh` closes it with the derived cost once the deletion is
  confirmed.
- **Bounds.** Three runs: at most US$93.60. Every allowed run (3 per t = 14 cell, 2 for t = 13): at most 8 × US$31.20
  = US$249.60.
- **Recorded.** `price.jsonl`, `pod-create.json`, `pod-record.json`, `cost.json` (costPerHr × pod lifetime, derived,
  with the reason of the delete).

## 6. Admission and receipts

A run is ADMITTED only through P2d or P2m3, that is when its completion is secured, a memory peak is observable and
within the limit of section 4.2, and all of these hold (the first four are the EKS verify-card pairs):

1. `binary.sha256` has the four binary hashes pinned in `common.sh` at `/usr/local/bin/…`.
2. `host.txt` has `argv: <stage>` and `rayon: 12`.
3. `progress.txt` has exactly one `start X-m2-tT-kK-c6` line, `end X-m2-tT-kK-c6 exit 0` and
   `end X-m2-tT-kK-c6.gridzeros exit 0`, and the row file has its row.
4. `runner_state.json` has `state done`, `exit 0` and the start recorded in `start.json`.
5. `runner_request.json`'s `verified_sha256` equals the pinned hashes for the files the runner checks: its `BIN`
   list (the 4 binaries and `entry_r2.sh`) and `run.sh` (`build-info.json` is uploaded and recorded, not checked).
6. The two fetches' manifests are identical, and `local-sha256-<k>.txt` equals the kept tree when classified.
7. `pod-create.json` shows `machine.secureCloud` true, `cpu5m`, 16 vCPU, at least the memory floor, and `costPerHr`
   equal to the price in `price.jsonl`; `pod-request.json` shows `interruptible: false`, the reserved tier and
   `cpuFlavorIds ["cpu5m"]`.
8. Before the row is used (S9): `cost.json` with `confirmed_gone: true` and the `deleted and no longer listed` line
   in `fetch.log`. The delete is accepted with HTTP 200 or 204, or 404 when the pod was already gone (the deletion time
   is then an upper bound). Until the deletion is confirmed the class stands but the row is held, and `fetch.sh`
   retries the delete.

**Receipts of each run**, in `runpod_capped/receipts/<name>/` (development runs in `receipts_dev/`), never
overwritten: `run.log`, `price.jsonl`, `pod-request.json`, `boot-template.sh`, `pod-create.json`, `pod-record.json`,
`status-ready.json`, `upload.jsonl`, `prestart-results-files.txt`, `cgroup-prestart.json`, `pod-state.json`,
`start.json`, `pod-record-start.json`; per check-in a line in `checkins.jsonl` with `status-<stamp>.json` and
`rest-<stamp>.json`; `fetch.log`, `session-actions.jsonl`; per collection `collect-started-<k>.json`,
`transfer-<k>.json`, `local-sha256-<k>.txt`, `tarball-<k>.sha256`, `admission-lines-<k>.txt`, `collect-<k>.json`;
`outcome.json`, `upload-check.txt`; `pod-record-final-<stamp>.json`, `delete.http`, `delete-<stamp>-<n>.response`,
`pods-after-delete-<stamp>.jsonl`, `cost.json`. The `.log` receipts are re-included in git by the route's
`.gitignore` (the local `.git/info/exclude` would hide them). The account identifier `consumerUserId` and the pod
environment are removed from every saved API record. The results trees `results/<name>/collect-<k>/` go into the job's
`bench.record/v1` with S3 locations by SHA-256. Records kept as on EKS: the resume checkpoint before launch, the
director's launch and verify cards, and their receipts.

## 7. Host dependence

- **The bracket readings depend on the host.** They use degrees, `oversize` and `timed_out`, and `--budget` is
  wall-clock, so whether a run reaches its budget depends on the CPU. At t = 14 this can decide the reading: the cap 6
  wall time went from 1,762.8 s at t = 12 (Xeon 8488C) to 11,570 s at t = 13 (Xeon 6975P-C), a factor of 6.56 across
  different hosts, and the same factor gives about 75,900 s at t = 14, within 12% of 86,400 s (derived, from the
  first review).
- One flavor is pinned for every study run, with no fallback (section 2.1), **but the flavor does not pin the CPU
  model**: the two development smokes on `cpu5m` ran on an AMD Ryzen Threadripper 7960X and on an AMD EPYC 9575F. The
  two t = 14 draws, and any `-r1`, may land on different CPU models, and at t = 14 the CPU model can decide whether a
  run reaches its budget (`timed_out`), since the projected wall time (about 75,900 s, a derived estimate) is within
  12% of 86,400 s. The CPU model of each run is recorded from `host.txt`, and the report states it next to the bracket
  reading for every row. No rule changes because of it.
- **The t = 13 median mixes hosts (S10).** Its draw 1 is the EKS row (Xeon 6975P-C, 12-CPU pod, Y4); its draw 0 would
  be this route's `-r3` row (a RunPod `cpu5m` host, 12 threads on 16 vCPU). The report states both hosts next to the
  t = 13 reading.
- A censored row is reported with its host, never dropped. `muladds` are compared only between rows with the same
  thread count (Amendment 9); the thread count here is 12.
- **Memory request versus pod memory.** Every run here has a 128 GB pod, whereas the registered request of the 56Gi
  runs is 56Gi. `--max-dense` stops a run at the same point as on EKS. The memory rule of section 4.2 handles a 56Gi
  run whose peak exceeds 56 GiB.

## 8. Unchanged

- Binaries (static musl builds of `f13a690df016d0a457bd40579fbb9d054f486adf`, by SHA-256), commands, stages, cells,
  draws, caps; `--budget 86400`; `--max-dense` per Amendment 14; 12 threads.
- Hypotheses and §6 rules; Amendments 7 B, 9 and 14 (the OOM branch and the oversize path included).
- The US$40 per job and US$400 study ceilings.
- The requests in `requests_r2i/`, which are not edited. Their `provenance.protocol_sha256` (`69af89b7…`) predates
  Amendments 16-18; the protocol in force at each launch is logged by `run.sh` (SHA-256 of the preregistration and
  the git HEAD).

## 9. Points settled, and checks that need a pod

1. Relaunches (section 4.3): owner decision 13, 2026-11-15.
2. NOT_ADMITTED (section 4.1): accepted by the main session on 2026-11-15.
3. Enforcement of the deadline and the US$40 ceiling by session check-ins and the scheduled `reap.sh`, with no
   enforcement independent of Claude sessions (section 2.6): owner decision 14, 2026-11-15.
4. The memory rule (section 4.2): the main session's decisions of 2026-11-15 (the rule, the pilot `VmHWM` as the
   primary peak source, and N4 for `oversize true` rows); P2p, the 112 GiB step for `-m112` attempts, confirmed by
   owner decision 15, 2026-11-15.
5. Checks made on a pod. The second development smoke (2026-11-15, scripts `a18-r3`; `SMOKE.md`) showed: the new
   start command boots and the runner answers; cgroup v2 with `memory.max` 128,000,000,000; `lastStatusChange` at
   creation reads "Rented by User: …"; the create response carries `machine.secureCloud` and `costPerHr`; the
   `--delete RECORD` path after a `benchrec` upload; `reap.sh` on the live list. Still not observed:
   - whether `lastStartedAt` changes on a container restart;
   - what `GET /pods/{id}` returns for a deleted pod (form 2 of section 4.3 assumes 404) and for a live pod's `env`
     (the token recovery assumes it; no receipt shows it, `SMOKE.md`), and whether `GET /pods` is complete;
   - the text of a RunPod-side status change;
   - whether the pod sees `RUNPOD_POD_ID` and a pod-scoped key (S12; recorded only).
6. Offline tests: `tests/README.md` (every refusal path, the classifier on synthetic receipt sets for each class and
   for both reviews' conflicting cases, the order, the reaper), with the count of cases that need the image layer.

## 10. Script hashes at this revision

Recorded again at the freeze, after the development smoke (any edit made after it changes these). At the freeze the
first four lines below go into the preregistration as `sha256sum` output; `run.sh` and `fetch.sh` refuse study runs
otherwise.

| file | SHA-256 |
|:--|:--|
| `common.sh` | `5c29eb2ab71ed4a33aed5d9d1d88664ab8726fa94cd493e03704971c3df68f24` |
| `run.sh` | `c6a191f57e51402f681a2c2826f1f43b765e81ac088afd215ff5d2cc414b7bfb` |
| `fetch.sh` | `cec7c7b734d9a55de3c53df5baf0a7c9318424d0280f58083c1a846db6933a19` |
| `reap.sh` | `686307f32a84a268183959345b4f6ecd21ba772fc77f3c01cb06fee9d1e77d3a` |
| `make_requests_m112.sh` | `1459e3d3ee821133e936cc40671946b94b6304413325d4d247e29c234e16d58b` |
| `rp_pinned.sh` | `b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d` (= main's `runpod/rp.sh` at `d2a16c723d`) |
| `runpod/server.py` (pinned, not copied) | `f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550` |
