# Prime31 mixed-base cold Rust result — 2026-11-17 UTC

## 2026-11-17 ~04:35Z — two complete results read back and independently replayed [pcc:unverified]

### What was done (commands, files, features)
- Revisited merged #4503 and source-specific [Actions 44017666839](https://github.com/isogeny-labs/crypto/actions/runs/44017666839), now SUCCESS. All 40 native library tests and both new mixed-base tests passed. Downloaded [raw ZIP artifact 14299004221](https://github.com/isogeny-labs/crypto/actions/runs/44017666839/artifacts/14299004221), inspected full two-case JSON (63,968 bytes, SHA256 `029f8948aef3cdaa14e1ae664b7c54d3ce083f0603c0e55b2787aea31d52c53c`) and source SHA.
- Results: two public Q both 62/62 complete unknown-scalar rank after exactly 62 valid signed relations and zero failed/duplicate rows; 61/61 factor-base logs independently checked per Q in Rust. Actual data: Q=(872133043,510239525), k=2062758563, cold2115457523ns; Q=(1841259055,779429965), k=651689658, cold2122593909ns. Four-sum table builds accounted 2051902033ns and2042133375ns. The two total cold calls sum 4238051432ns. This is **not** a cheaper candidate vs historical APS.
- Independently reconstructed first rational-x31 and Dickson-depth6 special points with a separate affine finite-field implementation for a supplementary *producer-side* check, identifying the one shared affine point (3,166645833). Rechecked [k]G=Q for both, all124 original curve signed half-relation sums, all124 full coefficient rows mod2147446831, both full 62-column modular RREF solutions, both target recovered scalars and every recovered base log. No mismatches. These are not PCC-signed.
- Committed report `research/prime31_ic_20261115/mixed_base/MIXED_4P4_IC_RESULT_20261117.md` and compact audit `MIXED_4P4_IC_AUDIT_20261117.json` on a successor branch because #4503 was already merged and deleted; source unchanged.
- Mathematical successor #4505 committed exact mixed formal-net count 381434284809 with Rust tests, its CI was still compiling at latest check. Do not claim CI passed without inspecting final status.

### Decisions made (and why)
- Preserve result as honest **positive correctness / negative complete-cost** index-calculus experiment, producer [pcc:unverified]. No fastest-rho or new non-generic claim. A separate GitHub Actions run (not matched or CPU-isolated) had prior APS ~81.681ms per24Q, so 2.12s per Q here warrants no further identical cold run.
- Focus next on public-R-only nonenumerative RR9/S9 signed norm extraction using merged #4466 Hermite/norm machinery and quotient/rootfinding from #4228/#4349. Only after B4/B8 exact signed-fibre recovery attempt full B31 unknown log and fastest matched rho comparisons. No hidden support enumeration.

### Open threads (unfinished, pick up next time)
- Independently signed PCC producer replay and permanent raw artifact custody, pending. Source-specific theorem CI #44018093813 also pending.
- Implement a cost-feasible nonenumerative RR9/S9 extractor, exploiting mixed bases only if it reduces priced algebraic elimination; compare with strongest native WZ and mature external IC on fresh same-Q panels.

### Errors (what went wrong, how it was handled)
- No code run or arithmetic failure in mixed-base CI, but full cold cost is orders of magnitude above current development single-target incumbent, despite complete answers; report this null performance outcome rather than cherry-pick.
- Prior #4503 was auto-merged before its test completed. Follow-on results are a separate PR and do not rewrite prior source or frozen benchmark.
