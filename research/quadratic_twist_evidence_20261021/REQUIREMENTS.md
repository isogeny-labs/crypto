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
| Checked Lean implications and proof map | partial | 18 checked theorems, including actual Frobenius point maps and rational lifts; twist-to-minus, summation-lifting and pairing bridges remain open |
| Algebraic identity certificates | verified for three stated identities | IDC1 certificates, 32+16 point replay, false controls, exact Lean source pointers |
| Exact curve/model/subgroup identities | verified within stated representations | 1,921 registered monic models, 27 EC1 representations; unused subgroup choices stay unknown |
| Shared Conductor check | blocked by unavailable tool/configuration | CLI exits 127; shared workflow/state unavailable; no reservation claimed |
| Repository `cargo test --release --lib --locked` | passed on final code and registry | 4,474 passed, 119 existing ignores; prior count-guard and socket-sandbox failures retained in `validation/` |
| Native metadata/backend tests | passed | 39 tests; actual registry/certificate byte replay also checked |
| Source-linked report, graph and inspected PDF | supplied | `reports/native-20261021T150125Z/` |
| General GHS implementation / asymptotic or hardness claim | unresolved, not established | finite examples and conditional algebra do not establish either |

The original request remains broader than the finite evidence. Closing a native
implementation gap or passing CI does not settle general ECDLP hardness.
