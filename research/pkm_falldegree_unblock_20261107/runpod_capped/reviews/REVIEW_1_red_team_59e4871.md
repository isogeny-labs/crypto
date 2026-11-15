# Red-team review: draft Amendment 18 (R2-H), capped runs on reserved RunPod CPU pods

Reviewed: commit `59e487150f` on branch `claude/serene-davinci-ujoi58`. Every reviewed file is committed there, and the
working tree was clean before and after this review. Paths below are relative to
`research/pkm_falldegree_unblock_20261107/` unless they start with `research/`. "Draft" means
`runpod_capped/AMENDMENT_R2_H_runpod_capped.md`. Everything here is `[pcc:unverified]`. I edited no file, made no RunPod or
AWS call, and did not read `rpc_state/`.

## Verdict: do not freeze (at 59e487150f)

**What runs is unchanged, and I checked that myself.** That covers argv, binaries, threads, `--budget`, `--max-dense`
and the entry-script code path, compared with the frozen requests and Amendments 7, 9 and 14.

**What does not hold yet is the accounting around a run:**
- when a run counts as lost, censored, aborted or admitted;
- how many attempts a cell may have;
- what stops a pod if the session dies;
- two admission criteria that cannot be met as written;
- one place where the draft says Amendment 14 is unchanged but changes its OOM branch.

None of the fixes needs new science. Several of them change `run.sh` and `fetch.sh`, which were smoke-tested once, and
one post-smoke change has never run on a pod. So the corrected scripts should be checked again before the freeze.
"Freeze with corrections" would skip that check.

## MUST (blocks the freeze)

### M1. Outcome classes conflict, and the producer can pick among them after seeing the result

The draft defines "censored at the deadline" (draft:64-67), "lost" (draft:117-123) and the admission list (draft:165-176).
`fetch.sh` labels collections `done`, `deadline` or `abort` (fetch.sh:25-31). These overlap or leave gaps:

- **Deadline against lost.** At or after the deadline, any state that is not `done` becomes `reason=deadline`
  (fetch.sh:28). That includes `idle` (pod restarted) and `unreachable`. By draft:66 that run is censored, with no
  relaunch. By draft:118-120 a restart is lost, with a relaunch.
- **One unreachable reading at the deadline.** A single `unreachable` status at the deadline deletes the pod
  (fetch.sh:28, 34, 65). draft:120 needs two unreachable check-ins at least 1 h apart before a run is lost. The run is
  then neither lost nor censored.
- **Completed, then restarted.** Suppose a run completes (`runner_state.json` done, exit 0) and the container then
  restarts before collection. Draft §4.1 calls it lost, yet it passes every admission rule in §6.
- **`--abort` has no outcome.** `--abort` (fetch.sh:27) is never classified. draft:129-130 only mentions it to exclude it
  from the exit-137 rule. Nothing says whether an abort uses the cell's relaunch. Because `timed_out` depends on the CPU
  (see M7), an abort followed by a relaunch lets the producer re-draw the host flavor.
- **Done after the deadline.** `fetch.sh` checks the deadline only when the state is not `done` (fetch.sh:26). A runner
  that ended after start + 90,000 s is therefore collected as `done` and passes admission. EKS would have killed it at
  `seconds` 90000.
- **No restart detector in admission.** Admission does not require:
  - exactly one `start` line in `progress.txt`;
  - `runner_state.started` equal to the `start.json` receipt;
  - an unchanged `lastStartedAt` between `pod-record.json` and `pod-record-final.json` (both receipts already carry that
    field).

  Without these, a restart, or a manual re-`/start` on the same pod, can go undetected.

- **Fix.** Write one decision table, fixed before data, keyed only on receipts:
  - inputs: runner state at collection; `runner_state.json` (state, exit, started, ended); the `start` and `end` lines in
    `progress.txt`; `ended` against the deadline; `lastStartedAt` before and after; the fetch reason;
  - outcomes, each exactly one of: row (complete, or censored by budget or limit); censored by job limit (no row, no
    relaunch); lost (relaunch per §4.2); session abort (uses the attempt, no relaunch without the owner); not an attempt
    (failed before `STARTED`).
- **Cheapest test.** Run `fetch.sh`'s branch logic, with no network, on the smoke receipts plus five synthetic status
  files (idle, unreachable, running, done before the deadline, done after it), and check that each maps to exactly one
  outcome.

### M2. The attempt limit is not enforced, and evidence can be overwritten

- **Any suffix is accepted.** The name check `^pkm-x-t…-c([0-9]+)(-[a-z0-9]+)*$` (run.sh:47) accepts any suffix. In my
  offline run, `pkm-x-t14-k0-c6-r9-anything` passed the name, stage and hash checks.
- **Only a same-name live pod is refused** (run.sh:72-73). Three things are therefore possible:
  1. Concurrent runs of one cell under two suffixes, with the more favourable one admitted.
  2. More than the one relaunch that draft:125-126 allows.
  3. Reuse of a name once its pod is deleted.
- **Serial order is not enforced.** draft:32 says jobs run serially. `list_pkm` exists (common.sh:25), but `run.sh` never
  calls it.
- **Reusing a name destroys the earlier attempt's evidence.** `run.sh` truncates or overwrites `price.jsonl`,
  `pod-request.json`, `pod-create.json`, `pod-record.json`, `upload.jsonl`, `start.json` and `pod-state.json` in the same
  receipts directory (run.sh:41, 76, 95, 102, 109, 132, 137). `fetch.sh` runs `rm -rf "$OUT"` on the earlier results
  tree (fetch.sh:49).
- **Re-running `fetch.sh` after a finished collection corrupts the receipts.** The status comes back unreachable, so
  `--abort` deletes again and overwrites `pod-record-final.json`, `delete.http`, `pods-after-delete.jsonl` and
  `cost.json`, with `reason: abort` and a new `deleted` time (fetch.sh:64-79).
- **Fix:**
  - an exact allow-list of five names: `pkm-x-t14-k0-c6`, `pkm-x-t14-k0-c6-r1`, `pkm-x-t14-k1-c6`, `pkm-x-t14-k1-c6-r1`,
    `pkm-x-t13-k0-c6-r3`;
  - allow `-r1` only when the original's receipts record a loss;
  - refuse if `receipts/NAME/` already exists or any `pkm-x-*` pod is live;
  - `fetch.sh` refuses if `cost.json` exists, and never removes an existing results tree;
  - a rule that a cell's row is its earliest-launched admissible attempt.
- **Cheapest test.** The same offline harness I used (dead proxy, scratch `RPC_RECEIPTS`): a second launch under the same
  name, and under `-r2`, must both exit 3 before any API call.

### M3. Nothing outside the session enforces the 90,000 s limit or the US$40 ceiling (decision 4)

- **The backstop is gone.** On EKS, `seconds` 90000 was enforced by Kubernetes. Here it becomes a deadline that only
  session check-ins enforce (draft:64-70), and the runner has none (`runpod/server.py`). If the session dies, the pod
  idles and keeps billing. Hours to US$40: 38.5 h at US$1.04/h (cpu5m), 62.5 h at US$0.64/h (cpu3g). draft:70 ("the
  owner deletes it") names no trigger that would tell the owner.
- **Other paths that leave a pod running:**
  - `fetch.sh` runs under `set -euo pipefail`. A failed fetch, `gzip -t` or `tar` past the deadline exits before the
    delete (fetch.sh:37-38).
  - Exit 6 keeps the pod on purpose (fetch.sh:45).
  - A curl timeout on create (run.sh:97, `-m 60`) exits through `set -e` before the trap is set (run.sh:106). The pod may
    exist on RunPod with no token file and no log line, and exit 4's "deleted" is not true in that case.
  - A transient API error in the deletion check (fetch.sh:69) exits before `cost.json` is written.
- **Fix (either):**
  - an independent backstop that deletes `pkm-x-*` pods older than 26 h, such as a pod-side self-delete using the pod's
    own RunPod credentials (if RunPod provides them: verify on a dev pod) or an external scheduled reaper; or
  - explicit owner acceptance that the ceiling depends on the session staying alive.
- **Also:**
  - At reason `deadline` or `abort`, `fetch.sh` must delete the pod even when the transfer fails, after recording the
    failure.
  - Any create failure must list pods by name and delete what it finds.
  - Set the trap before the create call.

### M4. The draft changes Amendment 14's OOM branch while saying Amendment 14 is unchanged

- **Amendment 14** (`research/prime_fall_degree_20261027/PREREGISTRATION.md:508-515`):
  - a run killed by the memory limit (`OOMKilled`) is censored by limit;
  - the cell is then re-run at `-m112`, and that run is the cell's row;
  - the `-m112` run does not use the relaunch;
  - exit 137 is classified by the pod and container reason, never by the exit code alone.
- **The draft** treats an unexplained exit 137 as lost, uses the relaunch for it (draft:129-131 with 124-126), and leaves
  `-m112` unreachable (draft:136-138). Yet draft:12-13 and draft:207 say Amendment 14 is unchanged.
- **Unsupported memory claim.** "The memory available to the run is therefore the API record's `memoryInGb`" (draft:75)
  is not measured. `MemTotal` is the host's figure, nothing reads the cgroup limit, and the reused runner can only report
  the first three `/proc/meminfo` lines (server.py:93).
- **Fix (either):**
  - make OOM observable without touching `server.py`: have the boot command copy `/sys/fs/cgroup/memory.max` and
    `memory.events` into `/results` (once at start, and periodically) before it runs the unchanged runner, and read
    `oom_kill > 0` as Amendment 14's `OOMKilled`; or
  - list the deviation under "What changes" with its authority, and say whether a 137 uses the relaunch and whether
    `-m112` stays available.
- **Cheapest test.** One `pkm-x-dev-*` pod that reports `memory.max`.

### M5. The reused client's hash will be wrong when the draft runs

- **PR #4173 edits the client.** At head `b3f7605e` it modifies `runpod/rp.sh`: its `start` command uses `SHA_FILE` when
  that is set.
- **The draft orders itself after #4173** ("appended after 17 once 17 is on main", draft:9) and still pins
  `rp.sh b4b60199…` as "reused unchanged" (draft:103). Once #4173 merges, that pin is false.
- **The hash is logged, not checked.** `run.sh` logs the hashes of `server.py` and `rp.sh` but does not compare them with
  anything (run.sh:96).
- **`SHA_FILE` in the environment.** If it is set, `start` sends the rebuild's hashes. The runner then refuses with 409
  and the pod is deleted. That fails closed, but it costs an attempt whose class is undefined (M1).
- **Fix.** Pin the client: copy the `b4b60199` version into `runpod_capped/`, or re-pin the post-#4173 hash and refuse
  when `SHA_FILE` is set. Make `run.sh` refuse on any hash mismatch for `server.py` or `rp.sh`.

### M6. Admission rule 7 cannot be met from the file it names, and the reserved-pod check is vacuous

- **Wrong file.** `smoke/receipts/pod-record.json` (GET `/pods/{id}`) has `"machine": {}` and no `interruptible` field.
  `secureCloud: true` and `EU-RO-1` appear only in `pod-create.json`. Rule 7 (draft:174) asks `pod-record.json` for "a
  Secure Cloud machine". SMOKE.md:98 cites the same facts to "API pod record".
- **Vacuous check.** `.interruptible != true` (run.sh:111) passes on a missing field, so the refusal of an interruptible
  pod that draft:42-43 describes never fires.
- **Fix:**
  - Name `pod-create.json` for `machine.secureCloud`.
  - Check it in `run.sh` before start.
  - Take `costPerHr` equal to the `securePrice` read for that flavor (the on-demand Secure price) as the API-side
    evidence that the pod is reserved. The smoke pod matches: 1.04 = 1.04. Keep `pod-request.json` as the request record.

### M7. Wrong statement: "The bracket readings do not depend on the host" (draft:180)

- **`timed_out` depends on the host.** The readings use `timed_out`, and `--budget` is wall-clock; the very next bullet,
  draft:181-183, says so. At t = 14 this decides the result, not just the wording:
  - NONSPOT_T14.md:11 already says the run "may reach the budget".
  - Derived: the cap-6 wall time went from 1,762.8 s at t = 12 (X4, Xeon 8488C) to 11,570 s at t = 13 (Y4, Xeon 6975P-C),
    a factor of 6.56 across different hosts. The same factor gives about 75,900 s at t = 14, within 12% of 86,400 s.
- **The flavor can decide censoring.** The fallback list covers two CPU generations (`cpu5*`, `cpu3*`; draft:37-38). So
  the flavor RunPod hands out can decide whether the decisive cell is censored, and the two draws can land on different
  flavors.
- **Fix.** Correct the sentence, and fix the flavor policy for both t = 14 draws before data:
  - either pin one flavor (for example `cpu5m`, and wait for stock rather than fall back);
  - or accept the fallback explicitly, saying what it does to a two-draw median.

## SHOULD

S1. **Protocol state at launch.** The requests' `protocol_sha256` (`69af89b7…`, after Amendment 15) already differs from
the current preregistration (`07bc177e…`). `run.sh` should:
- refuse unless `## Amendment 18` is in `PREREGISTRATION.md`;
- log that file's SHA-256 and the git HEAD.

Then "frozen before the run" can be checked by machine.

S2. **Durability.** `fetch.sh` deletes the pod (fetch.sh:65) before the raw tree reaches S3. No committed rule excludes
`results/rp-*/` from git; the existing exclusions are per directory in `.git/info/exclude`, which is local. Run
`benchrec add-raw`/`upload` before the delete, or keep the pod until the upload is confirmed.

S3. **Disclosure.** Amendments 14-16 each record what their author had seen; this draft has no such paragraph. Add one
covering:
- the t = 12 and t = 13 rows;
- the partial F4 log of the lost third t = 13 draw-0 attempt, which sits in the working tree at
  `results/Y6/0b275a39-…/runs/X-m2-t13-k0-c6.log` (10,564 bytes, git-excluded).

S4. **Pod token in argv.** The "never in argv" claim (draft:110) covers the API key only, and that part is true: it goes
through a 0600 header file. The pod token, however, is in argv at:
- run.sh:91 (`jq --arg t`);
- run.sh:97 (`curl -d "$body"`);
- rp.sh:133, 137, 138, 139 (`-H "X-Token: …"`).

It also sits in the RunPod pod's environment, where anyone holding the shared account key can read it. After start that
gives read-only access only, and interference before start fails closed at `/start`. Say so, or pass the token through
files.

S5. **Where `RPC_STATE` lives is not checked** (common.sh:16-20). If it were inside the worktree, `api.hdr` holding the key
could be committed. Refuse a path inside the worktree, and remove `api.hdr` on exit.

S6. **No deadline recorded.** If `run.sh` dies between `started=1` and writing the deadline (run.sh:134-137), `fetch.sh`
can never reach `reason=deadline`. Write the deadline before start, or derive it from the runner's `started`.

S7. **Wording of admission rules 5 and 8.**
- Rule 5 (draft:172): `image.json` lists `entry.sh`, which is never uploaded, and lacks `build-info.json`, which is. Name
  the checked set: `server.py`'s `BIN` plus `run.sh`.
- Rule 8 (draft:176): `pods-after-delete.jsonl` is written with `|| true` (fetch.sh:72) and is empty when the API call
  fails. Cite the `deleted and no longer listed` line (fetch.sh:83), exit 0 and `delete.http` 204 instead.

S8. **`-m112` coverage.** Derived: `max_dense_entries` grew ×3.47 from t = 12 to t = 13 at cap 6 (1.36e9 to 4.72e9).
Another ×3.47 gives about 1.64e10 at t = 14, above 1.28e10, so `oversize true` is plausible at t = 14.
- Consider covering the `-m112` route now: a generated request, a 121 GB floor, and `cpu5m`/`cpu3m` only.
- State that the next bracket step needs its own request and amendment in both directions: cap 7 if cap 6 does not
  refute, and cap 5 if it refutes (Amendment 7 B). draft:32 names only cap 7.

S9. **Relaunch authority.** Decision 3 is about Spot reclaims (DECISIONS.md:14). The R3 draft said it does not apply to a
reserved pod (AMENDMENT_R3_RUNPOD.md:61). Label the extension in draft:125 as the main session's reading, for the owner to
confirm.

S10. **The US$400 check covers only this route.** The study-wide check (draft:159-160) sums only this route's `cost.json`,
and `run.sh` does not implement it. Name the study ledger: the EKS jobs, `pfd-b-rp` under Amendment 16 (live during the
smoke, SMOKE.md:160-161), any Amendment 17 work, and this route.

S11. **Unreachable rule.** Before declaring a run lost for an unreachable proxy, check the REST record (`desiredStatus`,
`lastStartedAt`). The REST API does not go through the pod proxy, so an outage of the proxy alone would not kill a healthy
run.

S12. **Binary provenance.** Log in `run.log` that `RPC_BIN` came from layer `a0f7e28e…`. I verified that link, but only in
the session scratchpad, which is ephemeral.

S13. **Wording of draft:41.** "Matches an EKS 16-vCPU node running a 12-CPU pod" is loose. On EKS the container saw
`nproc 12` (Y4's `host.txt`); here 12 threads run over 16 vCPUs. `muladds` is unaffected, because the F4 chunk size uses
`rayon::current_num_threads()` (`src/cryptanalysis/f4_fp_tower.rs:1694` at f13a690df0). Wall time can differ.

S14. **Smaller points:**
- Numbering: say what happens if #4173 closes unmerged.
- The committed receipts carry `consumerUserId`, an account identifier rather than a credential; consider redacting it.
- `MIN_VCPU` 12 allows a 12-vCPU pod although 16 is requested. That is harmless but undocumented.

## What I verified myself

- **The entry-script claim reproduces (draft:50-51), and more strictly than claimed.**
  - Whole file: `entry.sh` without line 2 is byte-identical to `entry_r2.sh` without lines 2, 11 and 63-68 (the `m4n:`
    branch).
  - Each of the three regions also compares equal with `cmp`: preamble (25 lines), `cap:*)` branch (8 lines), tail
    (3 lines).
- **Hashes match the stated values:** `entry.sh 4866aa7e…`, `entry_r2.sh 25b01841…`, `run.sh c35ea88e…`,
  `server.py f56fd08e…`, `rp.sh b4b60199…`. `image.json`, `binaries.sha256` and `build-info.json` agree with each other.
- **Stages are exactly `command[1]`.** The three draft stages equal `command[1]` of `requests_r2i/pfd-x-t14-k{0,1}-c6.json`
  and `pfd-x-t13-k0-c6.json`; `run.sh:50` enforces this by exact JSON equality.
  - The r2i `.job` blocks equal those of `requests_r2h/` (Y4's request, and Y6's apart from the job name).
  - Every request has `cpu 12`, `56Gi`, `seconds 90000` and `retries 0`.
- **Refusal paths reproduce offline.** I ran with the network blocked through a dead proxy, `RUNPOD_API_KEY` unset, and
  scratch receipts. Seven refusal cases all exit 3 before any network call: wrong `--max-dense`, dev stage at t = 12, a
  foreign name, `g1`, draw mismatch, cap-7 name, and a one-byte-modified `pfd_grid_zeros`.
- **The binaries come from the frozen image.** In the scratchpad copy the smoke used:
  - `r2g_dl/layer.tgz` has SHA-256 `a0f7e28e…`, equal to `image.json`'s `layer.digest`;
  - its gunzipped form hashes to `83493138…`, equal to `diff_id`;
  - all six files hash to `binaries.sha256`;
  - the layer's `/opt/study/build-info.json` and `run.sh` equal the uploaded ones.
- **Threads, budget and limits are unchanged.**
  - `rp.sh` start sends `rayon 12`; `server.py` sets `RAYON_NUM_THREADS`; `entry.sh` keeps it.
  - The F4 chunk size depends on the rayon thread count, not on `nproc`.
  - The budget check fires every 256 rows (`f4_fp_tower.rs:1321`).
- **The create body** differs from Amendment 16's `rp.sh create` body only by `cloudType: "SECURE"` (plus the name and the
  vCPU argument).
- **The post-smoke `pod-request.json` line works offline.** It withholds the token and the runner text (tested with a fake
  token). `bash -n` passes on all three scripts.
- **The smoke receipts are consistent:** prices, costs (47 s × US$1.04/h = US$0.0136), the deadline (start + 90,000 s) and
  the timeline. No 48-hex token-shaped string and no auth header appears in `runpod_capped/` or `runpod/`.
- **The cost arithmetic is right:** 1.04, 0.88, 0.736 and 0.64 × 26 h give 27.04, 22.88, 19.14 and 16.64. Three runs cost
  at most 81.12; five runs at most 135.20.
- **Bash cleanup:** the `trap … EXIT` runs on TERM, HUP and INT (local test).
- **PR #4173** is open and not merged, and it modifies `runpod/rp.sh` (REST file list and patch).
- **No earlier t = 14 attempt exists** (cards, receipts, SPOT_LOSSES.md), so t = 14 relaunch accounting starts clean.
  t = 13 draw 0 has used three attempts (SPOT_LOSSES.md:10-12).

## Taken from the text, not verified

- RunPod behaviour:
  - that `interruptible:false` and `cloudType:SECURE` are honoured;
  - that billing equals `costPerHr` and the on-demand price is fixed for the pod's life;
  - how memory is enforced (see M4);
  - whether `lastStartedAt` changes on a container restart;
  - whether a pod has credentials to delete itself.
- That the placeholder key got HTTP 401 (draft:107-108); no receipt exists.
- That only one smoke pod was created; I saw only its receipts.
- That a budget-stopped pilot exits 0. I inferred it from the absence of `process::exit` in
  `examples/pkm_tower_pilot.rs` at f13a690df0; I did not observe it.
