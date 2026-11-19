# Workstream 6: live ECBench solver and implementation coverage

**Status:** source census executed; a native inventory omission was repaired
and verified in the executable; the old 92-spec campaign is correctly rejected
by `--require-all`. No performance timing, common-unit ranking, frontier move,
or leaderboard update is claimed `[pcc:unverified]`.

## Result

The exact release executable reports **111 top-level methods** and **37 nested
algorithms**:

| live class | count | 2026-10-31 comparison |
|:--|--:|--:|
| top-level methods | 111 | 74 |
| system solvers | 21 | 11 |
| PDP3 engines | 12 | not separately closed at 12 |
| PKM-tower engines | 4 | not in the old 11-solver total |

The historical 74/11 statement was correct for its pinned executable. It is not
the current roster. The complete IDs, strategies, parameter roles, accounting
scopes, and claim scopes are retained in
[`receipts/inventory.json`](receipts/inventory.json).

The first live inventory exposed its own omission: `Pdp3Engine::parse` accepts
12 engines, while `optimization::PDP3_ENGINES` listed nine. The missing names
were `rootfind-negfold-batch`, `rootfind-negfold-batch-harvest`, and
`rootfind-negfold-batch-harvest-torsion`. The additive inventory change in
`src/cryptanalysis/ecbench/optimization.rs` makes all three visible and adds
explicit assertions. The pre-fix 111/34 receipt remains beside the repaired
111/37 receipt.

## Campaign closure

Re-auditing all 92 previously frozen shard specs with `--require-all` exits 1:

| requirement | required | covered | missing |
|:--|--:|--:|--:|
| top-level methods | 111 | 20 | 91 |
| nested algorithms | 37 | 17 | 20 |

All 92 specs parsed with zero per-spec structural errors and have no seed
leakage. The failure is exact registry coverage, not malformed old inputs. The
full delta is [`missing-coverage.json`](missing-coverage.json); no missing ID is
assigned a zero cost, timeout, or loss.

## Requested technique map

- F4 and matrix/inherited/M4GB variants are live. The
  Faugère–Lachartre split is inside the F4 kernels, not a separate solver ID.
- F5 is live through `matrix-f5` and PKM `sig-f5`.
- F6 is live through `f6-ic`, `f6-ic-pair`, and PKM `f6`.
- SAT-CDCL and PRIMO-SMT are live. WD-SAT remains an external historical path
  and is blocked by the retained false-UNSAT soundness failure.
- Scalar, GPU, and wide FES plus the two FES-then-* hybrids are live. Their
  Gray-code engines are implementation mechanisms, not separate common-priced
  solver IDs.
- Crossbred covers the registered guess-and-determine/hybrid route.
- Twelve root/table engines are explicit in [`catalog.json`](catalog.json),
  including the three batch/harvest variants the old inventory omitted.

None of Magma, msolve, Singular, FGb, M4GB CLI, FLINT/NTL CLI,
CryptoMiniSat, Kissat, CaDiCaL, or WDSat was on PATH. That blocks identical-input
external controls; it does not make the native implementations fastest.

## Evidence and checks

- [`MANIFEST.json`](MANIFEST.json) binds the base commit, exact additive source
  blob/diff, source closure, compiler, release binary, receipts, and pushed
  integrated source commit `da5760cb903d355c4bcb1128de8a2b0d6aee0801`.
- [`SOURCE_SHA256SUMS`](SOURCE_SHA256SUMS) locks every registry/dispatch source
  used by the census. [`RECEIPT_SHA256SUMS`](RECEIPT_SHA256SUMS) locks both
  pre-fix and repaired receipts.
- [`validate.sh`](validate.sh) checks hashes, exact unique counts, required
  F4/F5/F6/SAT/FES/root identities, the 92-spec audit, the manual nested catalog,
  and the binary digest.
- [`selftest.sh`](selftest.sh) rejected 3/3 corruptions: a missing nested engine,
  an inadmissible audit relabelled admissible, and the incomplete pre-fix
  inventory.
- The targeted library-test build initially exposed two identical Rust test
  names in `plugins.rs`. The weaker duplicate was removed; it was fully
  subsumed by the retained test, which also checks the `Tr(a)=1` refusal. The
  exact inventory unit assertion then passed 1/1 with 7,717 other tests
  filtered. Its source-bound receipt is
  [`receipts/unit-test.txt`](receipts/unit-test.txt).

The next action is not timing a convenient subset. It is building development
specs for every exact missing ID, rerunning `--require-all`, freezing selected
holdouts prospectively, and only then submitting the admitted L2 E2E campaign.

## Sources checked

The shared research-KB executable was unavailable. The fallback sources were
`research/ecbench_e2e_frontier_20261031/{LITERATURE.md,IMPLEMENTATIONS.md,COVERAGE.md,protocol.json}`,
`research/quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md`
sections F0–F5, `docs/ecbench/README.md`, and the exact source files locked in
`SOURCE_SHA256SUMS`. Primary scope used from the literature receipt: F4/F5
native work is component-local until full construction and pipeline costs are
charged; SAT/FES native units are not interchangeable; same-input outputs and
strongest matched rho remain mandatory.
