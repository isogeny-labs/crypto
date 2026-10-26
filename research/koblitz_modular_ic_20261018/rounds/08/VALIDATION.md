# Round08 validation

Raw receipts, including failed attempts, are in runs.tar.xz.

| Check | Receipt | Outcome |
|---|---|---|
| Root library | root-lib-tests-01.log | 4405 passed,0 failed,118 ignored |
| Native study | native-tests-02.log | 106 passed,0 failed,1 ignored |
| First replay | replay-01.log | Initial six JSON outputs identical |
| Final archive replay | ARCHIVE_VALIDATION.txt | All eight outputs identical; separate checkers pass |
| Prior round07 | prior-round07-replay-01.log | All50 outputs/equations/Lean replay unchanged |
| Complete norm family | m10-full-line-family.json,m11-fixed-infinity-row.json | Exact support,geometric joins,rank12/1308;nonzero ranks11 |
| Separate relation check | m10b-small-field-check.json,m11b-small-field-check.json | All coordinates of600/660 column images,both signs;all principal sums;separate ranks |
| Miller functions | q6-intrinsic-Kummer-charts.json,q6b-parsed-Miller-check.json | 40 polynomial functions,720 finite valuations,640 pointwise checks |
| Global gluing | q6c-global-chart-gluing.json | Six covering charts per graph;200 exact transition identities and constant roots |
| Parsed gluing check | q6d-parsed-gluing-check.json | 200 parsed identities/inverses,100 curve identities,400 point translations |
| Lean kernel | lean-build-04.log | Full pinned project passes;16 new named results |
| Transitive audit | lean-axioms-02.log | 17 declarations,0 failures,0 native trusts |
| Identity certificates | record-01.log | Two Lean-bound identities,each64 issuer plus128 extra checks |
| Dashboard/site | dashboard-check-01.log,site-tests-01.log | Dashboard current;49 site tests pass |
| Workflow parse | workflow-tests-01.log | Four tests pass; focused25-minute cap unchanged |
| Raw/source hashes | record-raw-hash-check-01.log,record-source-hash-check-01.log | Every ledger binding verified |
| Archive | ARCHIVE_VALIDATION.txt | Extraction/member hashes,fresh replay and deterministic repack |
| Report | build_report.py,report.typ,routes.svg | Editable sources;every PDF page inspected |

Build01 failed because collection type inference needed an explicit Vec.
The first M11 output omitted infinity from its metadata; the corrected
execution preserves the same mathematical rows and ranks. Its original
JSON/log are retained separately. Failed patch applications changed no
source and produced no accepted arithmetic output. Build warnings about
an unused import and a Greek identifier were corrected.

The column population grows with the relation ranks; these results are not
relation-cost or destination-solver measurements. The global Kummer atlas
is over F2^360; effective F2 coordinates and a closed projective embedding
remain open, as does Jacobian identification. All native field semantics,
modular/Picard geometry and the dual-isogeny bridge are not fully formalized
in Lean. Formal scopes are stated in PROOF_MAP.md. Actual kernel build and
transitive audit ran; lean4checker is unavailable. Shared primitives and
same-agent verification are disclosed; no external audit is claimed.

Canonical status narratives, dashboard metadata and the study visual are
updated. Performance/leaderboard cost plots remain unchanged because no
cost was measured. Current-head remote checks must be checked separately;
local results do not represent pending CI as passed.
