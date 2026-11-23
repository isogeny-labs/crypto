# Prime31 signed-slot weight-signature audit — prospective protocol (2026-11-19)

## Objective and preregistration

The original [four-window B8 rank-31 generic-grader control](https://github.com/isogeny-labs/crypto/pull/4728) reports **64,737** distinct four-sums from 16^4=65,536 words on each of its two half-lists, versus approximately 1 accidental collision pair expected for a random mapping into a group of order 2,147,446,831. Test whether this deficiency is entirely explained by exact **integer coefficient-vector identities** in the historical positional weights [1,3,5,7,11,13,17,19]. Compare one deliberately chosen weight alternative [1,2,4,8,16,32,64,128] with zero predicted formal 4-slot collisions. This is a test of the *generic grading geometry*, not yet a new index-calculus relation extractor.

Use the unchanged ordinary cofactor-one Prime31 curve y²=x³+847514846x+2087444724 (mod 2^31-1), group order n=2147446831 and G=(625037715,2100627251); original first-rational-x31 base and four fixed windows of [#4728](https://github.com/isogeny-labs/crypto/pull/4728). Reuse the original Rust `prime31-algebraic-ic` `Arithmetic`, `Point`, `PRIME31`, and the same four 8-point windows.

## Exact mathematical control

For a signed half-word u=(u_0,...,u_3), slot j carries a sign epsilon_j and a base index i_j. Define c_i(u)=sum_{j: i_j=i} epsilon_j w_j, an 8-dimensional signed integer coefficient vector. Equal c implies equal elliptic group sums on **every** curve and every base (formal collision); the converse need not hold. Enumerate all 65,536 words per half and group by c. For a fiber size m, count binom(m,2) formal colliding unordered word pairs; report the maximum fiber size. An independent direct integer enumeration before Rust source implementation predicts, per half, historical weights: 64,737 signatures, 904 colliding word-pairs, maximum multiplicity 16; powers-of-two: 65,536 signatures, 0 pairs, maximum 1. These numbers are precommitted assertions for the native Rust experiment, not observed EC measurements.

Compute actual distinct *EC point sums* in the two 4-slot halves for both arms and all four windows, and report distinct weighted x images across 64 unsigned slot leaves. A group-sum count below formal unique signatures indicates a genuine additional EC collision; do not silently suppress it.

## Natural-public matched target panel and controls

Generate **256** public nonidentity R=[k]G test points from an explicit deterministic splitmix64 seed (one panel of 64 residuals per window); candidate membership code receives only R and public slot point lists, not k. Run both arms against **identical** 64 public points within each window. The out-of-band generic 4+4 table is a *grading oracle only*: public test R is a hit if any left half-sum plus right half-sum equals R; no supplied word, planted witness, hidden scalar, lookup table, or membership mask is transmitted to a non-generic extractor. Report per window old/new hits, both hit, old-only, new-only, neither; setup/probe counts and separately measured wall times.

Do not claim independent uniform random sampling beyond the deterministic splitmix64 mapping. Do not infer a yield advantage from the formal collision calculation alone. Do not claim a speed advantage from a generic grader, nor confuse ordered witnesses with distinct target support.

## Eligibility and interpretation gates

1. PASS exact identity explanation only if Rust reproduces **both** 64,737 historical and 65,536 proposed formal signature cardinalities and the formal pair counts.
2. PASS weighted-slot disjointness only if all 64 unsigned slot x images are pairwise distinct for each arm/window; otherwise flag the RR9 resultant encoding as ineligible.
3. Report actual EC half-support and matched paired hit counts *without assuming* that removing formal collisions improves 8-slot target coverage or extraction cost.
4. Existing [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns genuine unknown-index RR9/F4 extraction. No duplicate F4 implementation or substitution of grading tables. A resulting candidate must still pass 46-positive/18-negative B8 unknown-index tests and then 31-rank/1024 blind-target full solver versus same-runner optimized shared BSGS and optimized rho, all costs charged.

## Prior work and literature receipt

- [#4728](https://github.com/isogeny-labs/crypto/pull/4728) original four windows, weights and 64,737 half-support; use exact original Rust arithmetic.
- [#4803](https://github.com/isogeny-labs/crypto/pull/4803) unweighted support/cancellation obstruction; not interchangeable with position-weighted W8.
- [#4722](https://github.com/isogeny-labs/crypto/pull/4722) six-weight formal collision/yield ceiling; this experiment targets **different eight-weight half-list** structures.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) unknown-index algebraic solver owner; [#4793](https://github.com/isogeny-labs/crypto/pull/4793), [#4532](https://github.com/isogeny-labs/crypto/pull/4532) historical full-solver baselines and stronger optimized shared BSGS.
- Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, FFA 51 (2018), DOI 10.1016/j.ffa.2018.01.009. Relevant prime-field index calculus, not claims of novelty here.
- The repository's shared research-KB service and EKS runner are unavailable in this connector-only runtime; original GitHub sources and web publication records are the fallback, **not** an exhaustive novelty audit.

Validation is source-bound GitHub Actions on pinned original arithmetic commit 1df714645aa683cf975bc76fcacb9842cb4780fd and Rust 1.100.0, plus separately computed pure-integer signature controls; [pcc:unverified] until independent signed verification.
