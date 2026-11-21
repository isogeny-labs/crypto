# Decisions behind the capped RunPod route (Amendment 33 draft; draft Amendment 18 until 2026-11-15)

This file records the decisions that `AMENDMENT_R2_H_runpod_capped.md` and the route scripts rely on, so that a reader
can find them in a committed file (second review, S13). Owner decisions are the owner's answers, recorded in full in
`research/open_leads_execution_20261108/DECISIONS.md`; they are only cited here. Main-session decisions were made by
the main session of the PKM fall-degree work and given to the subagent that revised the draft; they are records of a
decision, not results. Every entry is `[pcc:unverified]`.

## Owner decisions

| # | date | decision | where it acts |
|:--|:--|:--|:--|
| 3 | 2026-11-08 | A reclaimed non-resumable cell is reported lost and may be relaunched once under a new name. | Read through decision 13 below. |
| 4 | 2026-11-08 | At most US$40 per job and US$400 for the study. | Section 5; `common.sh` `CEIL_USD`, `STUDY_CEIL_USD`. |
| 10 | 2026-11-14 | t = 14, and a further attempt of the lost t = 13 draw 0 cell, run on non-Spot capacity. | Section 1 (the `-r3` name). |
| 12 | 2026-11-14 | "Use runpod": command jobs on RunPod on-demand reserved pods under dated amendments. | The whole route. |
| 13 | 2026-11-15 | One relaunch per t = 14 cell on positive host-side evidence of loss only; an abort, a no-evidence outcome or a loss without host-side evidence goes to the owner; t = 13 gets none. | Section 4.3; `fetch.sh` `relaunch`; `common.sh` `cell_next`. |
| 14 | 2026-11-15 | The deadline and the cost cap are enforced by session check-ins and a scheduled reaper; no enforcement independent of Claude sessions. | Section 2.6; `reap.sh`. |
| 15 | 2026-11-15 | A 112 GiB attempt whose own process peak exceeds 112 GiB counts as OOM (censored). | Rule P2p. |
| 20 | 2026-11-21 | Run exactly one cell on this route, within the budget. | The allow-list holds one study name and its one relaunch (`common.sh` `STUDY_NAMES`). |
| 21 | 2026-11-21 | Study ceiling US$540, study spend guard US$510 (Amendment 20 had US$500 and US$470); the per-job cap stays US$40. | `common.sh` `STUDY_GUARD_USD`, `STUDY_CEILING_USD`, `CEIL_USD`; `run.sh` step 5. |
| 22 | 2026-11-21 | Run without S3 (this session's AWS credentials are invalid): results come back to the session and go into git; no S3 raw upload and no S3 resume checkpoint. A disclosed deviation. | `fetch.sh --delete REF` (the git gate); `.gitignore`; amendment sections 2.5 and 6. |
| 23 | 2026-11-21 | The cell is t = 14, draw 0, cap 7 at Amendment 14's -m112 setting, stage `cap:14:0:7:27900000000` (Amendment 31 route F). Reading: refutes, d_F = 7; completes without refuting, d_F >= 8; oversize or a memory kill at 112Gi, censored and d_F >= 7 stands. | `common.sh` `STUDY_STAGE`; `run.sh` step 1; amendment section 1. |

## Main-session decisions

| id | date | decision | reason (one line) |
|:--|:--|:--|:--|
| D1 | 2026-11-14 | One outcome table, keyed only on receipts, with a fixed precedence, in the amendment and in `fetch.sh` as the single classifier; LIMIT mirrors an EKS job killed at `activeDeadlineSeconds`. | The first review found overlapping classes the producer could choose among after seeing a result. |
| D2 | 2026-11-14 | An exact allow-list of names, each launched once, in a fixed order; development names only with separate receipts; results never overwritten. | Any suffix was accepted and a reused name destroyed earlier evidence. |
| D3 | 2026-11-14 | Deadlines written before start; delete at the deadline even if the download fails; a reaper as the backstop; raw upload before deleting a finished pod. | Nothing outside the session enforced the 90,000 s limit or the US$40 ceiling. |
| D4 | 2026-11-14 | Keep Amendment 14's OOM branch; make OOM observable by a cgroup copy loop in the pod's start command, without changing `server.py`. | An unexplained exit 137 had been counted as a loss, contrary to Amendment 14. |
| D5 | 2026-11-14 | Cover the `-m112` route now, with requests derived from `requests_r2i` exactly as Amendment 14 says; the next bracket step needs its own amendment. | `oversize true` is plausible at t = 14 and the route could not run `-m112`. |
| D6 | 2026-11-14 | Pin a copy of `runpod/rp.sh` (main at `d2a16c723d`) and pin both its and `server.py`'s SHA-256; refuse when `SHA_FILE` is set. | PR #4173 edits `runpod/rp.sh`. |
| D7 | 2026-11-14 | Check the reserved tier, flavor, size and price on the create response before start; not interruptible from the request record. | The earlier check passed on a missing field. |
| D8 | 2026-11-14 | Pin one flavor, `cpu5m` (16 vCPU, 128 GB), with no fallback; out of stock is not an attempt. | `timed_out` depends on the host, and a fallback could put the two t = 14 draws on different CPU generations. |
| NOT_ADMITTED | 2026-11-15 | A completed run that fails an admission rule is NOT_ADMITTED: no row, no relaunch, owner. | D1's class list had no place for this case. |
| memory rule | 2026-11-15 | A 56Gi attempt whose memory peak exceeds 56 GiB counts as OOM under Amendment 14 and is classified as the cell's `-m112` attempt (admitted up to 112 GiB, OOM above); no observable peak means NOT_ADMITTED. | Every pod here has 128 GB, so a 56Gi run could finish where EKS would have killed it. |
| peak source | 2026-11-15 | The pilot's own `VmHWM` field is the primary peak source; the cgroup figure is the fallback only when the log has none; both are recorded. | The cgroup figure includes reclaimable file cache and the runner; an EKS OOM kill depends on unreclaimable memory. |
| validator | 2026-11-15 | Run the `-m112` requests through the validator the `requests_r2i` requests passed, and save the output. | The first run found the provenance note over the 2,048-character limit; it was shortened. |
| N1 | 2026-11-15 | An ABORTED run uses its attempt and grants no relaunch; any further attempt needs a new owner decision. | An abort is under the producer's control, a loss is not. |
| N2 | 2026-11-15 | Only positive evidence of loss is LOST; no information (unreachable runner, session network down, failed transfer check) is retried until the hard deadline, then the pod is deleted for cost and the class is NO_EVIDENCE (owner). A healthy RUNNING REST record with unchanged `lastStartedAt` always means "wait". | Absence of evidence had become a loss that bought a relaunch and deleted healthy pods. |
| N3 | 2026-11-15 | (a) Record `lastStatusChange` at every check-in; a LOST without host-side evidence goes to the owner; log the session's own deletes with their reason before the API call. (b) Never discard or overwrite: numbered collections; an orphan tree is kept and must agree with a later collection of the same pod. (c) The reaper deletes only past the hard deadline. | RunPod keeps no record of who ended a pod; the recovery procedure discarded a tree already seen; the reaper could kill a running pod. |
| N4 | 2026-11-15 | A 56Gi run counts as the `-m112` attempt only when its peak exceeds 56 GiB and its row has `oversize false`; with `oversize true`, Amendment 14's oversize path applies unchanged. | The shortcut is equivalent to the `-m112` run only when `--max-dense` did not stop the 56Gi run. |
| N5 | 2026-11-15 | Re-include the receipts' and fixtures' `.log` files in the route's `.gitignore`; make the tests' dependence on the image layer explicit (SKIP-NEEDS-LAYER with counts). | The local exclude rule hid fixture logs and would hide `run.log` and `fetch.log`; the test count did not reproduce from the commit. |
| N6 | 2026-11-15 | (i) `run.sh` refuses a name with any ledger row; (ii) the session commits and pushes receipts and the ledger after each launch and each `fetch.sh` call; (iii) `fetch.sh` recovers a missing pod token from the REST pod record into `RPC_STATE` only. | Attempt accounting depended on state that dies with the launching session. |
| N3a aligned | 2026-11-15 | Host-side evidence for an automatic relaunch is either form the owner's question for decision 13 named: a RunPod-side status change, or the pod gone from the list with GET 404, each recorded before any logged session action. | Aligns the route with owner decision 13 as answered, including its caveat that RunPod keeps no record of who ended a pod. |
| round 3 (R1-R3, S1) | 2026-11-15 | Host-side evidence counts only when tied to the loss (between the last check-in that saw the runner alive and the first loss; a user-named status at the loss check-in, or a runner seen done before the loss, sends the relaunch to the owner); the text is brought up to date with the second smoke (cgroup v2, two CPU models under `cpu5m`); `reap.sh` takes the per-name lock and skips a held pod; classifier `a18-r4`. | The third review found host-side evidence not tied to the loss, a loss after an observed completion getting an automatic relaunch, stale text, and a reaper without the lock. |
| tests | 2026-11-15 | The test suite writes into the repository only with `--write`. | The reviewer had to run it on an archive copy because it rewrote committed files. |

## Port decisions (2026-11-21, Amendment 33 draft)

Taken by the port session on the main session's instructions of 2026-11-21. Records of a decision, not results; every
entry is `[pcc:unverified]`.

| id | date | decision | reason (one line) |
|:--|:--|:--|:--|
| P1 allow-list | 2026-11-21 | The allow-list is `pkm-x-t14-k0-c7-m112` and `pkm-x-t14-k0-c7-m112-r1` (owner decision 13: only after a LOST with relaunch `auto`); `pkm-x-dev-*` (t <= 10) stays. Every draft Amendment 18 name is removed. `next_allowed` reduces to base, then the relaunch. | Owner decisions 20 and 23: one cell. |
| P2 request | 2026-11-21 | No frozen `pfd-x-t14-k0-c7-m112` request exists on main (searched `requests*/`, `runpod/`, `results/` and the study's text files). `make_requests_m112.sh` derives it from `requests_r2i/pfd-x-t14-k0-c6.json`: command[1] `cap:14:0:7:27900000000`, memory 112Gi, job name `pfd-x-t14-k0-c7-m112-20261108`, one sentence appended to the note; every other field copied. Its SHA-256 `44e009f1…` is pinned in `common.sh`; `run.sh` checks the pin and re-derives it. It passes the validator the r2i requests passed. The three cap-6 `-m112` requests were removed. | Amendment 7 B (cap 6 to 7) and Amendment 14 (-m112) define every changed field. |
| P3 stage | 2026-11-21 | `run.sh` refuses any stage other than `cap:14:0:7:27900000000` for a study name; development stages take `--max-dense` 12800000000 or 27900000000. | One stage per name; a development check may exercise the -m112 stage shape. |
| P4 REST v2 | 2026-11-21 | Every REST call moves to `api.runpod.io/v2` (price from the CPU catalog, create, get, list with its envelope and cursor pages, delete), with main's `runpod/rp.sh` create body and the v2 field names of `V2_FIELDS.md`. The create must answer 201. | REST v1 answers 502 from here (Amendment 32); v2 answers 200. |
| P5 lastStatusChange | 2026-11-21 | v2 has no `lastStatusChange`, so the host-side form "a RunPod-side status change" can no longer be seen and never counts. Host-side evidence is only the pod gone from a successful list with GET 404. Classifier `a33-r1`. | Fail closed: a missing field must not open an automatic relaunch. |
| P6 interruptible | 2026-11-21 | v2 has no interruptible field in the request or the record; rule 7 requires `cloud` `SECURE` on the create response and GET record and no `interruptible` key in `pod-request.json`. | The not-interruptible fact now rests on v2 having no spot pod field (spec `CreatePodRequest`). |
| P7 data center | 2026-11-21 | The data center id is checked on a GET record taken once the runner answers (`pod-record-ready.json`), not on the create response. | v2 documents `dataCenterId` as assigned by the scheduler; it may be null at create. |
| P8 image | 2026-11-21 | The pod image is main's pinned digest `python:3.12-slim@sha256:5e2bdb1b…` (the smokes used the tag `python:3.12-slim`). | Main's v2 smoke reached the runner with it; a digest cannot move. |
| P9 runner | 2026-11-21 | `run.sh` embeds `server_pinned.py`, a vendored copy of `runpod/server.py` at `d2a16c723d` (SHA-256 `f56fd08e…`), not main's current `runpod/server.py` (`c6d8afb1…`). | This route was reviewed with that runner; main's file changed. |
| P10 git gate | 2026-11-21 | `fetch.sh NAME --delete REF` replaces the bench-record gate: the pod is deleted only when every file of the kept tree, `outcome.json` and `local-sha256-<k>.txt` are in commit REF with the blob hash of the local file, each local file still has its manifest SHA-256, and REF is an ancestor of a branch head that `git ls-remote --heads origin` lists now. The study results trees are not git-ignored; development trees are. The hard deadline still deletes regardless (2 h wait at most). | Owner decision 22: no S3; git is the durable copy. |
| P11 ledger | 2026-11-21 | `ledger.tsv` takes RunPod's own billing per pod (`GET /v2/billing/pods`, `billing_v2_20261121.tsv`) for every study pod whose id is in a study file, the eight Amendment 31 pods (US$370.84, Amendment 32), and one row `a32-remainder/unitemised` that brings the pre-Amendment-31 spend to Amendment 32's US$95. Sum 465.8424. Two snapshot pods of unclear study (US$72.96) are listed as unknown, not as rows. | Owner decision 21 rests on Amendment 32's total of about US$466. |
| P12 memory | 2026-11-21 | The 56 GiB step of the memory rule is removed (P2m and `as_m112`): the attempt here is the cell's -m112 attempt itself (the 56Gi attempt was the other session's, Amendment 30). P2p (owner decision 15) and the pre-start cgroup check (at least 112 GiB or `max`) stay; the GET record's `cpu.memory` floor is 121 GB for every name. | One attempt size; owner decision 15 already covers it. |
| P13 bench record | 2026-11-21 | The run's `bench.record/v1` stays a Git record without S3 upload; the RESUME note is committed and pushed instead of the S3 checkpoint. | Owner decision 22. |

## Owner decision 24 and the d54 retarget (2026-11-21, Amendment 33 draft revision 6)

| # | date | decision | where it acts |
|:--|:--|:--|:--|
| 24 (owner) | 2026-11-21 | After the bigger-memory design (`t14_bigmem/`): draw 0 only, as a probe, stage `cap:14:0:7:54000000000` on a 32 vCPU / 256 GB reserved pod; per-job cap US$65, study spend guard US$535, ceiling US$540 (decision 21 had US$510 / US$540). Draw 1 needs a later decision from draw 0's outcome. | `common.sh` `STUDY_STAGE`, `STUDY_NAMES`, `VCPU`, `CEIL_USD`, `STUDY_GUARD_USD`; amendment sections 1, 2, 5. |

| id | date | decision | reason (one line) |
|:--|:--|:--|:--|
| Q1 names | 2026-11-21 | The allow-list is `pkm-x-t14-k0-c7-d54` and `-r1` (decision 13: host-side evidence only), plus `pkm-x-dev-*`. The names of decision 23 are retired; their request and generator stay as a record (`requests_m112/`, `make_requests_m112.sh`, now self-contained and reproducing `44e009f1…`). | The `-m112` cell is measured (`results/SNAPSHOT_20261117`, oversize at step 35, censored). |
| Q2 request | 2026-11-21 | `make_requests_d54.sh` derives `requests_d54/pfd-x-t14-k0-c7-d54.json` from `requests_r2i/pfd-x-t14-k0-c6.json`: command[1] `cap:14:0:7:54000000000`, memory 238Gi, job name `pfd-x-t14-k0-c7-d54-20261108`, one sentence in the note; pinned SHA-256 `089176f2…`; it passes the r2i validator (a first, longer sentence failed the 2,048-character limit). | The changes the design lists; 238Gi is the largest whole GiB of the 256 GB pod and gives `4·C <= request − 36 GiB` at `C = 54·10⁹`. |
| Q3 pod | 2026-11-21 | `cpu5m` at 32 vCPU / 256 GB, no fallback (`cpu3m` has the same memory but would be a second deviation); `cpu.memory` floor 256 GB; cgroup `memory.max` before start `max` or at least 256,000,000,000 bytes (238.42 GiB = 201.17 GiB of `B'` cap + the design's 36 GiB headroom + 1.25 GiB; the design's worst-case peak 230.9 GiB). `RAYON_NUM_THREADS` stays 12. | `t14_bigmem/MEMORY.md` section 5, `HOSTS.md` section 2; Amendment 9. |
| Q4 budget | 2026-11-21 | Per job: price × 30 h <= US$65 (2.08 × 30 = 62.40). Study: ledger sum + worst case <= US$535 (465.84 + 62.40 = 528.24). | Owner decision 24. |
| Q5 memory rule | 2026-11-21 | P2p (a 112Gi attempt with a peak above 112 GiB) is replaced: OOM is cgroup `oom_kill > 0` with exit 137 (P2b) or without (P2k), or a memory peak above the pod's numeric `memory.max` (P2p; the tree's value, else the one read before start). All are censored at the new size. Classifier `a33-r2`. | Owner decision 15 mirrored an EKS 112Gi request; here the limit is the pod's own. |
| Q6 reading | 2026-11-21 | The classifier writes a `reading` into `outcome.json` from the class, the row and the log's stop line: refutes, `d_F = 7`; degree 7 completed without refuting, `d_F >= 8`; oversize (stop line `oversize`), budget (stop line `budget`), OOM or LIMIT, censored with `d_F >= 7` standing; anything else, no reading (owner). | The pilot writes `timed_out = timed_out || oversize`, so the row alone does not tell the two stops apart. |
| Q7 amendment | 2026-11-21 | The route draft and the design's "proposed Amendment 34" are merged into one draft Amendment 33; `t14_bigmem/project.py` is an analysis script, not part of the route. | One frozen text for the route and its one job. |

## Owner decisions 25-27 and the fourth review (2026-11-21, Amendment 33 draft revision 7)

| # | date | decision | where it acts |
|:--|:--|:--|:--|
| 25 (owner) | 2026-11-21 | While the pod exists, a scheduled task starts a fresh cloud session every hour that runs the route's reaper, independently of this session; removed when the pod is gone. Supersedes decision 14 for this job. | `INDEPENDENT_REAPER.md` (the stored prompt); `reap.sh` header and `REAPER BLIND` exit; amendment section 2.6. |
| 26 (owner) | 2026-11-21 | The shared balance covers US$62.40: Amendment 29's per-batch confirmation for this job; results are fetched every 15 minutes while it runs. | `watch.sh`; amendment sections 2.6 and 5. |
| 27 (owner) | 2026-11-21 | `pkm-dense-rerun-20261115` (US$37.26) is study spend; study ceiling US$600, guard US$595; per-job cap stays US$65. | `ledger.tsv` row; `common.sh` `STUDY_GUARD_USD`, `STUDY_CEILING_USD`; amendment section 5. |

| id | date | change | reason (one line) |
|:--|:--|:--|:--|
| R4-M1 | 2026-11-21 | Section 2.6 rewritten: who deletes the pod (fetch.sh through the git gate, fetch.sh at the hard deadline, reap.sh from the session and from the hourly independent task), how Amendment 32's relaunch precondition is met, and the worst-case pod life with each cadence. | The old sentence claimed a backstop that shared Amendment 32's root cause. |
| R4-M2 | 2026-11-21 | Amendment 29 applied: decision 26 is the balance confirmation; `watch.sh` is the 15-minute fetch (every 900 s, segments of at most 2 h, deletes nothing itself). | Frozen precondition not addressed before. |
| R4-M3 | 2026-11-21 | Settled by decision 27; `bdr-growthlaw` stays outside (another study, `docs/session-logs/2026-11-15.md:4536`). | Ledger attribution. |
| R4-M4 | 2026-11-21 | The tests read `tests/fixtures/ledger_fixture.tsv` (one row of US$500), never the live ledger; B03/B04/B04b boundaries computed against it (65.05 > 65; 595.004 > 595; 594.994 <= 595). | The live ledger moved under the fixed increments. |
| R4-M5 | 2026-11-21 | `V2_FIELDS.md`: list entries carry `env` and `args` (only six fields are kept); 32 vCPU / 256 GB floors; smoke3 results and what is still unobserved. `run.sh` comment at step 5. The 230.9 GiB peak is called a projection with the review's extra terms. The stock sentence matches the code (exit 10 also on an empty `dataCenters` list). Worst-case pod life 30.8 h, US$64.06, used by every cost check (`WORST_HOURS=30.8`). | Text the smoke and the API contradicted. |
| R4-S1 | 2026-11-21 | The boot loop and `cgroup-prestart.json` record `memory.oom.group`, `memory.swap.max` and `vm.max_map_count`; section 4.2 says how a group OOM and an allocation abort are classified. | Record only; no rule reads them. |
| R4-S3 | 2026-11-21 | `memory.max` `max` refused for study names (allowed for development names). | The headroom and P2p need the numeric limit. |
| R4-S4 | 2026-11-21 | The reading table says "no contrary stop line"; tests R10 and R11 for a missing stop line. | The code already accepted it; a deadline at the step loop prints no stop line. |
| R4-S5, S6, S9, S14 | 2026-11-21 | Text: the existing grid-zeros certificate for draw 0; the relaunch fits the guard only after a base pod life of about 13 h or less; "structurally identical" traces; the time-band corrections. | Disclosure. |
| R4-S8 | 2026-11-21 | An OOM with the peak under 90 percent of `memory.max` carries `oom_note` (likely a host-level or group kill). Classifier `a33-r3`. | So it is not read as "54·10⁹ is too small". |
| R4-S11 | 2026-11-21 | Stale "guard US$510" comments in `common.sh` and `ledger.tsv` fixed; the repeated paragraph in `ledger_SOURCES.md` removed. | Hygiene. |
| R4-S13 | 2026-11-21 | A failed create's non-JSON response is not copied to `run.log`; only its size is. | It could echo the body with the pod token. |
| Not done | 2026-11-21 | S7 (reaper fallback to known pod ids when the list is unreadable; a pod after a timed-out create without a ledger row; the unconfirmed pre-start delete), S10 (token recovery on a real pod), S12 (main's numbering, to check at the freeze), the request's inherited first note sentence (S11, documentary). | Larger changes or not doable offline; the reaper now reports a blind pass instead. |
