# DRAFT Amendment 33: the capped RunPod route (REST v2, results in git) and one probe, `t = 14` draw 0 cap 7 at `--max-dense 54000000000` on a 256 GB pod

**Status: draft revision 7, NOT frozen (2026-11-21).** One amendment for two things: the reviewed capped-run route in
`runpod_capped/` (ported to RunPod REST v2) and the one job it runs, owner decision 24's probe designed in
`t14_bigmem/` (whose `DRAFT_AMENDMENT.md`, "proposed Amendment 34", is merged here and not frozen separately). When
frozen it is appended to `research/prime_fall_degree_20261027/PREREGISTRATION.md` as Amendment 33 (main ends at
Amendment 32), with the SHA-256 of the four route scripts (section 10), before the job below is started. `run.sh`
refuses the study names until the preregistration has the heading `## Amendment 33` and those hashes. Every number is
`[pcc:unverified]`. Decisions, with dates and reasons: `runpod_capped/DECISIONS_ROUTE.md`. Owner decisions:
`research/open_leads_execution_20261108/DECISIONS.md`.

## 0. History and status

- **2026-11-14 to 15: the route, draft Amendment 18, revisions 1-4.** Three independent red-team reviews (`59e487150f`,
  `7305d2ee8d`, `a1c9a4b79c`) and one validation (`c895a6fd38`); two development smokes (`SMOKE.md`).
- **2026-11-15 07:50Z: stood down.** Another session had committed its own Amendments 18-20 for the same cells and
  launched pods for them; running this route as well would have run them twice.
- **2026-11-15 to 18: the other session's route.** At `t = 14`, cap 5 and cap 6 complete without refuting on both
  draws (`d_F >= 7`); cap 7 at `--max-dense 12800000000` stopped `oversize` at degree 6 on both draws (Amendment 30);
  Amendment 31 launched route F (`-m112`, `--max-dense 27900000000`, 32 vCPU / 256 GB pods) and route C. Amendment 32
  records that batch as lost at a measured US$370.84 (pods billed about 30.5 h because nothing deleted them when their
  jobs ended) and holds every relaunch.
- **2026-11-21: owner decisions 20-23 and the port.** One cell on this route within the budget (20); study ceiling
  US$540 and guard US$510 (21); no S3, results into git (22); the cell `t = 14` draw 0 cap 7 at `-m112` (23). The route
  was ported to REST v2 and re-targeted (revision 5).
- **2026-11-21: the `-m112` cell found already measured.** `results/SNAPSHOT_20261117/README.md` (a third session,
  2026-11-17) holds both route-F result archives, fetched before the pods were deleted: `inconsistent false`, `oversize
  true`, `timed_out true`, `degree_reached 7` on both draws, stopped at step 35 asking for 33,961,233,968 `B'` entries
  (`t14_bigmem/MEMORY.md` section 2a). Under Amendments 7 B and 14 that is censored and `d_F >= 7` stands. Amendment
  32's "No row was fetched" is superseded by those archives. Revision 5 was not launched.
- **2026-11-21: owner decision 24.** After the bigger-memory design (`t14_bigmem/`): draw 0 only, as a probe, stage
  `cap:14:0:7:54000000000` on a 32 vCPU / 256 GB reserved pod; per-job cap US$65, study guard US$535, ceiling US$540.
  Draw 1 needs a later decision from draw 0's outcome. Revision 6 implemented it.
- **2026-11-21: the third development smoke** (`receipts_dev/pkm-x-dev-smoke3/`, pod `klogu58zoe2owr`, t = 8, not
  data) on the 32 vCPU / 256 GB `cpu5m` pod with REST v2: create 201, `cloud` `SECURE`, `cpu` 32 vCPU / 256 GB, `cost`
  2.08, data center `EUR-IS-1`, cgroup `memory.max` 256,000,000,000, classified ADMITTED, the git gate passed, `DELETE`
  204 and the pod gone from the list; about US$0.09 [pcc:unverified].
- **2026-11-21: the fourth independent review** (`reviews/REVIEW_4_red_team_A33.md`, of `a5c6d89147`; verdict "freeze
  with corrections"; M1-M5, S1-S14) and **owner decisions 25-27**: an independent hourly reaper in a fresh cloud session
  while the pod exists, superseding decision 14 for this job (25); the shared balance covers US$62.40, Amendment 29's
  per-batch confirmation, and results are fetched every 15 minutes (26); `pkm-dense-rerun-20261115` is study spend,
  ceiling US$600 and guard US$595, per-job cap US$65 (27). This revision (7) answers the review and implements them
  (`DECISIONS_ROUTE.md`).

## 1. The job, its relaunch and the request

| name | when it may run | request | stage |
|:--|:--|:--|:--|
| `pkm-x-t14-k0-c7-d54` | first and only | `runpod_capped/requests_d54/pfd-x-t14-k0-c7-d54.json` (SHA-256 `089176f2a75dc228a817dc1b88852cd1953fbfcd63e0452189acd4765f70d025`, pinned in `common.sh`) | `cap:14:0:7:54000000000` |
| `pkm-x-t14-k0-c7-d54-r1` | only after the base is LOST with relaunch `auto` (owner decision 13; section 4.3) | same | same |
| `pkm-x-t14-k1-c7-d54` | **not registered**; only by a later owner decision after the draw-0 outcome | (none) | (`cap:14:1:7:54000000000`) |

- Kummer `m = 2`, P1, `t = 14`, draw 0, cap 7, seed `1347112276` (recorded, not passed), `--budget 86400`
  (Amendment 7 B), `--max-dense 54000000000`, `RAYON_NUM_THREADS = 12` (Amendment 9; the design's 32-thread option was
  not chosen).
- `run.sh` refuses every other study name and every other stage for these names (exit 3, before any API call). The
  names of owner decision 23 (`pkm-x-t14-k0-c7-m112`, `-r1`) are retired: their cell is measured. Development names
  `pkm-x-dev-<word>` stay, `t <= 10`, `--max-dense` 12800000000, 27900000000 or 54000000000, receipts in
  `receipts_dev/`; they are not data.
- **A name is launched at most once, ever** (receipts directory, results directory or ledger row; decision N6).
- **The request.** No frozen request for this job exists. `make_requests_d54.sh` derives it from the frozen
  `requests_r2i/pfd-x-t14-k0-c6.json`. Changed paths (`make_requests_d54_output.txt`): `job.command.1`
  `cap:14:0:6:12800000000` to `cap:14:0:7:54000000000`; `job.memory` 56Gi to 238Gi; `job.job_name`
  `pfd-x-t14-k0-c6-20261108` to `pfd-x-t14-k0-c7-d54-20261108`; `provenance.note` with one sentence appended (1,995
  characters, within the validator's 2,048). Every other field is copied, `provenance.protocol_sha256` included. It
  passes the request validator the `requests_r2i` requests passed (`requests_d54_validate_output.txt`; a first version
  of the sentence made the note 2,056 characters and failed, and was shortened). `run.sh` checks the pin and re-derives
  the file. **238Gi**: the pod is 256·10⁹ bytes = 238.42 GiB, and 4 × 54·10⁹ bytes = 201.17 GiB plus the design's
  36 GiB headroom is 237.17 GiB, so 238Gi is the largest whole GiB the pod holds and the formula `4·C <= request − 36 GiB`
  gives `C = 54·10⁹` (`t14_bigmem/MEMORY.md` section 5). The record of decision 23's request stays in `requests_m112/`
  (`make_requests_m112.sh`, now self-contained and marked retired).

### 1.1 Reading (fixed before any row of this job)

The reading uses the row's `inconsistent`, `oversize`, `timed_out` and `degree_reached` and the stop line of
`runs/X-m2-t14-k0-c7.log`, because the pilot writes `timed_out = timed_out || oversize`
(`examples/pkm_tower_pilot.rs:1504-1506` at `f13a690df0`) and names the stop `budget` or `oversize: --max-nnz`
(`:1186-1189`). The classifier writes it into `outcome.json` as `reading` (section 4.1).

| outcome | reading for draw 0 |
|:--|:--|
| ADMITTED, `inconsistent true` (refutes at cap 7) | `d_F = 7` |
| ADMITTED, degree 7 completed: `inconsistent false`, `oversize false`, `timed_out false`, `degree_reached 7` | `d_F >= 8` (projected out of reach, section 7) |
| ADMITTED, `oversize true` at 54·10⁹ and no contrary stop line; or class OOM (section 4.2) | censored at the new size; `d_F >= 7` stands |
| ADMITTED, `oversize false`, `timed_out true` and no contrary stop line (a deadline caught at the top of the step loop prints none, `f4_fp_tower.rs:2435-2438`); or class LIMIT | censored; `d_F >= 7` stands |
| ADMITTED but no case above (for example the row and the stop line disagree) | no reading; owner |
| LOST, ABORTED, NO_EVIDENCE, NOT_ADMITTED, NOT_AN_ATTEMPT | no row (section 4) |

A draw-0 `d_F = 7` with draw 1 not run reads "undecided at `t = 14`" under Amendment 7 D.5, which needs both draws. No
further bracket step follows from this amendment; cap 8 is not registered (its first degree-8 step alone is projected at
3.6 to 15 TiB, `t14_bigmem/MEMORY.md` section 4).

- **Decomposable draws (Amendment 7 C).** The reading table does not read the grid-zeros output. For this system the
  certificate already exists: the 2026-11-17 snapshot records grid zeros 0 for `t = 14` draw 0 (`results/SNAPSHOT_20261117/
  README.md`; the `-m112` tree's `X-m2-t14-k0-c7.gridzeros.jsonl` holds `{"decomposable":false,"grid_zeros":0}`, review 4
  S5), and the run's own gridzeros line is checked by admission rule 3.
- **The relaunch is nearly vacuous (review 4 S6).** `-r1` passes the US$595 guard only if the ledger, with the base
  attempt's actual cost, leaves room for another 64.06: 595 − 64.06 − 503.19 = US$27.75, a base pod life of at most about
  13 h. A loss later than that sends the cell to the owner in practice.

## 2. What changes

1. **Host: one pinned flavor, no fallback (decision D8; owner decision 24).** `cpu5m` at 32 vCPU and 256 GB, US$2.08/h
   (catalog 2026-11-21: `price.securePerVcpu` 0.065 × 32; availability `HIGH`, data centers `EU-RO-1` and `EUR-IS-1`
   at 32 vCPU at 21:3xZ, none listed at 20:45Z), image
   `python:3.12-slim@sha256:5e2bdb1b0d542d5039cffb8181c65930093949194a65c365dfb1b0fe13a4fa5b`, 20 GB disk, cloud tier
   `SECURE` [pcc:unverified]. RunPod REST v2 has no interruptible or spot field for pods, so such a pod is on-demand.
   `cpu3m` also has 256 GB at 32 vCPU, on CPUs no study file records; it is not a fallback (it would be a second
   deviation and could put the two draws on different generations, `t14_bigmem/HOSTS.md` section 2). If the catalog
   shows `cpu5m` at 32 vCPU with availability `NONE` or an empty `dataCenters` list (as at 20:45Z), `run.sh` exits 10
   without creating anything and the owner decides; if the catalog shows stock but the create fails, the run exits 4
   with no pod and no cost (review 4 M5e). The 32 vCPUs are bought
   for memory (RunPod sells 8 GB per vCPU); with 12 rayon threads the other 20 do no F4 work. The create body (REST v2,
   main's `runpod/rp.sh` shape): `{name, cloud: "SECURE", cpu: {id: "cpu5m", vcpuCount: 32}, image, disk: 20, ports,
   env: {RUNNER_TOKEN}, entrypoint: ["bash", "-c"], cmd: [<boot script>]}`.
   - The create must answer HTTP 201. Before anything runs, `run.sh` checks [pcc:unverified]: on the create response
     `cloud` `SECURE`, `cpu.id` `cpu5m`, `cpu.vcpuCount` 32, no `gpu`, and `cost` equal (to 4 decimals) to the catalog
     price read just before create; on the GET record right after, the same and **`cpu.memory` at least 256 (GB)**;
     on the record taken once the runner answers, `status` `RUNNING` and a data center id; `pod-request.json` with
     `cloud` `SECURE`, `cpu {id: cpu5m, vcpuCount: 32}` and no `interruptible` key. Any failed check deletes the pod
     (exit 4). v1 to v2 field mapping: `V2_FIELDS.md`.
   - **The pod's cgroup memory limit is read before start (S4).** The boot loop copies the cgroup files into
     `/results/cgroup/` before anything else exists there; `run.sh` fetches that tree once before `/start` and records
     the limit in `cgroup-prestart.json`. **For the study names it must be at least 256,000,000,000 bytes
     (238.42 GiB); `max` is accepted only for development names** (review 4 S3; smoke3 showed exactly 256,000,000,000):
     the `B'` cap 4 × 54·10⁹ = 216·10⁹ bytes (201.17 GiB) plus the design's 36 GiB headroom (basis up to 24 GiB, residue
     echelon 4.7 GiB, remainder 1 GiB, about 5 GiB margin) and 1.25 GiB more. Otherwise, or with no limit file, the pod
     is deleted before start (NOT_AN_ATTEMPT; owner).
   - **The design's 230.9 GiB is a projection of the process peak under stated assumptions, not a bound**
     (`t14_bigmem/MEMORY.md` sections 3 and 5; review 4 M5d). It assumes a basis of at most 24 GiB and the echelon of a
     step with `nq` at most 50,256, and leaves out kernel page tables charged to the cgroup (about 0.45 GiB), allocator
     page rounding per `B'` row (about 1.3 GiB at 686k rows) and a later step with a larger `nq` (a near-full echelon
     at `nq` = 83k is about 12.8 GiB); any of these with a 24 GiB basis can use up the 7.5 GiB between it and the
     limit. A peak above the limit reads as an OOM, censored, the same reading as an `oversize` stop: the risk is spend
     and classification clarity, not a wrong reading. The boot loop and `cgroup-prestart.json` also record
     `memory.oom.group`, `memory.swap.max` and `vm.max_map_count` (review 4 S1; recorded only, no rule reads them;
     section 4.2 says how a group OOM or an allocation abort is classified).
2. **Threads (S13).** `rayon 12` on a pod whose affinity gives `nproc 32`. `muladds` does not depend on the pod
   (the F4 chunk size is set from the rayon thread count, `src/cryptanalysis/f4_fp_tower.rs:1694` at `f13a690df0`);
   wall time can.
3. **Binaries and entry script.** The frozen image's study layer `a0f7e28e…` (the four static binaries built from
   `f13a690df016d0a457bd40579fbb9d054f486adf` and `entry_r2.sh`, by the SHA-256 pinned in `common.sh`), not
   Amendment 17's rebuild; both sets are admitted (Amendment 17 by exact reproduction). `run.sh` refuses `SHA_FILE`. The
   job invokes `/usr/local/bin/entry_r2.sh STAGE` where the request names `entry.sh`; for a `cap:` stage they are the
   same commands, file names and `progress.txt` lines (`entry_r2.sh` adds only the `m4n:` branch; checked with `cmp` on
   the three extracted regions).
4. **Pod start command (decision D4).** `bash -c` with a short script (`boot-template.sh`, its SHA-256 in
   `pod-request.json`) that copies the cgroup memory files into `/results/cgroup/` at once, every 30 s and once after
   `runner_state.json` appears, then `exec`s the runner `server_pinned.py`, an unchanged copy of `runpod/server.py` at
   `d2a16c723d` (SHA-256 `f56fd08e…`), the version this route was reviewed with (main's file has since changed).
5. **Results path and durability (S2, S8; owner decision 22).** `entry_r2.sh`'s `aws s3` calls go to the runner's shim
   in the pod's `/results`. Each collection `k` writes an intent record, downloads `/results.tar.gz` twice through the
   pod's proxy URL, compares the per-file SHA-256 manifests and keeps one tree in `runpod_capped/results/<name>/collect-
   <k>/`; nothing is overwritten. **No S3 (a disclosed deviation).** The study results trees are not git-ignored. Before
   a completed run's pod is deleted the session commits and pushes the kept tree and the receipts; `fetch.sh NAME
   --delete REF` deletes the pod only when the git gate passes: every file of the kept tree, `outcome.json` and
   `local-sha256-<k>.txt` are in commit `REF` with the blob hash of the local file, each local file still has its
   manifest SHA-256, and `REF` is an ancestor of a branch head `git ls-remote --heads origin` lists then (`git-gate.txt`).
   The pod waits at most 2 h for it, never past the hard deadline, then is deleted regardless. The run's
   `bench.record/v1` stays a Git record with no S3 upload; the resume note is committed and pushed before the launch and
   after every check-in.
6. **Time limit and the backstop (decision D3; owner decisions 25 and 26).** Run deadline = the runner's recorded
   start + 90,000 s; hard deadline = creation + 26 h; the runner starts within 55 minutes of creation. No information is
   never a loss (N2): it is retried until the hard deadline, then the pod is deleted for cost (NO_EVIDENCE).
   - **Who deletes the pod.** (a) `fetch.sh`, when it collects a completed run: through the git gate, at most 2 h after
     the collection, never past the hard deadline. (b) `fetch.sh` at the hard deadline, whatever it has. (c) `reap.sh`
     past the hard deadline, from two places: the main session's passes, and **an hourly scheduled task that starts a
     fresh cloud session independent of this one** (owner decision 25, which supersedes decision 14 for this job). Its
     stored prompt is `INDEPENDENT_REAPER.md`; it runs `reap.sh` in a fresh checkout with an empty `RPC_STATE` (the token
     comes from the v2 record, the key from the environment), commits what the pass wrote under `runpod_capped/`, and
     never launches, never touches another pod and never deletes before the hard deadline. When the pod list cannot be
     read, `reap.sh` exits 1 with `REAPER BLIND` and the scheduled session reports it.
   - **Fetches.** While the job runs, `watch.sh` runs `fetch.sh` every 900 s (Amendment 29's 15-minute fetch, owner
     decision 26), in background segments of at most 2 h that the session restarts; it deletes nothing itself, and it
     stops when `fetch.sh` reports a collection (the git gate is then the session's), a delete, or the owner.
   - **Amendment 32's relaunch precondition** ("the client deletes each pod when its job ends, and no batch starts unless
     a check-in is scheduled within the shortest job limit"): the job's end is seen within 15 minutes by `watch.sh`; the
     pod is deleted through the git gate within at most 2 h after the collection, or at the hard deadline; the hourly
     independent reaper covers a session outage. The session creates that task right after `run.sh` returns and confirms
     its first pass; `run.sh` does not check for it (procedure, not code).
   - **Amendment 29** (frozen: owner balance confirmation per batch, fetches every 15 minutes): owner decision 26 is
     this job's per-batch confirmation (the shared balance covers US$62.40), and `watch.sh` is the 15-minute fetch. A
     capped run's partial tree is never read for a rule, so the fetches add loss evidence and receipts only.
   - **Worst-case pod life and cost.** With the hourly reaper: 26 h + 1 h between passes + about 0.8 h for one pass =
     about 27.8 h, US$57.82. If the hourly task missed passes and only the session's 4-hourly pass ran: 26 + 4 + 0.8 =
     **30.8 h, US$64.06**, the figure every cost check uses (`common.sh` `WORST_HOURS`; review 4 M5f). If both the
     session and the scheduled task stopped, nothing on RunPod's side stops the bill (v2 has no pod lifetime field).
7. **Host lines.** The pod id, flavor, `cost`, data center and CPU model (from `host.txt`) are recorded with the row;
   `cpu5m` does not pin the CPU model (the Amendment 31 pods ran on a Threadripper 7960X and an EPYC 9575F).

## 3. Deviations, stated plainly

- **A new `--max-dense` step (from Amendments 7 A, 14 and 31).** Amendment 14 made the `-m112` run at
  `--max-dense 27900000000` the cell's last size step, censored when stopped by its limit. Running the same cell at
  `54000000000` deviates from that registered last step. The `-m112` rows stay recorded as they are (censored,
  `d_F >= 7`); this run is an additional, separately named attempt read only under section 1.1. Amendment 7 A's
  formula `4·C <= request − 8 GiB` is replaced by `4·C <= pod limit − 36 GiB`, because at degree 7 the basis alone is
  11.4 GiB before step 35 (`t14_bigmem/MEMORY.md` section 3). **Why it is the same measurement:** same binaries,
  system, seed, cap, budget, thread count and entry script; `--max-dense` is read once per step as a check before `B'`
  is allocated (`src/cryptanalysis/f4_fp_tower.rs:1577-1580`) and steers nothing else; steps 1-30 are identical at
  12.8·10⁹ and 27.9·10⁹, and steps 1-34 are structurally identical on two CPU models and both draws (rows, columns,
  `B'`, fresh, basis and pair counts; `muladds` differ at steps 28 and 30-32 and one `nnz` by one, review 4 S9).
  **Disclosure:** 54·10⁹ was chosen after
  seeing that step 35 asks for 33.96·10⁹; the limit cannot change what the run computes, only whether it may finish.
- **Owner decision 15's 112 GiB step is not used.** It mirrored an EKS 112Gi request on a larger pod; here the limit is
  the pod's own, and OOM means cgroup `oom_kill > 0` or a peak above the pod's `memory.max` (section 4.2).
- **RunPod REST v2 in place of v1** (v1 answered 502; Amendment 32). The host-side relaunch evidence loses one form
  (section 4.3).
- **Git in place of S3** (owner decision 22; section 2.5).
- **Per-job cap US$65, study guard US$595 and ceiling US$600** (owner decisions 24 and 27; section 5), above the US$40
  and US$510 / US$540 of decisions 4
  and 21.

## 4. Outcome classes, relaunch and limits

`fetch.sh` holds the only classifier (version `a33-r2`), reading receipts only: `start.json` (the runner start `s0`;
a zero-byte file is read as absent), the REST records, every check-in, the session's logged actions, the collection
records, the kept trees and `cgroup-prestart.json`. Every `outcome.json` records the version and the SHA-256 of
`fetch.sh` and `common.sh`. An unparsable receipt fails closed (P0).

### 4.1 One outcome table (decision D1)

**Positive evidence of loss:** a reachable runner that reports `idle` after its start, or a start other than `s0`; a
REST v2 record (HTTP 200) whose `status` is not `RUNNING`; the pod missing from a successful list and its GET returning
404; more than one `start` line, or a `runner_state.json` start other than `s0`. A changed `startedAt` alone is
recorded, not evidence. An unreachable runner, an unreadable REST record or a failed transfer check is no information.
A **completion is secured** when the collection reason is `done`, both fetches agree, the pre-fetch check-in shows the
runner `done` under `s0`, no loss evidence is dated at or before it, and the tree's `runner_state.json` agrees with one
start line.

| step | condition | class |
|:--|:--|:--|
| P0 | a receipt the classifier reads is unparsable | NO_EVIDENCE |
| P0b | two collections made with the runner done have different trees | NO_EVIDENCE |
| P1 | no receipt shows a runner start | NOT_AN_ATTEMPT |
| P2a | completion secured, runner ended after the run deadline | LIMIT |
| P2b | completion secured, an `end` line has exit 137 and cgroup `oom_kill` > 0 | OOM |
| P2c | completion secured, an `end` line has exit 137, `oom_kill` 0 or no cgroup file | LOST |
| P2k | completion secured, cgroup `oom_kill` > 0 without exit 137 | OOM |
| P2p | completion secured, the memory peak exceeds the pod's numeric cgroup `memory.max` (the tree's, else the one read before start) | OOM |
| P2u | completion secured, no memory peak observable | NOT_ADMITTED |
| P2d | completion secured, admission rules 1-7 hold (section 6) | ADMITTED, with a reading |
| P2e | completion secured, otherwise | NOT_ADMITTED |
| P3 | positive evidence of loss dated before the run deadline | LOST |
| P4 | a check-in at or after the run deadline shows the runner running, or done after it, under `s0` | LIMIT |
| P5 | collected on `--abort` | ABORTED |
| P6 | positive evidence of loss dated at or after the run deadline | LOST |
| P7 | collected (at the hard deadline) with none of the above | NO_EVIDENCE |
| P8 | not collected yet | (pending) |

### 4.2 What each class means

- **ADMITTED.** The run's row, read by section 1.1.
- **OOM** (P2b, P2k, P2p): censored at the new size, `d_F >= 7` stands. Exit 137 is classified by the cgroup evidence,
  never by the exit code alone. The memory peak is the pilot's own field (the largest `memory R MB (peak P MB)` value in
  `runs/<cell>.log`, its `VmHWM` in MiB × 1,048,576), else the cgroup's (`memory.peak`, else v1
  `memory.max_usage_in_bytes`); with `memory.max` reading `max` and no numeric limit before start, P2p cannot fire.
  `VmHWM` covers only the pilot and is printed when an F4 step ends, so it can miss a final rise (S5).
  How the other two failure shapes are classified (review 4 S1): if `memory.oom.group` is 1, an OOM kills the runner
  with the pilot, the container exits, and the result reads LOST (relaunch owner) or NO_EVIDENCE, not OOM, because the
  last `memory.events` copy predates the kill; an allocation failure (for example a refused `mmap` near
  `vm.max_map_count`) aborts the pilot with exit 134 and reads P2e NOT_ADMITTED. Both go to the owner; neither gives a
  wrong reading. An OOM whose peak is under 90 percent of the pod's `memory.max` carries `oom_note` in `outcome.json`:
  a host-level or group kill is likely, not a sign that 54·10⁹ is too small (review 4 S8).
- **LIMIT**: censored, not re-run, no relaunch (an EKS job killed at its time limit; the deadline counts from the
  runner's start, at most 55 minutes after the pod's).
- **LOST**: nothing admitted or analysed; relaunch `auto` or `owner` (section 4.3).
- **ABORTED** uses the attempt, no relaunch (N1). **NO_EVIDENCE**: owner (N2). **NOT_AN_ATTEMPT**: no relaunch; a
  pre-start failure that created a pod leaves a ledger row, so the name is not launched again (N6).

### 4.3 Relaunch (owner decision 13)

One relaunch, `pkm-x-t14-k0-c7-d54-r1`, only on positive host-side evidence of loss; an abort, NO_EVIDENCE or a loss
without it goes to the owner. REST v2 can show one form of host-side evidence: the pod gone from a successful list with
its GET returning 404, recorded before any session action logged for the pod and tied to the loss (after the last
check-in that saw the runner alive under `s0`, or at the first-loss check-in, and at or before the first loss); a
runner seen `done` before the loss sends the relaunch to the owner. v1's other form, a RunPod-side `lastStatusChange`,
has no v2 field and never counts (fail closed).

## 5. Cost guard (owner decisions 24, 26 and 27)

- **Price** (catalog 2026-11-21): US$2.08/h. `run.sh` reads it before the create (`price.jsonl`); the pod's `cost` must
  equal it.
- **Per job:** refused before create if price × 30.8 h > US$65 (the worst-case pod life of section 2.6). Worst case
  today 2.08 × 30.8 = US$64.06. Expected (derived, `t14_bigmem/HOSTS.md` section 4): about US$44 if it refutes near
  70,000 s, about US$53 if the budget stops it.
- **Study: guard US$595, ceiling US$600** (owner decision 27). `ledger.tsv` sums to 503.1885: EKS rows; RunPod's own
  billing per pod from REST v2 `GET /billing/pods` (`billing_v2_20261121.tsv`); the Amendment 31 batch, US$370.84; one
  row bringing the pre-Amendment-31 spend to Amendment 32's US$95; smoke3 (0.0896); and `pkm-dense-rerun-20261115`
  (37.2565, counted as this study by owner decision 27). `bdr-growthlaw` (35.7037) stays outside: another study
  (`docs/session-logs/2026-11-15.md:4536`, benchmark record `20261115T1952Z-growth-law-cells-rerun`). `run.sh` refuses if
  the sum plus the job's worst case exceeds US$595: today 503.19 + 64.06 = 567.25 (`ledger_SOURCES.md` section 5). A
  draw-1 job later would need 503.19 + 2 × 64.06 = 631.31, above the ceiling: a further owner decision.
- **Balance** (Amendment 29): owner decision 26 confirms the shared balance covers US$62.40, the job's worst case at the
  route's earlier 30 h figure; the 30.8 h figure adds US$1.66.

## 6. Admission and receipts

ADMITTED only through P2d: completion secured, a memory peak observable and not above the pod's numeric limit, no
`oom_kill`, and all of [pcc:unverified]:

1. `binary.sha256` has the four binary hashes pinned in `common.sh` at `/usr/local/bin/…`.
2. `host.txt` has `argv: cap:14:0:7:54000000000` and `rayon: 12`.
3. `progress.txt` has exactly one `start X-m2-t14-k0-c7` line, `end X-m2-t14-k0-c7 exit 0` and
   `end X-m2-t14-k0-c7.gridzeros exit 0`, and the row file has its row.
4. `runner_state.json` has `state done`, `exit 0` and the start in `start.json`.
5. `runner_request.json`'s `verified_sha256` equals the pinned hashes.
6. The two fetches' manifests are identical, and `local-sha256-<k>.txt` equals the kept tree.
7. [pcc:unverified] `pod-create.json`: `cloud` `SECURE`, `cpu.id` `cpu5m`, `cpu.vcpuCount` 32, `cost` equal to the price in
   `price.jsonl`; `pod-record.json`: `cpu.memory` at least 256; `pod-request.json`: `cloud` `SECURE`,
   `cpu {id: cpu5m, vcpuCount: 32}`, no `interruptible` key.
8. Before the row is used: `cost.json` with `confirmed_gone: true` (delete HTTP 200, 204, or 404 with the deletion time
   an upper bound).

**Receipts** (`runpod_capped/receipts/<name>/`, never overwritten): `run.log`, `price.jsonl`, `pod-request.json`,
`boot-template.sh`, `pod-create.json`, `pod-record.json`, `pod-record-ready.json`, `status-ready.json`, `upload.jsonl`,
`prestart-results-files.txt`, `cgroup-prestart.json`, `pod-state.json`, `start.json`, `pod-record-start.json`,
`checkins.jsonl` with `status-<stamp>.json` and `rest-<stamp>.json`, `fetch.log`, `session-actions.jsonl`, per
collection `collect-started-<k>.json`, `transfer-<k>.json`, `local-sha256-<k>.txt`, `tarball-<k>.sha256`,
`admission-lines-<k>.txt`, `collect-<k>.json`; `outcome.json`, `git-gate.txt`, the delete records and `cost.json`. Saved
API records drop `env`, `args`, `cmd`, `entrypoint`, `ssh` and any account identifier.

## 7. Projection and risks (`t14_bigmem/`)

- **Memory (derived; `t14_bigmem/MEMORY.md` section 4).** Step 35 is deterministic and needs 33.96·10⁹ `B'` entries,
  about 144-148 GiB of process memory. Bands for finishing degree 7, from the degree-6 histories of `t = 10..13`: low
  (`t = 10`-like) 33.96·10⁹, 144-148 GiB; central (`t = 11`-like) 35.9·10⁹, 155-168 GiB; high (`t = 12`-like)
  159.5·10⁹, about 640 GiB; beyond (`t = 13`-like, or no refutation at degree 7) 2.0-2.6 TiB. The 256 GB pod at
  54·10⁹ holds the low and central bands; the high band ends `oversize`.
- **Reach.** The outcome `d_F >= 8` is out of reach by a factor of about 9 or more in memory on any RunPod CPU pod; the
  probe can return `d_F = 7` or a censored row, and a censored row cannot tell `d_F = 7` with larger growth from
  `d_F >= 8`.
- **Time (derived; `t14_bigmem/HOSTS.md` section 3).** Steps 1-34 take about 21,000-22,000 s at 12 threads; step 35
  ends 27,600 to 66,000 s after the F4 start. The low band finishes inside the 86,400 s budget; the central band needs a
  second all-pairs degree-7 step and is projected at 90,000-140,000 s, so the budget would stop it (censored). Pods of
  one CPU model differed by 1.27× and 1.56× on identical runs, as large as the low band's margin. Two corrections
  (review 4 S14): the "fast exit" lower end ignores that filling the echelon needs at least `nq` = 50,256 reduced S-rows,
  about 2,200 s more; and the low band's upper end, 70,000 s, times the 1.27 host spread is about 89,000 s, past the
  budget, so even the low band can be censored by time on a slow host.
- **Risks.** The projection rests on four degree-6 analogues, one (`t = 11`) without a trace; the basis could grow past
  about 30 GiB and bring a memory kill before a clean `oversize` stop (both censored); the wall budget at 12 threads.
- `t14_bigmem/project.py` is an analysis script that re-derives these numbers; it is not part of the route and runs no
  job.

## 8. Unchanged

- Binaries (frozen layer `a0f7e28e…`), the cell, seed, draw, cap, `--budget 86400`, 12 threads, the entry script.
- Hypotheses and §6 rules; Amendments 7 B and 9; the readings of Amendments 7 D and 31.
- The requests in `requests_r2i/` are not edited. The `-m112` rows of Amendment 31 stand as recorded.

## 9. Points settled, checks that need a pod, and disclosure

1. Owner decisions 13 (relaunch), 20 (one cell), 22 (no S3), 24 (this probe), 25 (the independent hourly reaper; it
   supersedes decision 14 for this job), 26 (Amendment 29's balance confirmation; 15-minute fetches), 27 (the ledger's
   unattributed pod counted; ceiling US$600, guard US$595). Decision 21's guard and ceiling are superseded by 24 and 27.
2. Checks made on a pod. Smokes 1 and 2 (2026-11-14 and 15, REST v1, 16 vCPU / 128 GB): the start command, the runner,
   cgroup v2 with `memory.max` 128,000,000,000. **Smoke 3** (2026-11-21, REST v2, the job's 32 vCPU / 256 GB `cpu5m`
   pod, `receipts_dev/pkm-x-dev-smoke3/`; [pcc:unverified]): create 201 with `cloud` `SECURE`, `cpu` 32 vCPU / 256 GB and `cost` 2.08;
   the GET and ready records (`RUNNING`, `EUR-IS-1`, already set at create); the runner through the proxy with the bash
   entrypoint and the pinned image; `memory.max` 256,000,000,000 before start; classifier P2d; the git gate; `DELETE`
   204 and the pod gone from the list; about US$0.09. A later read-only GET of the deleted pod answered 404 (review 4).
   **Still unobserved:** token recovery from the record's `env` (offline tests only), `startedAt` and `status` on a
   container restart, and `memory.oom.group` and `vm.max_map_count` on such a pod (recorded from now on).
3. Offline tests: `tests/README.md` (counts with and without the image layer).
4. Not done from review 4 (listed in `DECISIONS_ROUTE.md`): the reaper's fallback to known pod ids when the list is
   unreadable (S7; it reports instead), the unpriced pod after a timed-out create and the unconfirmed pre-start delete
   (S7, round-3 S5), and an exercise of token recovery on a pod (S10). Main's numbering is to be checked again at the
   freeze (S12).

**Disclosure (what the registering sessions have seen).** Every committed row of this study before 2026-11-21: all
`t = 12` and `t = 13` rows, the partial log of the lost third `t = 13` draw 0 attempt, the `t = 14` cap 5 and cap 6 rows
(both draws, not refuting), cap 7 at 12.8·10⁹ (oversize at degree 6), and the 2026-11-17 snapshot's route-F archives,
whose rows and logs `t14_bigmem/MEMORY.md` reads (cap 7 at 27.9·10⁹: oversize at step 35, degree 7, not refuted, on both
draws; step-by-step memory and times). The size 54·10⁹ was chosen with that knowledge (section 3). The reading of
section 1.1 is fixed by Amendment 7 B and does not depend on it.

## 10. Script hashes at this revision

Recorded again at the freeze. At the freeze the first four lines go into the preregistration as `sha256sum` output;
`run.sh` and `fetch.sh` refuse study runs otherwise. `watch.sh` and `INDEPENDENT_REAPER.md` are procedure files: their
hashes are recorded here, and no script checks them.

| file | SHA-256 |
|:--|:--|
| `common.sh` | `e6b716a5f862d5c9056e295b7ba057bc152855c8198edb27964113d21de79e23` |
| `run.sh` | `5476bca055eadbe8c1b98baf3316c2c8967c86d561d27bde5bc926ff9ca02bbb` |
| `fetch.sh` | `3027d1bb25bd19d0686aed8b1fe4caaefc51a948d86d419f5149af72f41391d5` |
| `reap.sh` | `90f3fbe7be95dac868d248b9a2321eea8d400fb0a06d4aff0defe65ca0e4becd` |
| `watch.sh` | `cea7bf4d774b289505fddd6d3a4c377d862c706f43a9dd24a4a91bfd8185fb73` |
| `INDEPENDENT_REAPER.md` | `efe1afbf99b59803e7110f31d9a7fd4b6ff0cf0d45da5ddfdce307ddf9e542a5` |
| `make_requests_d54.sh` | `b5e5c65570ce7b4cd228352d0ade8a571deac3d8c62813ae917ab6a11b155cb2` |
| `requests_d54/pfd-x-t14-k0-c7-d54.json` | `089176f2a75dc228a817dc1b88852cd1953fbfcd63e0452189acd4765f70d025` |
| `rp_pinned.sh` | `b4b6019934e02ffa32043e2e6b9aee4eb4dcc53b42442b2e01d1b30ccf536c5d` (= main's `runpod/rp.sh` at `d2a16c723d`) |
| `server_pinned.py` | `f56fd08e01318512713f8ee392d8539fb94dc969444d45234179789930d86550` (= `runpod/server.py` at `d2a16c723d`) |
| retired: `make_requests_m112.sh`, `requests_m112/pfd-x-t14-k0-c7-m112.json` | `dd75c92a7d86ba0927886c0926f2b25f59fd4645cd22a8a7a9713af4b62ddd8d`, `44e009f18d19a9c61e5a281e0c2a6a219c4f3cc3252ca63bff71049fdef4c7f9` |
