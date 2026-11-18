# Frozen experiment: six distinct-weight slots, native Rust Prime31 whole-solve control

**2026-11-18 | prospectively fixed parameters | [pcc:unverified]**

## Goal and classification

Test whether six independent public scalar images of the same B31 original-curve factor base admit a more efficient exact parity relation extractor than the previous eight-slot weighted generic 4+4 table. Reuse the original pinned Rust arithmetic and implement a full 31-column relation-rank / 1,024 blinded-public-target logarithm recovery experiment.

**This implementation deliberately uses a generic group 3+3 meet-in-the-middle XOR-parity oracle. It is an index-calculus-style control, not a new non-generic or asymptotic algorithm. A component timing gain does not establish a cheaper ECDLP solver.**

## Frozen source fixture

- Original, ordinary, cofactor-one curve over Fp, p=2,147,483,647 = 2^31−1.
- y² = x³ + 847514846 x + 2087444724; full prime order n=2,147,446,831; G=(625037715,2100627251).
- Pinned native Rust source revision `1df714645aa683cf975bc76fcacb9842cb4780fd`; Rust 1.100.0, locked Cargo.
- 31 first rational abscissae in increasing order, canonical ordinate min(y,p−y); base construction independent of target scalars.
- Six public, different scalar weights `[1,3,5,7,11,13]`. Every slot uses `[weight_i]P_j`, j=0..30, preserving the 31 unknown base logarithms.
- Build separate exact XOR-parity 3-summand tables (29,791 ordered candidate words each) using the existing EC arithmetic. Count group additions, field multiplications and inversions, successful and failed tests, rank and wall time.
- Training: `TRAIN_SEED=0x4d36545241494e31`, 512 uniformly styled SplitMix public residuals `[a]G`, caller-only `a`, no preselected planted positives. Stop early only on rank=31.
- Final target corpus: `TARGET_SEED=0x4d36544152474554`, exactly 1,024 caller-generated public targets, each with maximum 64 deterministic shifts `Q+[j]G` for 0≤j<64, no generic fallback.
- All relations must be replayed on the *original* source curve. Each base logarithm and target logarithm independently checked with scalar multiplication.
- Exhaustive B2 weighted parity truth-table regression: all 64 ordered words checked against left/right group-algebra convolution.

## Success / failure gates

Full technical success requires rank=31 and **1,024/1,024** final target logarithms recovered from genuinely discovered natural weighted-parity relations, with no generic fallback and exact original-curve arithmetic replay. If either fails or is capped, report rank, throughput and censorship, **never** silently reuse caller secrets or substitute BSGS.

Even if 1,024/1,024 succeeds, this is *generic-equivalent* index-calculus control. A genuinely non-generic win requires replacing the 3+3 EC table with sign-independent RR9/S4/F4, retained rank and full target recovery, and *same-runner* fully charged beating of optimized shared BSGS, optimized negation-map rho and strongest IC frontier. Measurements from unrelated runners are not matched speed ratios. Do not promote to a leaderboard without signed PCC and `VALIDATION.json`.

No change to curve order, generator, target corpus, shift limit, number of slots, weights or frozen seeds after seeing results. Any altered experiment requires a distinct protocol/version.

## Prior leads and novelty audit

- [#4714](https://github.com/isogeny-labs/crypto/pull/4714) — distinct-weight **eight** slots (4+4) attained 7/12 natural odd-parity residuals at B31 but 1,852,467 setup EC adds plus 11,065,988 query adds; *generic* diagnostic; original native Rust reused here.
- [#4716](https://github.com/isogeny-labs/crypto/pull/4716) — independently masked *identical-image* slots gave 0/12, with exact `C(38,8)/n` support ceiling; that theorem does **not** apply to six distinct scalar images.
- [ANCHOR2_M6_F4_CI_REPORT_20261116.md](ANCHOR2_M6_F4_CI_REPORT_20261116.md) — fixed two anchors, six *signed*, unweighted remaining points: F4 censored on known positive and 0/16 natural registered points. Not the current full six distinct-weight experiment.
- [M4_NATURAL_YIELD_ARITY_GATE_20261115.md](M4_NATURAL_YIELD_ARITY_GATE_20261115.md) — support ceiling for same signed factor-base vectors; distinct scalar-slot images require separate analysis.
- [#4623](https://github.com/isogeny-labs/crypto/pull/4623) — merged full-rank 1,024-target generic 4+4 complete rank control. Its 7.658s and historical shared BSGS 170.113ms are not same-host matched evidence.
- [#4596](https://github.com/isogeny-labs/crypto/pull/4596), [#4711](https://github.com/isogeny-labs/crypto/pull/4711) — matched IC/shared BSGS and optimized EC k-sum benchmark lanes. #4711 reports rho still winning all its measured cells.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) — active nonenumerative RR9 shared-coefficient tower experiment; no complete solver improvement currently reported.
- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves* (2004), [ePrint 2004/031](https://eprint.iacr.org/2004/031); Petit–Kosters–Messeng (PKC 2016), [DOI](https://doi.org/10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala (FFA 2018), [DOI](https://doi.org/10.1016/j.ffa.2018.01.009); Diem, *On the discrete logarithm problem in elliptic curves* (2011), [paper](https://www.math.uni-leipzig.de/~diem/preprints/ind-cal-ell-curves.pdf).
- [#4698](https://github.com/isogeny-labs/crypto/pull/4698) is an open paper-level subexponential prime-field ECDLP claim **[pcc:unverified]**, not a practical Prime31 native implementation.
- Shared research-KB MCP tools are not mounted in this execution runtime. Fallback was reading current repository primary files, GitHub PR metadata and published primary literature. This bounded audit does **not** establish exhaustive novelty.

## Required outputs

The source `examples/prime31_weighted_m6_complete.rs`; independent integer-only Python arithmetic replay `verify_prime31_weighted_m6_complete.py`; exact-source sparse checkout GitHub Actions; complete raw original-curve transcript as workflow artifact; scientific report recording each positive and negative result, setup and query counts, attempted queries and rank. Keep each new observation `[pcc:unverified]` absent independently signed standing.

## Distinguishing experiment

For B31, two raw half tables are 31³=29,791 each versus 31⁴=923,521 for the previous eight-slot control: a **31-fold reduction in raw tuple candidates per half**. This is combinatorial, not a measured 31× wall or complete-solve gain. Distinct weighted six-sum support and odd parity yield must be *measured*; naive independent uniform-tuple heuristics do not prove target coverage.
