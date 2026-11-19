# CT234 — native two-large-prime graph panel: record audit and next cost gate

19 November 2026 UTC. Existing draft [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715). **Producer-log record audit only, [pcc:unverified].**

## Evidence and exact source
* [CT233 native+independent oracle workflow 44528817307](https://github.com/isogeny-labs/crypto/actions/runs/44528817307) completed both jobs successfully; native job **135576407970**, independent finite-field oracle **135576408543**.
* The hosted checkout resolved **84a62f1190dcf05e11af58b22c4243dff74079d3**. The native job passed weighted signed-cycle mutation controls and emitted **four native records**, then CI grep echoed those same four lines: the echo is **not** a second run.
* Frozen verbatim fields: `ct234_ct233_hosted_panel.jsonl`, with provenance header. Independent deterministic record-only checker: `ct234_two_lp_cost_audit.py`, which additionally checks both source scalar results from an independent F101 point-addition implementation, the graph Euler identity, costs, field counters, LP rank and 11 deliberate corrupted inputs. It cannot check the actual relation matrix without raw relations and is not a signed PCC.
* Prior work inspected: CT231/232 single-LP negative report, CT233 frozen protocol and Rust implementation, concurrent generic 2LP #4587, genus-4 prime-order IC #4693, transfer infrastructure #4697, and merged #3877. Shared KB MCP unavailable in this runtime; no novelty claim. Method: standard double-large-prime elimination, cf. Gaudry–Thomé–Thériault–Diem, Math. Comp. 76 (2007).

## Single hosted native panel: two targets Q37 and Q73, shared setup

| Signed base | Independent base rank | Candidates | Verified E logs | Total two-target candidate | Inline two-target BSGS | Candidate slowdown |
|---:|---:|---:|---|---:|---:|---:|
| 24 | 24/24 | 2,882 | 37, 73 | 96.205044 ms | 4.436 µs | 21,687× |
| 36 | 36/36 | 1,596 | 37, 73 | 53.586254 ms | 3.826 µs | 14,006× |
| 48 | 48/48 | 1,421 | 37, 73 | 47.896874 ms | 3.986 µs | 12,016× |
| 54 | 54/54 | 1,421 | 37, 73 | 47.928841 ms | 3.795 µs | 12,629× |

This is **actual complete toy two-target recovery** for all four bases, not a projection-only or smoothness-only outcome. In particular 24/24 is a correctness advancement relative to CT232's 17/24 failure with single LP. However **none** approaches inline BSGS. The 48-vs-54 apparent candidate-only difference is **31,967 ns, about 0.067%** in this one unisolated execution; it is below any defensible evidence threshold for improvement. Do not report it as a win. The BSGS timings run before the even arms and after the odd arms, with no ABBA replication or uncertainty interval. The candidate is charged for a *shared two-target* setup; it is therefore not a completed cold **single-target** benchmark.

## Mathematical and measurement adjudication

* Let `L=54-basis` be LP vertices plus one fixed sentinel. For the reported signed-edge multigraph, `cycles=edges-(L+1)+components` holds in all four records; the counts alone do not prove an LP dependency because a sign-unbalanced cycle can have full LP rank in characteristic 109.
* 24: 64 edges, 3 components, 36 topological cycles, 28 independent LP pivots and 28 uniquely recovered LP logs. 36: 52 edges, 2 components, 35 topological cycles, 17 pivots and 17 uniquely recovered LP logs. 48: 26 edges, 1 component, 20 cycles and all 6 LP logs recovered. With no graph-edge list published in these records, no stronger claim about component balance or correctness of all relation equations is independently certified.
* All 4 rows have verified-relation count equal to full + single-LP + double-LP. Five charged phases (setup, relations, LA, log verification, recovery) reconcile to the reported complete two-target candidate time with only 250–450 ns residuals. Process VmHWM is **3,636–3,728 KiB** and is whole-process, not isolated algorithm peak; `logical_bytes_peak` excludes allocator overhead. Physical `C169`, `C13a`, `C13b`, `D` costs of the original 695/701 fixture are **not** measured here.
* Distinguish CT233's full-Jacobian composite order from the *prime-order genus-4 Jacobians* in #4693. Their favorable index-calculus/rho ratios cannot be transferred to this source by assumption.

## Next run: cold complete single-target experiment

1. Preserve this record and audit verdict, then run a precommitted separate **Q73-only** native cold IC panel using the unchanged two-LP collector; no Q37 recovery inside the timed region. Run paired ABBA/BAAB order and independently verify source scalar, factor-base logs and exact relation equations.
2. Add genuinely matched optimized negation rho and BSGS, plus a same-size IC frontier reference. Charge group operations/primitives, physical setup/norm projector, every failed relation, linear algebra, recovery and peak memory. Report confidence intervals and producer/independent/PCC level separately.
3. The distinct original 695/701 norm-point reuse route already has **unsigned finite parity** in cryptanalysis; authenticated original-source replay and physical `C169/C13a/C13b/N2_actual/D` require approved custody and compute. Do not bypass S3/EKS budget controls.

Reproduce: `python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct234_two_lp_cost_audit.py --self-test`. No paid compute, merge, or publication to validated solver leaderboards.
