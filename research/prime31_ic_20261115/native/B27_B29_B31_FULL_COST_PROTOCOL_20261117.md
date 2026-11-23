# Frozen prospective experiment: actual-prime31 B27/B29/B31 complete-cost sensitivity

Date registered: 2026-11-17 (UTC). This protocol is committed **before Rust changes and measurements**. Draft PR #4629. Scientific standing [pcc:unverified].

## Question and non-novelty restriction

Can reducing the number of actual Dickson depth-six factor-base points from 31 to 29 or 27 improve the **complete, charged** 256-unknown-log generic-equivalent signed-eight-point index-calculus-style Rust pipeline, despite lower factor-base coverage? A negative answer is useful. This experiment **cannot** establish a truly non-generic ECDLP algorithm, a new exponent, or a security claim because the existing 4A+4A HashMap explicitly enumerates group sums.

## Frozen arithmetic and dataset

- Exact ordinary cofactor-one curve over p=2147483647, prime subgroup order n=2147446831, a=847514846, b=2087444724, G=(625037715,2100627251). Original implementation `research/prime31_ic_20261115/native` and the same Rust 1.100 lock file as #4623/#4626; no replacement arithmetic.
- Factor bases: **prefixes B=27,29,31** of `Base::build(PRIME31,6)` at original p. Each x must be distinct and y canonical as built. Not different depth parameters, and do not label them as such. This tests arity/factor-base-size only.
- Target corpus: first 256 distinct generator multiples from the **same frozen #4623 1024-Q seed** `0x4d49583450333131` using unchanged splitmix64 and rejection of repeated k; assert first point (872133043,510239525). The private known-answer k values are held by the test harness and NEVER passed to relation search or rank solver.
- Factor-base log learning sources: original #4623 seed `0x4241534533315f31`, independent known public-source scalars; at most 128 sources and public shifts `a=0..8`. Construct full 31-column Row with zeros in inactive columns, compute rank until B pivots; replay every recovered base log as [ell]G=P. Verify 16 more known-public-source equations when available, otherwise report the exact cap failure. Generic-equivalent source generation is not an algebraic random relation algorithm.
- Each target: attempt residual `Q+[a]G` for a from 0 through 8, using original `recover_batched` and `FourSet::build` index. Verify exact signed-eight-point witness against original curve. Verify recovered discrete log [k]G=Q and with caller-only k, not just subgroup membership. If no witness after nine shifts, run and charge original cold total BSGS fallback (never omit unresolved targets).
- Base sizes and panel order fixed at [27,29,31]; no adaptive tuning on outcome. Capture source-specific SHA, runner and immutable arithmetic, exact 4A cardinalities, signed-count model `signature_count(B,4/8)`, setup, rank learning, 16 controls, target solve+verify and fallbacks, total process phase times, all attempted/exhaustively failed probes, rank and full 256 results, native group / field operation counters. Capture full JSON as durable committed evidence (only after success) and workflow logs.

## Evaluation and comparison gates

- Primary panel metric is **fully charged wall** from building curve and 4A through learning all B factor-base logs and solving/replaying all 256 blinded target logs, including **all failed public shifts** and fallback costs. Print per-phase times, not only a favorable query-only ratio. All three panels run sequentially on the same GitHub-hosted runner; fixed-order wall alone is descriptive, not independently calibrated.
- Pass correctness only if rank B/B and 256/256 unknown logs replay for each arm. Verify frozen B31 256 first-shift coverage 237/256 and 4A cardinality 618885 as control. Assess explicit shifts, negatives, fallbacks and native field costs. If B27/B29 cannot learn rank in 128 sources, record failure rather than silently extend cap.
- Report historical #4623 1024-Q complete generic-equivalent control 7658.159ms and historical #4532 shared BSGS 170.113ms/1024 Q as **different-run, different-cohort** context only, no matched claimed ratios or speedup. Matched fully competitive comparison belongs to #4596.
- No claim of a novel index-calculus solver on success. A genuine next-gate candidate must use public-R-only **sign-independent algebraic** RR9/S9 or K961-S5, not the 4A table, and must pass B8 positive/two negatives plus B31 repeated/exceptional tests, then be inserted into this complete Rust pipeline.

## Prior-lead and literature audit performed before experiment

Read #4623 original 1024 complete generic-equivalent control, #4626 matched B31 Dickson/random/first-x coverage null, #4615 shifts, #4618 Montgomery batch, #4596 matched full-cost owner, #4604 sparse D9 negative certificates, #4624 48-curve synthetic base study, and #4594 lead portfolio; cross-repo autoresearcher #3301/#3305, cryptanalysis #1536 in existing audited references. See published Petit-Kosters-Messeng PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori-Pintore-Sala FFA 2018 DOI 10.1016/j.ffa.2018.01.009. The shared research-KB MCP and repository CLI are unavailable in this tool runtime: this is a bounded review, not a novelty clearance.

Validation status: producer CI only until an independently signed PCC verifier covers source, replay, measurements, literature and no-hidden-advice condition. No paid compute, new generic solver algorithm claims, merge or leaderboard promotion.
