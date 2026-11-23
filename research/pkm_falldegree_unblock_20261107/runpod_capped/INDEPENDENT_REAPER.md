# The independent hourly reaper (owner decision 25)

While the study pod of Amendment 33 exists, a scheduled task starts a **fresh** cloud session every hour that runs this
route's `reap.sh`, so that the pod is deleted past its hard deadline even when the session that launched it is down
(the failure that cost US$370.84 in Amendment 32). The main session creates the task right after `run.sh` starts the
pod, stores the prompt below in it unchanged (with `BRANCH` replaced by the branch that holds the launch receipts), and
removes the task once `fetch.sh` or the reaper has confirmed the pod gone (`cost.json` with `confirmed_gone: true`, or
the pod absent from the list). It supersedes owner decision 14 for this job. Every statement here is
`[pcc:unverified]`.

What a pass can do is fixed by `reap.sh` (section 2.6 of the amendment): it lists the account's pods; it acts only on
pods whose name starts with `pkm-x-`; it deletes one only past its hard deadline (the receipts' `pod-state.json`, or
creation + 26 h), first through `fetch.sh` (which collects what it can, classifies, and deletes), then directly if the
pod is still listed; it logs every delete before the API call; it exits 1 with `REAPER BLIND` when the list cannot be
read. It needs no earlier state: `RPC_STATE` is created on first use, the locks are created on first use, and
`fetch.sh` recovers a missing pod token from the REST v2 record's `env` (offline tests F10 and F11).

**Why the pass commits more than `reaper/`.** When a pass deletes the study pod through `fetch.sh`, `fetch.sh` writes
the run's outcome, collection, cost and delete receipts under `receipts/<name>/` and `results/<name>/`, and closes the
ledger row. Those files must reach the branch: if only `reaper/` were committed, the main session's next `fetch.sh`
would find the pod gone with a GET 404 and no logged session action, which the classifier reads as host-side evidence
(relaunch `auto`). So the prompt commits everything the pass wrote under `runpod_capped/`, and nothing else.

## The prompt (stored verbatim in the hourly task; replace BRANCH)

```text
You are the independent reaper of the PKM fall-degree study's capped RunPod route (owner decision 25). You run once,
in this fresh session, and then stop. Do exactly the steps below and nothing else.

Hard rules: never run run.sh, watch.sh or any other launcher; never create, start, stop or restart a pod; never delete
a pod yourself and never call the RunPod API directly: only reap.sh may delete, and it deletes only pkm-x- pods past
their hard deadline. Never touch a pod whose name does not start with pkm-x-. Never edit route scripts, the amendment
or any file outside research/pkm_falldegree_unblock_20261107/runpod_capped/. Never force-push and never rewrite history.
Never print or commit a secret: the API key comes from the environment (RUNPOD_API_KEY) and pod tokens stay in
RPC_STATE, which is outside the repository.

1. Get the repository isogeny-labs/crypto at branch BRANCH: if it is already checked out, run `git fetch origin BRANCH`
   and `git checkout BRANCH` and `git merge --ff-only origin/BRANCH`; otherwise clone it and check out BRANCH.
2. Create a fresh private state directory outside the repository:
   `export RPC_STATE=$(mktemp -d /tmp/rpc-state.XXXXXX)` (mode 0700; it holds the API header and pod tokens).
3. Run the reaper from the route directory and keep its exit code:
   `cd research/pkm_falldegree_unblock_20261107/runpod_capped && ./reap.sh; rc=$?`
4. If `git status --porcelain -- .` shows changes, commit exactly those files under runpod_capped/ (reaper/ receipts,
   and any receipts/<name>/, results/<name>/ and ledger.tsv files that reap.sh or the fetch.sh it ran wrote), with a
   plain commit: `git add -- . && git commit -m "runpod_capped: independent reaper pass (owner decision 25), reap.sh exit <rc>"`.
   Push with `git push origin BRANCH`. If the push is rejected, run `git fetch origin BRANCH` and
   `git merge --no-edit origin/BRANCH`, then push again. If the merge has a conflict, do not resolve it by discarding
   either side: run `git merge --abort`, push your commit to a new branch `reaper/<UTC date and time>` instead, and say
   so in the report.
5. Remove the state directory: `rm -rf "$RPC_STATE"`.
6. Report in a few lines: the reap.sh exit code; for every pkm-x- pod in today's reaper/reap-YYYYMMDD.jsonl lines of this
   pass, its name, id, hard deadline, whether it was due, the action and whether it is gone; the commit and branch you
   pushed (or that nothing changed). Exit code meanings: 0 no due pod remains; 1 the pod list or a deadline could not
   be read (REAPER BLIND: nothing was checked or deleted; tell the owner); 7 a due pod is still listed after the
   delete (tell the owner). If reap.sh printed "lock held" for a pod, say that it was skipped and the next pass retries.
```

## Main-session duties around the task

- Create it right after `run.sh` exits 0 (one fresh session per hour, `create_new_session_on_fire`), and confirm the
  first pass's report before relying on it.
- Keep running `watch.sh` (15-minute fetches, owner decision 26) and the 4-hourly session reaper pass; the hourly task
  is the backstop, not the only path.
- Remove the task once the pod is confirmed gone; record the removal in the session log.
