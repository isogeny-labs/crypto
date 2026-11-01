# Registry-wide ECBench optimization loop

This workstream turns “optimize every implementation” into a fail-closed native
loop over the live ECBench registry. It covers configuration search and
source-level implementation changes, but it does not turn adaptive search data
into a speed claim.

## Current state

- Current source registry: **78 methods**.
- Nested algebraic inventory: **11 system solvers** and **8 PDP3 engines**,
  including matrix/inherited F4, matrix F5, F6-IC, SAT-CDCL, both FES/Gray-code
  routes, exhaustive search and all four root-finding table variants.
- Native implementation: `ecbench optimize inventory` and `ecbench optimize
  audit`.
- Complete development roster: **109 specs**, **78/78 methods** and **19/19
  nested algorithms**, with no structural errors or seed leakage. The native
  `--require-all` audit is admitted.
- Fresh diagnostic execution: **33 L0 sessions**, **955 records**, **902
  verified outcomes**, and **742/742 eligible deterministic measured replays**
  reproduced with zero audit problems.
- Actual execution coverage: **72/72 runnable native methods**, **11/11 system
  solvers**, and **8/8 PDP3 engines**. The remaining six registry entries are
  external CM wrappers and require a real hash-bound backend.
- After merging current `main`, the inventory and 109-spec audit remained
  byte-identical and a fresh merged-binary audit again reproduced **742/742**
  eligible runs across all 33 admitted sessions. The separate invalid
  pre-fix three-leads negative control is now refused during planning, as
  intended. Receipts are under
  [`execution/current-head-replay/`](execution/current-head-replay/).
- No leaderboard row changes. These are one-target/four-round development
  diagnostics, not L2 confirmatory holdouts.
- Confirmatory execution is blocked because this managed runtime has no ready
  checkpoint upload identity. EKS submission is not attempted without the
  mandatory resumable-output gate.

The admitted roster receipt is
[`execution/full-development-audit.json`](execution/full-development-audit.json).
The earlier [`execution/development-audit.json`](execution/development-audit.json)
is retained as the pre-expansion negative control. The execution totals,
bottlenecks, and diagnostic comparisons are in
[`execution/SMOKE.md`](execution/SMOKE.md).
The comprehensive cross-campaign frontier, algorithm history, and bottleneck
report remains
[`../ecbench_e2e_frontier_20261031/REPORT.pdf`](../ecbench_e2e_frontier_20261031/REPORT.pdf);
this workstream is its current-source registry and execution addendum.

## Native commands

```text
ecbench optimize inventory
ecbench optimize inventory --json

ecbench optimize audit \
  --development specs/development/*.json \
  --holdout specs/holdout/*.json \
  --require-all
```

`inventory` is derived from `methods::registry()` at runtime. The optimizer has
an explicit policy row for every method and a unit test compares the two sets
for exact equality. Registering a new method without choosing its search,
accounting and claim scopes fails closed.

`audit` validates method parameters through the normal registry resolver and
checks:

- complete development coverage when `--require-all` is selected;
- the 11 solver backends and eight PDP3 engines separately;
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
