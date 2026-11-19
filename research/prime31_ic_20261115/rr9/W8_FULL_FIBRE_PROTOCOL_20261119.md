# Prime31 W8 full-fibre multiplicity study — preregistration (2026-11-19)

**Status at this commit: protocol only; NO measured outcome.** Follow-up on [#4807](https://github.com/isogeny-labs/crypto/pull/4807). Keep the existing original `prime31-algebraic-ic` Rust arithmetic at source `1df714645aa683cf975bc76fcacb9842cb4780fd`; do not alter the curve to be weak.

## Objective and scientific motivation

The earlier held-out 1024-target *binary-hit* experiment recorded 772/1024 for old weights and 902/1024 for powers-of-two (on the same public targets); it did not count all witness multiplicities. The ~12.7pp jump is not explained by a 1.2% gain in four-slot formal signatures. We need **exact total ordered eight-slot witness counts**, not first-hit flags, to differentiate coverage from excessive collisions and compare with a non-dyadic right half. This is a **generic enumerative grading experiment, not an RR9/F4 extractor, no non-generic solver result**.

## Frozen inputs

- Field: p=2147483647; E: y^2=x^3+847514846*x+2087444724; full group order and subgroup order n=2147446831, cofactor=1; G=(625037715,2100627251).
- Canonical base: first 31 rational x with canonical smaller positive y, same as #4807. Windows W0=(0..7), W1=(8..15), W2=(16..23), W3=(23..30).
- Each eight-slot word chooses among 8 points independently and one sign (16 possibilities/slot); multiplicities count all 16^8 **ordered** signed words, without quotienting duplicate EC sums.
- Fixed arms, labels and weights:
  - `OLD`: [1,3,5,7,11,13,17,19] (historical reference)
  - `POW2`: [1,2,4,8,16,32,64,128] (#4807 reference)
  - `NEAR127`: [1,2,4,8,16,32,64,127] (break the exact S+[16]S right-half equality; exploratory)
  - `TERNARY`: [1,3,9,27,81,243,729,2187] (strong existing #4770 complete generic-IC control).
- New seed `0x5748_385F_4845_5432` (ASCII WH8_HET2) for SplitMix64; use unbiased rejection `threshold=(-n) mod n`, reject k=0; generate 1,024 public Q=[k]G points **before** constructing graders; exactly 256 consecutive draws per window. Do not pass k to any grader. The earlier #4807 and #4805 cohorts must not be reused. This deterministic cohort is one single curve and not an across-curve validation.
- Execution order rotated by window `[OLD,POW2,NEAR127,TERNARY]`, then cyclic shifts by window index. This limits order bias for timing; statistical outcomes are order-independent.
- No external oracle, rho/BSGS fallback, planted witness access, or HSM/production keys.

## Exact algorithm and invariants

For each arm/window, enumerate 16^4=65,536 signed four-slot words into two finite-group multi-maps `L(P)` and `H(P)` containing **counts and one sample word**. For every public target R, compute

`m(R) = sum_{P in supp(L)} L(P) * H(R-P)`.

This is exactly the number of ordered signed eight-slot witnesses (not merely a Boolean hit). Record each m(R), summary histograms, sums and mean over positive hits, zero counts, distinct half-sums, build/probe wall clock, and EC addition counts. Assert sum over right half=65,536 and sum over left half=65,536 and that all reported sample words replay via original elliptic-curve arithmetic. Report `m(R)>0` hits and pairwise discordances, including POw2 against NEAR127 and TERNARY. No early exit; no generic-table data becomes an RR9/F4 performance result.

Independent replay: separately coded Python affine arithmetic on *at least 16 selected witness certificates* across windows/arms, checked against public R. Verify source SHA and raw result digest. If execution exceeds the 20-minute CI budget, record censored without inventing results.

## Predeclared comparisons and decision

**Primary**: paired NEAR127 vs POW2 difference in hit count and mean/variance of m(R), on all 1,024 new public targets. **Secondary**: TERNARY versus POW2 and OLD, and comparison of exact witness-multiplicity distribution to Poisson(16^8/n) as descriptive heuristic only. Describe per-window, not just pooled percentages. Treat no observed advantage as negative evidence for alternate right-half weights. Do not select further weights on this cohort and label them held-out.

**Gate for next stage:** only if a chosen arm has materially better hit/yield and manageable multiplicity, implement *public-only, sign-free, index-free* RR9/F4 extraction with strict zero generic-table accesses on frozen 46-positive/18-negative B8 controls in existing owner [#4692](https://github.com/isogeny-labs/crypto/pull/4692). An algebraic solver must return independently verified signed witness indices or a certified no-solution, and charge failed queries, memory and elimination. Then integrate rank31/1024 public log Rust solver on same-runner fastest cached BSGS, rho and strongest IC. This W8 experiment alone can never satisfy the complete-solver gate.

## Prior work and bounded literature review

Reviewed: #4807 (binary-hit and exact scaled-half collision relation), #4805 (weight signatures), #4770 (ternary complete generic-grader IC), #4728 (rank31/1024 generic four-window pipeline), #4692 (nonenumerative F4/remainder-tower owner), #4803 (unweighted support obstruction), #4801/#4779 (interpolation/exterior span obstructions), #4793 and #4532 (full-cost/cached-BSGS controls), and negative determinant/norm leads reported from crypto-autoresearcher SA-04/SA-18. Prime-field Semaev literature: Petit–Kosters–Messeng PKC 2016, Amadori–Pintore–Sala Finite Fields Appl. 2018, Diem On DLP in elliptic curves. No novelty-by-absence claim. `research-kb` MCP not installed in this connector runtime and local wrapper not runnable; bounded GitHub source and literature checks substituted. This is not a Lean-backed claim.

## Validation and disclosure

The result is producer/source-specific until independent signed validation. Keep raw JSON and timings, do not promote to research leaderboard without VALIDATION.json. No remote compute charges beyond ordinary GitHub Actions runner. Preserve the original Prime31 fixture and existing Rust source. Report any CI failures and the exact source revision.
