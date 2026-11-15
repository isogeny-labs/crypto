# Decisions behind the capped RunPod route (draft Amendment 18)

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
