# Requirement-to-evidence map

| Requested/required item | Status | Evidence or remaining gap |
| --- | --- | --- |
| PR in `aburan28/crypto` | complete | PR #2381, extended on its existing branch |
| Visible executed experiment evidence | verified | frozen historical logs plus native run `run-20261021T150125Z` |
| Preserve historical source, failures, and inputs | verified | original archive unchanged; new failed port/proof attempts retained |
| Full native census, S3 and S4 workloads | verified | 942 models; 4,188,162 triples; 113,342 quadruples; every historical deterministic field matches |
| Full native subfield and trace-zero workloads | verified | original four trace-zero fixtures and subfield example rerun |
| Full native relation collection and solver | verified | same seeds, relation matrices, ranks, attempt counts, and 81 complete logarithms |
| Independent verification of saved/new IC data | verified | tuple-based auditor verifies three headline and 81 complete-log certificates |
| Native controls | verified | false new relation, removed log certificate and false historical census count rejected |
| Historical general readable arguments | preserved | archived report remains available with its original hypotheses |
| Checked Lean implications and proof map | partial overall; explicit short-model and finite pairing bridges plus yield/rank/cost implications verified | 106 named theorems / 127 audited declarations, zero extra trusted; actual point/group/eigenspace, finite Miller table, relation-set/rank transport, whole-group yield and conditional degree-cost statements checked; general geometry and actual useful-yield/regularity/work bridges remain open |
| Algebraic identity certificates | verified for ten stated identities | previous nine unchanged, plus cleared-budget IDC1h1fac46ceb41801ac; 32+16 replay, arithmetic false identities and false source hashes |
| Exact curve/model/subgroup identities | verified within stated representations | 1,921 registered monic models, 27 EC1 representations; unused subgroup choices stay unknown |
| Shared Conductor check | blocked by unavailable tool/configuration | CLI exits 127; shared workflow/state unavailable; no reservation claimed |
| Repository `cargo test --release --lib --locked` | passed on refreshed code and unchanged registry | 4,479 passed, 119 existing ignores; earlier 4,474 receipt and prior failures retained in `validation/` |
| Native metadata/backend tests | passed | earlier 39/40 metadata tests and 22 new isolated-auditor backend tests; exact registry/certificate replay checked |
| Source-linked report, graph and inspected PDF | supplied | `reports/native-20261021T150125Z/` |
| General GHS implementation / asymptotic or hardness claim | unresolved, not established | finite examples and conditional algebra do not establish either |
| Continued falsification experiments (2026-10-22) | verified finite coverage | 74,693,760 signed words, 942 models and 274 subgroup cases; independent Rust arithmetic/DP replay; no component counterexample or rank gain |
| Oddness/square-parameter/replay controls | verified | 14,688 nonzero two-torsion exceptions; 939 square-parameter controls; false coefficient and missing model rejected |
| New source-linked report and inspected PDF | supplied | `reports/mixed-20261022T0718Z/`; earlier reports retained |
| PR-caused stale Jacobian metadata (2026-10-23) | verified repaired | unchanged native producer; base passes and old head fails; new report covers exactly 4,493 models, preserving all 2,572 prior objects and the cost model; altered file rejected |
| Group-map continuation report, diagram and PDF | supplied | `reports/group-map-20261023/`; actual Lean proof and new native certificate receipts in `validation/group-map-20261023/` |
| Positive twist-assisted pairing angle (2026-10-24) | verified finite connection | actual F43 Miller evaluations and all 121 generator pairs checked in Lean; all eleven scalar logs recovered by native finite-field IC, independently checked |
| Auxiliary-point/nondegeneracy census | independently verified finite coverage | 942 models, 274 shared-prime and 649 nonshared-prime cases; 4,616 auxiliary points, 3,312 nonzero targets; degenerate/undefined/wrong-subgroup and corruption controls retained |
| Pairing continuation report, graph and PDF | supplied | `reports/pairing-20261024T0442Z/`; seven-module gate, full native replay, 22 backend and four metadata tests in `validation/pairing-20261024T0442Z/` |
| Uniform geometric pairing reduction / practical advantage | still open / not measured | finite table does not establish general Weil correctness/nondegeneracy; no matched-cost or external-fastest comparison admitted |
| User-requested twist/main relation-yield theorem | verified in stated domains | matched factor bases preserve the exact coefficient set and rank under actual psi; uniform whole-group raw rate1/N and twist/main ratio N/Nt; these are different comparisons |
| Potential benefit from higher useful yield/lower degree | conditional theorem verified | explicit useful yield and dense binomial-degree work model, exact overhead threshold; no actual twist regularity bound or speedup claimed |
| Raw-yield/nonzero-row control | kernel-checked | F5 points of orders9 and3, one versus three zeros over nine coefficients, zero nonzero twist rows modulo3; synthetic budget gain/reversal examples are separate |
| Yield continuation report and proof/identity replay | supplied | `reports/yield-20261024/`, `validation/yield-20261024/`; all four PDF pages inspected |

The fresh library suite after the current-main refresh passes 4,479 tests with
119 existing ignores. The new isolated native auditor's backend suite passes
22 tests; the earlier 39/40 metadata-backend receipts remain historical records.

The original request remains broader than the finite evidence. Closing a native
implementation gap or passing CI does not settle general ECDLP hardness.
