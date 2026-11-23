# Coordinator ruling: P28 infrastructure failure (XP-20261029-packed-filter)

**Date:** 2026-10-29, after 20:45Z. **Role:** coordinator. **Scope:** the two first-launch P28 jobs, the launch card's ordering gate, and the P32 launch. This ruling changes no research status, launches nothing, and does not amend `PROTOCOL.md`.

**Binding texts:**
- `backlog/XP-20261029-packed-filter/PROTOCOL.md`, sha256 `703bde26…7e51` (freeze commit `f870408c17`): §4, §6.1, §9;
- `APPROVAL.md`, sha256 `0a35923c…6a40`;
- `cards/card-launch.md`: lines 13–21, 32–37, 195–241 and 505–511;
- `cards/card-analysis.md`: steps 2–4 and the Note.

§4 points to XP-20261029-dg-wall §4 for host loss (`/dev/shm/wt-chain/backlog/XP-20261029-dg-wall/PROTOCOL.md` line 78). That clause says: "A host loss is a pod lost before `job-exit.json` is written … A job that wrote `job-exit.json` with a non-zero code is not a host loss and is not rerun."

**What I read for this ruling:**
- the texts above;
- `taskq/deploy/eks-karpenter/run-ecbench`, `remote/run-request`, `remote/launch-remote`, `remote/verify-build`, `remote/buildspec.yml`;
- `remote/bridge.yaml` lines 127–128;
- `HARNESS.md` lines 49–62 and 195–222;
- P24's `wall/eks-results/job-exit.json` and `runner-identity.json`;
- the 19:12Z to 20:45Z entries of `docs/session-logs/2026-10-29.md`.

I opened no `records.jsonl`, no `session.json` and no P28 file of any kind. Every P28 fact below comes from the main session's report.

## Facts used, and what they do not establish

- **Wall job** `ic-dgbf-p28-20261029`, pod `cd784143-…`:
  - `job-exit.json` records `exit_code` 143 on ip-10-84-191-237. It was uploaded at 20:07:38Z;
  - `session.json` gives 20:06:10Z → 20:07:36Z;
  - `audit.json`, `l2-gate.json` and `control-comparison.json` are absent.

  In `run-ecbench`, exit 143 comes from `trap 'exit 143' TERM` (line 28). The finish trap then wrote `job-exit.json` and `SHA256SUMS` and uploaded them. The absent `audit.json` means the signal arrived during `ecbench verify --replay-all` (line 50), so no replay audit, L2 gate or native A/A admission exists for this session.
- **Counted job** `ic-dgbf-rho-p28-20261029`: nothing was under `runs/<job>/` at 20:36Z. This does not establish that its pod is lost. A pod that never started, or one that is still pending, looks the same in S3.
- **Launchers.** Both CodeBuild builds were TIMED_OUT at 2,704 s, with `timeoutInMinutes` 45. `run-request` waits `job.seconds + 180` = 3,780 s after submission (line 54), so it could never have finished inside 45 minutes once the pod waited about 53 minutes. Per the 20:02Z log entry, the project now reports 70 (`bridge.yaml` declares 70), and the P24 builds also recorded 45.
- **Cause of the SIGTERM: unknown.** The kill came about 3,277 s after the build started (and 2 s after the session finished). That is earlier than a 3,600 s Job deadline counted from submission, so a deadline kill as `HARNESS.md` describes it ("the Job deadline includes provisioning time") does not explain it as stated. Spot interruption, Karpenter disruption (drift or consolidation), and an infrastructure deployment near 20:00Z (the timeout changed from 45 to 70 at some point) are all consistent with the facts. None is verified. Nothing in this ruling depends on which one it was.
- **Disclosure of outcome-adjacent knowledge.** I record it here because it bears on (a). While diagnosing, the main session opened the P28 wall `records.jsonl` and `run.stderr`. It saw:
  - (i) one pooled aggregate: Σ `time.solve_wall_ns` over all 1,344 records, all four arms and the warm-ups pooled, = 35.39 s;
  - (ii) the session's span, 86 s, from `session.json`;
  - (iii) that the progress lines of `run.stderr` all report L2.

  It computed no per-arm figure and no ratio. I was told (i) to (iii) in the dispatch. I made no estimate from them, and no part of this ruling uses them. (i) and (ii) cannot be split by arm without the records, which I have not read. (iii) is instrument information, not effect information.

---

## (a) The P28 wall session (exit 143 after a complete session)

### What the frozen text decides

- **The size is undecidable.** §6.1 condition 1 requires `exit_code` 0 and says that "Any other code makes the size undecidable … except that exit 42 can trigger the one noise rerun of §4." 143 is not 42. Conditions 2 (`audit.json`), 5 (`l2-gate.json`) and 6 (`control-comparison.json`) also cannot hold, because the files do not exist. §6.2: "If wall-session §6.1 fails, the size is undecidable."
- **It is never rerun under this protocol.**
  - §4: "A job that wrote `job-exit.json` with a non-zero code is not rerun, except under the noise-rerun clause". That clause requires exit 42 (trigger condition 1).
  - The host-loss definition (dg-wall §4, incorporated by §4) requires that the pod was lost *before* `job-exit.json` was written. This pod wrote it.
  - card-launch line 511 says the same: "A job that wrote a job-exit.json with a non-zero code is never rerun by any other route."
- **Mechanically**, RELAUNCH with UNIT `r2` refuses at step 4: `PRED_JOB_EXIT_COUNT ic-dgbf-p28-20261029 1` → `STOP: … this is not a host loss`.
- **What this result means.** It is infrastructure signal: a pod killed by an external signal during the post-session audit. It is not a null, a regression, or any other reading of the filter at 28 bits (AGENTS.md: "A timeout, crash, or implementation failure is infrastructure signal, never a negative mathematical result"). The README reports P28 as "undecidable (infrastructure: SIGTERM during the in-pod audit, exit 143; no audit, L2 gate or A/A admission)". It enters no table and no `benchmarks/` row.
- **No post-hoc cure.** Running `ecbench verify`, the L2-gate `jq` or the A/A compare outside the pod would not satisfy condition 1. §6.1 also reads the conditions "from the job's own files". A locally produced `audit.json` or `l2-gate.json` would be a repaired result, and it is not permitted as decision evidence.

### Amendment: none

I make no amendment, and the `ic-dgbf-p28-20261029-r2` route with `request-P28-r2.json` is **refused**, for three independent reasons:

1. **§9 forbids it.** "No amendment is allowed after the first launch of any of the six jobs". `APPROVAL.md` ("Not changed") left §9 standing as written. Six jobs have now launched. Overriding the clause that exists to stop post-launch rule changes, at the first moment it binds, is the conduct it forbids.
2. **It would not be free of outcome knowledge.**
   - The first session's records have been opened, and (i) to (iii) above were seen.
   - The protocol's own standard for any rerun of a wall session is §4's blinding: no agent opens the first session's `records.jsonl` before the rerun has written its `job-exit.json`. That standard cannot now be met for this session.
   - (iii) in particular tells the reader that the lost session would probably have passed the gate. That is exactly the kind of knowledge that makes a "rerun or not" choice outcome-dependent.
   - An amendment whose only effect is to turn a known-undecidable size into a fresh chance, made after that look, cannot be shown to be blind, even though the trigger I would write (exit 143 after a complete session) is itself an infrastructure fact.
3. **The named request is bound to a different route.**
   - `request-P28-r2.json` and the job name `-r2` mean "host-loss rerun" everywhere they appear: §4's trigger text, card lines 18 and 511, the step-4 refusal and card-analysis's `WALL r2`.
   - Using them for a session that is not a host loss would make the record say something false. It would also need a card edit that removes the step-4 check, which is the check that encodes the frozen definition.

### What remains open: a successor protocol, not approved here

The 28-bit question can be asked again only by a **new, separately frozen and approved protocol**. Its terms:
- a new XP id;
- fresh curves and seeds at 28 bits (none of §3's P28 slugs or seeds `202610296283/4/6/8`);
- the same arms, statistic and decision rule;
- its own approval;
- a §8 disclosure of (i) to (iii) above and of this ruling;
- a pre-registered infrastructure-kill clause (see "Process note" at the end).

I do not approve one now. I will rank it **at the P32 verdict decision**, against the roadmap's bias toward exponent work. This is constant engineering, and its value depends on what P32 shows. At that decision I record either "successor approved for drafting" or "no successor".

### Seal (effective now)

The P28 wall session `ic-dgbf-p28-20261029` (pod `cd784143-…`) is **sealed**. Until I record "no successor", or a successor's verdict commit is pushed, no agent does any of the following:
- reads `records.jsonl`, `run.stderr` or `session.json` of that session beyond hashing them;
- runs `ecbench compare`, `band`, `work`, `phase_sums.py` or `positions.py` on it;
- computes or reads any wall field of it.

What remains permitted:
- downloading it and checking it against `SHA256SUMS`;
- recording its file list, `job-exit.json` and `build-info.json`.

When the seal lifts, the session is processed descriptively only, and it is never pooled. The main session's diagnostic look (i) to (iii) happened before this seal and is disclosed above. It is not a breach of the frozen text, whose blinding clause binds only first sessions that exited 42.

---

## (b) The P28 counted session (no `job-exit.json`)

### What the frozen text decides

- **This is the frozen host-loss route, if the pod is lost.** §4 (dg-wall §4) defines a host loss as "a pod lost before `job-exit.json` is written", rerun whole as `-r2`, with the loss recorded. The card's route is RELAUNCH, b = 28, UNIT `rho-r2` (`request-P28-rho-r2.json`, sha256 `7919707…4f04698`). Step 4 requires the predecessor's launcher pid to have exited and `PRED_JOB_EXIT_COUNT ic-dgbf-rho-p28-20261029 0`.
- **The frozen text does not decide when the pod counts as "lost".** An empty prefix is also what a still-pending pod leaves behind. If you relaunch while the original can still start, two counted sessions could run. The conditions below are therefore my dispatch conditions. They are not a card change and not an amendment, because the card says that "The main (reasoning-tier) session decides the route and which dispatch to send next".

### Conditions before RELAUNCH rho-r2 (all required, in this order)

- **B1. Deadline elapsed.** Let E be the `endTime` of the original counted build, from `PFD/receipts-P28-rho/build.json` (about 19:58Z; use the recorded value, not this estimate). The Job was submitted inside that build, so its creation time is at most E. The relaunch may be dispatched only when the current UTC time is ≥ E + 3,600 s + 900 s (about 21:13Z).
  - The 900 s of slack covers pod termination grace, the finish trap's S3 upload and clock skew.
  - **Stated assumption:** the Job's 3,600 s deadline counts from Job creation, including pending time (`HARNESS.md` line 201). This is unverified for this deployment.
- **B2. Kubernetes evidence, if any becomes available.** If the owner or any authorized route reports the original Job's status before the relaunch:
  - Job `active` or pending: do not relaunch. Ask the owner for an authorized Job deletion (`HARNESS.md` lines 215–217), then restart at B3.
  - Job failed, complete or absent: record the status and continue.

  If no status is available, B1 and B3 are sufficient.
- **B3. Re-list immediately before the launch.** In the same main-session turn, with no other dispatch in between:
  1. Dispatch VERIFY, b = 28, UNIT PAIR. Its output must show:
     - `S3_KEYCOUNT ic-dgbf-rho-p28-20261029 0`. This counts the whole prefix: any object at all, including a `launcher/` receipt, is a stop;
     - the `P28-rho` pid exited;
     - no build carrying the `P28-rho` request key with `buildStatus` `IN_PROGRESS`.
  2. Then dispatch RELAUNCH, b = 28, UNIT `rho-r2`. Its step 4 re-counts `job-exit.json` (it must be 0), and step 5 checks that the `-r2` prefix is empty.
- **B4. The loss is recorded before the launch.** The main session commits and pushes `research/ic_packed_filter_20261029/P28/P28-HOSTLOSS.md` before step 8 of the relaunch. It holds:
  - the two build ids with their `buildStatus`, `endTime` and `timeoutInMinutes`;
  - E and the B1 arithmetic;
  - the VERIFY output of B3, verbatim;
  - any Kubernetes status from B2;
  - a citation of this ruling.
- **B5. Rule if the original appears later (fixed now, before any counted data).**
  - Which counted session counts is decided from file existence alone, at the moment the counted analysis is dispatched and before any counted field is read.
  - If `runs/ic-dgbf-rho-p28-20261029/` then holds a `job-exit.json`, the host-loss classification was wrong:
    - the original is the counted session of record (card-analysis COUNTED `first`);
    - `-r2` is kept and reported descriptively, never pooled.
  - Otherwise `-r2` is the session of record (COUNTED `r2`).
  - A prefix that holds objects but no `job-exit.json` stays a host loss.
- **B6. A second host loss.** dg-wall §4 allows `-r3`, but the card has no `rho-r3` unit. If `-r2` also loses its host, the case comes back to me; the director must not improvise a unit.

P28 counted analysis is **deferred** while the (a) seal holds. card-analysis always processes the wall session, and its records check compares the counted records against the wall records. When the seal lifts, dispatch card-analysis with b = 28, WALL `first` and COUNTED as B5 decides.

---

## (c) Card amendment for launcher timeouts

### What the frozen text decides

**LAUNCH b = 32 cannot pass as frozen.** Step 4 (line 210) needs `receipts-P28/result.json` and `receipts-P28-rho/result.json`, and those files will never exist.

The requirement is a card-level ordering device, not a protocol condition:
- §6.1 reads every condition "from the job's own files after the `SHA256SUMS` check", never from `result.json`;
- the analysis tools never read `result.json` (checked: no reference in `cards/tools/{receipt,session_facts,wall_gate,counted_gate}.py` or `analysis-prelude.sh`).

The frozen gate also never required success. P24's counted launcher ended `state failed`, exit 23, and its `result.json` satisfied line 210.

So the fix is a **card revision** written by the director. It is not a protocol amendment, and §9 does not bar it. It is outcome-blind because:
- its gates are terminal-state and identity facts only;
- it reads no record field;
- it is written before P32 launches;
- it changes nothing about how P24 or P28 are read.

### Required content of `cards/card-launch-v2.md`

`card-launch.md` stays as the record of the P24 and P28 launches. Its sha256 is cited in v2 as superseded from the next dispatch on.

- **C1. Freezing.**
  - New file, new `CARD_SHA256`.
  - Every new helper (for example `s3verify.py`) goes in TOOLS, bound by sha256 in Step 2.
  - The file is committed and pushed before its first dispatch, and it cites this ruling.
  - Every unchanged procedure (L steps 0–3 and 5–8, W, V, T) is copied without change.
- **C2. New MODE `S3VERIFY`** (b, UNIT and TAG as for VERIFY; one tag per dispatch).
  - **Scope.** It reads S3 and writes only:
    - `PFD/s3verify-<TAG>/`: a few KB of check output and `s3receipt.json`;
    - a download directory `/dev/shm/pf-s3verify/<TAG>/`, which must not exist beforehand.

    It hashes `records.jsonl` and reads no field of it.
  - **Facts.** Each fact is printed as `S3V_<NAME>=true|false|absent`. The checks mirror `run-request` lines 68–79 and `verify-build`, with the pod-uid prefix taken from S3 rather than from Kubernetes:
    - `S3V_POD_UID_COUNT`: exactly one UUID prefix under `runs/<job>/`, with `launcher/` excluded;
    - `S3V_JOB_EXIT_PRESENT`, and `S3V_JOB_EXIT_IDENTITY`: `job_name` equals the tag's job and `pod_uid` equals the prefix;
    - `S3V_EXIT_CODE=<n>`, printed verbatim;
    - `S3V_SHA256SUMS_OK`: `sha256sum -c` is all OK (`run-request` line 74);
    - `S3V_MANIFEST_COMPLETE`: every downloaded file except `SHA256SUMS` is listed, and every listed file is present. This is stricter than `run-request`;
    - `S3V_BUILD_INFO`: `build-info.json` `source_revision` and `ecbench_sha256` equal the frozen request's `job.build` (`verify-build` lines 7–10);
    - `S3V_BINARY_SHA`: the first field of `binary.sha256` equals `job.build.ecbench_sha256` (`verify-build` line 11);
    - `S3V_INPUT_SPEC`: `input-spec.json` is JSON-equal to the frozen request's `job.spec` (`verify-build` lines 17–19);
    - `S3V_IMAGE_DIGEST=unverifiable_without_pod_status`. The value is `true` or `false` only when a pod-status JSON from an authorized Kubernetes read is supplied as a pinned input (`verify-build` lines 12–16). It is never true by default;
    - outcome facts, printed and never gated:
      - `S3V_EXIT0` (`run-request` line 76);
      - `S3V_L2GATE` (line 77, the jq expression verbatim);
      - `S3V_AUDIT` (line 78);
      - `S3V_CONTROL` (line 79).
  - **The gate.** `S3_TERMINAL=true` if and only if all of these hold:
    - `S3V_POD_UID_COUNT` is 1;
    - `S3V_JOB_EXIT_PRESENT` and `S3V_JOB_EXIT_IDENTITY`;
    - `S3V_SHA256SUMS_OK` and `S3V_MANIFEST_COMPLETE`;
    - `S3V_BUILD_INFO`, `S3V_BINARY_SHA` and `S3V_INPUT_SPEC`.

    The outcome facts and the image digest do not enter it. The frozen gate accepted a failed job's receipt, and a gate on success would make one failed size block every later size.
  - **Receipt.** It writes `s3receipt.json` holding:
    - every fact;
    - the S3 listing;
    - the sha256 of `SHA256SUMS`;
    - the request sha256 and the card sha256.

    It prints the receipt's sha256.
- **C3. Step 4 for LAUNCH (b = 28 or 32).** Line 210 is replaced. The dispatch names `PREV_WALL_TAG`, from {`P<prev>`, `P<prev>-r2`, `P<prev>-nr`, `P<prev>-nr-r2`}, and `PREV_COUNTED_TAG`, from {`P<prev>-rho`, `P<prev>-rho-r2`}. These are the sessions of record, chosen by the main session under §4 and, for P28, under this ruling (B5). For each of the two tags, step 4 requires:
  - the pid exited (`PAIR_DONE`);
  - either `receipts-<TAG>/result.json` exists, or `PFD/s3verify-<TAG>/s3receipt.json` exists with `S3_TERMINAL` true.

  A lost predecessor cannot be `S3_TERMINAL`; its rerun must be. Anything else is a `STOP`.

  For LAUNCH 32 the expected values are `PREV_WALL_TAG=P28` (exit 143, terminal) and `PREV_COUNTED_TAG=P28-rho-r2`, or `P28-rho` if B5 applies.
- **C4. Launcher budget check, fail-closed** (new step between Steps 6 and 7, LAUNCH and RELAUNCH):
  - `aws codebuild batch-get-projects --names isolab-experiment-launcher --query 'projects[0].timeoutInMinutes'` must print an integer ≥ 70. The 70 is `bridge.yaml`'s declared value. It covers run-request's 63-minute wait plus 7 minutes for setup, download and verification;
  - after Step 8, the card prints `timeoutInMinutes` from each `receipts-<TAG>/build-start.json`, as a reported fact;
  - a value < 70 at the pre-launch check is `STOP: launcher budget below 70 minutes`. The card never changes the project.
- **C5. WAIT.** W1 and W2 are unchanged. When W2 shows `result.json` absent for a tag whose S3 prefix holds a `job-exit.json`, the next dispatch is S3VERIFY for that tag. When the prefix is empty, the case goes to the main session under the conditions of (b).
- **C6. Analysis.** card-analysis needs no change: it reads S3 directly and checks `SHA256SUMS` itself (step 4). Under §10, when a tag has no launcher receipt, the main session copies `PFD/s3verify-<TAG>/` into the study directory in place of the receipts. The README and `VALIDATION.json` then state, for that job:
  - that the launcher receipt is absent, and why;
  - that the pod image-digest check of `verify-build` was not made, and that the binary is bound instead by the in-pod check (`run-ecbench` lines 33–36, exit 44 otherwise) and by `S3V_BINARY_SHA`.

  The validator recomputes C2's identity facts with its own code.
- **C7. Authority.** The card adds no authority. A `STOP` in S3VERIFY or in the new step 4 is a `specification_error` returned to the main session, and from there to me.

---

## (d) P32: launch now or fix the launcher budget first

**Ruling: do not launch P32 now.** Launch it only when D1 to D4 hold.

- **D1.** `card-launch-v2.md` (C1 to C4) is frozen, committed and pushed. Until then, LAUNCH 32 cannot pass anyway.
- **D2. Launcher budget.**
  - The new C4 step reads the project's `timeoutInMinutes`, and the value must be ≥ 70.
  - The 20:02Z log entry records that the project already reports 70. If it does at launch, no agent changes anything.
  - If it reads < 70, the change is the human owner's to make. The `isolab-experiment-launcher` project is shared AWS infrastructure for every study ("Changes to the launcher or infrastructure still need an explicit deployment", `HARNESS.md` line 237), and no agent message can authorize it. No agent runs `codebuild update-project`, `deploy-remote` or any CloudFormation change for this study.
- **D3.** P28 counted is resolved: `rho-r2` terminal (by `result.json` or `S3_TERMINAL`), or the case is back with me under B6. The two-slot order of the card stands.
- **D4. The pending hazard**, which a timeout fix does not cure. Under the frozen text, a P32 wall pod that is SIGTERMed after writing `job-exit.json` is undecidable for good, with no rerun, exactly as P28 was. A P32 session runs much longer than P28's 86 s. If the pod again waits about 53 minutes, and the Job deadline (or whatever killed P28) counts that time, P32 is likely to be killed mid-session.
  - P32 therefore waits for the owner's answer to questions 2–4 of the message below.
  - **Deterministic fallback.** If no answer has come 24 h after the message is sent, P32 launches once D1 to D3 hold and a VERIFY shows 0 builds in progress. The risk is then accepted and recorded in the dispatch's session-log entry, citing this clause.

  The goal remains active throughout. This is a hold on one task with a fixed lift condition.

### What to tell the owner

The main session sends the message. The decision whether to change shared infrastructure is the owner's alone.

> Shared isolab infrastructure: XP-20261029-packed-filter's P28 jobs failed for infrastructure reasons, and we need four answers before we launch P32.
>
> 1. **CodeBuild budget.** The `isolab-experiment-launcher` builds of P24 and P28 recorded `timeoutInMinutes` 45. `remote/run-request` waits up to `job.seconds + 180` = 3,780 s, so a 3,600 s job cannot be supervised to completion in 45 minutes. Both P28 launchers TIMED_OUT at 2,704 s, and neither wrote `result.json`. The project now reports 70, which is `bridge.yaml`'s declared value. Please confirm who changed it and that 70 is intended. No agent has authority to change it, and none will.
> 2. **Pending time.** Both P28 pods (`ic-dgbf-p28-20261029`, `ic-dgbf-rho-p28-20261029`, submitted shortly after 19:13Z) waited about 53 minutes before starting, or have not started. Was the benchmark pool (32 vCPU) occupied, or was Spot c8a.2xlarge capacity short? Does the Job's 3,600 s deadline count pending time?
> 3. **SIGTERM.** Pod `cd784143-0134-418f-a84b-061e39ecc0c7` on ip-10-84-191-237 received SIGTERM at about 20:07:36Z, 2 s after its session finished, which was not its deadline. Was this a Spot interruption, a Karpenter disruption (drift or consolidation; benchmark pods carry no `karpenter.sh/do-not-disrupt`), or an infrastructure deployment around 20:00Z?
> 4. **Before the Job TTL (about 24 h) deletes them**, please capture the Job and pod status JSON, and the events, for both P28 jobs into `s3://isolab-results-104186811093-us-east-2/runs/<job>/launcher/manual-20261029/`. If `ic-dgbf-rho-p28-20261029` is still active or pending, please delete it (authorized Job deletion), so that it cannot run alongside its `-r2` rerun.

---

## Process note (future protocols only; changes nothing here)

Every future wall-session protocol should pre-register an **infrastructure-kill clause**, before any data. It allows one rerun of a session whose `job-exit.json` code is 143 or 137, whose `session.json` is complete, and whose kill is not the session's own deadline. The clause carries the same blinding as §4: the killed session's records stay sealed until the rerun has written its `job-exit.json`. Without such a clause, a kill that lands after `job-exit.json` is written spends a size for good, as it did here.

---

## Decision record (to archive)

I had no shell in this engagement, so I could not allocate ids. The session that commits this file must:
- allocate both ids with `python3 tools/allocate_id.py --next … --date 20261029` and confirm each with `--check`;
- archive the two records below as `ledger/decisions/DEC-20261029-<tok>.yaml` and `ledger/handoffs/TASK-20261029-<tok>.yaml`;
- record this file's sha256 on `card-launch-v2.md`.

```yaml
type: coordinator_decision
id: DEC-20261029-<allocate>
date: 2026-10-29
subject: XP-20261029-packed-filter P28 infrastructure failure; launch-card ordering gate; P32 launch
context: >
  P28 wall ic-dgbf-p28-20261029 wrote job-exit 143 (SIGTERM during in-pod audit) after a complete
  1,344-record session; no audit.json, l2-gate.json or control-comparison.json. P28 counted
  ic-dgbf-rho-p28-20261029 wrote nothing by 20:36Z. Both CodeBuild launchers TIMED_OUT at 2,704 s
  (timeoutInMinutes 45) without result.json; card-launch LAUNCH 32 step 4 cannot pass.
decision: revise
rulings:
  a: "P28 wall undecidable under PROTOCOL §6.1 cond. 1 (and 2, 5, 6); never rerun (§4; dg-wall §4 host-loss definition; card l.511). No amendment: §9 bars post-launch amendment, the session's records were opened (pooled wall sum 35.39 s, 86 s span, L2 progress lines), and request-P28-r2 is bound to the host-loss route. Session sealed until the coordinator records 'no successor' or a successor verdict is pushed; successor (fresh curves/seeds, new freeze) ranked at the P32 verdict decision, not approved here."
  b: "Frozen host-loss route RELAUNCH b=28 UNIT rho-r2, dispatchable only after B1 (now >= original counted build endTime + 3600 s + 900 s), B2 (any Kubernetes status honoured; active/pending -> owner deletion first), B3 (VERIFY PAIR immediately before: whole-prefix S3_KEYCOUNT 0, pid exited, no IN_PROGRESS build), B4 (P28-HOSTLOSS.md pushed before launch). B5 fixed now: if the original prefix holds a job-exit.json at counted-analysis dispatch, the original is of record and -r2 is descriptive. B6: a second loss returns to coordinator. P28 analysis deferred while the seal holds."
  c: "Director writes card-launch-v2.md (card revision, not a protocol amendment): MODE S3VERIFY with run-request/verify-build identity facts from S3 and S3_TERMINAL gate (identity + terminal only; outcome facts printed, never gated; image digest unverifiable without pod status, disclosed); LAUNCH step 4 accepts result.json or S3_TERMINAL for the named sessions of record; fail-closed check that the CodeBuild project timeoutInMinutes >= 70; card-analysis unchanged."
  d: "P32 on hold until D1 card-launch-v2 frozen and pushed, D2 project timeoutInMinutes >= 70 (owner-only change if not), D3 P28 counted resolved, D4 owner answers on pending time / SIGTERM / deadline semantics, or 24 h without answer (risk accepted and recorded). Any CodeBuild change is the human owner's."
target_ids: [XP-20261029-packed-filter]
rationale:
  - "PROTOCOL §6.1 cond. 1 admits only exit 0 (42 -> noise rerun); 143 is neither. §4 and dg-wall §4 define host loss by absence of job-exit.json."
  - "§9: no amendment after the first launch; APPROVAL.md left §9 as written. A rerun licence after the records were opened cannot meet §4's blinding standard."
  - "Empty S3 prefix does not prove loss; pending pods are indistinguishable. B1 bounds Job creation by the build's endTime; B5 fixes the resolution before any counted data."
  - "result.json is a card ordering device; §6.1 and the analysis tools read job files only; the frozen gate already accepted failed-job receipts (P24 counted exit 23), so the S3 substitute gates on terminal state and identity, not success."
  - "An infra kill after job-exit.json spends a size permanently under the frozen text; P32 runs longer than P28 and is exposed to the same pending/kill pattern."
evidence_refs:
  - backlog/XP-20261029-packed-filter/PROTOCOL.md §4, §6.1, §6.2, §9 (sha256 703bde26…7e51)
  - backlog/XP-20261029-dg-wall/PROTOCOL.md line 78 (host-loss definition)
  - research/ic_packed_filter_20261029/cards/card-launch.md lines 13-21, 32-37, 195-241, 505-511
  - research/ic_packed_filter_20261029/cards/card-analysis.md steps 2-4
  - taskq/deploy/eks-karpenter/run-ecbench lines 5-29, 49-69; remote/run-request lines 46-80; remote/launch-remote; remote/verify-build; remote/bridge.yaml 127-128; HARNESS.md 195-222
  - docs/session-logs/2026-10-29.md entries 19:12Z-20:45Z (P28 facts as reported by the main session)
limitations:
  - "All P28 facts are the main session's report; the coordinator opened no P28 file."
  - "SIGTERM cause and Job-deadline semantics unverified; B1 rests on HARNESS.md's statement that the deadline includes provisioning time; B5 covers the case where it does not."
  - "Without pod status the image-digest leg of verify-build cannot be checked for launcher-less jobs."
knowledge_promotion: "not_warranted: infrastructure ruling; no evidence record, no claim"
next_actions:
  - "Main session: send the owner message of (d); commit and push this ruling with the DEC/TASK records; append the session log."
  - "Main session: at or after E + 4,500 s, satisfy B2-B4, then dispatch VERIFY b=28 PAIR followed immediately by RELAUNCH b=28 rho-r2 with the frozen card."
  - "Experiment-director: write card-launch-v2.md per C1-C7 (handoff below)."
  - "Coordinator: at the P32 verdict decision, rule on a P28 successor protocol (lifts or keeps the seal)."
```

```yaml
type: handoff
id: TASK-20261029-<allocate>
date: 2026-10-29
from: coordinator
to: experiment-director
objective: "Write the frozen launch card revision research/ic_packed_filter_20261029/cards/card-launch-v2.md per RULING-P28.md (c) C1-C7, so that LAUNCH, RELAUNCH, WAIT and the ordering gate work when a launcher times out but the job's S3 outputs exist."
inputs:
  - "research/ic_packed_filter_20261029/cards/RULING-P28.md, section (c) (worker context: C1-C7 and (b) B5 for the session-of-record values)"
  - "research/ic_packed_filter_20261029/cards/card-launch.md (base text; procedures L, W, V, T copied unchanged except Step 4 and the new budget step)"
  - "taskq/deploy/eks-karpenter/remote/run-request lines 68-79 and remote/verify-build (the checks S3VERIFY mirrors)"
constraints:
  - "No change to PROTOCOL.md, APPROVAL.md, the specs, the requests or card-analysis.md."
  - "S3VERIFY reads no field of records.jsonl (hash only) and no wall field; P28 wall seal of RULING-P28.md (a) applies."
  - "The card never changes AWS infrastructure; a budget below 70 minutes is a STOP."
  - "Outcome facts (exit 0, L2 gate, audit, control comparison) are printed, never gated."
deliverables:
  - research/ic_packed_filter_20261029/cards/card-launch-v2.md
  - "TOOLS/s3verify.py (or equivalent helper), bound by sha256 in the card's Step 2, with a copy in research/ic_packed_filter_20261029/cards/tools/"
  - "a dry-run transcript of S3VERIFY's logic on the committed P24 wall copy (research/ic_packed_filter_20261029/P24/wall/eks-results/), showing S3_TERMINAL=true and the outcome facts, without network access to S3 if the helper supports a local directory argument"
completion_gate: "card-launch-v2.md and its helper committed and pushed with their sha256 recorded; the card cites RULING-P28.md's sha256; the P24 dry run prints S3_TERMINAL=true; no job launched and no AWS resource changed by this task."
archival_task: "one snapshot archive of research/ic_packed_filter_20261029/cards/ only"
budget: {}
```
