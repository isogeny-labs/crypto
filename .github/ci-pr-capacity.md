# PR capacity, batching and branch hygiene

Status: added by `ci/pr-branch-hygiene`. Nothing here is a required check. The
repository has no branch protection and no required status contexts
(`.github/CI.md`), so every cap and every cleanup in this note is advisory or
operator-run. Nothing described under "Owner-only steps" has been applied.

## Why

Since PR #2339 (`codex/ready-pr-policy-20261020`, `AGENTS.md` line 10, "Use
regular PRs") every PR is opened ready for review, and a draft is never a
holding queue. That is deliberate and unchanged here. Its side effect is that a
PR starts all its PR-triggered workflows on the first push. Measured on a replay
of 60 merged PRs, the median PR started 8.5 workflows and the worst started 128
(426 workflows exist, 133 of them PR-triggered). At about 38 merged PRs per day
and about 2400 remote branches, a lane that opens many PRs at once delays
everyone, and merged branches pile up.

PR #2339 changed `AGENTS.md` only (policy text). It added no workflow, label,
cap, count or cleanup, so this work does not duplicate it. This work does not
convert PRs to draft and does not mark PRs ready, because `AGENTS.md` forbids
draft as a queue and two one-shot gates
(`ecc2k130-263-all-lines.yml`, `ecc2k130-s3-sparse-n19-growth.yml`) still contain
draft-PR assumptions in pre-outcome states.

## What is provided

| File | Role |
|---|---|
| `.github/scripts/pr_hygiene.py` | Standard-library tool: `report`, `plan` (dry run unless `--apply`), `notice`. |
| `.github/scripts/test_pr_hygiene.py` | Offline unit tests with fixtures. No network, no `gh`. |
| `.github/workflows/pr-hygiene.yml` | Weekly report to the job summary; manual `apply` job (default off). |
| `.github/workflows/pr-lane-cap.yml` | Advisory notice when an opened or reopened PR is over its lane cap. |
| `.github/branch-protect.txt` | Human-maintained list of branches never to delete. |

Both workflows are additive. They do not change any existing check name,
`llm-review.yml`, `autofix-queue.yml`, `AGENTS.md` or `CI.md`.

## Lanes and the cap

A PR's lane is the first path segment of its head branch: `codex`, `claude`,
`cursor`, `research`, `feat`, `fix`; anything else is `other`. A PR whose head
repository is not this repository is `fork`: it is listed but never counted and
never acted on.

Soft caps on open same-repo PRs per lane (guesses, to be tuned from the first
weekly reports; no live data was available when they were chosen):

| Lane | Default cap |
|---|---:|
| codex | 30 |
| claude | 30 |
| cursor | 15 |
| research | 15 |
| feat | 10 |
| fix | 10 |
| other | 10 |

Override with the repository variable `PR_LANE_CAPS`, a JSON object merged over
the defaults. `null` or `0` means no cap for that lane:

```json
{"codex": 40, "claude": 40, "research": null}
```

PRs are ordered by number. The newest PRs beyond the cap are "over the cap". A
PR labelled `cap-exempt` is neither counted nor flagged; use it for a bounded
main-health fix (for example a `fix/` PR repairing main CI). Draft PRs count.

What happens when a PR is over the cap:

- `pr-lane-cap.yml` runs on `pull_request` `opened` and `reopened` only. For a
  same-repo PR it adds one label, `over-lane-cap`, and one comment (marker
  comment, so a reopen does not repeat it).
- It cannot block a merge: the job has `continue-on-error: true`, the script
  always exits 0, and no ruleset or required check refers to it.
- It does not run PR code. It checks out the default branch only (sparse,
  `persist-credentials: false`), reads the PR as API data, and passes the PR
  number through `env:`.
- A label or comment written with `GITHUB_TOKEN` does not start further workflow
  runs (documented GitHub behaviour; not verified in this repository). It never
  touches labels named `run-*`.
- It is evaluated once, at open. Closing an earlier PR does not remove the label.
- Cost: it adds one very small job (5-minute limit) per opened PR.
- Skipping `llm-review.yml` and the autofix queue for labelled PRs would need a
  one-line condition in each file. That is not done here.

## Batching guidance for agents

- One PR per coherent change. Do not split one change into a chain of PRs.
- If your branch already has an open PR, extend it and update its title and
  description (`AGENTS.md` line 7). Do not open a second.
- Batch compatible experiments into one focused PR (`AGENTS.md` line 582,
  "compatible group of experiments"). Each PR starts a median of 8.5 workflows.
- Before opening a PR, check how many PRs your lane has open. If the lane is at
  its cap, finish, merge or extend an existing PR instead of opening another.
- Rebase onto `origin/main` and then merge. Do not merge `main` into a branch
  that has no conflict, because each push restarts the PR workflows.
- Stack only when the parent PR is still open, and say so in the description.
  Never retarget or delete a branch that is another open PR's base.
- Close a PR you have superseded, with a comment naming its replacement. Closing
  keeps the branch, so evidence stays reachable (`AGENTS.md` research PR rules).
- After a PR merges, start the next change from a fresh branch cut from
  `origin/main`; do not reuse the merged branch. Push with
  `git push origin HEAD:<new-name>`.
- Never delete remote branches by hand from an agent session. `AGENTS.md`
  line 1271 ("Never delete remote objects") is in the store section, and this
  note applies the same caution to branches.
- Opt-out labels: `keep-open` (never stale-closed), `no-auto-delete` (see
  below), `cap-exempt` (not counted against the lane cap).

## Weekly hygiene sweep (`pr-hygiene.yml`)

Triggers: `schedule` (Mondays 06:41 UTC) and `workflow_dispatch`. It has no
`pull_request` or `pull_request_target` trigger, so a PR author cannot change
what runs. Top-level `permissions: {}`; each job states its own.

`report` job (`contents: read`, `pull-requests: read`; always runs):

1. Runs the offline unit tests.
2. Writes the report to the job summary: open PRs per lane against the cap,
   stale PRs, merged-but-undeleted branches, and branches with no PR older than
   `orphan_days`.
3. Prints the dry-run plan.

`apply` job (`contents: write`, `pull-requests: write`, `issues: write`): runs
only for a manual dispatch with `dry_run` set to `false`, from the default
branch, and not while `PR_HYGIENE_PAUSED` is set. The plan is recomputed from
fresh data. The script refuses `--apply` without a non-empty `allow` glob list.

Dispatch inputs: `dry_run` (default `true`), `allow`, `stale_days` (14),
`close_after_days` (7), `merged_days` (3), `closed_days` (30), `orphan_days`
(14), `max_deletes` (50). All inputs reach the script through `env:`; none is
interpolated into a shell command.

### Branch deletion rules

A branch is deleted only when all of these hold:

1. Its lane is on the allowlist: `codex/`, `claude/`, `cursor/`, `feat/`, `fix/`.
   Every other name (including names without a slash, `ci/`, `infra/`) is never
   touched.
2. Its name matches `^[A-Za-z0-9._/-]{1,200}$` and has no `..`, `//`, trailing
   `.` or `.lock`. Valid git branch names such as ``codex/x$(id)`` fail this
   check and are skipped, not deleted.
3. It is not protected (below) and not the head or base of any open PR.
4. One rule matches:
   - **A**: the tip is an ancestor of the default branch and a merged PR exists
     for that head ref, merged at least `merged_days` ago;
   - **B**: a merged PR's final head SHA equals the tip (the squash-merge
     case), merged at least `merged_days` ago;
   - **C**: a closed-unmerged PR's final head SHA equals the tip, closed at
     least `closed_days` ago.
5. The branch name matches an `--allow` glob.
6. It is deleted with
   `git push --force-with-lease=refs/heads/<branch>:<sha> <url> --delete <branch>`,
   so a branch that gained a commit after the decision is not removed. At most
   `max_deletes` per run.

Never deleted, whatever else is true: the default branch (read from the API),
`main`, `perf-history`, `research/**`, `lit2026*`, `gh-readonly-queue/**`,
`copilot/**`, `dependabot/**`, `release/**`, `refs/**`; any branch named
anywhere in a workflow file on the checked-out ref (found by scanning
`.github/workflows` at run time); any name or glob in
`.github/branch-protect.txt`; branches with commits not in the default branch
and no PR (reported only); and release tags (the tool only deals with
branches; it never addresses `refs/tags` or `refs/pull`). A tip's commit age
alone never qualifies a branch, because a new agent branch cut from an old
commit has an old tip date.

The workflow-name scan is intentionally broad: any mention of a branch name in
a workflow file, even in a comment, protects it. A wildcard in a workflow such
as `codex/*` would protect the whole lane, so do not write such patterns in
workflow comments.

The tool fails closed: an API error, a missing page, a listing above 400 pages,
an empty PR or branch list, or a missing manifest or workflows directory aborts
the run with a non-zero exit before anything is changed.

`.github/branch-protect.txt` was seeded from the 132 branch names that existed on
`origin` and were cited in tracked docs, evidence or code. The scan excluded
`*.json`, `*.csv`, `*.log` and `*.txt` and used substring matching, so it is a
lower bound. It is regenerated by a human; CI does not search the repository.
Future evidence should cite commit SHAs, not branch names.

`no-auto-delete` is a convention for people: add the branch name to
`branch-protect.txt`. The tool does not read PR labels for deletion (a branch
with an open PR is already kept).

### Stale PRs

An open same-repo PR whose `updated_at` is at least `stale_days` old gets one
warning comment and the label `pr-hygiene:stale`. If it still has that label and
`updated_at` is at least `close_after_days` old, it is closed with a comment that
records the branch name and head SHA and says "reopen to restore". The branch is
kept. Exempt: labels `keep-open`, `needs-human`, `autofix:running`,
`large-file-ok`, any `run-*`; PRs whose head is protected or is the base of
another PR; fork PRs. At most 10 PR actions per run. The stale clock uses
`updated_at`, which any comment or label also advances; the tool errs toward
leaving PRs open.

## Owner-only steps (shown, NOT executed)

These need repository administration, which `GITHUB_TOKEN` cannot hold. Nothing
here has been run, and no part of this change depends on it.

Enable automatic deletion of head branches when a PR is merged:

```sh
gh api -X PATCH repos/aburan28/crypto -F delete_branch_on_merge=true
# check:
gh api repos/aburan28/crypto --jq .delete_branch_on_merge
```

Behaviour from memory, not verified here: it applies only to PRs merged through
GitHub from a branch in the same repository, and does not remove branches of PRs
merged another way. Branches already merged stay, which is what the sweep above
is for.

Create the labels the tools use (so they have a description and colour):

```sh
gh label create over-lane-cap  --repo aburan28/crypto --color d93f0b --description "Opened above the advisory per-lane PR cap"
gh label create cap-exempt     --repo aburan28/crypto --color 0e8a16 --description "Not counted against the per-lane PR cap"
gh label create keep-open      --repo aburan28/crypto --color 0e8a16 --description "Never stale-closed"
gh label create "pr-hygiene:stale" --repo aburan28/crypto --color fbca04 --description "Stale warning given; closes if still idle"
```

Optionally set caps and the pause switch:

```sh
gh variable set PR_LANE_CAPS --repo aburan28/crypto --body '{"codex": 40, "claude": 40}'
gh variable set PR_HYGIENE_PAUSED --repo aburan28/crypto --body true   # stops notices and apply
```

Recommended server-side backstop (not applied; create it in Settings, Rules,
Rulesets, rather than from a copied payload): a branch ruleset that blocks
deletion and non-fast-forward pushes for `main`, `perf-history`, `research/**`,
`lit2026/**` and release patterns. It protects those branches even if this
tool has a bug. A ruleset that blocks deletion also stops `delete_branch_on_merge`
from removing matching branches.

First supervised use of `apply`, after reading a dry-run plan:

```sh
gh workflow run pr-hygiene.yml --repo aburan28/crypto -f dry_run=false -f allow='codex/*' -f max_deletes=5
```

Do this on the default branch, on a small allow pattern, and check one
deletion before widening.

## Not verified

This was built with no network and no GitHub access. The unit tests, actionlint
and the repository's workflow parse and PR-budget checks were run locally; no
workflow has run on GitHub. Not verified: `gh api` behaviour and rate limits,
whether `GITHUB_TOKEN` may delete head refs, that `actions/checkout` accepts
`filter: tree:0` with `fetch-depth: 0` and a sparse checkout, that adding a
label with `GITHUB_TOKEN` creates a missing label, runtime of the sweep on the
real ~2400 branches and ~2300 PRs, and that a lease-guarded delete over HTTPS
from an empty repository works against GitHub (it was tested against a local
bare repository only). The first scheduled run is report-only for this reason.
