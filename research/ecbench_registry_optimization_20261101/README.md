# Registry-wide ECBench optimization loop

This workstream turns “optimize every implementation” into a fail-closed native
loop over the live ECBench registry. It covers configuration search and
source-level implementation changes, but it does not turn adaptive search data
into a speed claim.

## Current state

- Current source registry: **78 methods**.
- Nested algebraic inventory: **11 system solvers**, **9 PDP3 engines**, and
  **4 prime PKM tower engines**, including matrix/inherited F4, matrix and
  signature F5, both F6-IC closures, SAT-CDCL, both FES/Gray-code routes,
  exhaustive search, all four root-finding table variants, and PKM geometric
  enumeration.
- Native implementation: `ecbench optimize inventory` and `ecbench optimize
  audit`.
- Complete development roster: **111 specs**, **78/78 methods** and **24/24
  nested algorithms**, with no structural errors or seed leakage. The native
  `--require-all` audit is admitted.
- Fresh diagnostic execution: **35 L0 sessions**, **1,005 records**, **952
  verified outcomes**, and **782/782 eligible deterministic measured replays**
  reproduced with zero audit problems.
- Actual execution coverage: **72/72 runnable native methods**, **11/11 system
  solvers**, **9/9 PDP3 engines**, and **4/4 PKM tower engines**. The remaining
  six registry entries are external CM wrappers and require a real hash-bound
  backend.
- Exact merged-head replay at source `d955159129` and release-binary SHA-256
  `146c4892...d35eb` reproduces all **782/782** eligible measured runs across
  all 35 admitted sessions. Its receipts and aggregate are under
  [`execution/current-merged-head-replay/`](execution/current-merged-head-replay/);
  the earlier 33-session replay is retained separately.
- The pair-index F6 and four PKM tower panels verify. The first pair-index
  replay also exposed and retained an accounting-cache regression; resetting
  the cache at the independent-run boundary makes the corrected session
  reproduce 16/16.
- Newer `main` also contains a standalone N83 chained-S3 meet-in-the-middle
  lookup. Its separate 50-record experiment reports zero soundness flags and
  successful public-target lookup refutations through `m = 8`, but it is not
  an ECBench-selectable method or nested engine and is therefore not counted
  in the 78+24 claim. Wiring and pricing that decomposition stage end to end is
  an explicit coverage gap, not an assumed result.
- The same reconciliation registered a distinct `chained-x` decomposition
  oracle in `ic bench`. A matched Koblitz n=13 diagnostic now covers all three
  engines at `m = 2, 3` and inherited F4 at `m = 4`: inherited F4 remains
  24.71x, 144.09x, and 794.37x matched rho, respectively. Both `m = 4`
  matrix-engine cells censored. This is end-to-end IC-framework evidence, but
  it still lacks an ECBench spec/receipt and is not part of 78+24.
- No leaderboard row changes. These are one-target/four-round development
  diagnostics, not L2 confirmatory holdouts.
- Confirmatory execution is blocked because this managed runtime has no ready
  checkpoint upload identity. EKS submission is not attempted without the
  mandatory resumable-output gate.
- All 37 raw `records.jsonl` payloads remain local and untracked under the
  repository artifact policy: the uploader found them, but this runtime has
  neither the AWS CLI nor an artifact identity. Their hash-bound audit receipts
  and compact comparisons are tracked; they are not promotion evidence until
  hydrated.

The admitted roster receipt is
[`execution/full-development-audit.json`](execution/full-development-audit.json).
The earlier [`execution/development-audit.json`](execution/development-audit.json)
is retained as the pre-expansion negative control. The execution totals,
bottlenecks, and diagnostic comparisons are in
[`execution/SMOKE.md`](execution/SMOKE.md).
The separate chained-x addendum is
[`execution/chained-x-post-main-20261101/SUMMARY.json`](execution/chained-x-post-main-20261101/SUMMARY.json).
The comprehensive cross-campaign frontier, algorithm history, and bottleneck
report remains
[`../ecbench_e2e_frontier_20261031/REPORT.pdf`](../ecbench_e2e_frontier_20261031/REPORT.pdf);
this workstream is its current-source registry and execution addendum.

## Native commands

```text
ecbench optimize inventory
ecbench optimize inventory --json

ecbench optimize audit \
  --development ../ecbench_e2e_frontier_20261031/specs/shards/*.json \
                specs/development/*.json \
  --require-all
```

`inventory` is derived from `methods::registry()` at runtime. The optimizer has
an explicit policy row for every method and a unit test compares the two sets
for exact equality. Registering a new method without choosing its search,
accounting and claim scopes fails closed.

This boundary is exhaustive for implementations exposed through ECBench, not
for every research example in the repository. A standalone example must be
wired into the registry (or into an explicitly audited nested-engine list)
before it can inherit the all-implementations coverage statement.

`audit` validates method parameters through the normal registry resolver and
checks:

- complete development coverage when `--require-all` is selected;
- the 11 solver backends, nine PDP3 engines, and four PKM tower engines
  separately;
- at least four balanced development rounds and six balanced holdout rounds;
- a warm-up, a reference, a baseline/candidate and an exact A/A duplicate;
- public targets and L2-or-better isolation in holdout;
- disjoint development/holdout target and measurement seeds;
- fixed censoring, resource, seed, external-binding and pricing parameters
  between same-method baseline/candidate arms.

The complete procedure is in [`PROTOCOL.md`](PROTOCOL.md). Prior work and the
external methodological sources that determined the design are in
[`LITERATURE.md`](LITERATURE.md).

## What “optimization” means here

Every method enters one of five configuration strategies:

| strategy | treatment |
|:--|:--|
| `native_adaptive` | conditional TPE/search with scale rungs; development only |
| `native_grid` | prospectively fixed finite grid or successive halving; development only |
| `implementation_ab` | no legitimate configuration knob; profile and compare source revisions with identical output and counts |
| `external_backend_ab` | hash-bind and compare the supplied native backend |
| `replay_control` | preserve historical identity as a replay/accounting control |

All non-historical methods also remain eligible for source-level A/B work,
including algorithms with configuration knobs. A cap, timeout, seed or target
multiplicity is not treated as a speed knob.

## Evidence boundary

This directory establishes the complete loop and a replay-verified L0
diagnostic boundary. It does **not** establish a new performance frontier. The
Koblitz n=29 generator-orbit arm is the only fresh candidate selected for
confirmatory work; its one-target deterministic-work advantage is not a
leaderboard result. A candidate reaches the leaderboard only after a fresh
public-target holdout, L2 A/A admission, identical correctness/count
fingerprints, complete symmetric pricing, independent replay and the existing
validation gate. Regressions, failures, timeouts, errors and exhausted runs
remain in the record.
