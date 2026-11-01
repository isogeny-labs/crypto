# Registry-wide ECBench optimization loop

This workstream turns “optimize every implementation” into a fail-closed native
loop over the live ECBench registry. It covers configuration search and
source-level implementation changes, but it does not turn adaptive search data
into a speed claim.

## Current state

- Current source registry: **79 methods**.
- Nested algebraic inventory: **11 system solvers**, **9 PDP3 engines**, and
  **4 prime PKM tower engines**, including matrix/inherited F4, matrix and
  signature F5, both F6-IC closures, SAT-CDCL, both FES/Gray-code routes,
  exhaustive search, all four root-finding table variants, and PKM geometric
  enumeration.
- Native implementation: `ecbench optimize inventory` and `ecbench optimize
  audit`.
- Complete development roster: **114 specs**, **79/79 methods** and **24/24
  nested algorithms**, with no structural errors or seed leakage. The native
  `--require-all` audit is admitted.
- Fresh diagnostic execution: **38 L0 sessions**, **1,095 records**, **1,042
  verified outcomes**, and **854/854 eligible deterministic measured replays**
  reproduced with zero audit problems.
- Actual execution coverage: **73/73 runnable native methods**, **11/11 system
  solvers**, **9/9 PDP3 engines**, and **4/4 PKM tower engines**. The remaining
  six registry entries are external CM wrappers and require a real hash-bound
  backend.
- The late-main `ic.prime_orbit_relations_v1` method is now inside the
  fail-closed policy and measured boundary. Its best generic development arm
  uses affine coordinates, eight factor-base orbits, and 1.5 relations/orbit:
  it costs 0.8721 [0.754, 1.008] of the 16-orbit affine baseline, while
  Jacobian costs 1.3351 [1.229, 1.450] times matched affine. It remains 5.4169
  [3.389, 8.658] times negation rho on the generic shape, 4.2011 [3.085,
  5.721] times cubic rho on j=0, and 5.7901 [3.824, 8.768] times quartic rho
  on j=1728. All 72 measured deterministic runs reproduced exactly. These are
  one-target L0 operation comparisons with unpriced work, not wall or
  leaderboard results.
- Exact merged-head replay at source `907d68631e79` and release-binary SHA-256
  `e91c0ec7...f065f65` reproduces all **782/782** eligible measured runs across
  all 35 admitted sessions. Its receipts and aggregate are under
  [`execution/current-ghs-merged-head-replay/`](execution/current-ghs-merged-head-replay/);
  the earlier 35- and 33-session replays are retained separately.
- The pair-index F6 and four PKM tower panels verify. The first pair-index
  replay also exposed and retained an accounting-cache regression; resetting
  the cache at the independent-run boundary makes the corrected session
  reproduce 16/16.
- Newer `main` also contains a standalone N83 chained-S3 meet-in-the-middle
  lookup. Its separate 50-record experiment reports zero soundness flags and
  successful public-target lookup refutations through `m = 8`, but it is not
  an ECBench-selectable method or nested engine and is therefore not counted
  in the 79+24 claim. Wiring and pricing that decomposition stage end to end is
  an explicit coverage gap, not an assumed result.
- The same reconciliation registered a distinct `chained-x` decomposition
  oracle in `ic bench`. A matched Koblitz n=13 diagnostic now covers all three
  engines at `m = 2, 3` and inherited F4 at `m = 4`: inherited F4 remains
  24.71x, 144.09x, and 794.37x matched rho, respectively. Both `m = 4`
  matrix-engine cells censored. This is end-to-end IC-framework evidence, but
  it still lacks an ECBench spec/receipt and is not part of 79+24.
- Current `main` also adds a separate end-to-end GHS experiment outside
  ECBench. It verified IC, rho, and BSGS on all 600 planted logs from 30 toy
  instances at N=12..28. Every priced IC row remains generic-led: the best
  IC/best-generic ratio is 1.91 [1.72, 2.11], and the best IC/rho ratio is
  1.60 [1.25, 2.12]. The n=4 family fit crosses at N about 32 [31, 33], but
  that point is extrapolated and unmeasured. This result does not erase the
  older, differently specified `E_0/F_(q^3)` evidence; it does prevent that
  restricted historical exception from being generalized to the new GHS
  roster. The new experiment has no ECBench receipt and is leaderboard
  ineligible.
- A subsequent skeptic-reviewed package evaluates 12 prime-field IC leads.
  None changes the exponent boundary: sparse/dense/Bochum-style bases match a
  size-matched null at the tested sizes, the characteristic-2 SAT gadget has a
  2.58 conflict slope and 3x worse CPU at its largest cell, retained clauses
  move setup into advice, and the best multiplier-encoding result is a 0.78
  [0.67, 0.92] one-size constant. The ring-linear propagator was not run. The
  surviving exponent questions are PKM at m>=4 and the contractor exponent,
  not a measured speedup.
- R7.4 separately proves and toy-checks that binary-Edwards 2-power-torsion
  symmetrisation has the same algebraic systems after a linear substitution.
  Its pair-table experiment approaches the predicted null after quotienting,
  with finite-size results depending on whether `c` or `c+1` columns are
  charged. No Groebner/F4 oracle or end-to-end ECDLP comparison was run, so it
  adds no frontier or leaderboard row.
- No leaderboard row changes. These are one-target/four-round development
  diagnostics, not L2 confirmatory holdouts.
- Confirmatory execution is blocked because this managed runtime has no ready
  checkpoint upload identity. EKS submission is not attempted without the
  mandatory resumable-output gate.
- The trusted artifact workflow has uploaded, independently downloaded, and
  replayed **34** earlier campaign payloads, replacing them with hash-bound
  manifests. Three older post-merge diagnostic payloads remain local. The
  three new prime-orbit payloads are staged for the same trusted migration in
  this change. Artifact delivery does not promote L0 evidence.

The current admitted roster receipt is
[`execution/full-development-audit-79.json`](execution/full-development-audit-79.json).
The 78-method [`execution/full-development-audit.json`](execution/full-development-audit.json)
and earlier [`execution/development-audit.json`](execution/development-audit.json)
are retained as provenance snapshots. The execution totals,
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
