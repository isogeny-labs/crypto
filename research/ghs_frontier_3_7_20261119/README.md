# GHS frontier workstreams 3–7

This study integrates five follow-up workstreams on one source line. The
implementation and evidence contracts are substantially stronger; no new
admitted timing, IC/rho ratio, frontier row, or leaderboard promotion exists
`[pcc:unverified]`.

Draft PR: #4814, stacked on #4757.

## Current disposition

| Lane | Status | What exists | What still blocks a performance result |
|:--|:--|:--|:--|
| WS3 parallel rho | partial | clock-free production-math control, indexed starts, sharded DP table, bounded queue, exact replay, honest record hashes | actual threads, affinity/NUMA policy, D00–D10 closure, L2 wall panel |
| WS4 composition | implemented-unverified | exact 14-arm baseline/R17/R36/R64 closure, disjoint development/holdout protocol, native corruption gate | source-bound admitted executions and holdout |
| WS5 matched GHS | implemented-unverified | 11-size/three-seed protocol, 141 primary runs, 27 diagnostics, 33 calibrations, native result gate | source-bound runner, historical archive hydration, custom-binary L2 route, all grid results |
| WS6 solver coverage | partial | live source-derived method/nested-engine census and fail-closed coverage audit | most registered methods and nested engines are absent from the frozen execution specs |
| WS7 publication | implemented-unverified | native 35-corruption publication authorizer and explicit blocked fixture | 76 upstream evidence gates, including measurements, replay, independent validation, PCC and generated views |
| EKS admission | blocked | read-only route audit and non-admitting offline custom-binary contract | AWS/checkpoint/capacity identity and a trusted native GHS isolation grader |

The standalone L2 contract deliberately exits 3 in normal verification. A
Pod-authored `l2-gate.json` cannot establish its own isolation. The smallest
safe production route is a native GHS method/record adapter inside ECBench,
which can reuse the existing trusted isolation, replay, A/A and S3 readback
path.

## Standing boundary

Until a new admitted session passes every frozen gate, the established
boundary is unchanged:

- generic prime, random-binary, and Koblitz curves remain rho-led in the
  admitted cold one-target cells;
- the restricted GHS-weak `E_0/F_(q^3)` family remains the separate IC-led
  exception;
- the validated prime P20 `N=45` factor base remains 7.95 / 8.24 / 9.62 times
  matched rho across the frozen pricing band;
- historical F5/F6 gains remain solver-local rather than cold ECDLP gains; and
- optimized signed-Frobenius rho retains its 1.246642-fold instruction
  reduction, while isolated L2 wall performance remains unmeasured.

These are historical controls, not outcomes of this study.

## Evidence map

- [`REPORT.md`](REPORT.md) and `REPORT.pdf`: integrated decision report.
- [`SOURCES.md`](SOURCES.md): literature and prior-evidence receipt.
- [`PROTOCOL.md`](PROTOCOL.md): common hypotheses, controls and status terms.
- `ws3_parallel_rho/`: production-math controls and D00–D10 ledger.
- `ws4_composition/`: source-bound composition protocol and validator.
- `ws5_matched_ghs/`: frozen restricted-family grid and native gates.
- `ws6_solver_coverage/`: live implementation/solver census.
- `ws7_validation_publish/`: independent validation/publication authorizer.
- `eks_admission/` and `l2_bridge/`: route evidence and exact unblocker.

