# Completed CI triage and sparse-workflow correction

Observed head: `9f286da9f813359bdae4151cfc2a79667fac47f1`.
Base: `c6d64436a33359320ad9d5b42c4acebee2968b9a`.

## Change-caused failure and fix

[Leaderboard job 117489596181](https://github.com/aburan28/crypto/actions/runs/39064637258/job/117489596181)
failed before tests: `git sparse-checkout add --no-cone --stdin` exits 129
because `add` does not accept `--no-cone`. The checkout step has already
selected non-cone mode. The fix removes that option from `add`.

The old invocation was reproduced with exit 129 in a new sparse working
directory. The corrected invocation succeeded. Then the three actual `run`
blocks were extracted from the corrected workflow and executed in order:
materialization, format/tests/clippy, and freshness/locked reproduction.
They exited 0, with 20 tests passing and all 115 pinned files/five output
hashes agreeing. See `workflow-smoke.log.gz`, `workflow-run-blocks.sh`,
`sparse-before.log.gz`, and `reproduction.json`.

The isolated Git index/worktree used the existing read-only object store as
an alternate and cached Cargo dependencies (`CARGO_NET_OFFLINE=true`). Only
173 selected working files were needed. Local Git was 2.52.0; hosted Git
2.55.0 reported the same unsupported option. A first shared-clone attempt
failed on unavailable promisor objects and was discarded. Off-sparse object
warnings during fixture checkout do not supply missing evidence: the successful
run checks every required input by its pinned hash. This is a workflow/software
test on the same machine, not an independent-host algorithm replay.

## Inherited failures

`base-comparison.json` records identical base/head Git object IDs. No source
repair or frozen-evidence modification was made for these failures.

| Gate | Observed failure | Attribution / evidence |
|---|---|---|
| Shared CI scope and workflow-parser tests | `ci-scope.mjs:50`: duplicate `root` declaration | Exact base file also fails `node --check`; `scope.log.gz`, `parse.log.gz`, `base-scope-check.log` |
| Rust all-targets and ecbench harness | Fail-closed scope/metadata guard | Upstream scope failure prevents validation; these jobs did not compile/test the algorithms; `root-guard.log.gz`, `harness-guard.log.gz` |
| Backlog lab and validation | Same CI-scope syntax error | `backlog-lab.log.gz`, `backlog-validate.log.gz` |
| Workflow syntax | Duplicate keys in codeql, curve-covers and weak-curve-database workflows | Those workflow files are unchanged; `workflow-syntax.log.gz` |
| Curve names | Historical unregistered identifiers and registry/alias mismatch | No reported path belongs to the new leaderboard/docs/tool; checker, registry, aliases and research tree are unchanged; `curve-names.log.gz` |
| Site build | Lab-browser `unpriced_zero_runs` KeyError, malformed embedded JSON and cached panel index | Builder/data are unchanged; `site.log.gz`. The hosted KeyError is additional evidence beyond the earlier local sparse-reader limitation |

The root's separately observed 26 release-library compile errors remain in
`../reproduction-root-lib.log.gz`. Hosted guard failures are not represented as
completed library tests. Bugbot's neutral quota-limited result is not a code
review; the existing submitted approval belongs to the earlier `a86219ad` head.

## Commands and bounds

Conductor was attempted before edits and remains unavailable (exit 127).
The scoped correction is `.github/workflows/backend-leaderboard.yml`; the
other changes record validation and the named baseline obstruction.

```sh
node --check /tmp/crypto-pr2103-base-ci-scope.mjs
# Expected/observed: exit 1 for the exact base file.

# From the new sparse checkout, with Rust 1.99.1 and locked crates cached:
bash /tmp/crypto-pr2103-ci-workflow.sh
# Observed: exit 0. This script is retained as workflow-run-blocks.sh.
```

`job-logs.json` binds each retained decoded log to its GitHub job and SHA-256.
No solver campaign, new performance result, R1 independent replay or R2 timing
replication was executed. Merge remains blocked until all applicable checks
and reviews pass on the final head. The
[named obstruction](../../../../../problems/backend-frontier-baseline-ci-gates.md)
records the unmet gates and exit conditions.
