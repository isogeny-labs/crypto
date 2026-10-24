# Autofix queue throttles

`autofix-queue.yml` (hourly, plus the `autofix` label) hands PRs with merge
conflicts or failing checks to Haiku, then Sonnet. Before this change nothing
bounded how often a PR could be retried, how stale a PR could be, or how busy
the runners were when a sweep started. Three throttles now apply to **sweeps**.
Logic lives in `.github/scripts/autofix_triage.py`; tests are in
`.github/scripts/test_autofix_triage.py` (run by `autofix-triage-tests.yml`
when the queue's files change).

This note is separate from `CI.md`, which is not edited here.

## What is skipped, in the order it is checked

| Step | Rule | Cost of the check |
|---|---|---|
| 1 | Not open, fork PR, draft (unless `--include-drafts`), labelled `autofix:off` / `needs-human` / `autofix:running` | none (listing only) |
| 2 | **Behind main**: more than `AUTOFIX_MAX_BEHIND` commits behind the base (default **50**; `0` disables). Conflicting PRs included. Skipped PRs are left for a human or the hygiene job to rebase or close. If GitHub's compare call fails the PR is skipped, not guessed | 1 PR detail + 1 compare call |
| 3 | **Already green**: no conflict and no failing check. Ladder labels are cleared as before. This now happens *before* the head-commit lookup, so green PRs no longer pay for it | 1 check-runs call |
| 4 | Quiet period: head commit younger than `AUTOFIX_MIN_AGE_MINUTES` (45) | 1 commit call |
| 5 | **Lifetime attempt cap** (below) | 1 comments call, only for PRs that would otherwise be queued |

An explicit request (`autofix` label, or `workflow_dispatch` with a PR number)
bypasses steps 2 and 4 and the queue-depth pause, but **not** the fork, label,
or attempt-cap checks.

### Optional shortcut, off by default

`AUTOFIX_TRUST_CLEAN_STATE=true` (an environment variable you would add to the workflow's `env:`; it is not wired to a repository variable) skips
a PR whose `mergeable_state` is `clean` and which carries no ladder labels,
without calling check-runs at all. It is off because it is unverified that
`clean` always means "all checks pass" on a repository without branch
protection (CI.md records none). Enable it only after comparing it against the
check-runs path on live data.

## Lifetime attempt cap

The `autofix:haiku` / `autofix:sonnet` labels give at most two attempts per
ladder, but they are removed whenever the PR goes green, so a PR that keeps
breaking restarts the ladder forever. The cap needs a counter that survives
that reset.

* **Counter**: each finished attempt posts its normal report comment ending in a
  hidden `<!-- autofix-attempt n=K -->`, where `K` is the number of model
  attempts in that run (Haiku and/or Sonnet). A branch that moved mid-attempt
  ("stale") now also posts a one-line comment carrying the marker, because the
  model still ran. A comment is used rather than a counter label so no
  `labeled` event fires: label events start a run of this and other workflows.
* **Counted**: markers in comments by a bot account; earlier reports that start
  with `**autofix** ` (before markers existed) count as one each. Hand-off
  comments (`**autofix:** ...`) and anything written by a human are ignored, so
  a pasted marker cannot change the count.
* **Cap**: `AUTOFIX_MAX_ATTEMPTS` (default **4**, i.e. two full ladders; `0`
  disables). When a PR that would be queued has used the cap, triage adds
  `needs-human`, comments once, and the existing `needs-human` skip keeps it out
  of every later sweep.
* **Restart**: a comment `/autofix reset` from an `OWNER`, `MEMBER` or
  `COLLABORATOR` zeroes the count so far. Also remove `needs-human` (and
  `autofix:sonnet` if present).
* Fails closed: if the comment history cannot be read, or has 1000 or more
  comments, the PR is not queued.

## Queue-depth pause

Before triage, a sweep counts workflow runs with `status=queued` plus
`status=in_progress` (`gh api repos/{repo}/actions/runs?status=...&per_page=1`,
reading `total_count`). Its own run is discounted by one. If the total is above
`AUTOFIX_MAX_QUEUED` (default **40**; `0` disables) the sweep logs
`::notice title=autofix sweep paused::<why>`, emits `matrix=[]`, `count=0`,
`paused=true`, and touches no PR, label or comment. The next hourly sweep tries
again. Explicit single-PR requests are not paused.

Limits to keep in mind:

* It counts **runs, not jobs**. A run with some jobs executing and others
  waiting for a runner is `in_progress`, so a deep job backlog behind few runs
  is under-reported. The default of 40 is a first guess, not a measurement;
  tune it from the queue depths a few days of logs show.
* If the count cannot be read (for example a permissions error) the sweep
  **fails open** with a `::warning::`, so a transient API error does not stop
  the queue silently forever. The attempt cap and `AUTOFIX_MAX_PER_SWEEP` still
  bound spend.
* The `triage` job still runs on the same runner routing as before, so on the
  EKS pool it may itself wait behind the queue it is trying to measure. Moving
  it to a GitHub-hosted runner is a separate change (see "Not changed").

## Variables

| Variable | Default | Meaning |
|---|---|---|
| `AUTOFIX_MAX_ATTEMPTS` | 4 | lifetime model attempts per PR; 0 disables |
| `AUTOFIX_MAX_BEHIND` | 50 | commits behind base before a sweep skips a PR; 0 disables |
| `AUTOFIX_MAX_QUEUED` | 40 | queued + in-progress runs above which a sweep pauses; 0 disables |
| `AUTOFIX_MAX_PER_SWEEP`, `AUTOFIX_MIN_AGE_MINUTES`, `AUTOFIX_PAUSED` | 3, 45, unset | unchanged |

## Safety properties preserved

* The fix job, `autofix_run.py`, its allowed tools, protected-path and
  no-workflow-edit checks, and the plain (non-force) push are unchanged.
* Fork PRs are never queued, including by explicit request (tested).
* The only new permission is `actions: read` on the `triage` job, for the run
  counts. `fix` already had `actions: write`.
* Job timeouts are unchanged (`triage` 15 min, `fix` 75 min). The `fix` job
  already exceeded 25 minutes on the `labeled` event; the budget checker
  (`check-pr-budgets.py`) does not evaluate that event because `labeled` is not
  a routine PR action. This change does not lengthen any job. The new test
  workflow has a 5-minute job.

## Not changed

* `autofix_run.py` (the attempt marker is written by the workflow's existing
  "Report and release" step).
* Runner routing for `triage` / `fix`; `route_workflows.py` would re-add the
  EKS expression to a plain `ubuntu-24.04` line if it were hand-edited.
* The workflow-level `concurrency: autofix-queue` group. Any `labeled` event on
  any PR still creates a run in that group even though `triage` is then
  skipped, and GitHub keeps only the newest pending run per group, so a burst
  of unrelated label changes can displace a waiting hourly sweep. This is
  untested here; a fix is a separate change.
* The open-PR listing still stops at 1000 PRs (it now logs a warning).

## Not verified (no network or CI access when this was written)

* No run against GitHub. The REST shapes (`compare` -> `behind_by`,
  `actions/runs?status=` -> `total_count`, issue comments with
  `user.type` and `author_association`) are from memory of the API.
* The unit tests use fixtures and mocks for `gh`; they show the selection logic,
  not that the live calls return what the fixtures assume.
* Whether `GITHUB_TOKEN` comments carry `user.type == "Bot"` is assumed.
* Default values (4, 50, 40) are not tuned from live data. To pick them, see
  live data: the `behind_by` distribution of open PRs, queued + in-progress run
  counts over a few days, and autofix comments per PR.
