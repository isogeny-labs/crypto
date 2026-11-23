# Prospective protocol — actual-prime31 matched B31 factor-base full-cost trial
Frozen 2026-11-18 01:24 UTC **before changing the Rust candidate**. [pcc:unverified]

## Question
Does the first-31-rational-x factor base reduce *complete 256 unknown-log* runtime against the Dickson31 basis, rather than merely improving 256-Q membership coverage? This is a generic-equivalent index-calculus-style **control**, not a novel algebraic attack. Curve and subgroup are unchanged, p=2147483647, n=2147446831, y^2=x^3+847514846*x+2087444724, G=(625037715,2100627251), cofactor 1.

## Frozen inputs and independent-variable restriction
Exactly two panels, fixed order [Dickson31, first-x31], same `CORPUS_SEED=0x4d49583450333131`, `RELATION_SEED=0x4241534533315f31`, 256 distinct deterministic caller-only unknown scalars/public Q, 31 columns, same signed 8-leaf arity, same 512-wide batched complement and shared 4A+4A group-index engine. Select first-x points from ascending nonnegative x with a square RHS, taking min(y,p-y), as in prior control #4626; Dickson from `Base::build(PRIME31,6)`. No known scalar exposed to membership or target log recovery; known relation-source coefficients are permitted as standard offline index calculus. Reuse original Rust 1.100 pinned immutable native arithmetic 1df714645aa683cf975bc76fcacb9842cb4780fd.

## End-to-end gates
Build same FourSet, collect independent authentic random known-source relations to rank 31/31 within 128 sources and shifts 0..8, verify 31 source factor-base logs by original-curve scalar multiplication, verify 16 additional fresh source relations, recover all 256 blinded public Q scalars (with fully charged generic fallback if a shift misses), assert original E(Q), log correctness, and 8-leaf relation identity. Original-curve p, n, G all unchanged. All failed searches, batch lanes, setup, field operation counts, source validations, matrix operations, shift attempts, target verification and fallback are charged.

## Recorded outcomes
Capture distinct 4A group image count, formal signatures, 256 first-shift hit masks, all 256 target public Q/k/witness/shift or charged fallback, all 31 base coordinates/recovered logs, all rank rows, per-phase elapsed wall ns, Fp group operation counters and batch statistics. Compare total charged time, per-target time and target attempts. At least one exact-source GitHub Actions native release build/test including 40 original lib tests; collect unmodified raw JSON. If practical rerun on same workflow. No result assumed in advance. A performance difference must be described as sequential single-host diagnostic unless independently calibrated; only compare equal target counts.

## Prior related work and novelty boundaries
- #4629 B27/B29/B31 matched full-cost; #4626 first-x 244/256 vs Dickson 237/256 same target corpus (coverage-only); #4623 original complete B31 baseline; #4618 batched complements; #4596 separately owned matched 1024-Q BSGS/IC; #4604 sign-independent D9 certificate attempt; autoresearcher #3301/#3305 negative result atlas; cryptanalysis #1536 matched random bases.
- Petit–Kosters–Messeng (2016), DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala (2018), DOI 10.1016/j.ffa.2018.01.009; Kudo–Yokota–Takahashi–Yasuda (2018) hybrid Gröbner limitation. Existing implementation enumerates signed group sums, unlike a sign-independent Semaev-based extractor; no new non-generic claim.
- Shared research-KB service is not available to this connector/runtime; cross-repo/bibliography audit is bounded. No paid compute, merging, historical benchmark rewriting or leaderboard promotion.

## Next decision
If first-x wins, quantify full-cost advantage against Dickson only, and explore whether the base improves *algebraic* x-only RR9 extraction. Otherwise retain negative result. **Novelty gate** requires public-residual-only signed-eight extraction via RR9/S9 or K961/S5, with no FourSet and no support/hidden-scalar oracle; this control does not satisfy it. Independently signed PCC remains outstanding.
