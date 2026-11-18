# Red-team review, round 2: draft Amendment 18 (R2-H), capped runs on reserved RunPod CPU pods

Reviewed: commit `7305d2ee8d` on branch `claude/serene-davinci-ujoi58`. The working tree was clean for the reviewed paths.
Paths are relative to `research/pkm_falldegree_unblock_20261107/runpod_capped/` unless they start with `research/`.
"Draft" means `AMENDMENT_R2_H_runpod_capped.md`. Every number here is `[pcc:unverified]`.

What I did not do: edit any repository file, commit, call RunPod or AWS, create or touch a pod, or read
`scratchpad/rpc_state`. I made one read-only GitHub REST call to list PR #4173's files.

How I ran things: all tests and probes ran on a `git archive` copy of the commit in the scratchpad. The suite rewrites
`tests/README.md`, `tests/results.tsv` and `tests/out/`, so running it in place would have dirtied the repository.

## Verdict: do not freeze (at 7305d2ee8d)

The commands, binaries, stage strings, budgets, `--max-dense` values and thread count are unchanged. I checked that
myself (see "Verified myself"). Most of round 1's MUST items are now closed in code, and offline tests cover them.

Five things block the freeze:

1. **Choosing the outcome after seeing data.** The producer can still turn a run into an outcome that buys a relaunch,
   after seeing partial or complete data. Three cheap paths exist: `--abort`, check-ins made while the network is
   blocked, and one unreachable reading at the deadline (N1, N2, N3).
2. **The memory rule changes Amendment 14.** In one plausible t = 14 case it censors the cell where Amendment 14 would
   run it again at `--max-dense 27900000000` (N4).
3. **"130 passed" does not reproduce.** From the committed files the suite gives **122/130**: two fixture logs are
   git-excluded. The same local exclude rule will silently drop every run's `run.log` and `fetch.log` receipts (N5).
4. **Attempt accounting needs the launching session to live 25 h.** It rests on receipts and pod tokens that exist only
   in that session (N6).
5. **Pre-freeze conditions in the draft itself are not met.**
   - The draft requires a development smoke of the revised scripts first (§9 point 5, §10). No line of revision 2 has
     run on a pod.
   - Three owner confirmations are open: the relaunch reading (§4.3), the session-dependent ceiling (§9 point 3) and the
     memory rule (§9 point 4).

None of the fixes needs new science. Most are a few lines in `fetch.sh`, `common.sh` and `reap.sh`, plus one
`.gitignore` line. Because they change the classifier and the reaper, the tests and the dev smoke should run on the
corrected scripts before the freeze.

## Round-1 MUST items M1-M7: closure in the current code

I used `reviews/REVIEW_1_red_team_59e4871.md` only for this table. "Test" names a case in `tests/run_tests.sh` that I
ran myself; every named case passes once the two missing fixture logs are restored (see N5).

| item | status | evidence (file:line; test) |
|:--|:--|:--|
| **M1** conflicting classes, producer choice | **Partly closed** | One table: `fetch.sh:124-194`. It decides deadline vs lost (`fetch.sh:182-190`; D14, D14b, C07), completed-then-restarted (`fetch.sh:152-156`; D16, D16b), done after the deadline (`fetch.sh:168`; C09, D03) and the restart detectors (`fetch.sh:128-149,155-156`; D19, D20, C14). **Not closed: abort.** Round 1 named "abort followed by a relaunch lets the producer re-draw the host". ABORTED is now `lostlike` (`common.sh:94`), so `cell_next` grants `-r1` after it (`common.sh:105-107`; E03 expects `-r1`). See N1. One unreachable reading at the deadline now has a fixed class (LOST P6; D15, C08), but that class grants a relaunch (N2). |
| **M2** attempt limit, overwritten evidence | **Closed in code; durability open (N6)** | Exact allow-list: `common.sh:34-36`, `run.sh:18` (A01, A02). Any live `pkm-x-` pod refused: `run.sh:100-101` (B01). Order: `common.sh:101-131`, `run.sh:47-48` (A06-A10, A23-A25, E01-E20). Receipts and results never reused: `run.sh:27-28,155` (A04, A05, A26). `fetch.sh` refuses after `cost.json` and never removes a results tree: `fetch.sh:209,317` (C02 fourth call exits 9). The earliest-admissible rule is stated in draft §1. |
| **M3** no backstop for 90,000 s / US$40 | **Partly closed** | Fetch, gzip or tar failures no longer exit: `fetch.sh:327-332`. Delete at the deadline or on abort even without a transfer: `fetch.sh:293-294,363` (C07, C08, C03). A timed-out create is found by name and deleted: `run.sh:120-152` (B07, B08). Trap set before create: `run.sh:157`. An unconfirmed delete resumes: `fetch.sh:250,280` (C15). The reaper exists (`reap.sh`; F01-F04). **Open:** the reaper is scheduled by the same session (draft §2.6), so it is not independent of the session. The owner's acceptance that the ceiling depends on the session is asked for (§9 point 3) but not recorded. New reaper defect: N3(c). |
| **M4** Amendment 14 OOM branch | **Closed for the round-1 defect; new deviation (N4)** | Cgroup copy in the start command: `run.sh:163-182` (B12, B13; offline only, never on a pod). OOM read from `oom_kill`: `fetch.sh:175-176` (D10-D12, C10, C11). `-m112` reachable: `common.sh:97,111-115` (E05-E07, E13). The `memoryInGb` claim is gone (draft §2.7). |
| **M5** client hash | **Closed** | `rp_pinned.sh` SHA-256 `b4b60199…` equals main's `runpod/rp.sh` at `d2a16c723d` (I ran `git show`). Pins enforced: `run.sh:60-64` (A13, A14, G-pin). `SHA_FILE` refused: `run.sh:25` (A03). PR #4173 does not touch `server.py`, `image.json` or `binaries.sha256` (REST file list). |
| **M6** admission rule 7, vacuous reserved check | **Closed** | `machine.secureCloud`, data center, flavor, 16 vCPU, memory floor and `costPerHr == securePrice` are read from `pod-create.json` before start (`run.sh:204-208`; B05, B06). The request record is checked (`run.sh:209-210`). Admission rule r7: `fetch.sh:105-108` (D23). On the API side, "reserved" rests only on the price equality (`interruptible: null` passes), as the draft states. |
| **M7** "readings do not depend on the host" | **Closed** | §7 rewritten. One flavor, no fallback: `common.sh:18`, `run.sh:103-110` (B02). Residual risk, disclosed in general: the CPU model can still vary within `cpu5m`. The smoke pod was an AMD Threadripper 7960X (`tests/fixtures/smoke_tree/host.txt`). |

## New MUST items

### N1. An abort buys a relaunch: round 1's re-draw lever remains (wrong)

- **What the draft says.** Draft:284-285: ABORTED "is charged exactly like LOST for relaunch accounting, so an abort cannot
  buy a different host beyond what a loss would".
- **Why that is wrong.** A loss is not under the producer's control; an abort is.
  - `fetch.sh NAME --abort` on any running run before its deadline gives ABORTED P5 (`fetch.sh:187-188`; C03).
  - `cell_next` then allows `-r1` (`common.sh:94,105-107`; E03).
- **What the producer can see before choosing.**
  - Mid-run, the pod token lets the session call `rp_pinned.sh fetch` at will. That returns `host.txt` (CPU model) and
    `runs/<cell>.log`: F4 step degrees and timings, which the runner copies into `/results` on every fetch
    (`runpod/server.py:96`).
  - So a run heading for `timed_out` on a slow host can be aborted and re-drawn. At t = 14 the budget can decide the
    reading (draft §7: estimate 75,900 s against 86,400 s).
- **Fix.** ABORTED uses the attempt, and any further attempt needs the owner (round 1's proposal). Change `lostlike`, or
  treat ABORTED as RESOLVED in `cell_next`, and E03 changes with it.
- **Cheapest test.** E03 must expect the next cell, not `-r1`.

### N2. "No information" counts as a loss: a healthy pod is deleted and the run gets a relaunch (wrong)

There are three paths. In each, the absence of evidence becomes LOST, which grants a relaunch, and a healthy pod is
deleted.

- **(a) A network outage on the session's side for an hour or more.**
  - The unreachable rule takes "REST not healthy" to include "REST not readable" (`fetch.sh:126-127,143`; draft:196-201).
    If the session's own egress is down, the runner, the REST record and the list are all unreadable.
  - Two such check-ins at least 1 h apart give LOST P3. Probe: two check-ins 2 h apart with `live null`, `rest {http:0}`,
    `listed null` give `LOST P3`. D07 already shows the case with HTTP 500.
  - `fetch.sh` then sets `reason=lost` and deletes the pod on the next call that has network access.
  - Anyone can produce this on purpose: run `fetch.sh` twice an hour apart with `HTTPS_PROXY` pointing at a dead port, as
    the tests do. It also happens by accident during proxy outages.
- **(b) One unreachable status at or after the run deadline.**
  - This gives `reason=deadline`, no fetch, LOST P6, and deletion. That holds even when the REST record is a healthy
    RUNNING pod (C08: the stub's REST record is healthy) and even when the runner had finished before the deadline.
  - A budget-stopped t = 14 run ends at about 86,4xx s, and the deadline check-in comes about 1 h later. So the one
    reading that decides the case is also the easiest to spoil.
- **(c) A failed transfer check at or after the deadline on a run the runner reports done.** This gives LOST P6 (D21).
- **Fix.**
  - A loss needs positive evidence: a REST HTTP 200 with a status other than RUNNING (or a changed `lastStartedAt`, see
    S1), a successful list without the pod corroborated by GET 404, or a reachable runner that is idle or reports another
    start.
  - "Nothing readable" is no information. At or after the deadline with the REST record healthy, retry the status and the
    fetch until the hard deadline (creation + 26 h).
  - Only then delete, under a class that grants no automatic relaunch (for example UNOBSERVED, which goes to the owner).
- **Cheapest test.** The probe above must give PENDING. C08 and D21 must keep the pod and retry until the hard deadline.

### N3. A loss can be manufactured after the result is seen, and the receipts cannot tell (unsupported; partly wrong)

- **(a) Read access and delete power.** The session holds the pod token, which gives read access to `/results.tar.gz` at
  any time; draft:179-186 calls it "read access only". It also holds the account key, which can delete, stop or restart
  pods. The REST record and the list keep no independent record of who ended a pod. On EKS, by contrast, losses were
  attributed from the audit log ("imminent node shutdown", `SPOT_LOSSES.md`).
  - So the producer can fetch, see the row, delete the pod, and get LOST P3 plus a relaunch.
  - Draft §4.3 already asks the owner to confirm the relaunch reading. The owner must be told that on this route every
    LOST can be manufactured.
  - Fix:
    - record `lastStatusChange` from each REST check-in (it is in `rest-*.json` but not in `checkins.jsonl`) and cite it
      in the loss reason;
    - a LOST whose only evidence is "not listed", "desiredStatus not RUNNING" or "unreachable", with no host-side cause,
      gets its relaunch only by owner decision;
    - state that no `status` or `fetch` call is made outside `run.sh` and `fetch.sh`.
- **(b) The manual-recovery paragraph (draft:171-178) throws away a tree the producer has already seen.**
  - After `fetch.sh` dies between `mv` and `collect.json`, the kept tree, which the session can read, becomes an orphan
    that "enters no rule". The class is then recomputed. If the pod has meanwhile disappeared, deleted out of band or
    lost, the class is LOST with a relaunch, although a completed, manifest-checked tree exists.
  - Re-running `fetch.sh` also overwrites `local-sha256.txt`, `tarball.sha256` and `admission-lines.txt` (`fetch.sh:342-350`
    use `>`). That contradicts step (1) "changes no receipt" and draft §6 "never overwritten".
  - Fix:
    - write a `collect-started.json` intent record before the first fetch;
    - classify from the orphan tree when its manifest equals the `local-sha256.txt` written right after the `mv` and the
      prefetch check-in shows done under `s0`;
    - never overwrite those receipts.
- **(c) Reaper defect: it deletes a running study pod.**
  - `reap.sh:32` marks a pod due when `start.json` has no start 70 min after creation. It then runs `fetch.sh`, and
    deletes directly if the pod is still listed, whatever `fetch.sh` returned (`reap.sh:40-47`).
  - When `start.json` is missing but the runner is running, `fetch.sh` takes `s0` from the live status and exits 41. The
    reaper still deletes, and the next `fetch.sh` gives LOST P3.
  - Probe in the stub sandbox: `run.sh` dev launch, `rm start.json`, `createdAt` set 90 min back, `reap.sh` prints
    `fetch.sh exit 41; direct DELETE HTTP 204`, and the next `fetch.sh` gives `LOST P3`.
  - `start.json` goes missing if `run.sh` dies during the start call: the redirect at `run.sh:237` creates the file before
    the response arrives. It also goes missing if someone deletes a local receipt.
  - Fix: for the "no runner start" reason, do not delete when `fetch.sh` exits 41.
  - Cheapest test: the probe above must leave the pod running.

### N4. The memory rule censors a cell that Amendment 14 would run again at the larger limit (wrong: it changes Amendment 14)

- **What the draft says.** Draft:266-272: a 56Gi-stage run whose peak is above 56 GiB "counts as" the cell's `-m112`
  attempt. "It keeps its own stage (`--max-dense 12800000000`); if its row has `oversize true`, it is censored as an
  `-m112` run stopped on its limit". In code, `cell_next` skips `needs_m112` whenever `as_m112` is true
  (`common.sh:111`).
- **What Amendment 14 says** (`research/prime_fall_degree_20261027/PREREGISTRATION.md:508-515`). Such a run would have
  been `OOMKilled` at 56 GiB and re-run at `--max-dense 27900000000`. That `-m112` run is censored only if it stops on
  its own limit, 27.9e9, not 12.8e9.
- **Why the shortcut fails in this case.** It is equivalent only when the 56Gi-stage row has `oversize false`: then the
  `-m112` run computes the same thing.
- **Why the case is plausible at t = 14.**
  - The `--max-dense` check runs at every F4 step (`src/cryptanalysis/f4_fp_tower.rs:1578` at `f13a690df0`).
  - By Amendment 7 A's formula, a step whose dense block is near 12.8e9 entries leaves 8 GiB of headroom. A peak just
    above 56 GiB at one step followed by `oversize` at the next is exactly the boundary case.
  - Round 1 derived about 1.64e10 dense entries at t = 14, above 1.28e10.
- **Probes I ran.**
  - The classifier, on a synthetic tree with pilot peak 61,440 MiB and row `oversize true`, gives `ADMITTED P2m3`,
    `as_m112 true`, row `oversize true`.
  - `next_allowed` with that outcome moves on to `pkm-x-t14-k1-c6`, so no 27.9e9 run follows.
- **Fix.** When `as_m112` holds and the row has `oversize true`, the attempt is the 56Gi attempt censored by limit. The
  `-m112` run at 27.9e9 still follows, and the relaunch accounting is unchanged. Equivalently, apply "counts as the
  `-m112` attempt" only when `oversize` is false.
- **Cheapest test.** An E-case `pkm-x-t14-k0-c6:ADMITTED:true:true` must expect `pkm-x-t14-k0-c6-m112`.

### N5. "130 passed" depends on working-tree-only files, and receipts would be silently dropped (unsupported)

- **The fixtures.** `tests/fixtures/smoke_tree/runs/X-m2-t8-k0-c5.log` and `.gridzeros.log` are not committed. The local
  rule `research/*_20261107/**/*.log` in `.git/info/exclude` hides them (`git check-ignore -v`).
- **What a run of the committed files gives.** From a `git archive` of `7305d2ee8d` the suite gives **122 passed,
  8 failed**:
  - C02, D02, D13, D16, D17, D22 and D23 become `NOT_ADMITTED P2u`, because no pilot peak line is found;
  - D28 becomes `ADMITTED P2m3 as_m112=true`, because the cgroup fallback is used.

  With the two working-tree logs copied in, it gives 130/130. `tests/README.md:150` ("130 passed") and the commit message
  are therefore not reproducible from the commit.
- **The receipts.** The same exclude rule matches `receipts/<name>/run.log` and `fetch.log`. Admission rule 8 (draft:333)
  cites a `fetch.log` line, and the launch log carries the protocol hash and git HEAD.
  - The smoke's logs are committed only because they were force-added.
  - I checked in a scratch repository that a negation in the route's own `.gitignore` re-includes such files, because
    `.gitignore` takes precedence over `info/exclude`.
- **Fix:**
  - add `!*.log` scoped to `receipts/`, `receipts_dev/` and `tests/fixtures/` in `runpod_capped/.gitignore`;
  - commit the two fixture logs;
  - re-run the suite from a clean clone.

  The suite also needs the layer tarball and binaries from the session scratchpad (`run_tests.sh:9-11`). Add a script
  that fetches them by `image.json`'s digest, or document how to obtain them.

### N6. Attempt accounting and classification depend on state that dies with the launching session (unpriced)

- **What is local.** `run.sh` and `fetch.sh` write `receipts/`, `ledger.tsv` and the tokens in `RPC_STATE` locally. The
  draft says nothing about committing or pushing them during a run of up to 25 h.
- **What happens if the session is lost, the expected case for a 24 h run here:**
  - **(i) No tokens.** The new session has no pod token. Every status call comes back 403 and reads as "unreachable", so
    the first check-in after the deadline gives LOST P6 with a relaunch, even for a completed run (N2b). `reap.sh` then
    deletes the pod directly ("no pod token here").
  - **(ii) No receipts.** Without receipts, `next_allowed` reports the base name as NONE. Once the reaper has removed the
    old pod, `run.sh` will launch the same name again. That is an uncounted second attempt; `run.sh` never consults
    `ledger.tsv`.
- **Fix:**
  - commit and push `receipts/<name>/` and `ledger.tsv` right after `run.sh` exits 0 and after every `fetch.sh` call;
  - `run.sh` refuses a study name that has any ledger row whose source is not a `.prestart-*` or `.nocreate-*` path;
  - `fetch.sh` recovers a missing token from the REST pod record's `env`, which the API returns (`run.sh` redacts it from
    receipts). The token goes into `RPC_STATE`, never into receipts.
- **Cheapest test.** In the stub sandbox, delete `receipts/pkm-x-t14-k0-c6` and the token after a launch, let the reaper
  remove the pod, and check that `run.sh pkm-x-t14-k0-c6` is refused.

## SHOULD

- **S1. `lastStartedAt` as a loss detector kills runs that are provably alive.** C14 classifies "lastStartedAt changed
  while the runner reports running under `s0`" as LOST P3. `server.py` keeps its state in process memory, so a runner
  reporting `running` with `started == s0` proves the container has not restarted. Count a `lastStartedAt` change only
  when the runner is unreachable or not running under `s0`. Whether the field changes on restart is still unknown
  (§9 point 5).
- **S2. Unparsable receipts are read as empty, so the classifier fails open.**
  - `fetch.sh:56` falls back to `[]` when `checkins.jsonl` has one bad line. My probe: a run whose idle check-in gives
    `LOST P3` gives `PENDING P7` once one malformed line is appended.
  - A partial line, written when `fetch.sh` is killed mid-`tee`, can erase a P4 LIMIT check-in, and the run becomes P6.
    The same holds for `collect.json`.
  - Fail closed: exit with an error.
- **S3. Pin the classifier's version.**
  - `outcome.json` does not record the SHA-256 of `fetch.sh` or `common.sh`.
  - `run.sh` logs the route hashes (`run.sh:57`) but does not compare them with the frozen §10 values.
  - Make `run.sh` and `fetch.sh` refuse unless their hashes appear in the frozen Amendment 18 text in
    `PREREGISTRATION.md`, and write the hashes into `outcome.json`.
  - Likewise hard-code the layer digest `a0f7e28e…` and the binary hashes in `common.sh` instead of reading the mutable
    `image.json` and `binaries.sha256`.
- **S4. The cgroup memory limit is never checked.**
  - P2b and P2m1 read "exit 137 and `oom_kill > 0`" as OOM. For a `-m112` run, or a P2m1 run, that matches Amendment
    14's `OOMKilled` at 112 GiB only if the pod's `memory.max` is at least 112 GiB (120,259,084,288 bytes) or `max`.
  - `memory.max` is recorded (`fetch.sh:78`) but enters no rule; the -m112 floor is checked only against the API's
    `memoryInGb`.
  - Add the check. If it fails, the case goes to the owner.
- **S5. The direction of the VmHWM bias.**
  - The pilot's VmHWM leaves out what counted against EKS's 56Gi cgroup limit (requests equal limits,
    `taskq/deploy/eks-karpenter/submit-job:80-81`): the `aws s3 sync` process every 300 s, the shells, page tables and
    unreclaimable kernel memory.
  - So the rule admits some runs EKS would have `OOMKilled`, in a band of roughly the top 0.5 GiB below 56 GiB.
  - State the direction and the band, or use a margin.
  - Also copy the largest `peak` log line into `admission-lines.txt`. The `.log` file is git-excluded, so the memory
    decision should be checkable from git.
- **S6. Reaper robustness.**
  - An unparsable or missing `createdAt` makes `epoch` fail under `set -e`, and `reap.sh` exits mid-loop (`reap.sh:22`).
  - A pod with a null `name` makes `list_pkm`'s `startswith` fail. Command substitution does not inherit `set -e`, so
    the list is silently truncated, and that affects the serial rule in `run.sh:100` too.
  - Treat an unparsable age as due, and fail `list_pkm` on any jq error.
  - Corroborate "not listed" with GET 404, because list pagination is unverified.
- **S7. Ledger and cost bounds.**
  - `precleanup` closes the ledger row even when the deletion was not confirmed (exit 11; `run.sh:148-151`). Keep the
    row open at the worst case.
  - §5 ("8 × US$27.04"), the ledger's open rows and the US$400 check all use 26 h. The draft's own reaper bound is about
    30 h (US$31.11; 8 runs make US$248.9). Use one figure.
- **S8. The upload wait holds an idle pod until the run deadline.**
  - The completed-run branch keeps the pod until `--delete RECORD` or the run deadline (`fetch.sh:288-296`). A t = 13
    run at EKS speed (about 11,570 s) would idle for about 21.8 h, roughly US$22.7 more.
  - If this container still has no AWS credentials (Amendment 16's disclosure), every run costs the full 26 h.
  - Cap the wait, for example at 2 h after collection, or decide it with the owner beforehand.
- **S9. Gaps in the classifier procedure.**
  - If `fetch.sh` dies after `collect.json` but before `outcome.json`, the collected branch never classifies. The name
    stays INFLIGHT and the order is blocked forever.
  - Rule 8 says "a 204" while the code accepts 200 or 204. A secured completion whose pod later returns 404 on delete
    fails rule 8, and the draft does not say what a rule-8 failure means for an ADMITTED row.
- **S10. Disclosures.**
  - The t = 13 median will combine an EKS Xeon draw 1 with a RunPod draw 0. §7 discloses the cross-host case only for
    t = 14.
  - Name the smoke host CPU (AMD Ryzen Threadripper 7960X, 24 cores, 48 threads).
  - The run deadline counts from the runner's start, not from the job start as EKS's `activeDeadlineSeconds` does.
- **S11. Re-drawing the host before start.** Before start the session sees `nproc` (48, the host's threads) and the host
  `MemTotal` (`smoke/receipts/status-ready.json`), which reveal the CPU class. Record pre-start failures ended by a
  signal (rc of 128 or more), and send a second such failure to the owner.
- **S12. A session-independent ceiling.** On the dev pod, check whether RunPod injects `RUNPOD_POD_ID` and a pod-scoped
  key. If it does, a self-delete at creation + 26 h in the start command would make the US$40 ceiling independent of the
  session.
- **S13. Authority that is not on record.**
  - Decisions D1-D8 and the memory-rule decision of 2026-11-15 are cited as authority (draft:66, 89, 108, 151, 191, 254).
  - They appear in no committed file I could find: not `DECISIONS.md`, not `docs/session-logs/2026-11-15.md`.
  - Record them.
- **S14. Small points.**
  - There is no lock shared by `run.sh` and `fetch.sh --abort` or `reap.sh` on the same receipts.
  - `tests/README.md` says the memory cases are D25-D34; they run to D36.
  - The `-m112` notes still open with "--max-dense 12800000000"; the added sentence corrects it.

## Focus questions, briefly

1. **Choosing an outcome after seeing a result.** Yes, through:
   - abort (N1);
   - check-ins with the network blocked, or a spoiled deadline reading (N2);
   - out-of-band deletion after a token fetch, the orphan procedure and the reaper path (N3).

   The classifier itself is deterministic, the precedence P1-P7 is unambiguous on the cases I tried, and the memory
   source is not a choice: pilot first, cgroup only when the log has no peak line.
2. **Changes to what is computed.** None found: argv, binaries, `--budget`, `--max-dense` and rayon 12 are unchanged.
   Comparability changes beyond the disclosures:
   - N4: a censored cell where Amendment 14 would give a 27.9e9 run;
   - S10: the cross-host t = 13 median and the deadline clock.
3. **Pod lifetime and cost.**
   - If the session dies and the reaper runs in that session, nothing deletes the pod (disclosed; the owner must accept).
   - The reaper touches only `pkm-x-*` pods, which I confirmed in code and with F01, but it can kill a running study pod
     (N3c).
   - The US$400 check relies on a ledger the producer asserts is complete (`complete_as_of never` today).
4. **Secrets.** No new leak found:
   - the key is never in argv, since it goes through a 0600 header file and the `printf` builtin;
   - the token is out of argv for the create call;
   - B10 checks that neither the token nor the key is in receipts.

   Residual: if a create response is not JSON, its first 500 bytes go to `run.log` (`run.sh:197`). An error that echoes
   the request body would expose the token of a pod that was never created, which is harmless.
5. **Memory rule against Amendment 14.**
   - Faithful when the row has `oversize false` (equivalent row) and for exit 137 with oom evidence, assuming
     `memory.max` is at least 112 GiB (S4).
   - Not faithful when `as_m112` holds and the row has `oversize true` (N4).
   - P2p mirrors Amendment 14's `-m112` OOM.
   - Edge cases tested: D25-D36. I added the exact 112 GiB boundary (`ADMITTED P2m3`, not above). The `as_m112` +
     `oversize` order case is untested.
6. **Reproducibility from the files.**
   - The hashes in §10, the request derivation, the validator result, the `entry_r2.sh` equivalence and the 20,618 MiB
     peak all reproduce.
   - The test summary does not (N5).
   - Decisions D1-D8 are not on record (S13).

## Verified myself versus taken from the text

**Verified myself:**
- **Hashes.** `common.sh`, `run.sh`, `fetch.sh`, `reap.sh`, `make_requests_m112.sh`, `rp_pinned.sh` and
  `runpod/server.py` hash to draft §10. `rp_pinned.sh` equals `runpod/rp.sh` at `d2a16c723d`. The scratchpad layer
  `r2g_dl/layer.tgz` is `a0f7e28e…`, and the six files in `r2g_layer/usr/local/bin` match `image/binaries.sha256` and
  `image.json`.
- **Test suite.** On a `git archive` of the commit: 122/130 (the 8 failures listed in N5). With the two git-excluded
  fixture logs added: 130/130.
- **Entry script.** `entry.sh` minus line 2 is identical to `entry_r2.sh` minus lines 2, 11 and 63-68.
- **`-m112` requests.**
  - `make_requests_m112.sh` regenerates byte-identical requests, and its output equals `make_requests_m112_output.txt`.
  - The diff against r2i shows exactly `job_name`, `command[1]`, `memory` and `provenance.note`.
  - `validate_command_request.sh` reports VALID for all six requests; the notes are 1,964 characters.
- **`--max-dense` formula.** It gives 12,800,000,000 at 56Gi and 27,900,000,000 at 112Gi.
- **EKS limits.** EKS sets memory limits equal to requests (`submit-job:80-81`).
- **Pilot and F4 source at `f13a690df0`.**
  - The pilot prints `memory R MB (peak P MB)`, as VmHWM in MiB, in every step line and in stop lines.
  - The `--max-dense` check runs per step at `f4_fp_tower.rs:1578`, and the chunk size comes from the rayon thread count
    at 1693-1694.
  - The largest peak in the local t = 13 draw 1 log is 20,618. I read only that maximum.
- **Runner behaviour (`server.py`).** Uploads are accepted only while idle, `/start` works once, and `/status` and
  `/results.tar.gz` stay readable after start. `os.cpu_count()` reads 48 on the smoke host, while `host.txt` shows
  `nproc` 16.
- **Probes on the scratch copy:**
  - session network outage gives LOST P3;
  - one malformed check-in line erases loss evidence;
  - the reaper deletes a running pod whose `start.json` is missing, and the run becomes LOST P3;
  - `as_m112` with `oversize true` gives ADMITTED P2m3, and the order moves to the next cell without a 27.9e9 run;
  - a peak of exactly 112 GiB gives P2m3.
- **Git and records.**
  - `.git/info/exclude` hides `receipts/*/run.log`, and a route `.gitignore` negation re-includes it (scratch
    repository).
  - Decisions 3, 4, 10 and 12 read as the draft cites them.
  - PR #4173 is open and unmerged at `b3f7605e`, and it changes `runpod/rp.sh`, `PREREGISTRATION.md`, `runpod/gate17/*`
    and a session log.
  - The ledger reads `complete_as_of never` and has one development row.

**Taken from the text, not verified:**
- **RunPod behaviour:**
  - which cgroup files a CPU pod exposes, and what `memory.max` reads;
  - whether `lastStartedAt` changes on a container restart;
  - whether `GET /pods` is complete (pagination) and always carries `createdAt` and `name`;
  - whether `GET /pods/{id}` returns `env`;
  - that `interruptible: false` and `SECURE` are honoured, and that billing equals `costPerHr`.
- **Main-session decisions.** That decisions D1-D8 and the 2026-11-15 memory-rule decision were made as described.
- **History and environment.**
  - That the first validator run failed at 2,088 characters.
  - That this container lacks S3 credentials today (Amendment 16 says so for its own session).
