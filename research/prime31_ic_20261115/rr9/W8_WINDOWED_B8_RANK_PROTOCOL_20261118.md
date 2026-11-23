# Prospective experiment: B8 algebraic-window factor-base coverage (Prime31, 2026-11-18)

**Existing lane:** [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728); downstream unknown-chart owner [#4692](https://github.com/isogeny-labs/crypto/pull/4692). This protocol is published **before** new native implementation/results. All experimental evidence is `[pcc:unverified]` until independently signed.

## Research hypothesis
A public-only nonenumerative RR9/F4 extractor may be dramatically cheaper with eight weighted slots, each exposing B=8 base points, than with B=31. We can cover the existing B31 factor base using **four overlapping B8 windows**, collecting independent, ordinary known-offset public relations in each window and sharing the globally learned 31 factor logs. This is a *proposed index-calculus architecture*, not yet an efficient algebraic extractor.

Use exactly the unchanged ordinary cofactor-one curve
`p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251)`.
The 31 canonical first rational-x base points and eight signed scalar weights `[1,3,5,7,11,13,17,19]` are unchanged.
Frozen 8-point windows of global factor-base indices: `[0..7], [8..15], [16..23], [23..30]`. The fourth shares index 23 with the third; the union is exactly 31 elements.

## Exact finite-group first moment
For every window whose eight signed slot lists have size 16 (even if their sums collide), the **ordered witness count** averaged over uniformly distributed group targets is exactly `16^8/n = 4294967296/2147446831 = 2.000034289...`. For B31 it is `62^8/n = 101674.2777669...`. This is not a probability of at least one witness, nor a runtime prediction or a bound on Gröbner difficulty. The prior frozen B8 panel had 46/64 natural hits, 115 total witnesses; independently reproduce the new window outcomes rather than assuming that rate transfers.

## Frozen diagnostic gate (generic grading oracle only)
1. Use **original existing Rust** `prime31-algebraic-ic::{Arithmetic,Point,PRIME31,check_curve}`, pinned original source `1df714645aa683cf975bc76fcacb9842cb4780fd`. Reuse the existing B8 signed 4+4 grader design strictly as a truth oracle; no new DLP algorithm is asserted for that grader.
2. For each window, construct all eight signed weighted supports and an exact 4+4 group-sum grader. All input points and target Q must be public; the grader must not receive the known scalar used to create training/target fixtures.
3. Draw deterministic independent nonzero uniform scalars using splitmix64 with **unbiased rejection** and fixed seed `0x7738423442383231 ^ 0x57494e444f575338` for training and independent XOR-distinct seed `0x7738423442383231 ^ 0x5441524745545338` for the 1024-query target panel. For each training trial, publish `R=[k]G` to the oracle (not `k`); a returned eight-slot witness gives a **real linear equation** `sum sign_j w_j log_G(P_{window[j%8]}) = k (mod n)`.
4. Grow a global exact modular RREF through the windows in order. Each window gets a maximum of **128** fresh training trials to reach cumulative ranks 8,16,24,31. Record every attempted, successful, dependent, and independent relation. Fail closed if rank cannot be achieved. Independently verify all recovered 31 factor logs by original group scalar multiplication; no test-known factor logs are passed into elimination.
5. For **1,024** deterministic public test points `Q=[k]G`, use an **independent** sequence of shifted residuals `Q+[alpha]G`; the grader may see residuals and public window support, but not secret `k`. Cycle four windows across attempts, up to **32 offsets** per target. Recover `log Q = sum sign_j w_j log P_i - alpha (mod n)` with no rho/BSGS fallback. Verify all 1024 results by `[log]G==Q` and by independently comparing against known fixture k. Record full setup, all failures, training/rank/online, recovery, group additions, field costs, and wall time.
6. Separate Python BigInt verifier recreates original curve, all 31 base points, signed slot witnesses, modular relations, all 31 log identities, and every target result and checksum. Preserve transcripts and failed/censored outcomes. This is an **oracle-only structural test**, not competitive non-generic index calculus. A complete oracle test is allowed to be slower than BSGS.

## Non-generic extraction and full-cost promotion gates
- Replace **every** generic 4+4 grader with genuinely public-target-only RR9/F4/quotient unknown-x selection at B8, **without** table lookups, planted charts or scalar hints. Test frozen natural B8 positive/negative fibres and every window, including saturation/exception branches, and account for all time/memory.
- Preserve successful rank-31 and 1024-target transcript tests; perform matched same-runner AB/BA whole-program cold/cached comparisons to existing mixed IC, shared BSGS, optimized negation-map rho. Prior shared BSGS 144.049ms / 1024 targets (source [#4714](https://github.com/isogeny-labs/crypto/pull/4714)) is a historical reference, NOT a new measurement.
- Reject any claim that `B8` windowing itself is a non-generic extraction advance or an asymptotic ECDLP result.

## Prior work and novelty boundary
The previous B4/B8 oracle [#4728](https://github.com/isogeny-labs/crypto/pull/4728) supplied true 46/64 B8 natural hits but did not learn 31 logs; [#4714](https://github.com/isogeny-labs/crypto/pull/4714) provided an older complete B31 generic-equivalent solver and matched negative BSGS control; [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns RR9/F4 unknown-chart extraction; [#4722](https://github.com/isogeny-labs/crypto/pull/4722) bounded M6 support; [#4746](https://github.com/isogeny-labs/crypto/pull/4746) withdrew an overoptimistic PKM exponent. Searched the current GitHub code index in `crypto`, `crypto-autoresearcher`, `cryptanalysis` for this specific windowing mechanism: no complete same-curve benchmark located; this is **not** a novelty proof. Prime-field Semaev prior art: Petit–Kosters–Messeng (PKC 2016), Amadori–Pintore–Sala (FFA 2018). The research-KB MCP and local wrapper were inaccessible in this tool session; fallback is named primary PRs, exact files and papers.

No paid compute, infrastructure mutations, merges or secrets. Native CI results must be appended **after** checking the exact run.
