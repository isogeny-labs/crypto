# DRAFT Amendment 33: `t = 14` draw 0 cap 7 at Amendment 14's `-m112` setting, one capped run on a reserved RunPod CPU pod (REST v2, results in git)

**Status: draft revision 5, NOT frozen (2026-11-21).** This is the route reviewed three times as draft Amendment 18
(revisions 1-4, `reviews/`), ported to RunPod REST v2 and re-targeted to one cell by owner decisions 20-23. When frozen
it is appended to `research/prime_fall_degree_20261027/PREREGISTRATION.md` as Amendment 33 (main ends at Amendment 32),
with the SHA-256 of the four route scripts (section 10), before the job below is started. `run.sh` refuses the study
names until the preregistration has the heading `## Amendment 33` and those hashes. Every number is `[pcc:unverified]`.
The decisions this text relies on, with their dates and reasons, are listed in `runpod_capped/DECISIONS_ROUTE.md`.

**Not launched (2026-11-21, main session).** The cell is already measured. `results/SNAPSHOT_20261117/README.md` (a
third session, 2026-11-17) records that pod `pfd-x-t14-k0-c7-m112-rp` (stage `cap:14:0:7`) ran on 2026-11-16 to state
`done`, exit 0, and that its results were fetched and committed: `inconsistent false`, `oversize true`, `timed_out
true`, `degree_reached 7`, grid zeros 0; draw 1 gave the same. Under Amendment 31's route-F reading that is censored,
and `d_F >= 7` stands. Amendment 32's "No row was fetched" does not hold for these two pods. Another run of the same
stage, with binaries admitted as equivalent and the same `--max-dense`, would be expected to stop the same way, so this
route is not launched for it. The port is kept as a reviewed record. Owner decision 23 rested on the main session's
statement that the cell was unmeasured; that statement was wrong.

## 0. History, status and an open owner question

- **2026-11-14 to 15: draft Amendment 18, revisions 1-4.** Three independent red-team reviews (`59e487150f`,
  `7305d2ee8d`, `a1c9a4b79c`) and one validation (`c895a6fd38`); two development smokes (`SMOKE.md`).
- **2026-11-15 07:50Z: stood down.** Another session had committed its own Amendments 18-20 for the same cells and
  launched pods for them. Running this route as well would have run the decisive cells twice, so it was not launched.
- **2026-11-15 to 18: the other session's route.** Cap 6 at `t = 14` completed without refuting on both draws
  (`d_F >= 7`); cap 7 at 56Gi stopped `oversize` at degree 6 on both draws (Amendment 30); Amendment 31 launched route F
  (`pfd-x-t14-k{0,1}-c7-m112-rp`, the registered `-m112` step) and route C. Amendment 32 records that batch as lost,
  "No row was fetched", at a measured US$370.84: every pod stayed billed about 30.5 h because nothing deleted it when
  its job ended. Amendment 32 holds every relaunch until the owner raises the ceiling or picks a subset, and asks for a
  client that deletes each pod when its job ends. This route already has that (`fetch.sh`, `reap.sh`, section 2.6).
- **2026-11-21: owner decisions 20-23.** Run exactly one cell on this route within the budget (20); study ceiling
  US$540 and spend guard US$510, per-job cap US$40 (21); run without S3, results into git (22); the cell is `t = 14`,
  draw 0, cap 7 at the `-m112` setting, stage `cap:14:0:7:27900000000` (23).
- **Open owner question, to settle before the freeze.** `results/SNAPSHOT_20261117/README.md`, written by a third
  session on 2026-11-17 at the owner's request, records that the runners of both route-F pods reported `state done`,
  `exit 0`, and that their result archives were fetched and committed under `results/SNAPSHOT_20261117/` before the
  pods were deleted; it states the row fields of both draws. That is at odds with Amendment 32's "No row was fetched".
  Whether those archives are an attempt of this cell (and so whether this run would repeat a completed attempt) is for
  the owner; nothing here reads or scores them. The port session read that README while searching for pod ids for the
  ledger (disclosure below).

## 1. The job, its relaunch and the request

| name | when it may run | request | stage |
|:--|:--|:--|:--|
| `pkm-x-t14-k0-c7-m112` | first and only | `runpod_capped/requests_m112/pfd-x-t14-k0-c7-m112.json` (SHA-256 `44e009f18d19a9c61e5a281e0c2a6a219c4f3cc3252ca63bff71049fdef4c7f9`, pinned in `common.sh`) | `cap:14:0:7:27900000000` |
| `pkm-x-t14-k0-c7-m112-r1` | only after the base is LOST with relaunch `auto` (owner decision 13; section 4.3) | same | same |

- Kummer `m = 2`, P1, `t = 14`, draw 0, cap 7, `--budget 86400` (Amendment 7 B), `--max-dense 27900000000`
  (Amendment 14's `-m112` value), `RAYON_NUM_THREADS = 12` (Amendment 9).
- `run.sh` refuses every other study name and every other stage for these names (exit 3, before any API call).
  Development names `pkm-x-dev-<word>` stay, with `t <= 10`, `--max-dense` 12800000000 or 27900000000, receipts in
  `receipts_dev/`; they are not data.
- **A name is launched at most once, ever.** `run.sh` refuses a name that already has a receipts directory, a results
  directory, or any row in `ledger.tsv` (decision N6).
- **The request.** No frozen `pfd-x-t14-k0-c7-m112` request exists on main (searched 2026-11-21: `requests*/`,
  `runpod/`, `results/` and the study's text files; the other session launched its cap-7 jobs from stage strings).
  `make_requests_m112.sh` derives it from the frozen `requests_r2i/pfd-x-t14-k0-c6.json` exactly as Amendments 7 B and
  14 say. Changed paths (`make_requests_m112_output.txt`): `job.command.1` `cap:14:0:6:12800000000` to
  `cap:14:0:7:27900000000` (cap 6 to 7, and the `-m112` `--max-dense`); `job.memory` 56Gi to 112Gi; `job.job_name`
  `pfd-x-t14-k0-c6-20261108` to `pfd-x-t14-k0-c7-m112-20261108`; `provenance.note` with one sentence appended (1,984
  characters, within the validator's 2,048). Every other field is copied, including `provenance.protocol_sha256`; the
  copied part of the note still names the cap-6, 56Gi values, and the appended sentence states the new ones. It passes
  the request validator the `requests_r2i` requests passed (`requests_m112_validate_output.txt`). `run.sh` checks the
  file's SHA-256 against the pin, re-derives it and refuses if they differ. The three cap-6 `-m112` requests of draft
  Amendment 18 were removed with their names.
- **Reading (Amendment 7 B, as Amendment 31 route F states it).** Refutes: `d_F = 7`. Completes without refuting:
  `d_F >= 8`. `oversize` or a memory kill at 112Gi: censored, and `d_F >= 7` stands. A budget stop (`timed_out true`)
  is censored as Amendment 7 B says. No further bracket step follows from this amendment in either direction.

## 2. What changes

1. **Host: one pinned flavor, no fallback (decision D8).** `cpu5m` at 16 vCPU and 128 GB, image
   `python:3.12-slim@sha256:5e2bdb1b0d542d5039cffb8181c65930093949194a65c365dfb1b0fe13a4fa5b` (main's pinned digest of
   the tag the smokes used), 20 GB disk, cloud tier `SECURE`. RunPod REST v2 has no interruptible or spot field for pods,
   so a v2 pod in that tier is on-demand [pcc:unverified]. If the catalog shows `cpu5m` without stock, `run.sh` exits 10 without creating
   anything, and no attempt is charged. The create body (REST v2, main's `runpod/rp.sh` shape):

   ```
   {name, cloud: "SECURE", cpu: {id: "cpu5m", vcpuCount: 16}, image, disk: 20, ports: ["8000/http"],
    env: {RUNNER_TOKEN}, entrypoint: ["bash", "-c"], cmd: [<boot script>]}
   ```

   - The create must answer HTTP 201. `run.sh` then checks, before anything runs: on the create response `cloud`
     `SECURE`, `cpu.id` `cpu5m`, `cpu.vcpuCount` 16, no `gpu`, and `cost` equal (to 4 decimals) to the catalog price
     read just before create; on the GET record taken right after, the same and `cpu.memory` at least 121 GB
     (112 GiB); on the record taken once the runner answers, `status` `RUNNING` and a data center id (v2 documents
     `dataCenterId` as assigned by the scheduler, so it may be null at create). `pod-request.json` must show `cloud`
     `SECURE`, `cpu {id, vcpuCount}` and no `interruptible` key. Any failed check deletes the pod (exit 4) [pcc:unverified]. The v1
     fields these checks used (`machine.secureCloud`, `costPerHr`, `cpuFlavorId`, `vcpuCount`, `memoryInGb`,
     `machine.dataCenterId`) and their v2 replacements are listed in `V2_FIELDS.md`.
   - **The pod's cgroup memory limit is read before start (S4).** The start command's copy loop (item 4) writes the
     cgroup files into `/results/cgroup/` before anything else exists there; `run.sh` fetches that tree once before
     `/start` and records the limit in `cgroup-prestart.json`. It must be `max` or at least 112 GiB (120,259,084,288
     bytes; the second smoke pod showed 128,000,000,000), so that an OOM kill here means what Amendment 14's
     `OOMKilled` means at the `-m112` size. Otherwise, or if no limit file exists, the pod is deleted before start and
     the case goes to the owner.
2. **Threads (S13).** `rayon 12` on a pod whose affinity gives `nproc 16`. `muladds` does not depend on this, because
   the F4 chunk size is set from the rayon thread count (`src/cryptanalysis/f4_fp_tower.rs:1694` at `f13a690df0`);
   wall time can.
3. **Binaries and entry script.** The frozen image's study layer `a0f7e28e…` (`image/image.json` `layer.digest`): the
   four static binaries built from `f13a690df016d0a457bd40579fbb9d054f486adf` and `entry_r2.sh`, by the SHA-256 pinned
   in `common.sh`, not Amendment 17's rebuild. Both sets are admitted (Amendment 17 admitted the rebuild by exact
   reproduction; the frozen layer is the original). `run.sh` refuses `SHA_FILE` (the rebuild's hash file). The jobs
   invoke `/usr/local/bin/entry_r2.sh STAGE` where the request names `entry.sh`; for a `cap:` stage the commands, file
   names and `progress.txt` lines are the same: `entry_r2.sh` differs only in its header comment and the added `m4n:`
   branch (checked with `cmp` on the three extracted regions, and by both earlier reviews).
4. **Pod start command (decision D4).** `bash -c` with a short script (recorded per run as `boot-template.sh`, with its
   SHA-256 in `pod-request.json`) that copies the cgroup memory files into `/results/cgroup/` at once, every 30 s, and
   once more within 5 s after `runner_state.json` appears (cgroup v2 `memory.max`, `memory.peak`, `memory.events`, or the
   v1 files), records `/proc/self/cgroup` and the cgroup file-system type, then `exec`s the runner. The runner is
   `server_pinned.py`, a vendored, unchanged copy of `runpod/server.py` at `d2a16c723d` (SHA-256 `f56fd08e…`), the
   version this route was reviewed with; main's `runpod/server.py` has since changed (`c6d8afb1…`) and is not used.
5. **Results path and durability (S2, S8; owner decision 22).**
   - `entry_r2.sh`'s `aws s3` calls go to the runner's shim, which copies into the pod's `/results` (Amendment 16).
   - Each collection attempt `k` first writes an intent record `collect-started-<k>.json`. `fetch.sh` then downloads
     `/results.tar.gz` twice through the pod's proxy URL, extracts both and compares the per-file SHA-256 manifests,
     writes `transfer-<k>.json`, `local-sha256-<k>.txt` and `tarball-<k>.sha256`, keeps one tree in
     `runpod_capped/results/<name>/collect-<k>/`, then writes `admission-lines-<k>.txt` and `collect-<k>.json`. Nothing
     is overwritten or removed.
   - **No S3 (owner decision 22, a disclosed deviation).** The study results trees are not git-ignored (development
     trees are). Before a completed run's pod is deleted the session commits the kept tree and the receipts and pushes
     them; `fetch.sh NAME --delete REF` deletes the pod only when the git gate passes: every file of the kept tree,
     `outcome.json` and `local-sha256-<k>.txt` are in commit `REF` with the blob hash of the local file (`git ls-tree -r`
     against `git hash-object`), each local file still has its manifest SHA-256, and `REF` is an ancestor of a branch
     head that `git ls-remote --heads origin` lists at that moment. Each check is logged in `git-gate.txt`. The pod
     waits for that at most 2 h after the collection, and never past the hard deadline; then it is deleted regardless
     (the tree is local). The run's `bench.record/v1` stays a Git record with no S3 upload. In place of the S3 resume
     checkpoint, the session's resume note is committed and pushed before the launch and after every check-in.
6. **Time limit and the backstop (decision D3, owner decision 14).**
   - The pilot's own `--budget 86400` is unchanged. The registered job limit of 90,000 s becomes the **run deadline**:
     the runner's recorded start (`start.json`) + 90,000 s. Before `/start`, `run.sh` writes `pod-state.json` with the
     creation time, the **hard deadline** (creation + 26 h) and a provisional run deadline. It starts the runner only
     within 55 minutes of creation.
   - **No information is never a loss (decision N2).** An unreachable runner, a session network outage or a failed
     transfer check changes nothing; `fetch.sh` retries at the next check-in, also after the run deadline, until the
     hard deadline. A healthy `RUNNING` REST record always means "wait". At the hard deadline the pod is deleted for cost
     after one more collection attempt; with no evidence the class is NO_EVIDENCE.
   - Check-ins: one about 4 h after start, then at intervals of at most 4 h, one at the run deadline, and at least one
     between the run deadline and the hard deadline.
   - **Backstop: `reap.sh`.** It lists the account's pods (every v2 page) and deletes a `pkm-x-` pod only past its hard
     deadline, first through `fetch.sh` (collect, classify, delete), then directly if the pod is still listed; a pod
     whose deadline cannot be read is reported, not deleted. It takes the same per-name lock as `run.sh` and `fetch.sh`,
     logs every delete before the API call, and appends one JSON line per `pkm-x-` pod to `reaper/reap-YYYYMMDD.jsonl`.
     The main session schedules it at intervals of at most 4 h while a study pod exists, reads that JSONL after every
     pass, and repeats a pass that skipped a due pod for a held lock.
   - **One worst-case pod life (S7): 30 h** (hard deadline 26 h plus one reaper interval), US$31.20 at US$1.04/h.
   - **The deadline and the US$40 cap are enforced by session check-ins and the scheduled reaper; there is no
     enforcement independent of Claude sessions (owner decision 14).** Amendment 32's loss is the case this guards
     against: a pod whose job ended and that nothing deleted.
7. **Host lines and memory.** `host.txt` is the frozen script's. The pod id, flavor, `cost`, data center and the CPU
   model (from `host.txt`) are recorded with the row. Pinning `cpu5m` does not pin the CPU model: the two development
   smokes ran on an AMD Ryzen Threadripper 7960X and an AMD EPYC 9575F. v2 has no machine id field.

## 3. Route files and procedure

All in `research/pkm_falldegree_unblock_20261107/runpod_capped/`, shell only (bash, jq, curl, awk, coreutils, git; no
Python): `common.sh` (settings, pins, allow-list, order, ledger, the v2 list), `run.sh` (launch), `fetch.sh`
(check-in, collection, the single outcome classifier, the git gate, deletion), `reap.sh` (backstop), `rp_pinned.sh`
(client copy for the pod proxy), `server_pinned.py` (runner copy), `make_requests_m112.sh` and `requests_m112/`,
`ledger.tsv`, `billing_v2_20261121.tsv`, `V2_FIELDS.md`, `tests/`, `DECISIONS_ROUTE.md`. `SMOKE.md` records the
development checks.

- **`run.sh NAME STAGE`** refuses, before any API call (exit 3): a name off the allow-list; an existing receipts or
  results directory, a ledger row, or two earlier pre-start failures for the name; a study name the order does not allow
  now (`next_allowed`: the base, then its relaunch only after a LOST with relaunch `auto`); a stage other than
  `cap:14:0:7:27900000000` for a study name; a request whose SHA-256 differs from the pin or from its derivation; a
  preregistration without `## Amendment 33` or without the four route hashes; `SHA_FILE` set; a client or runner whose
  SHA-256 differs from its pin; binaries, `entry_r2.sh`, `run.sh`, `image/binaries.sha256`, `image/image.json` or
  `build-info.json` disagreeing with the pins, and for a study name a missing or different `RPC_LAYER`; a ledger that is
  missing, malformed or (for a study name) dated more than 24 h ago; `RPC_STATE` inside the git worktree. Then, with the
  API: refuse if any `pkm-x-` pod is listed (serial rule); read the catalog price and stock (exit 10 without stock);
  refuse if price × 30 h > US$40 or if the ledger sum plus this worst case exceeds US$510 (section 5). Then it creates
  the receipts directory, sets the cleanup trap before the create call, creates the pod, checks it (section 2.1), waits
  for the runner, uploads the 7 files with `rp_pinned.sh` and compares each pod-side SHA-256, reads the cgroup limit,
  writes `pod-state.json` and starts the runner. A failure before the start deletes the pod (logged first), confirms by
  listing, writes `cost.json` and an outcome `NOT_AN_ATTEMPT`, and moves the receipts aside (exit 4, 5, or 11 if the
  deletion could not be confirmed).
- **`fetch.sh NAME [--abort | --delete REF]`** takes one check-in (runner status through the proxy; the REST v2 record
  with `status` and `startedAt`; list membership; one line in `checkins.jsonl`), decides with the classifier whether to
  collect, collects (section 2.5), writes `outcome.json` once, waits for the git gate when the run completed, and
  deletes (logged first, with the reason). It refuses (exit 9) when `cost.json` exists, resumes an unconfirmed delete,
  recovers an orphan collection (decision N3b), and recovers a missing pod token from the REST record's `env` into
  `RPC_STATE` only. For a study name it refuses unless its own and `common.sh`'s SHA-256 are in the frozen amendment.
- **After every launch and every `fetch.sh` call the session commits and pushes the run's receipts and `ledger.tsv`
  (decision N6),** and after a completed collection also the kept tree (section 2.5).
- **The session makes no `status` or `fetch` call with a pod token outside `run.sh` and `fetch.sh`, and no delete, stop
  or restart outside `run.sh`, `fetch.sh` and `reap.sh`, which log each delete with its reason before the call.**
- **Tokens and keys (S4).** The API key is in a mode-0600 header file in `RPC_STATE`, removed on exit, never in argv
  or receipts. The pod token goes to the create call inside a 0600 body file. Remaining exposures: `rp_pinned.sh`
  sends it as `-H "X-Token: …"` in curl's argv inside this single-user container; it sits in the pod's environment,
  readable through the REST record by anyone holding the shared account key. The runner refuses every upload and
  second start after `/start`.
- **The pod-proxy calls do not use the REST API.** `rp_pinned.sh upload | start | status | fetch` talk to
  `https://<pod id>-8000.proxy.runpod.net` with the pod token, so the v2 port does not change them. The pinned copy's
  v1 `create`, `pod` and `delete` commands are never called (offline test G-rpuse).

## 4. Outcome classes, relaunches and limits

### 4.1 One outcome table (decision D1; classifier version `a33-r1`)

`fetch.sh` holds the only classifier. It reads receipts only: `start.json` (the runner's recorded start `s0`; a
zero-byte file is read as absent), the REST records (`pod-create.json`, `pod-record.json`, `pod-record-start.json`),
every check-in, the session's logged actions, the collection records and the kept trees. Every `outcome.json` records
the classifier version and the SHA-256 of `fetch.sh` and `common.sh`. An unparsable receipt fails closed (P0).

**Positive evidence of loss** is any of: a reachable runner that reports `idle` after its start, or a start other
than `s0`; a REST v2 record (HTTP 200) whose `status` is not `RUNNING` (for example `EXITED`, `ERROR`, `STARTING`); the
pod missing from a successful list **and** its GET returning 404; more than one `start` line for the cell, or a
`runner_state.json` start other than `s0`. A changed `startedAt` alone is recorded but is not evidence (S1). An
unreachable runner, an unreadable REST record or a failed transfer check is no information.

A **completion is secured** when the collection reason is `done`, the two fetches agree, the pre-fetch check-in shows
the runner `done` under `s0`, no loss evidence is dated at or before that check-in, and the tree's `runner_state.json`
agrees (state, start, exit) with one start line.

| step | condition | class |
|:--|:--|:--|
| P0 | a receipt the classifier reads is unparsable | NO_EVIDENCE |
| P0b | two collections made with the runner done have different trees | NO_EVIDENCE |
| P1 | no receipt shows a runner start | NOT_AN_ATTEMPT |
| P2a | completion secured, runner ended after the run deadline | LIMIT |
| P2b | completion secured, an `end` line of the cell has exit 137, cgroup `oom_kill` > 0 | OOM |
| P2c | completion secured, an `end` line has exit 137, `oom_kill` 0 or no cgroup file | LOST |
| P2p | completion secured, a 112Gi attempt whose memory peak exceeds 112 GiB | OOM (owner decision 15) |
| P2u | completion secured, no memory peak observable | NOT_ADMITTED |
| P2d | completion secured, admission rules 1-7 hold (section 6) | ADMITTED |
| P2e | completion secured, otherwise | NOT_ADMITTED |
| P3 | positive evidence of loss dated before the run deadline | LOST |
| P4 | a check-in at or after the run deadline shows the runner running, or done after the deadline, under `s0` | LIMIT |
| P5 | collected on `--abort` | ABORTED |
| P6 | positive evidence of loss dated at or after the run deadline | LOST |
| P7 | collected (at the hard deadline) with none of the above | NO_EVIDENCE |
| P8 | not collected yet | (pending) |

Draft Amendment 18's steps P2m1-P2m4 (a 56Gi attempt counted as the cell's `-m112` attempt) are removed: the attempt
here is the `-m112` attempt itself; the cell's 56Gi attempt was the other session's (Amendment 30).

### 4.2 What each class means

- **ADMITTED.** The run's row is the cell's row; its `oversize` and `timed_out` act as Amendments 7 B and 14 say. At
  this size `oversize true` or `timed_out true` is censored, and `d_F >= 7` stands (section 1).
- **LIMIT** mirrors an EKS job killed at its time limit: censored, reported with its host, not re-run, no relaunch.
  The run deadline counts from the runner's start, not from the pod's (at most 55 minutes later).
- **OOM** is Amendment 14's `OOMKilled` at the `-m112` size: censored. Exit 137 is classified by the cgroup evidence,
  never by the exit code alone.
- **Memory rule.** The run's memory peak is the pilot's own field (the largest `memory R MB (peak P MB)` value in
  `runs/<cell>.log`, the process's `VmHWM` in MiB, × 1,048,576), else the cgroup figure (v2 `memory.peak`, else v1
  `memory.max_usage_in_bytes`); both are recorded in `outcome.json`. A peak above 112 GiB is OOM (P2p, owner decision
  15). No observable peak is NOT_ADMITTED (P2u). The pod has 128 GB, so the run can exceed 112 GiB without a kill; P2p
  censors it as the cluster would have. Disclosure (S5): `VmHWM` leaves out other processes and kernel memory and is
  printed only when an F4 step ends, so the rule leans toward admitting a run whose pilot peak lies just below 112 GiB.
- **LOST** is the analogue of a Spot loss: nothing from the run is admitted or analysed. A LOST carries `relaunch:
  auto` or `relaunch: owner` (section 4.3), and each loss is recorded with its reason.
- **ABORTED** uses the attempt and grants no relaunch (decision N1).
- **NO_EVIDENCE**: no row, no automatic relaunch; the owner decides (decision N2).
- **NOT_AN_ATTEMPT** (a failure before the runner starts) uses no relaunch, but a pre-start failure that created a pod
  leaves a ledger row, so the name is not launched again (N6) and the cell goes to the owner. Only a create that left no
  pod can be retried under the same name.

### 4.3 Relaunch (owner decision 13)

One relaunch, `pkm-x-t14-k0-c7-m112-r1`, only on positive host-side evidence of loss; an abort, a NO_EVIDENCE outcome
or a loss without host-side evidence goes to the owner. **REST v2 can show one form of host-side evidence: the pod gone
from a successful list with its GET returning 404,** recorded before any action the session logged for the pod and tied
to the loss (from a check-in after the last one that saw the runner alive under `s0`, or from the first-loss check-in,
and at or before the first loss). If any check-in before the loss saw the runner `done` under `s0`, the relaunch goes to
the owner. Draft Amendment 18 also counted a RunPod-side `lastStatusChange`; v2 has no such field, so that form can no
longer be observed and never counts (fail closed: fewer automatic relaunches, more cases for the owner). RunPod keeps no
independent record of who ended a pod, so the route relies on the session logging every delete before its API call.

## 5. Cost guard

- **Price** (read 2026-11-21, REST v2 `GET /catalog/cpus/cpu5m?include=AVAILABILITY&product=POD&vcpuCount=16`):
  `price.securePerVcpu` 0.065 × 16 = US$1.04/h, availability `HIGH`. `run.sh` reads it again before the create and
  records it in `price.jsonl`; the pod's `cost` must equal it.
- **Per job.** Refused before create if price × 30 h > US$40; worst case today US$31.20.
- **Study (owner decision 21): guard US$510, ceiling US$540.** `ledger.tsv` is the study-wide ledger: the EKS rows, every
  RunPod pod of the study with RunPod's own billing (REST v2 `GET /billing/pods`, read 2026-11-21,
  `billing_v2_20261121.tsv`), the Amendment 31 batch (eight rows, US$370.84, PREREGISTRATION.md Amendment 32), and one
  row that brings the spend before Amendment 31 to Amendment 32's "about US$95" in place of pods whose ids are in no
  study file. Sum 465.84 (`ledger_SOURCES.md` section 5). `run.sh` refuses the launch if that sum plus the job's worst
  case exceeds US$510: today 465.84 + 31.20 = 497.04. Two pods that the 2026-11-17 snapshot lists among the study's
  (US$72.96 billed) are not rows; counted, they would make the guard refuse. The owner decides whether they belong here.
- **Recorded.** `price.jsonl`, `pod-create.json`, `pod-record.json`, `cost.json` (cost per hour × pod lifetime, derived,
  with the reason of the delete).

## 6. Admission and receipts

A run is ADMITTED only through P2d, that is when its completion is secured, a memory peak is observable and at most
112 GiB, and all of these hold:

1. `binary.sha256` has the four binary hashes pinned in `common.sh` at `/usr/local/bin/…`.
2. `host.txt` has `argv: cap:14:0:7:27900000000` and `rayon: 12`.
3. `progress.txt` has exactly one `start X-m2-t14-k0-c7` line, `end X-m2-t14-k0-c7 exit 0` and
   `end X-m2-t14-k0-c7.gridzeros exit 0`, and the row file has its row.
4. `runner_state.json` has `state done`, `exit 0` and the start recorded in `start.json`.
5. `runner_request.json`'s `verified_sha256` equals the pinned hashes for the files the runner checks.
6. The two fetches' manifests are identical, and `local-sha256-<k>.txt` equals the kept tree when classified.
7. [pcc:unverified] `pod-create.json` shows `cloud` `SECURE`, `cpu.id` `cpu5m`, `cpu.vcpuCount` 16 and `cost` equal to the price in
   `price.jsonl`; `pod-record.json` shows `cpu.memory` at least 121 GB; `pod-request.json` shows `cloud` `SECURE`,
   `cpu {id: cpu5m, vcpuCount: 16}` and no `interruptible` key.
8. Before the row is used: `cost.json` with `confirmed_gone: true`. The delete is accepted with HTTP 200 or 204, or 404
   when the pod was already gone (the deletion time is then an upper bound).

**Receipts**, in `runpod_capped/receipts/<name>/`, never overwritten: `run.log`, `price.jsonl`, `pod-request.json`,
`boot-template.sh`, `pod-create.json`, `pod-record.json`, `pod-record-ready.json`, `status-ready.json`, `upload.jsonl`,
`prestart-results-files.txt`, `cgroup-prestart.json`, `pod-state.json`, `start.json`, `pod-record-start.json`;
`checkins.jsonl` with `status-<stamp>.json` and `rest-<stamp>.json`; `fetch.log`, `session-actions.jsonl`; per
collection `collect-started-<k>.json`, `transfer-<k>.json`, `local-sha256-<k>.txt`, `tarball-<k>.sha256`,
`admission-lines-<k>.txt`, `collect-<k>.json`; `outcome.json`, `git-gate.txt`; `pod-record-final-<stamp>.json`,
`delete.http`, `delete-<stamp>-<n>.response`, `pods-after-delete-<stamp>.jsonl`, `cost.json`. Saved API records drop
`env`, `args`, `cmd`, `entrypoint`, `ssh` and any account identifier. The results tree `results/<name>/collect-<k>/` is
committed with them (section 2.5).

## 7. Host dependence

- The bracket reading uses degrees, `oversize` and `timed_out`; `--budget` is wall-clock, so whether the run reaches
  its budget depends on the CPU, which `cpu5m` does not pin. The CPU model is recorded from `host.txt` and reported next
  to the reading.
- The cell's earlier attempts (cap 6 and cap 7 at 56Gi) ran on the other session's RunPod pods; this run may land on a
  different CPU model. No rule changes because of it.
- `muladds` are compared only between rows with the same thread count (12).

## 8. Unchanged

- Binaries (frozen layer `a0f7e28e…`, static musl builds of `f13a690df0`), the `cap:` stage's command, the cell, seed,
  draw, `--budget 86400`, `--max-dense 27900000000`, 12 threads.
- Hypotheses and §6 rules; Amendments 7 B, 9 and 14; owner decision 15.
- The US$40 per-job cap. The requests in `requests_r2i/` are not edited.

## 9. Points settled, and checks that need a pod

1. Relaunch: owner decision 13; with v2, the pod-gone form only (section 4.3).
2. Enforcement by session check-ins and the reaper: owner decision 14.
3. The 112 GiB step: owner decision 15.
4. One cell, the budget, no S3, the cell itself: owner decisions 20-23 (2026-11-21).
5. Checks made on a pod (the two smokes of 2026-11-14 and 15, REST v1; `SMOKE.md`): the start command boots, the runner
   answers, cgroup v2 with `memory.max` 128,000,000,000, two CPU models under `cpu5m`. **Not yet seen with REST v2,
   to be recorded at the development smoke at t = 8 the main session runs after review** (`V2_FIELDS.md`): the create
   response's fields and HTTP code; the live GET record (`cpu.memory`, `status`, `dataCenterId`, `startedAt`, `env`);
   the proxy URL and runner on a v2-created pod with the bash entrypoint and the pinned image digest; the cgroup files;
   `DELETE` (204), the list afterwards and the GET of a deleted pod (404).
6. Offline tests: `tests/README.md`.

**Disclosure (what the authors had seen).** Revisions 1-4: every `t = 12` and `t = 13` row and the partial log of the
lost third `t = 13` draw 0 attempt (`SPOT_LOSSES.md`). This revision: the committed text of Amendments 18-32, which
states that cap 5 and cap 6 at `t = 14` do not refute on either draw and that cap 7 at 56Gi stopped `oversize` at
degree 6 on both draws; and, read by the port session on 2026-11-21 while looking for pod ids for the ledger,
`results/SNAPSHOT_20261117/README.md`, which states the route-F row fields at cap 7 and 112Gi for both draws (`oversize
true`, `timed_out true`, not refuted, degree reached 7). No rule above depends on any of these: the bracket step and
its reading are fixed by Amendment 7 B.

## 10. Script hashes at this revision

Recorded again at the freeze, after the development smoke (any edit changes them). At the freeze the first four lines
go into the preregistration as `sha256sum` output; `run.sh` and `fetch.sh` refuse study runs otherwise.

| file | SHA-256 |
|:--|:--|
| `common.sh` | `183f84e9276f02981b466fb6190078b9c36f7b622f051dab5283c33f2a125ea8` |
| `run.sh` | `2815d8ad86bd5748494716143a6164a79e69fb1a836fbd4ef9590cb418dd4cc8` |
| `fetch.sh` | `0604617a9915fcdac40ae57025428e6bd29d204015222be14b50de2a0d5f3fed` |
| `reap.sh` | `2a289e60c5ae7c3fe98c757dd90ccea0975e0c5965165b433d23d2d0224ace8b` |
| `make_requests_m112.sh` | `a61862fa254f8760a15d26a1b7f9f6de8c4cc44442538d401cd2ef21ed74f912` |
| `requests_m112/pfd-x-t14-k0-c7-m112.json` | `44e009f18d19a9c61e5a281e0c2a6a219c4f3cc3252ca63bff71049fdef4c7f9` |
| `rp_pinned.sh` | `b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d` (= main's `runpod/rp.sh` at `d2a16c723d`) |
| `server_pinned.py` | `f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550` (= `runpod/server.py` at `d2a16c723d`) |
