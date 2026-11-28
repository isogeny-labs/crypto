# Run51 outcome recovery (lane `run51-outcome`, 2026-11-18)

## Question

Run51 (`RESUME_UNIT51.md`, `unit51-protocol/`, plan `unit-deferred-plan.json`,
runner `run-unit-deferred-task.sh`) was dispatched once on Runpod pod
`7zki4wnz56fzad` from source `f5fdb2d04c2efdc89381d74ecae9da9bcea2d511`
(2026-11-17, sole POST accepted 202 at 12:30:41Z, guardian deadline 14:29:44Z).
Was its result ever collected? Does the pod still exist or bill?

## Answer (short)

- **No Run51 result exists anywhere this lane could read.** Nothing in Git
  (local, `origin` heads, PR #4537 commits/body/comments, GitHub commit and
  issue/PR search) or in the session logs records a collected archive, a
  qualifier exit code, a deletion receipt or an outcome. [pcc:unverified]
- **The original archive is not recoverable from this machine.** The private
  metadata directory `/tmp/ic-descent51-native` and the stage directory
  `/workspace/ic-runpod-runs/runpod-unit-deferred-20261117-51` named in the
  session log do not exist here (they lived on the dispatching Codex host).
  [pcc:unverified]
- **Pod status is UNKNOWN.** Read-only Runpod queries failed:
  `GET https://rest.runpod.io/v1/pods/7zki4wnz56fzad` and `GET /v1/pods`
  returned proxy `502 upstream request failed` (5 attempts, 03:4x-03:51Z);
  GraphQL `api.runpod.io/graphql` `{ myself { id } }` returned `401`
  (`{"error":{}}`) although the environment lists network secrets for both
  hosts. So whether `7zki4wnz56fzad` was deleted by its guardian, or is an
  orphaned billing pod, could not be determined. [pcc:unverified]

Full evidence and the cost bound are in [`STATUS.md`](STATUS.md). A draft
read-only check for whoever holds a working Runpod credential is in
[`DRAFT_READONLY_POD_CHECK.md`](DRAFT_READONLY_POD_CHECK.md).

## What was done (commands)

```
grep -rIl "7zki4wnz56fzad\|f5fdb2d04c" .            # only docs/session-logs/2026-11-17.md
git log --all -i --grep="unit51\|run51\|unit-deferred"   # no hits in local refs
git ls-remote origin | grep -i "unit51\|unit-deferred\|run51\|measure/unit"
    # only refs/heads/measure/unit51-source = f5fdb2d04c (the frozen source)
git ls-remote --heads origin   # 2452 heads; no runpod-unit-deferred-20261117-51,
                               # no measure/unit51-run*, no result branch
gh api repos/isogeny-labs/crypto/pulls/4537           # merged 2026-11-17T15:14:40Z
gh api repos/isogeny-labs/crypto/pulls/4537/commits   # last science commit e394138bb7
                               # 12:34:27Z "Checkpoint frozen Runpod51 invocation ..."
gh api search/issues?q=repo:...+unit51+is:pull-request   # only #4537
gh api search/commits?q=repo:...+{unit51,7zki4wnz56fzad,Runpod51,descent51}
                               # only e394138bb7
gh api repos/.../contents/docs/session-logs?ref=main  # newest dated log 2026-11-17
ls /tmp/ic-descent51-native /workspace/ic-runpod-runs  # both absent
curl https://rest.runpod.io/v1/pods/7zki4wnz56fzad    # 502 upstream request failed
curl https://rest.runpod.io/v1/pods                   # 502 upstream request failed
curl -d '{"query":"{ myself { id } }"}' https://api.runpod.io/graphql   # 401
aws sts get-caller-identity    # aws CLI not installed; no S3 check possible
scripts/research-kb search ... # "no executable found" (KB unavailable)
```

`git fetch origin` failed (`fatal: write error: No space left on device`;
the shared disk had 548 MB free); `git ls-remote` and the GitHub REST API were
used instead for remote refs. No pod, job or run was created, stopped,
deleted or dispatched. Nothing was pushed or commented.

## Knowledge base receipt

`scripts/research-kb`: `research-kb: no executable found; set RESEARCH_KB_BIN
or put research-kb on PATH`. No `research-kb` MCP tool was available in this
session. No KB sources checked.

## Results

No scientific result. No analysis was run: the protocol's native analyzer
(`examples/jv_sieve_unit_deferred_check.rs`, built as
`/tmp/ic-descent51-native/checker`, sha256 `14a11428...7c41` in
`validation/unit51-build.sha256`) needs the original run directory, which was
not found. Prediction of `RESUME_UNIT51.md` (five-variable stream cheaper than
the adapted six-variable control) remains **untested**. [pcc:unverified]

## Open next steps

1. Someone with a working Runpod credential runs the read-only check in
   `DRAFT_READONLY_POD_CHECK.md` and, if `7zki4wnz56fzad` (or any `ic`-owned
   pod) still exists, the owner decides on deletion (this lane must not).
2. Ask the operator of the Codex host that dispatched Run51 whether
   `/workspace/ic-runpod-runs/runpod-unit-deferred-20261117-51` or
   `/tmp/ic-descent51-native` still exist there; if so, collect, hash and
   privacy-scan the original archive and run the frozen qualifier/analyzer
   unchanged.
3. If the archive is lost, Run51 must be recorded as a consumed registration
   with no result (not retried under the same id; any repeat is a new,
   separately frozen registration and new paid compute requiring owner
   authorization).
