# N83 prebuilt K=16,627 base: bounded natural m=4 query

The [frozen P2/A1/B/A2 run](reports/run-20261018T2052Z/RUN_REPORT.md) completed on the user-selected SSH agent. P2 confirmed the exact retained base and public target. A1 and A2 each produced the same independently replayed, rank-one **constructed-selector** relation. B, the sole unpinned natural public query, reached its internal 1,790-second budget and wrote `UNKNOWN_budget`; it did not write an attempt or relation receipt. The hard 4 GiB cgroup recorded no OOM. The B result is censored and cannot estimate natural relation yield or select a factor base.

| Requested work | State | Evidence or exact gap |
|:--|:--|:--|
| Prebuild and retain a symmetric N83 base | verified for one public K=16,627 signed-Frobenius point set | [construction and replay](../n83_k16627_prebuild_20261017/RUN_REPORT.md), [object identity](PROTOCOL.md) |
| Use that base for a natural m=4 public query | bounded execution complete, result censored | [B cap](spot-arms/b-natural-v2/run/cap.json), [run report](reports/run-20261018T2052Z/RUN_REPORT.md) |
| Functional controls and independent relation replay | verified for A1/A2 constructed selectors | [A1 replay](spot-replay/a1.json), [A2 replay](spot-replay/a2.json), changed-x/row rejections in [raw manifest](spot-raw-files.sha256) |
| Choose a high-quality symmetric factor base | open | common natural-query corpus, useful rank, full cold and setup-reuse accounting absent |
| Evaluate m=2 and m=3 | open | native accepted-event/base or partial-relation policies and measured natural rank/cost absent |
| Compare prebuilt with on-demand generation | open | same accepted point set, native revision, query corpus and explicit reuse counts not yet paired |

The observed worker walls are P2 43.737231 s, A1 60.034319 s, B 1,790.010187 s at the internal budget, and A2 68.854539 s. The different workloads and the delayed A2 prevent a runtime ratio. `selected_best_total_runtime` and complete IC runtime remain null. No result here moves the verified index-calculus baseline or its gap to matched rho.

The next measured iteration is a native census of the **direct-subgroup** points whose x coordinates lie in the 18-dimensional polynomial subspace. It tests whether at least 16,627 disjoint full signed-Frobenius orbits can be chosen with a compact source-x representation. If that size premise holds, export and replay the exact new closure, then compare natural relation/rank and full costs against this K=16,627 base on one frozen public corpus. Separately add a durable natural-oracle progress observer before another long capped m=4 search, so a cap can locate completed solver work. The [open problem](../../problems/n83-prebuilt-base-quality-m2-m4.md) retains the m=2/m=3 and full-cost obligations.

The [run diagram](reports/run-20261018T2052Z/sequence.svg) and PDFs accompany this result. The canonical index-calculus leaderboard, progress timeline and curve graphs were checked and left unchanged because the run supplies no admitted natural relation or end-to-end comparison.
