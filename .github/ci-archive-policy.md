# CI archive policy

Historical, manual-only workflows are moved out of `.github/workflows/` into
`.github/workflows_archive/`. The index, the per-file reasons and the restore
steps are in `.github/workflows_archive/README.md`; the machine-readable list
is `.github/workflows_archive/MANIFEST.json`.

## Why

`.github/workflows/` is the only directory GitHub reads. Every file in it is a
registered workflow: it shows in the Actions sidebar, appears in the routing
lists, is linted and parsed on every workflow change, and is a candidate for
the PR-budget scan. Two hundred finished-study gates that can only be started
by hand add cost and noise without adding any check. Moving them costs nothing
at run time and loses no history.

## What qualifies

A workflow moves only when all of these hold. Re-verify them on the commit you
are moving from; do not trust an old list.

1. `on:` is exactly `workflow_dispatch`. No `push`, `pull_request`, `schedule`,
   `workflow_call`, `workflow_run` or `repository_dispatch`.
2. No other workflow reaches it (`uses: ./.github/workflows/<file>`).
3. No script, test, fixture, evidence seal or generated artifact reads,
   hashes, dispatches or labels it by path. Search `origin/main` for
   `<file>` and for `gh workflow run`. Prose in research notes and the
   ARC runner lists (`taskq/deploy/eks-karpenter/arc/*-workflows.txt`) does
   not block a move: research notes are evidence and are not edited, and the
   ARC lists have no validator and are keyed on file names that only matter
   for files still in `.github/workflows/`. Record prose pointers in the
   manifest instead.
4. Its bytes are not sealed. `weak-curve-lean.yml` and
   `weak-curve-conjectures.yml` are bound to evidence hashes and never move.
5. It is a historical stage or finished study, not a live gate or an
   in-progress protocol.

Move with `git mv` and commit the renames with no content change, so every
move is a pure rename (R100). `scripts/check_curve_names.py --diff` flags
retired-curve-name prose on added lines; a pure rename adds none.

## Directory name

Use `workflows_archive` with an underscore, not `workflows-archive` or
`archive`. `scripts/build_curve_registry.py` scans every tracked `*.yml` and
takes the first path in sorted order as a generated source label. A hyphen
sorts before `/`, so a hyphenated directory would reorder those labels and
force a regeneration of `docs/curves/registry.json`, which is an embedded
input of the Rust and release CI scopes.

## What validates the archive

All repository enumerators use non-recursive globs of `.github/workflows`
(`check-pr-budgets.py`, `scripts/test_workflows_parse.py`, `workflow-lint.yml`,
the ARC `route_workflows.py`), so the archive directory is ignored by them. That
is intended: archived files are not registered workflows, so actionlint, the
trigger and timeout budgets and the routing lists do not apply to them.

The cheap invariants are still checked. The `Workflow files parse` job runs an
inline check that every archived file parses as YAML, declares only
`workflow_dispatch`, has jobs, does not also exist in `.github/workflows/`, and
matches `MANIFEST.json`. That workflow now also triggers on changes under
`.github/workflows_archive/`. Archived files are not run through actionlint
because a restore re-enters the normal lint path.

`scripts/test_workflows_parse.py` and `scripts/ci_disable_historical_koblitz_pr_triggers.py`
glob `koblitz-stage*.yml` in `.github/workflows/`. `koblitz-stage25-single-core.yml`
and `koblitz-stage132-current-gate-audit.yml` stay there for functional reasons,
which keeps both globs non-empty. If both ever move, those two scripts must
also read the archive directory, or the second one exits 1.

## Adding to or restoring from the archive

- To archive more: re-verify the criteria above, `git mv`, and add entries to
  `MANIFEST.json` and a row to the README index (the manifest guard fails if
  they disagree).
- To restore: `git mv .github/workflows_archive/<file> .github/workflows/<file>`
  and remove its entry from the manifest and README.
