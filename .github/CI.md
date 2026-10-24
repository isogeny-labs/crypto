# CI scope and runtime

Ordinary pull requests should validate the code and evidence they change,
without rebuilding unrelated crates, packaging unrelated clients, or rerunning
the entire historical research corpus. Failures and timeouts remain failures.

## Observed baseline (2026-10-11)

Historical: the problems this document started from. The current shape, with
the 2026-10-23 job counts, is under "Always-on PR gate shape" below. The
default branch now has 444 workflow files: 142 declare `pull_request`, nine
`merge_group`, 138 `push`, ten `schedule` and 411 `workflow_dispatch`.

- The default branch has 348 workflows: 216 declare a pull-request trigger,
  87 a push trigger, 292 manual dispatch, and five a schedule. These categories
  overlap. Most historical research workflows already have path filters.
- CodeQL analyzes five languages on every PR, including a README-only PR.
- Rust lint compiles both the root crate and the independent `cd_attack` crate
  on either crate's changes; archived research Rust also triggers it.
- [Rust lint run 38008529000](https://github.com/aburan28/crypto/actions/runs/38008529000)
  passed: rustfmt executed for 76 seconds, root clippy for 142 seconds, and
  `cd_attack` clippy for 93 seconds. They waited approximately 219–222 minutes
  for runners. Queue delay and execution time are different problems.
- The ecbench job allows 300 minutes and replays every committed deterministic
  run, including 320 column-sweep solves. It also performs dedicated historical
  replays before replaying those sessions again in the corpus loop.

The integration branch has 352 workflows. Its newer decision to retire
99 completed-study gates from automatic events is preserved. This does not
retire the n53 target-cyclic rank hash-only custody check: it remains an active
PR gate on its listed inputs, while only the separate outcome job requires the
`run-n53-rotation-once` label. Its historical preflight uses an isolated view
with the original nested lock bytes from the released commit and checks that
the live nested lock still matches the current root lock before substitution.
The labeled outcome retains its original exact-current-source and head guards;
the historical view does not authorize a measurement. The weak-curve conjecture
gate keeps its census replay, tests, Lean proof and axiom checks. Its original
60-minute Lean budget remains bound to its frozen workflow digest, as does the
separate 60-minute `proofs` job in `weak-curve-lean.yml`.

## Scope contract

[ci-scope.mjs](scripts/ci-scope.mjs) compares local git trees, with NUL-delimited
paths and both sides of renames. It has no changed-file API truncation. A missing
base, failed fetch/diff, missing source, or manual/scheduled event selects all
checks. PR comparisons use the tested merge commit against its own first parent
(the base tip that merge was built on), falling closed to all checks if the
checkout is not the merge of the event's head; merge groups use their base SHA.
Comparing against the event payload's base SHA instead would attribute every
file merged to main since the event to the PR, so a docs-only PR could select
Rust, CodeQL and ecbench work after any unrelated merge. Deleted files remain
relevant.

The core workflows keep their existing check names and run on every PR. Their
jobs or expensive steps decide applicability after classification, so an
irrelevant path cannot leave a required workflow permanently pending.

| Change | Applicable expensive checks |
| --- | --- |
| README or ordinary prose | No CodeQL language analysis, Rust compilation or historical ecbench replay |
| Root Rust source, Cargo dependencies, build/toolchain configuration | Root lint/compile; Rust CodeQL; shipped Rust packaging when consumed |
| `cd_attack` source or dependencies | `cd_attack` clippy; Rust CodeQL |
| Archived research Rust | Rust CodeQL; its existing research-specific workflow |
| Compile-time JSON, SQL or research fixture | Root lint/compile, derived from `include_str!`/`include_bytes!` inputs |
| Python/JS/C/CUDA source or respective dependencies | Corresponding CodeQL language; existing component checks |
| Native client / shared GPU headers | Applicable native/CUDA PR packaging |
| Workflow or classifier | The affected workflow; classifier changes exercise every scope |
| Frozen ecbench session or bounds/spec data | Integrity checks and fresh PR correctness replays; exhaustive weekly/manual audit |

Rust lint/compile and Rust packaging use sparse checkouts, then materialize the
exact compile-time inputs discovered from maintained Rust source. This avoids
checking out the multi-gigabyte research archives to compile the crate. Cargo's
declared target paths and their Rust source modules are discovered from the
current manifest and explicit `#[path]`/`include!` source dependencies, so
newly added frozen examples and external modules remain covered by the
all-target gate. Root lint retains `scripts/clippy-maintained.sh`, which selects
maintained examples while the all-target gate compiles frozen sources. The
release builds the six binaries it ships or smoke-tests, while rust-all-targets
continues to check every root target. Rust release builds and ecbench use caches.
Main-branch rolling releases retain their existing full packaging behavior.
The ecbench workflow also starts on every PR/merge group so dependency and
embedded-data changes cannot disappear behind an incomplete trigger allowlist.

## Always-on PR gate shape (2026-10-23)

Four workflows run on every pull request and merge group so that a required
check can never stay pending behind a path filter: `codeql.yml`, `rust-lint.yml`,
`rust-all-targets.yml` and `ecbench.yml` (`validation-gate.yml` and
`large-files.yml` are the two other unfiltered PR workflows and are unchanged).
Each starts with its own `scope` job (a sparse, blobless checkout and
`ci-scope.mjs`, five-minute budget) and every later job decides from the scope
outputs whether it has work.

**Skipped jobs cost no runner; empty matrix shells did.** A job-level `if` cannot
read the `matrix` context, so a matrix job that must stay named
`Analyze (${{ matrix.language }})` had to start a runner for every row, run a
fail-closed step and skip the rest. CodeQL's eight `analyze` rows and the two
`clippy` rows were such shells. They are now literal-named jobs with a
job-level condition of the same form `fmt`, `check` and `harness` already use:
`always() && !cancelled() && (needs.scope.result != 'success' || needs.scope.outputs.<scope> != 'false')`.
A failed scope job therefore runs the job, and its first step ("Require
successful scope classification") fails it; classification never silently
skips validation.

| Check name | Job id | Runs when |
| --- | --- | --- |
| `Analyze (actions)` | `analyze-actions` | `actions` scope |
| `Analyze (c-cpp)` | `analyze-c-cpp` | `c_cpp` scope |
| `Analyze (javascript-typescript)` | `analyze-javascript-typescript` | `javascript` scope |
| `Analyze (python)` | `analyze-python` | `python` scope |
| `Analyze (rust)` | `analyze` (budget exception key) | `rust` scope; needs the database and warm-cache jobs |
| `Build database (rust)`, `Warm cache (rust)` | unchanged | `rust` scope on PR and merge group |
| `rustfmt (changed files)` | `fmt` | `rust_fmt` scope, PR only |
| `clippy (crypto)` | `clippy-crypto` | `rust_root` scope |
| `clippy (cd_attack)` | `clippy-cd-attack` | `rust_cd` scope |
| `check` | `check` | `rust_root` scope |
| `harness`, `evidence_audit_pr` | unchanged | `ecbench_replay` scope |

Every check name is exactly the one the workflow reported before. The only job
ids that changed are those of the former matrix rows; the one that carries a
budget exception (`analyze` in `codeql.yml`) keeps its id. The non-Rust CodeQL
languages now wait only for `scope`; they used to wait for the Rust database
and warm-cache jobs as well. Their PR budget is 25 minutes (60 on push and
schedule, as before), where they previously inherited the Rust job's 60.

**One Rust analysis, not four.** The `rust`, `rust-boolean`, `rust-research` and
`rust-aux` rows all checked out the complete Rust source, ran the same analysis
and, on PRs and merge groups, uploaded the same `/language:rust` category; their
`rust_paths` lists were read only by the non-Rust checkout. They are now one
job. Pushes to main and scheduled scans upload `/language:rust` instead of four
per-row categories. This changes which code-scanning category holds main's
Rust alerts; see "Not verified" below.

**Baseline job counts.** Runner jobs started per event, derived from the job
conditions (not measured from runs):

| Change | Before | After |
| --- | --- | --- |
| README or prose only | 14 | 4 |
| Edit under `src/*.rs` | 20 | 12 |

README-only, before: four scope jobs, eight empty `analyze` shells and two empty
`clippy` shells; after: the four scope jobs. Rust edit, before: four scope jobs,
Build, Warm, eight `analyze` rows (four empty non-Rust shells and four identical
Rust analyses), `fmt`, `clippy (crypto)` plus an empty `clippy (cd_attack)`
shell, `check`, `harness` and `evidence_audit_pr`; after: four scope jobs,
Build, Warm, `Analyze (rust)`, `fmt`, `clippy (crypto)`, `check`, `harness` and
`evidence_audit_pr`. Label events start no runner job in these four workflows.
Run times and queue waits were not re-measured; the 2026-10-11 queue
observations above are the only timing data.

**Why there is still one scope job per workflow.** Sharing a single scope job
across the four workflows was evaluated and not done:

- `needs:` only spans one workflow. Calling a reusable scope workflow from each
  of them still runs one scope job per workflow, and renames that job's check
  to `scope / scope`; a reusable workflow carries no `timeout-minutes`, which
  `check-pr-budgets.py` rejects. Putting the heavy jobs in reusable workflows
  would rename every context to `caller / callee`.
- `workflow_run` attaches checks to the default branch's commit, not the PR
  head.
- A single PR/merge-group gate workflow holding all four sets of jobs would
  save three scope jobs per event, but moves the CodeQL analysis key
  (`codeql.yml:analyze`), the `(path, job id)` budget exceptions, the
  filename-keyed ARC routing variables (`CRYPTO_EKS_WORKFLOWS`, set outside the
  repository), `ci-scope.mjs` filename rules and the 340-line ecbench harness,
  whose steps interleave PR and weekly behavior. None of that can be checked
  without repository settings and run history, so it is left as a follow-up.

The PR comparisons in the scope jobs of `codeql.yml`, `rust-all-targets.yml` and
`ecbench.yml` now use the merge commit's first parent, as `rust-lint.yml`
already did (see the scope contract).

**CodeQL stays on pull requests.** The classifier already limits each language
to its own files, so a README-only PR runs no analysis step and, now, no
analysis job. Moving CodeQL to merge groups, pushes and the weekly schedule
would remove PR annotations and gates the PR on a merge queue whose existence
and rulesets cannot be read from the repository (the `merge_group` trigger is
present, but whether a queue is enabled is a repository setting). A
workflow-level `paths:` filter is not used: it leaves a required check pending
on an irrelevant path. Neither the tests nor this document forbid the change;
it is left for the owner once the queue and ruleset state are confirmed.

**Cargo cache policy.** `Swatinem/rust-cache` steps in the four workflows set
`save-if: ${{ github.ref == 'refs/heads/main' }}`: pull requests and merge
groups restore, and only runs on main save. Caches are scoped to the ref that
saved them (a PR's merge ref, a merge-queue ref), so a cache saved there cannot
be restored by other PRs or by main; saving it only consumes the repository's
cache quota and evicts caches others can read. `rust-all-targets.yml` had no
main-side event, so it gains a daily scheduled run (selects every check) to seed
the cache; `rust-lint.yml` seeds on push to main and `ecbench.yml` on its weekly
schedule. The clippy cache keys start cold once because the job ids changed,
and `rust-all-targets` PRs are cold until the first scheduled run (or a manual
dispatch on main) has saved a cache.

**`ecc2k130-m10-capacity-once.yml` is kept.** It is label-gated and branch-gated,
so every job is skipped and no runner starts on other PRs' label events (one
empty run per label event). It is not unreferenced: the frozen study under
`research/notes/ecc2k130/m10_export_capacity_20260925/` names its path
(`FROZEN.json`, `ci_replay.py`, `dispatch_gate.py`), and
`disable-retired-workflows.yml`, `docs/ci-runner-pool-plan.md` and the ARC
first-wave list name it as well. Deleting it is a decision about that study, not
a CI trim.

**Not verified offline.** This shape was checked with the local tools below, not
against GitHub: that a skipped literal-named job reports as a passing check
under the same name; branch protection, rulesets and merge-queue state; the
effect on code-scanning alert history of the moved analysis keys
(`codeql.yml:analyze-*`) and of main's Rust alerts moving to `/language:rust`;
the values of `CRYPTO_EKS_WORKFLOWS` and `CRYPTO_EKS_HIGHMEM_WORKFLOWS`; cache
sizes and hit rates; and real queue and run times.

## Budgets and evidence coverage

Every job eligible for an ordinary PR or merge group has an explicit execution
budget of at most 25 minutes, except the two exact frozen Lean proof workflows
described below. Longer scientific budgets remain available on
manual dispatch, scheduled work and main-branch recording, and explicitly
requested one-shot label experiments retain their existing limits. Historical
campaigns disabled by their owners remain disabled.

The wider research workflows retain correctness tests and frozen evidence
custody checks on PRs. Fresh paired timing panels, Callgrind/call-count profiles
and their dependent result verification/upload steps run in the full lane.
Immutable frozen panels react to their own snapshots and verifier inputs,
instead of unrelated changes to live source. They keep their existing full
manual measurement commands and archived results. PR validation does not
create evidence commits or rewrite frozen sessions. Obsolete PR jobs cancel;
manual measurements keep their existing cancellation policy.

Historical replay manifests that bind workflow metadata retain their original
bytes and hashes. Their historical custody checks use an isolated view with the
exact frozen workflow revision. All other source and evidence comes from the
current checkout, and native source tests still run there. Existing scientific
source drift and a historical verifier syntax error remain visible failures.
The rotated-admission custody command passes its historical replay, negative
controls and current-interface check. The n53 hash-only custody command checks
the original workflow and source freeze, historical lock-bound preflight,
negative controls and optional archive. Its separate label-gated outcome still
rejects changed released sources, including the current producer drift.

The event-aware [budget checker](scripts/check-pr-budgets.py) rejects missing,
unknown or over-budget timeouts on PR/merge-group jobs, accounting for job
conditions and dependencies. Its fixtures preserve the separate rules for labeled, manual and disabled work.

Besides the jobs named in `NAMED_BUDGET_EXCEPTIONS` in the checker (each exact on
workflow path, job id and timeout, with its reason), the only exceptions are the
`proofs` job in `weak-curve-lean.yml` and the
`evidence` job in `weak-curve-conjectures.yml`, each with a 60-minute timeout
included in its Lean proof evidence seal. These frozen jobs perform a cold
dependency build and fresh kernel replay; their hosted cold runtimes have not
been measured. The checker allows each only while its workflow bytes match the
hash in its evidence seal and that seal retains its reviewed hash. Any change
to those bytes or the job identity makes the exception fail closed. Updating
either timeout requires new proof evidence and a separate review. These
exceptions do not permit other long jobs or justify altering scientific
workloads.

| Lane | Execution budget |
| --- | --- |
| Scope / syntax / changed-file format | Five minutes |
| Each clippy crate | 15 minutes |
| Root all-target compile | 25 minutes |
| CodeQL actions, c-cpp, javascript-typescript, python (PR/merge group) | 25 minutes (60 on push/schedule) |
| Rust CodeQL database build | 25 minutes |
| Rust CodeQL cache warming (`warm-rust-cache`, named exception) | 45 minutes |
| Rust CodeQL queries (`analyze`, named exception) | 60 minutes |
| Each PR or merge-group packaging component | 25 minutes |
| ecbench PR harness (`harness`, named exception) | 45 minutes; the `evidence_audit_pr` job is 25, with a 12-minute integrity step |
| ecbench weekly/manual exhaustive audit | Existing 300-minute scientific budget |

The PR Rust CodeQL lane runs in three jobs that hand one database forward:
build the complete database, evaluate the type-inference diagnostic query to
completion and bundle the trimmed evaluation cache, then run the full retained
query configuration against that cache. Type inference is about two thirds of
query evaluation and every query depends on it, so splitting the query suite
does not shorten a job; and an interrupted evaluation leaves no reusable cache,
so each phase must finish in its own budget. Measured on a 4-core x86 host
(CodeQL 2.27.2): the warming query took 2,111 s cold, and the full 37-query
suite then took 438 s with identical results, against about 2,700 s cold.
The query job keeps its 60-minute named exception; with the warmed cache it
runs far below it. Main, scheduled and manual scans keep the single job.

The ecbench PR lane keeps unit/integration tests, session immutability,
hash/seal/answer validation, bounds/frontier validation, fresh pinned smoke
sessions, and claim-checker controls. For relevant source or evidence changes,
it audits every committed session with `--replay 0`: hashes, seals, plans and
recorded answers are still checked, while expensive historical solvers are not
rerun on a PR. Fresh bounded smoke sessions retain `--replay 8`, and unit,
integration and claim controls exercise current code. Prose-only changes skip the ecbench harness. These are bounded CI diagnostics: they do **not** certify exhaustive
independent replay or change any existing claim or frozen evidence.

The weekly Sunday/manual lane retains all-run historical independent receipts,
frozen claim reconstruction, and the complete all-session replay. A timeout in
either lane fails; no command turns failures into passes. The weekly lane has
no automatic PR cancellation and uses the same scientific validation as before.

The budgets are limits, not measured post-change completion times. Actual
CodeQL, cross-platform packaging and ecbench sample times must be checked after
deployment. The existing public queue data also indicates account-wide runner
congestion, which local workflow changes cannot immediately clear. Authenticated default-branch metadata currently reports no branch protection
or required status contexts. This change preserves check names and changes no
repository settings.

## Local validation

```sh
node --test .github/scripts/test-ci-scope.mjs
node --test .github/scripts/test-frozen-workflow-check.mjs
python3 -B scripts/test_workflows_parse.py -v
python3 -B .github/scripts/test-pr-budgets.py -v
python3 -B .github/scripts/check-pr-budgets.py
actionlint .github/workflows/{codeql,ecbench,release,rust-all-targets,rust-lint,workflows-parse}.yml
```

`workflow-lint` also checks all workflow GitHub Actions expressions, triggers
and job semantics with SHA-256 verified actionlint 1.7.12. Its YAML structure
check keeps its existing job name. `workflows-parse` owns YAML regression
fixtures, the historical-stage manual-only rule, classifier fixtures and the
PR budget contract. Both jobs use sparse checkouts of the metadata they read.
Frozen-workflow custody fixtures check the pinned manifest and workflow, current
source retention, altered or deleted evidence, failure propagation and temporary
view cleanup.
The status dashboard schedule runs every ten minutes, respecting GitHub's
five-minute minimum, and disabled historical campaigns use explicit conditions
that remain false while passing the semantic linter.

The classifier tests exercise real git history, dependencies, unrelated prose,
workflow changes, embedded data, renames/deletions, newline-containing paths,
more than 300 changed files, and fail-closed comparisons. No cryptographic
algorithm, scientific result, or frozen session is modified by this change.
