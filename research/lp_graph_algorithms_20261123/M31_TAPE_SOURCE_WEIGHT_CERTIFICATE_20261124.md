# M31 projection-tape source-weight certificate

2026-11-24; deterministic fixed-data reference [pcc:unverified]. Scope notice #5122 comment 7143921327. Native owners remain #5120 and #5076; no production edit.

## Exact interface and proof

At #5120 head 573d704563ed1375bd4cfe2fd8b771f46861c6a1, graph/tape.rs blob 9ea7bc201d665629a2d124779896d47924d4dd12 constructs a triangular row transform. With original LP matrix L, small matrix S, RHS b and retained dependent indices J, write the forward recurrence as v_i = scale_i (u_i - sum_j a_ij v_j), where all j are earlier pivot indices. Let T be that invertible triangular transform (all scales nonzero) and K = select_J T. Then K L = 0; moreover K[:,J] = I because no dependent index is ever used as a pivot and each dependent scale is 1. Thus K has exactly m-rank(L) independent rows, spanning the full LP left kernel. This proves completeness of the *projection* operator, not full rank of K S.

The existing apply_transpose computes K^T y internally, in its original array, then multiplies by S^T and discards that array. Expose the intermediate as a checked source-weight lift, ideally a shared private helper used by both functions. For any projected functional y, lift w=K^T y and check L^T w=0, S^T w=(K S)^T y, and b^T w=y^T K b on original rows. If claiming target recovery additionally verify all non-target unknown coefficients cancel, target coefficient is nonzero, and the derived scalar passes [x]G=Q. Output source IDs from original row positions; do not assume IDs contiguous or sorted. Keep the existing source-order fix #5118, group admission and cap semantics. Do not use a bilinear dot check alone as evidence of full operator correctness.

One lift costs O(m+T_n) field operations/index traversal and O(m) dense scratch, where T_n is tape dependency count. Direct replay adds O(sum of original row supports on nonzero weights). Materializing every basis witness is not necessary to certify a single projected dependency. A sparse reverse skip is exact when the *current accumulated adjoint* is zero; a zero initial seed does not imply a zero later adjoint. Explicit stored K costs O(nnz(K)) to multiply; neither representation is automatically faster.

## Executed fixed reference

Input: #5076 head 584b4baf7944f1e3333c507e2bfc947dbdbc0987, research/lp_graph_algorithms_20261123/evidence/native-m31-capture-20261124/raw/m31/prefixes/first-512.jsonl, verified Git blob f1b02205745a74d1771578a68f0f1f435f4a8823. Same established curve p=2147483647, a=847514846, b=2087444724, r=2147446831, h=1, G=(625037715,2100627251), Q=(976979610,1228979761); historical source seed 0x7033315f6d340101 preserved. All arithmetic below over F_r. No new PRNG, random source, replicate, collection or timing; reused fixed public source. Original producer binary/rejection trace gaps remain.

Executed connector-side JS BigInt reference, not a native Rust test. Read all 512 JSONL rows, build least-column LP echelon in original input order, retain each pivot's normalized LP and small-plus-RHS maps, and record dependencies and scales as in tape.rs. For each dependent index k, reverse-accumulate from unit y=e_k. Replay directly on original rows, combining small and RHS as separate coordinates; compare to independently forward-eliminated small-plus-RHS map and require LP map empty. For each lift check coefficients at every dependent source position equal Kronecker delta. Also compare a single combined lift for y_k=2k+3 (zero-based k; deterministic not randomized) against a weighted sum of every explicit basis witness, then directly replay it. No discrete-log recovery attempted.

| Quantity | Exact result |
|---|---:|
| Original rows | 512 |
| LP rank / dependent rows | 157 / 355 |
| Dependency edges / LP coefficient visits in preparation | 366 / 485 |
| Basis direct LP/small/RHS replay checks | 355/355 |
| Dependent-position identity checks | 126025/126025 |
| Total explicit basis source weights / max per witness | 732 / 8 |
| Total explicit small-plus-RHS nonzeros / max per witness | 2853 / 20 |
| LP / small-plus-RHS terms visited in direct basis replays | 756 / 3636 |
| Nonzero-adjoint dependency visits over all basis lifts | 459 |
| Combined lift source weight nonzeros | 479 |
| Combined lift dependency visits, all / nonzero-adjoint | 366 / 359 |
| Combined replay small-plus-RHS nonzeros | 487 |

Combined lift and original replay equal the explicit combination exactly. No timings; the 366 edges vs 732 explicit basis weights exclude vector setup/scaling/output and cannot establish a speedup. The prefix is not the entire 613-row admitted stream or 645-row raw capture. This is fixed reference correctness, not independent validation or full projected rank.

## Handoff and reproducibility boundary

Native next step in #5120: factor the existing reverse loop into an internal lift helper, expose source-ID coefficients for a single final solver witness, preserve deterministic order and charge scratch allocation, replay, refusals and final verification. Regression: all dependent-coordinate unit lifts on the pinned admitted prefix plus original planted controls, source-ID permutations and zero-adjoint cancellation, with an independent forward-elimination oracle. No native build or production edit here. Existing source-pin/main and protected-ledger reconciliation remains owner work. Keep M31, experiment-seeds/v1, parallel-worker/resource, publication/no-merge/no-paid-compute and stop/impact instructions.

Read AGENTS, ACTIVE, main and branch Nov24 journals, graph catalog/SOURCES, current #5120 tape/compare/admission, #5122 prior reports and #5076/#5118/#4905 metadata; bounded open graph-rank PR search. KB MCP/local wrapper, local checkout, Rust/Cargo, Conductor and independent worker tools unavailable; primary repo source/catalog fallback; literature search returned no results, no novelty claim. One coordinator, zero workers, no private global lease/job census; no CPU/RSS/phase timing recorded. No new seeds, paid job, CI dispatch or merge.

Impact score: 4/10 — Established full LP projection-basis completeness and exact source-weight replay on all 355 dependent rows of the frozen 512-row M31 prefix, including a combined lift. Native API qualification and measured costs remain open.
