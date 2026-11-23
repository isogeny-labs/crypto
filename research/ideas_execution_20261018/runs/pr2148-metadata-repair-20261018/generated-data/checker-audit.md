# Leaderboard validation: static audit

Scope: read-only source inspection in `/workspace/crypto`. No tests, research,
generator, repository writer, Git command, index operation, or network request
against the repository was executed by this audit. This file is the only audit
output. Commands below are instructions for the owning agent, not passed checks.

## Official commands and side effects

`.github/workflows/ic-leaderboard.yml:140–147` runs these four commands, in order,
from the repository root:

```sh
python3 scripts/test_check_ic_leaderboard_paths.py
python3 scripts/check_ic_leaderboard_paths.py
python3 scripts/build_ic_leaderboard.py --check
python3 scripts/build_lab_browser.py --check
```

Set `PYTHONDONTWRITEBYTECODE=1` on each command to prevent incidental import-cache
writes. This preserves their validation semantics. CI uses Python 3.11 and gives
the whole job ten minutes.

| Command | Explicit side effects and limits |
| --- | --- |
| Path-checker self-tests | Standard-library unit tests for glob matching and workflow parsing. Create temporary workflow fixtures with `TemporaryDirectory`, then remove them. No repository data generation or scientific measurement. |
| Path coverage checker | Executes the actual leaderboard builder with `--check` under a read recorder. Reads default workflow `.github/workflows/ic-leaderboard.yml`; accepts explicit workflow paths as positional arguments. Checks both `pull_request` and `push` filters. No explicit file writes. |
| Leaderboard `--check` | Rebuilds the view in memory, compares exact text for all three outputs, and checks ledger-section coverage. No explicit output writes. Exit 1 for stale/missing outputs or uncovered ledger section. Other source/parse errors may raise directly. |
| Lab-browser `--check` | Builds metadata in memory, reads current output, compares structured data with the limited exclusions below. Runs local read-only `git ls-files` and, for absent sparse files, `git cat-file`. Sets `GIT_NO_LAZY_FETCH=1`; unavailable objects fail instead of silently fetching. No explicit output writes. |

The coverage checker catches **all `SystemExit` from the builder** and ignores
them so staleness remains the separate freshness check's job. Thus coverage PASS
does not imply builder/freshness PASS, complete input discovery, or data validity.
Non-`SystemExit` exceptions propagate. It records only files actually opened,
not unavailable files or all possible glob matches. Unsupported GitHub glob
syntax (`?`, `+`, `[]`, `!`) and `paths-ignore` fail closed; matching models literal
text, `*`, `**`, and `**/` only.

## Writers: owner-only operations

`python3 scripts/build_ic_leaderboard.py` writes exactly:

- `docs/ic/leaderboard.json`
- `docs/ic/LEADERBOARD.md`
- `docs/ic-leaderboard.html`

`--artifact PATH` also writes a fragment-style publishable copy **after rewriting
those three outputs**; it is not an isolated candidate-output option. When
combined with `--check`, execution returns from the check branch before any
artifact write. There is no leaderboard `--out` option.

`python3 scripts/build_lab_browser.py` writes `docs/browser/data.json`, creating
its parent directory if needed. It supports `--out PATH` for an alternative
output and `--check --out PATH` to validate that file.

These scripts are existing site metadata tooling. They launch no solver,
benchmark, curve experiment, or Rust/C++ research executable. AGENTS §§7b–7c
require generator-produced outputs and preservation of frozen evidence.

## Review semantic deltas, separately from freshness

1. **Sparse Koblitz inputs are a numeric risk.**
   `koblitz_phase_shares` (`build_ic_leaderboard.py:348`) discovers
   `research/ic_descent_20260930/runs-isolated/main/k{a}n{n}/M[1–4]/r*-candidate.price.json`
   using filesystem globbing. A set with no reports is silently skipped. It
   computes per-set medians, then a mean of available sets' normalized shares;
   `counts` comes from the first available report. Missing materialization can
   change `board[*].recipes[*].phases_s`, `best.phases_s`, and LA/trial metadata
   even while the frozen summary total `s` stays unchanged. All sets absent
   instead produce an error. Compare completeness against tracked inputs before
   treating any phase delta as intended. This builder has no HEAD overlay.
2. **Whole-pipeline numbers:** review board membership/recipe ordering, `best`,
   `s`, `s_setup`, `s_online`, `phases_s`, reference/floor S, ratios and intervals,
   before marks, online diagnostics, verification flags, sample/target counts,
   base points/columns/trials/relations, and leader selection. Leaders exclude
   construction-artifact rows when alternatives exist. Changes need source
   evidence, not merely a successful freshness check.
3. **Preserve family units.** A is one-target Koblitz clock units; B is
   prime/random-binary GAE counts; C is historical 32-target Koblitz batch clock
   units. S, floors, and reference ratios are comparable within a family only.
   Component/hosted/counted-lower-bound diagnostics remain outside tables A–C.
4. **Other numeric panels:** compare exponents/fit quality/point counts; oracle
   dimensions, first-fall range, hit rates, GAE/target, found/refuted/inconclusive
   counts; LA engine timings/core dimensions/ratios; frontier/n83/other bounded
   diagnostic selection/admission flags, counts, controls and intervals.
5. **Identity versus numeric changes:** registry/crosswalk digests, roster
   additions, YAML record count, `curves_yaml_key`, factor-base refs and link
   status are metadata changes, but still semantic. `curve_records()` keys by
   slug with last-record-wins semantics, so duplicate-slug aliases can replace a
   prior key/ref/status. Do not assume a digest-only delta without comparison.
   Pre-existing limitation: leaderboard `roster()` distinguishes `prime` from
   everything else and formats the latter as `GF(2^m)`; registry `extension`
   entries exist, so check field-label deltas rather than treating a successful
   regeneration as mathematical validation. Lab-browser `curve_rows()` does
   separately handle `extension` fields.
6. **Rounding:** only leaderboard JSON floats are normalized to twelve
   significant figures (`stable`, line 2089). Markdown/HTML are rendered from
   the unrounded in-memory object. The JSON comparator should preserve types
   and treat rounding-level changes separately from meaningful numeric deltas.
7. **Browser checks have deliberate exclusions.** `comparable()` removes only
   each candidate's `mentions`, `files`, and `where`; every other structured
   field is compared exactly. Browser source discovery uses tracked paths,
   worktree bytes for materialized inputs, and HEAD only for absent
   skip-worktree entries. Ordinary tracked deletions remain deleted. Review
   counts, curve/method/base/session/round identities, source hashes, and yield
   admission: zero charged cost with unpriced native work must retain `s=null`
   and `total_gae=null`, rather than become a zero-cost win.

## Additional focused tests (not in this workflow)

```sh
python3 scripts/test_build_lab_browser.py
python3 scripts/test_lab_browser_cost_admission.py
```

The first creates and mutates **temporary fixture Git repositories**, including
commits, skip-worktree flags, and removal of a fixture object. It does not mutate
the real checkout. The second consumes a frozen native session and tests metadata
admission in memory. Neither command was run here, and neither is part of the
four-command official workflow sequence above.

## Inspected source hashes

These hashes identify the working-file bytes inspected, not a Git revision or
an assertion that no other agent subsequently changed them:

```text
4269962bfb622b79822d065246d64f7658b0c34a1be7cc655878fd78f2c82788  scripts/build_ic_leaderboard.py
c75f52255515ef80ac36a33e869e4db3ce403c83bc37c527b60f8cc3a7b2b7a9  scripts/check_ic_leaderboard_paths.py
4f7daf470d37d1869bababb337aba3347fe9426cc2b9b54f9e5eb0e539b0ae84  scripts/test_check_ic_leaderboard_paths.py
af8c71a46b1b939dc0975e013cbde70594e2e1c7b018cdf77713b340be43fbf6  scripts/build_lab_browser.py
59b5e2a73f1a3f69ac8225ae970ee0c9113980b701aebd598ff57a6139ec827e  .github/workflows/ic-leaderboard.yml
```
