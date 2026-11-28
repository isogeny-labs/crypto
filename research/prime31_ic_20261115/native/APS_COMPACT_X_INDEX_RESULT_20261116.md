# Unique-x compact factorized APS index: a reproducible 15.3% full-cold improvement on ordinary prime31

2026-11-16. Development execution [pcc:unverified]; **not** independent PCC or a new non-generic ECDLP complexity algorithm.

## Reused code and prospective reproducibility

The existing Rust APS2018-style S3 one-relation solver in `examples/prime31_aps_one_relation.rs` previously stored its third public partition in `HashMap<u64,Vec<usize>>`. Existing `source_walk_profile_jac` enforces **global x-coordinate uniqueness** across the three lists, so every third-list x has exactly one stored index. The new Rust `HashMap<u64,usize>` variant removes per-x vector allocations; the algebraic S3 root formula, batch denominators, public labelled source, witness verification and complete scalar recovery are unchanged. It remains a **generic MITM-equivalent** labelled-point index-calculus-derived method.

The [frozen protocol](APS_COMPACT_X_INDEX_PROTOCOL_20261116.md) was committed at `5cc21506dbb1625d1d572f86a5626d2b244bc07d`, **before** source changes. Rust implementation `6b6a296801e7a9f0681038606a1701287eab822c` and focused workflow update `1a35fb6bf04c8fd8d810e468f0ea763d49000904` were then committed to the existing #4391 research branch. No new repository or language. The source checks unique x and returns an error for duplicate third-partition x; every original signed-relation index/sign/alpha/beta and missed round is asserted identical between the two variants.

**Exact-source [GitHub Actions run 43889692384](https://github.com/isogeny-labs/crypto/actions/runs/43889692384), job 133377222152, SUCCESS**, ran the full existing focused Rust 1.100.0 release workflow, including the compact-index unit test and opt-in `--compact-x-index-confirm`. The complete original 24-target/4-arm JSON, timestamps stripped only, is frozen as [APS_COMPACT_X_INDEX_CI_20261116.json](APS_COMPACT_X_INDEX_CI_20261116.json), evidence commit `fe85736dfc1d0a7a5f38a41deb9add5c2061d4d4`. Original JSON was independently parsed within the producer session; every final k matched caller-known-only answer, each old/new signed relation transcript and pair count matched, and all duration totals / paired wins were recalculated. Independent PCC signing and host-isolated reruns are still outstanding.

## Full cold 24 fresh targets; all 96 verified scalar logs

Same original **ordinary cofactor-one** curve over `p=2147483647=2^31-1`:
`E:y²=x³+847514846x+2087444724` over Fp; prime order `n=2147446831`; `G=(625037715,2100627251)`. Each of four arms per target rebuilds its public point sources or baby table and performs the target scalar replay; no input k enters discovery. Seeds, order and acceptance gates are frozen in the protocol.

| Complete Rust arm (24 fresh public Q) | Correct | Paid cold sum |
| --- | ---: | ---: |
| Previous factorized APS `HashMap<u64,Vec<usize>>` | 24/24 | **88.767780 ms** |
| **New factorized APS `HashMap<u64,usize>`** | **24/24** | **75.172131 ms** |
| New index APS A/A repeat | **24/24** | **74.805224 ms** |
| Independent generic Jacobian BSGS; new 46,340-baby table per Q | **24/24** | **73.817385 ms** |

The compact index is **15.316% cheaper (1.18087× faster)** than the immediately preceding APS source on this fresh 24-Q panel, faster on **24/24** matched cases, and passes the predeclared ≤0.90 aggregate ratio and ≥18 paired wins gate. Candidate A/A drift was **0.4881%**. It remains **1.8353% more costly** than the *same-run* cold BSGS, although new APS beat BSGS on 14/24 individual Q; the user-requested cheapest-complete outcome is therefore *not* met against this comparator.

Algebraic S3 pair count is **736,512 for every APS arm**, with exact relation agreement. Paid source setup times 41.908049 ms old versus 41.853328 ms compact; algebraic query/replay 40.074810 ms old versus 26.529655 ms compact (**33.80% lower**). This supports attributing the measured gain to the third-list index representation rather than source selection or altered relation probability. This is a producer-development matched-host observation, not an independent scientific speed ranking.

## Prior leads and where to spend effort

- Earlier same-run 24-Q BSGS experiment, [APS_VS_BSGS_COLD_RESULT_20261116.md](APS_VS_BSGS_COLD_RESULT_20261116.md), found the previous APS source 21.89% more costly than BSGS. That was a *different prospective panel*, so both original receipts are retained, not silently pooled or substituted for the new matched-panel ratio.
- Prior #4391 Jacobian source, Mersenne field, x-fibre square roots, equal/unequal partitions and negative profile ablations were reused. #4383's selected tuning result failed fresh confirmation; do not overfit or claim a universal win from one panel.
- Original-ideal target-conditioned symmetric S9 / 3+3+2 rational-root extraction remains the highest mathematical priority, using #4228/#4349 and the exact B4/B8 checker. A B8 *given-f* equivalence is not extraction. F4 m8 censored and fixed-two-anchor m6 support ceiling 3.7407% from #4373/#4391 rule out unchanged routes. Crypto-autoresearcher SA-02, SA-14b and SA-15b negative prior leads remain killed under their frozen criteria.
- Prior art: Amadori–Pintore–Sala, FFA 2018, DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009); Petit–Kosters–Messeng, PKC 2016, DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1). Generic pair-list scan remains generic-equivalent. Repository KB and Conductor tools unavailable in the GitHub connector runtime; the review is bounded to GitHub/atlas and primary publications.

## Falsifiable next experiments

1. **Further local index reduction, still an engineering control**: test an exact, collision-audited open-addressed 64-bit packed table keyed by unique source x, maintaining the exact old/new signed relations and accounting for all table construction. Preregister 24 *new* public Q and four-arm old-vs-new-vs-A/A-vs-cold BSGS. A hash shortcut that drops candidates fails correctness. Only claim beating BSGS if matched *complete cold* aggregate cost really wins; do not infer from lookup timing alone.
2. **Independent PCC and reference controls**: replay complete Rust scalars, recalc raw totals independently, include memory/RSS, optimized negation-map rho and mature PKM/APS/F4/CAS controls on matched exact source. Treat cold process startup / multi-target amortization as separate protocols.
3. **Non-generic algorithm**: target-conditioned original-ideal-certified symmetric eight-leaf S9 root extraction, B4/B8 exact positive/negative/infinity without enumerating candidate f, then B31 natural useful relation rank, full cold unknown-scalar solver. Abort on censored F4 or any hidden scan over 48,903,492 B31 f values.

**Decision:** accept this fully measured 15.3% complete-solver engineering improvement into the existing PR evidence, retain the fact that matched BSGS is still modestly ahead, and advance only changed mechanisms rather than repeat discarded prior leads.
