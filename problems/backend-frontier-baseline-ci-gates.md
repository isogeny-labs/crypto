# Backend frontier baseline CI gates

- **Status:** open
- **Kind:** implementation and validation gaps in the pinned base
- **Affected work:** [backend frontier PR #2103](https://github.com/aburan28/crypto/pull/2103), [reproduction contract](../docs/ecbench/BACKEND_REPRODUCTION.md)
- **Next action owner:** unassigned; maintainers of the shared CI, registry and site components
- **Last updated:** 2026-10-17

## Problem

The backend frontier PR cannot meet its current-head merge gates while shared
CI scripts, workflow syntax, registry naming and site checks inherited from
base `c6d64436a33359320ad9d5b42c4acebee2968b9a` fail. A separately tested
leaderboard producer does not waive these failures.

## Evidence and uncertainty

The [CI follow-up](../docs/ecbench/backend-frontier-report/validation/ci-20261017/README.md)
retains decoded job logs, commands, file hashes and exact base/head Git object
comparisons for head `9f286da9f813359bdae4151cfc2a79667fac47f1`.

The [blobless Cargo follow-up](../docs/ecbench/backend-frontier-report/validation/ci-20261017-cargo/README.md)
adds current-head logs and the same 19 object comparisons for `28bb1f67`.
That hosted leaderboard job reached Cargo and exposed a second change-caused
checkout defect: the root `.gitignore` blob was absent. A fresh shallow,
blobless negative control reproduced exit 101; selecting only `/.gitignore`
then passed all three workflow blocks, 20 tests and the unchanged 115/five
hash contract. Its correction still requires hosted CI on its own final head.

- **Observed:** `.github/scripts/ci-scope.mjs:50` redeclares `root`. The exact
  base file also fails `node --check` with `Identifier 'root' has already
  been declared`. This blocks scope/materialization and downstream Rust and
  ecbench guards, backlog jobs, and the workflow-parser test.
- **Observed:** actionlint reports duplicate keys in `codeql.yml`,
  `curve-covers.yml` and `weak-curve-database.yml`. All three files have the
  same Git object IDs in the base and tested PR head.
- **Observed:** the global curve-name check reports unregistered historical
  curve names and a registry/alias mismatch. No diagnostic names this PR's
  new paths; the checker, registry, aliases and research tree are unchanged.
- **Observed:** the site suite fails on `unpriced_zero_runs` in the lab-browser
  builder, malformed embedded progress JSON, and the 303-versus-305 cached
  panel index. The relevant builder/data are unchanged. These are distinct
  from the earlier local sparse-reader limitation; the hosted job reaches
  the builder and exposes the missing aggregation field.
- **Separate change-caused defect:** the new leaderboard workflow supplied
  unsupported `--no-cone` to `git sparse-checkout add`. That option belongs
  to mode selection, already performed by checkout. Removing it makes the
  isolated sparse-workflow validation pass. Hosted CI on the fix's own final
  head is still required.
- **What this does not establish:** no fresh algorithm, independent-host or
  timing reproduction was performed. The existing
  [root compilation blocker](ecbench-main-build-integration.md) and historical
  source/binary gaps remain separate and unresolved.

## What would unblock it

- [ ] Repair the shared CI syntax in its owning scope; pass its syntax and
  unit tests, materialization, and downstream validation without bypasses.
- [ ] Pass workflow syntax, registry/alias consistency and the full site suite
  while preserving frozen identities and evidence. Do not edit historical
  measurements or weaken assertions merely to obtain a green check.
- [ ] Obtain all required/applicable CI and review on the final PR head;
  no earlier-head success, neutral quota signal or skipped job substitutes.

These conditions restore the merge gates. They do not establish missing
degree-51/83 experiments, complete counts, independent replay or timing wins.

## Next discriminating step

Reproduce the base CI-scope syntax failure and review a focused maintainer
repair, then rerun its existing tests and affected gates. A passing parser
only restores the ability to run the Rust/harness checks; their results must
still be evaluated. Recheck when a prerequisite repair lands or current-head
CI changes. No solver campaign is needed to distinguish these CI failures.

## Resolution and history

2026-10-17: identified from completed PR #2103 checks. The PR's sparse-checkout
defect is corrected and locally validated; the inherited gates remain open.

2026-10-17, second check-in: the next hosted run exposed the omitted root
ignore blob. The initial sparse selection now includes it, with a fresh
blobless negative/positive control. The inherited failures and absent
current-head review remain open; no gate is waived.
