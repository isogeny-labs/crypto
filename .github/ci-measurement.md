# CI measurement: fan-out replay and live metrics

Two tools, one question: is CI getting cheaper? Scoping changes (path filters,
scope jobs, fewer always-on workflows) are only worth keeping if a number moves.

| Tool | Answers | Needs | Network |
|---|---|---|---|
| `.github/scripts/ci_fanout_report.py` | How many workflows does a merged PR trigger? Which workflows and shared `paths:` entries cause it? | `git`, Python 3.9+, PyYAML | none (git plumbing only) |
| `.github/scripts/ci_live_metrics.sh` | How long do jobs queue? How many runs are waiting? Where do the minutes go? How big are the rust caches? | `gh` with a token that can read Actions, `jq` | GitHub REST API, GET only |

Neither tool is wired into a workflow. `.github/CI.md` is not changed by this
document; it describes the policy, this file describes the measurement.

## 1. Fan-out replay: `ci_fanout_report.py`

For every first-parent merge commit in a range it takes the changed files
(`MERGE^1..MERGE`), evaluates each PR-triggered workflow's `on.pull_request`
(and `pull_request_target`) `paths`, `paths-ignore`, `branches`,
`branches-ignore` and `types`, and counts how many workflows would have started.
It reads everything with `git log`, `git diff-tree`, `git ls-tree` and
`git cat-file`; it never checks out a tree, so it is safe on the multi-GB
checkout and on a checkout with uncommitted work.

```sh
# Last 100 merged PRs on origin/main, workflows as they are at origin/main today.
python3 .github/scripts/ci_fanout_report.py --repo /path/to/crypto --range origin/main --max-prs 100

# Machine-readable, same numbers.
python3 .github/scripts/ci_fanout_report.py --repo /path/to/crypto --json

# Before/after for a scoping branch: replay the SAME PRs against two workflow sets.
R="--repo /path/to/crypto --range origin/main --max-prs 200 --json"
python3 .github/scripts/ci_fanout_report.py $R --workflows-ref origin/main      > before.json
python3 .github/scripts/ci_fanout_report.py $R --workflows-ref my-scoping-branch > after.json

# Score one prospective change set (what would THIS branch trigger?).
python3 .github/scripts/ci_fanout_report.py --repo . --diff origin/main HEAD
```

Options worth knowing:

- `--workflows-ref tip|merge|<rev>`: `tip` (default) reads workflows at the newest
  commit of the range, i.e. "what would today's configuration do to recent PRs".
  `merge` reads the workflows that existed at each PR's own merge commit
  (what really happened, workflow edits included). Any rev gives a what-if.
- `--mode first-parent` treats every first-parent commit as a PR, for squash or
  rebase merge histories. The default `merges` matches merge-commit history.
- `--events pull_request` leaves out `pull_request_target` workflows such as
  `llm-review.yml`. Both count by default because both start a run per PR.
- `--base-branch` (default `main`) is matched against `branches` filters.
- `--min-share N` (default 2) hides path entries used by fewer than N workflows
  in the shared-entries table. Use `1` to see every entry.

### What the output means

- **Workflows triggered per PR**: median, mean, p90, max over the replayed PRs,
  plus how many workflows every PR hits (the floor), and how many of those have
  no path or branch filter at all.
- **Top workflows by trigger frequency**: a path-filtered workflow near 100% is
  effectively always-on; its filter is not buying anything.
- **Shared path entries**: for each file a PR changed, the positive `paths:`
  entry that last included it gets credit. `triggers` counts (PR, workflow)
  pairs the entry helped start; `decisive` counts pairs where it was the only
  entry that did, so removing it from that workflow would have stopped the run.
  The `workflows` column is how many workflows list the entry: a one-line edit
  to `Cargo.toml` fans out to every workflow that lists it, whether or not the
  PR was about Rust.
- **Highest fan-out PRs**: the PRs to look at first when the max is high.

### Baseline, 2026-10-23, `origin/main` = `da3a1b5d53`

Workflows read at the tip, 100 most recent merge commits, `pull_request` and
`pull_request_target` both counted:

| | value |
|---|---|
| Workflow files / PR-triggered | 444 / 139 |
| Median workflows per PR | 10 |
| Mean / p90 / max | 13.4 / 21 / 131 |
| Hit by every PR | 7 (codeql, ecbench, large-files, llm-review, rust-all-targets, rust-lint, validation-gate) |
| PRs over 40 workflows | 2 of 100 |

The floor (7 of the median 10) is the always-on set, so the median can only
come down by removing or narrowing those. The max is driven by PRs that touch
many workflow files at once, because most workflows list their own file in
`paths:`.

### Limits of the replay (read before quoting a number)

- It counts **triggered workflows**, not jobs or minutes. A workflow whose jobs
  all skip after a scope job still counts; that is deliberate, since it still
  creates a run and check. Use `ci_live_metrics.sh` for runner cost.
- `?` is treated as GitHub documents it: "zero or one of the preceding
  character", not a one-character wildcard. `+` likewise. No workflow in the
  repo uses either today. `!` inside `paths-ignore` is treated as a re-include
  (same ordered rule as `paths`); GitHub's documentation is thinner there.
  I could not verify either against a real GitHub run (no network).
- GitHub evaluates path filters on a diff capped at 300 files. PRs above that
  are counted in the output; their trigger sets are approximate.
- Renames count as delete plus add, which can only over-count.
- The diff for a replayed PR is the merge commit against its first parent. The
  real PR diff was against the merge base at the time of each push, so a PR that
  was rebased or had `main` merged into it can differ.
- Draft state, fork restrictions, `workflow_run` chains and `merge_group` runs
  are not modelled.
- An earlier ad hoc replay of 60 PRs quoted a median of 8.5 and a max of 128.
  This tool gives 10 and 131 for the 60 most recent merges at the same tip. The
  difference is most likely the PR window and `pull_request_target`; I did not
  reproduce the ad hoc script, so I cannot say which.

## 2. Live metrics: `ci_live_metrics.sh`

Run by someone with a working token (`gh auth status`). It is read-only: the
only network call site is `ghget()`, which is `gh api --method GET`. It does not
cancel, re-run, dispatch or delete anything.

```sh
.github/scripts/ci_live_metrics.sh --dry-run            # list the API calls, call nothing
.github/scripts/ci_live_metrics.sh --repo OWNER/REPO    # default repo: current directory's remote
.github/scripts/ci_live_metrics.sh --days 7 --sample-workflows 25 --sample-runs 4 --out ./ci-metrics-raw
```

| Section | API | Notes |
|---|---|---|
| 1. Queue now | `GET /actions/runs?status=queued` (and in_progress, waiting, requested, pending) | counts, plus queued runs by workflow with age since `created_at` |
| 2. Queue time per workflow | `GET /actions/runs/{id}/jobs` for sampled runs | per job `started_at - created_at`, median / p90 / max per workflow and per runner label set; skipped jobs are counted separately |
| 3. Minutes per workflow | `GET /actions/runs?created=>=DATE` | wall-clock is exact for listed runs; "est runner" extrapolates the sampled runs' job minutes and is labelled as an estimate |
| 4. rust-cache | `GET /actions/cache/usage`, `GET /actions/caches` | saved sizes, reuse proxy, repository usage |

Honest limits:

- The Actions cache API does **not** report hits or misses. "Saved" is a cache
  created in the window; "reused" is `last_accessed_at` more than 120 s after
  `created_at`. A real hit rate needs the `cache-hit` output or the restore step
  logs.
- GitHub caps filtered run listings at 1000 results; the script says so when
  the window is truncated and `--max-pages` stops it early.
- Queue time is sampled (`--sample-runs` recent completed runs for each of the
  `--sample-workflows` busiest workflows). The dry run shows the upper bound on
  API calls (about 140 at defaults, against a 5000/hour limit).
- Only run it from a machine you trust with the token; it prints workflow and
  cache key names, not secrets.
- I verified the script only against a stub `gh` that served fixture JSON (and
  `shellcheck`, `bash -n`). It has never run against the real API, so the first
  real run may show a field name or shape I got wrong.

## 3. Thresholds that should trigger action

These are starting values. They are proposals, not measured optima; revisit
after two or three weeks of data.

Fan-out (from the replay, last 100 to 200 merges):

| Signal | Threshold | Action |
|---|---|---|
| Median workflows per PR | > 10 | Open the "top workflows" table; narrow or consolidate the always-on set first. |
| p90 workflows per PR | > 25 | Look at which shared entries drive the tail. |
| Max workflows per PR | > 40 | Open the highest fan-out PRs; usually a shared entry or a repo-wide edit. |
| Path-filtered workflow triggered by > 90% of PRs | any | Its filter is not selective; treat it as always-on or tighten it. |
| Shared entry listed in > 10 workflows and `decisive` > 0 | any | Replace with a scope job or a narrower pattern. `Cargo.toml` and `Cargo.lock` are the current examples (39 and 36 workflows). |
| Workflows hit by every PR | rises | New always-on workflows need a reason. |

Today (median 10, max 131) the median is at the line and the max is well over it.

Live metrics:

| Signal | Threshold | Action |
|---|---|---|
| Queued runs now | > 20, or median age of queued runs > 15 min, in two consecutive runs of the script | Runner capacity or too many jobs per PR; see section 2 by runner label. |
| Median job queue time, any workflow | > 120 s (hosted) | Check runner label routing and job counts for that workflow. |
| p90 job queue time | > 10 min | Same, and check for bursts after merges. |
| Skipped jobs / jobs for a workflow | > 50% | Shell jobs that still occupy the check list: candidates for literal-named non-matrix jobs with a job-level `if`. |
| One workflow's share of estimated runner minutes | > 25% while its PR trigger share is high | Cost target for scoping. |
| Repository cache usage | > 80% of 10 GiB | Caches are being evicted before use. |
| rust-cache entries never reused after creation | > 50% by MiB | PR-ref caches are wasted writes; consider saving only from `main`. |
| Any single rust cache | > 1.5 GiB | Trim cache contents or key by narrower inputs. |

## 4. Budget check mode and how it could be wired (not wired)

`ci_fanout_report.py --max-fanout N` (and `--max-median N`) exits **1** when the
budget is exceeded, **0** when it is met, **2** on a usage or environment error.
Nothing in the repository calls it. No required workflow runs it, and this PR
adds no workflow.

Two useful shapes:

```sh
# Replay gate: the current workflow set must keep recent history under budget.
python3 .github/scripts/ci_fanout_report.py --range origin/main --max-prs 100 --max-fanout 40 --max-median 10

# Change-set gate: what would this PR trigger?
python3 .github/scripts/ci_fanout_report.py --diff origin/main HEAD --max-fanout 40
```

Proposed wiring, for the owner to decide:

1. Start as an **advisory, non-required** workflow, `ci-fanout-budget.yml`, with
   `on: pull_request` and `paths: ['.github/workflows/**', '.github/scripts/ci_fanout_report.py']`.
   One job, `timeout-minutes: 10` (it must satisfy `check-pr-budgets.py`, which
   requires a literal 1 to 25 minute timeout), `actions/checkout` with
   `fetch-depth: 0`, `pip install pyyaml`, then the replay gate comparing base
   and head workflow sets so a PR is judged on what it changes:
   `--workflows-ref origin/${{ github.base_ref }}` versus `--workflows-ref HEAD`
   over the same range, failing if the median or max grows.
2. Keep it out of branch protection until it has run on a few weeks of PRs. A
   path-filtered required check that never reports leaves PRs pending, the
   hazard `.github/CI.md` already describes; a non-required job avoids it.
3. Note that the new workflow is itself a path-filtered workflow listing
   `.github/workflows/**`, which is already the second most expensive shared
   entry. Do not add it to a broad filter.
4. The `--diff` form is a noisy gate for ordinary code PRs: a Rust edit
   legitimately triggers many workflows. Use the base-versus-head comparison to
   gate *regressions in configuration*, and keep the absolute `--max-fanout`
   for the scheduled replay or a manual look.
5. If a scheduled run is preferred, run the replay weekly from `main` and post
   the JSON as an artifact; nothing here needs write access.

## Reproducing the tests

```sh
python3 .github/scripts/test_ci_fanout_report.py   # builds a synthetic repo in a temp dir
bash -n .github/scripts/ci_live_metrics.sh && shellcheck .github/scripts/ci_live_metrics.sh
```
