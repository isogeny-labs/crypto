# Coordinator ruling, addendum 1: D2 and D3 after the P28-rho-r2 launch (XP-20261029-packed-filter)

**Date:** 2026-10-29, after 21:25Z. **Role:** coordinator. **Scope:** conditions D2 and D3 of `cards/RULING-P28.md` (d), what P32 still needs, and the P28 wall seal. This addendum adds to that ruling and does not edit it. It changes no research status, launches nothing, amends neither `PROTOCOL.md` nor any card, and authorizes no change to AWS infrastructure.

**Binding texts:**
- `cards/RULING-P28.md`, sha256 `bff4e46df7b522f64c86a76631ae3088af7aa6f41a65e80cd1f9fadae8954275`: (a) Seal, (b) B1 to B6, (c) C2 to C6, (d) D1 to D4;
- `cards/card-launch-v2.md`, sha256 `8fa7610fa63ae4043a55c17459167ee874ebcfde7616e332c42ebe6a048cd526`: Step 4, Step 6A, Step 9, Procedure W step W3, Procedure S, and "Never retry";
- `P28/P28-HOSTLOSS.md` (the B4 record);
- `taskq/deploy/eks-karpenter/remote/run-request` (sha256 `e9706cc6…2192`): lines 18 to 35 and 54 to 65;
- `backlog/XP-20261029-packed-filter/PROTOCOL.md` §6.1 "Counted session".

**What I read for this addendum:** the texts above, and nothing else. I opened no P28 wall file and no counted record. Facts 1 to 5 of the dispatch come from the main session's receipts and AWS CLI reads, and I use them as reported.

**One fact from my own reading, which bears on (B):**
- `run-request` has no step that deletes a Job or stops a pod. Its `finish` trap writes `result.json` and uploads the receipts. Its only reaction to a timeout is `launcher_timed_out` (line 64), and line 46 says that "a timeout can leave a live Job".
- So when a CodeBuild build is stopped at its timeout, the build loses supervision: the Kubernetes state, the pod uid, `verify-build` and `result.json`. As far as this script goes, the job keeps running.
- It also means that a result.json with exit 23 (line 65, Kubernetes state `failed`) is written *before* `verify-build` runs. P24-rho and P28-rho-r2 therefore never went through `verify-build`, even though they have a `result.json`.

---

## (A) D3: is P28 counted resolved?

**Ruling: D3 is met.**

- **D3 as written.** It asks for "`rho-r2` terminal (by `result.json` or `S3_TERMINAL`), or the case back with me under B6".
  - `receipts-P28-rho-r2/result.json` exists (W3 printed `WAIT_ROUTE P28-rho-r2 RESULT_JSON_PRESENT`).
  - Card v2 Step 4 accepts exactly that file for `PREV_COUNTED_TAG=P28-rho-r2`.
- **B6 does not arise.** The `-r2` pod wrote `job-exit.json` (exit 42, pod `9f8f7676-…`), so it is not a host loss.
  - It is never rerun: card v2 "Never retry" says "A job that wrote a job-exit.json with a non-zero code is never rerun by any other route."
  - The 23/42 pairing is the same one P24-rho showed. Its reading belongs to the counted-session exit-42 clause of PROTOCOL §6.1 (card-analysis `EXIT42_CLAUSE`), at analysis time. I read nothing into it here, in either direction.
- **B5 does not keep D3 open.** The two govern different things:
  - D3 is a *launch-ordering* condition. It asks whether the P28 counted slot has reached a terminal state, so that the two-slot order can move on to 32.
  - B5 is a *classification* rule. It says which counted session is the session of record, and it is applied from file existence when card-analysis is dispatched. The ruling fixed it before any counted data, so it cannot be influenced by what happens to P32.
  - If the original `ic-dgbf-rho-p28-20261029` writes a `job-exit.json` later, B5 makes it the session of record, and `-r2` becomes descriptive. Nothing about P32's launch or reading depends on that choice: §6.1 says a counted-session failure "does not change the wall decision".
- **Dispatch values.** LAUNCH 32 names `PREV_COUNTED_TAG=P28-rho-r2`. That is the terminal counted session on hand, and naming it pre-judges nothing under B5.

---

## (B) D2: the launcher budget

### How D2 is judged

**Ruling: D2 is judged on the effective per-build timeout, and on that reading D2 is not met.**

- **Why the effective value governs.**
  - The 70 in C4 and D2 was chosen to cover `run-request`'s wait (`job.seconds + 180` = 3,780 s) plus setup. Only the timeout a build actually runs under can do that.
  - I chose the project field as the check because I assumed a build inherits it. Facts 1 to 3 refute that assumption: the project reports 70, last modified before every build, and all five builds of the study ran under 45. Every start request holds only `projectName` and `idempotencyToken`.
  - So card v2 Step 6A checks a necessary condition, not a sufficient one. That Step 6A printed 70 and passed is not evidence that any build gets 70 minutes.
- **What this is, and is not.** It is a reading of my own dispatch condition, made from infrastructure facts alone. It is not a protocol amendment, and it reads no outcome.
- **Why D2 cannot be met by any agent.**
  - The cause is unknown, and it lies in owner infrastructure.
  - The launcher is pinned (`launch-remote` sha256 `076ee44f…6f45`). Adding a `timeoutInMinutesOverride` to the start request would be a launcher change, and that needs an explicit deployment by the owner (`HARNESS.md` line 237).
  - No agent changes the project, the launcher or the bridge stack.

### What replaces D2 (D2′; fixed now, before any P32 data)

A 45-minute build does not kill the job (see the `run-request` fact above). It costs supervision only. Card v2 already gives a route that does not need the launcher: W3 routes to `S3VERIFY_NEXT`, the job is then shown `S3_TERMINAL`, and C6 sets out what must be disclosed.

Holding P32 for a fix that no agent may make, and whose cause nobody knows, would turn a hold with a fixed lift condition into an open-ended pause. **D2 is therefore replaced by D2′**, which holds when all of the following are done:

1. **Pre-registered consequence.** If a P32 launcher is TIMED_OUT, or exits with no `result.json`, that is infrastructure signal. It is never a reading of the filter.
   - The job is classified by its own files: W3, then S3VERIFY for the tag.
   - `S3V_IMAGE_DIGEST` is disclosed as `unverifiable_without_pod_status`, unless the owner supplies the pod status JSON and the dispatch names `POD_STATUS_SHA256`.
   - The binary stays bound by the in-pod check (`run-ecbench` lines 33 to 36, exit 44 otherwise) and by `S3V_BINARY_SHA`, as C6 states.
2. **No early host loss (B1 carried over to 32).**
   - At P32, a `WAIT_ROUTE … MAIN_SESSION` before E + 3,600 s + 900 s is not a host loss. Here E is the tag's build `endTime`: a pod may still be running after its build has timed out.
   - The main session re-dispatches WAIT for the unit (read-only) after that time.
   - Only if that WAIT still shows `MAIN_SESSION` do B2 to B6 of RULING-P28 (b) apply, with b = 32. That includes a `P32/P32-HOSTLOSS.md` record pushed before step 8 of the relaunch, and a return to me on a second loss.
3. **Owner follow-up sent.** The main session sends the owner the follow-up below, before or together with the P32 launch dispatch.
4. **Recorded at launch.** The LAUNCH 32 session-log entry quotes Step 6A and the Step 9 `BUILD_TIMEOUT_MINUTES` lines verbatim, and cites D2′.

### Card v3: not ordered

I order no card v3 for D2. I considered the two variants the dispatch raised, and neither is a sound gate:

- **"Fail closed after launch when `BUILD_TIMEOUT_MINUTES` < 70".**
  - This cannot fail closed. Step 9 runs after the launch, so the job already exists, and nothing comes after Step 9 that a refusal could stop.
  - Its only possible action would be to stop the build. That ends supervision and leaves the job running, which is strictly worse.
  - Step 9 already reports the value. That is the right form for this fact.
- **"Refuse LAUNCH 32 because a pre-launch probe cannot observe the per-build value".**
  - All five builds got 45, so this would be a permanent refusal that no agent can lift. That is D2 under another name, and D2′ replaces it for the reasons above.

Step 6A stays as frozen, as a necessary-condition check: it still refuses if the project field ever drops below 70. Step 6A passing is never cited as evidence that the budget is 70 or more.

If the owner reports a fix, it is verified by the next Step 9 line printing 70 or more. A card change is not needed for that.

### Owner follow-up (the main session sends it; any decision is the owner's)

> Follow-up on XP-20261029-packed-filter, question 1 of the earlier message, with a new fact. The CodeBuild project `isolab-experiment-launcher` reports `timeoutInMinutes` 70, and its `lastModified` (1792725034) is earlier than every build of this study. Yet all five builds of the study (P24, P24-rho, P28, P28-rho, P28-rho-r2) recorded `timeoutInMinutes` 45. Each start request carried only `projectName` and `idempotencyToken`, with no override. Can you say where the 45 comes from (for example an account or service limit, a different project or alias, or a wrapper), and whether builds are meant to get 70? Until we hear, P32 will run with launcher-less S3 verification as a fallback. For each P32 job, please capture the pod status JSON before the Job TTL, so that the image-digest check can be made. No agent will change the project, the launcher or the stack.

---

## (C) What else is needed before P32, and the seal

### Required before LAUNCH 32, beyond D1, D2′, D3 and D4

- **E1. S3VERIFY of the P28 wall session.**
  - P28 has no `result.json`, so card v2 Step 4 accepts `PREV_WALL_TAG=P28` only with `PFD/s3verify-P28/s3receipt.json` and `S3_TERMINAL` true.
  - Dispatch: MODE S3VERIFY, b = 28, UNIT PAIR, TAG P28, POD_STATUS_SHA256 NONE, before LAUNCH 32.
  - **This is within the seal.** Procedure S hashes every file and opens only `job-exit.json`, `SHA256SUMS`, `build-info.json`, `binary.sha256`, `input-spec.json`, and the outcome files `audit.json`, `l2-gate.json` and `control-comparison.json` if they exist. The three outcome files are absent for this session. These are identity files; none of them holds a wall field. I extend the seal's permitted list to exactly these files and this procedure. No other reading is permitted.
  - If S3VERIFY prints `S3_TERMINAL=false`, or a `STOP`, the case returns to me, and P32 does not launch.
- **E2. Archive first.**
  - Before the LAUNCH 32 dispatch, the main session commits and pushes, as ledger archives of their own:
    - this addendum and its decision record;
    - the `P28-rho-r2` receipts and the `s3verify-P28/` output, copied into the study directory under §10;
    - the session-log entry.
  - It confirms that `P28/P28-HOSTLOSS.md` reached `origin` before `LAUNCH_UTC_P28-rho-r2` (21:17:22Z), as B4 requires.
    - If it did not, that is recorded as a disclosed procedural deviation of B4.
    - It does not invalidate the `-r2` session: the record is the loss's audit trail, not a condition of the protocol.
- **D1** is met as far as the main session's reports show: Step 0 of the VERIFY and the RELAUNCH proved that card v2 (`8fa7610f…d526`) is committed and pushed.
- **D4 stands unchanged.**
  - P32 waits for the owner's answers to questions 2 to 4. If no answer has come 24 h after the message was sent (about 20:45Z on 2026-10-29, so about 20:45Z on 2026-10-30; use the recorded send time), P32 launches once D1, D2′, D3, E1 and E2 hold and the Step 7 slot check shows 0 builds in progress. The risk is then recorded in the session-log entry, citing D4.
  - The `-r2` pod started within minutes, which shows that capacity was free at 21:17Z. It says nothing about what sent the SIGTERM, and that is the hazard D4 exists for.

Nothing else is required. In particular, P32 does not wait for the P28 counted analysis.

### The wall seal: still deferred

**Ruling: lifting the seal is not in scope now.** It lifts only as RULING-P28 (a) says: when I record "no successor", or when a successor's verdict commit is pushed. I make that decision at the P32 verdict.

- Two reasons to keep it:
  - it keeps every remaining P32 decision (launch timing, any P32 rerun routing) blind to the 28-bit wall data;
  - none of D1 to D4 needs the seal lifted.
- **Consequence.** The P28 counted analysis stays deferred. card-analysis compares the counted records with the wall records, so it cannot run while the seal holds. When the seal lifts, card-analysis is dispatched with b = 28, WALL `first`, and COUNTED as B5 decides at that moment.

---

## Decision record (to archive)

I had no shell in this engagement, so I could neither allocate an id nor compute this file's sha256. The session that commits this file must:
- allocate the id with `python3 tools/allocate_id.py --next coordinator_decision --date 20261029` and confirm it with `--check`;
- archive the record below as `ledger/decisions/DEC-20261029-<tok>.yaml`;
- record this file's sha256 in the decision record and in the session log.

This addendum creates no new handoff. The dispatches it requires (S3VERIFY P28, the WAIT re-checks, LAUNCH 32) are dispatches of the frozen `card-launch-v2.md` under the existing launch task. Nothing in it is a task for the director.

```yaml
type: coordinator_decision
id: DEC-20261029-<allocate>
date: 2026-10-29
subject: XP-20261029-packed-filter RULING-P28 addendum 1 - D3 resolved, D2 judged on effective build timeout and replaced by D2', P32 prerequisites, seal kept
context: >
  P28-rho-r2 (launched 21:17:22Z) ended with result.json state failed exit 23 and job-exit.json exit 42
  (pod 9f8f7676-...), WAIT_ROUTE RESULT_JSON_PRESENT; the original counted prefix still holds 0 keys.
  The CodeBuild project reports timeoutInMinutes 70 (lastModified before every build), but all five builds
  of the study recorded 45, with no override in any start request; cause unknown, owner infrastructure.
  Owner questions of RULING-P28 (d) were relayed at about 20:45Z and are unanswered.
decision: revise
rulings:
  A: "D3 met: P28-rho-r2 is terminal by result.json, which card-launch-v2 Step 4 accepts for PREV_COUNTED_TAG=P28-rho-r2. It wrote job-exit.json, so it is not a host loss: B6 does not arise and it is never rerun. B5 is a classification rule applied at counted-analysis dispatch, not a launch-ordering condition, so it does not hold D3 open. Exit 23/42 is read only by the counted exit-42 clause at analysis."
  B: "D2 is judged on the effective per-build timeout; on that reading it is unmet. The project field is a necessary, not a sufficient, check. Because run-request deletes no Job, a 45-min build costs supervision only, and card v2's S3VERIFY route covers that case. D2 is replaced by D2': (1) a P32 launcher timeout is infrastructure signal, classified by the job's own files via W3/S3VERIFY, with the image digest disclosed as unverifiable unless the owner supplies pod status; (2) at P32, MAIN_SESSION before build endTime + 4,500 s is not a host loss: WAIT is re-dispatched after that time, then RULING-P28 B2-B6 apply with b=32; (3) the owner follow-up on the 70/45 discrepancy is sent; (4) the Step 6A and Step 9 lines are quoted in the launch log entry. No card v3: a post-launch fail-closed check cannot un-launch, and a pre-launch refusal would be a permanent hold. Step 6A stays as frozen. No AWS, launcher or stack change by any agent."
  C: "Also required before LAUNCH 32: E1 S3VERIFY b=28 PAIR TAG P28 POD_STATUS_SHA256 NONE with S3_TERMINAL=true (within the seal: identity files only, outcome files absent; anything else returns to the coordinator); E2 this addendum, the P28-rho-r2 receipts, s3verify-P28 and the session log committed and pushed, and the B4 timing confirmed (a late push is a disclosed deviation). D1 met; D4 unchanged (owner answers or the 24 h fallback from the recorded send time). Seal not lifted: it lifts only under RULING-P28 (a), decided at the P32 verdict. P28 counted analysis stays deferred; P32 does not wait for it."
target_ids: [XP-20261029-packed-filter]
rationale:
  - "D3's text is satisfied by result.json; B5 concerns which counted session counts, and PROTOCOL §6.1 says a counted failure does not change the wall decision."
  - "C4 chose 70 to cover run-request's 3,780 s wait; only the effective build timeout serves that purpose, and facts 1-3 show the project field does not set it."
  - "run-request lines 18-35, 46 and 54-65: a build timeout ends supervision but deletes no Job, so launcher-less S3 verification (RULING-P28 C2/C5/C6) is a complete route with one disclosed leg."
  - "A gate no agent can satisfy, on a cause nobody knows, would pause the goal indefinitely; D2' pre-registers the consequence before any P32 data instead."
  - "Card v2 Step 4 cannot accept PREV_WALL_TAG=P28 without an S3VERIFY receipt, since P28 has no result.json."
  - "Keeping the seal keeps every remaining P32 decision blind to the 28-bit wall data."
evidence_refs:
  - research/ic_packed_filter_20261029/cards/RULING-P28.md (sha256 bff4e46df7b522f64c86a76631ae3088af7aa6f41a65e80cd1f9fadae8954275), (a), (b), (c), (d)
  - research/ic_packed_filter_20261029/cards/card-launch-v2.md (sha256 8fa7610fa63ae4043a55c17459167ee874ebcfde7616e332c42ebe6a048cd526), Step 4, 6A, 9, W3, Procedure S, Never retry
  - research/ic_packed_filter_20261029/P28/P28-HOSTLOSS.md
  - taskq/deploy/eks-karpenter/remote/run-request lines 18-35, 46, 54-65 (sha256 e9706cc6...2192)
  - backlog/XP-20261029-packed-filter/PROTOCOL.md section 6.1 "Counted session" (sha256 703bde26...7e51)
  - "main-session receipts: CodeBuild project read (timeoutInMinutes 70, lastModified 1792725034); build.json/build-start.json of the five builds (45); start-request.json files; P28-rho-r2 result.json, job-exit.json and the W3 output"
limitations:
  - "All AWS and receipt facts are the main session's report; the coordinator opened no receipt, no P28 wall file and no counted record."
  - "The source of the 45-minute build timeout is unknown; D2' rests on run-request deleting no Job on a timeout, and does not cover any owner-side mechanism that might do so."
  - "Without pod status, the image-digest leg of verify-build stays unverified for any launcher-less P32 job, and for exit-23 sessions it never ran."
  - "The SIGTERM cause and the Job-deadline semantics remain unverified (D4 open)."
knowledge_promotion: "not_warranted: infrastructure and dispatch-condition ruling; no evidence record, no claim"
next_actions:
  - "Main session: commit and push this addendum and its DEC record (E2), with the P28-rho-r2 receipts and the session-log entry; record this file's sha256."
  - "Main session: send the owner follow-up of (B)."
  - "Main session: dispatch S3VERIFY b=28 UNIT PAIR TAG P28 POD_STATUS_SHA256 NONE with card-launch-v2; archive s3verify-P28/; on S3_TERMINAL=false or a STOP, return the case to the coordinator."
  - "Main session: when D1, D2', D3, E1, E2 and D4 hold (owner answers, or 24 h after the recorded send time), dispatch LAUNCH b=32 with PREV_WALL_TAG=P28 and PREV_COUNTED_TAG=P28-rho-r2; quote Step 6A and Step 9 in the log entry."
  - "Coordinator: at the P32 verdict decision, rule on a P28 successor protocol (lifts or keeps the seal); after that, P28 counted analysis is dispatched with COUNTED as B5 decides."
```
