# Prime31 run 56 — hybrid Dickson + y^7 factor-base rank and yield audit

Date: 2026-11-21 PST (2026-11-22 UTC). **Independent local arithmetic/census only. Native Rust source-linked rank implementation committed; its CI not yet verified. No new complete ECDLP solver or native Rust timing.** [pcc:unverified]

## Scope and prior-work audit

Same *ordinary, full prime-order* curve, unchanged throughout: p=2^31-1=2147483647, a=847514846, b=2087444724, n=2147446831. This is not a deliberately weak curve.

Reused:
- Original Rust `research/prime31_ic_20261115/native/lib.rs` (`Arithmetic`, `Base::build`, `check_curve`), and existing depth-10 Dickson2 yield source `examples/prime31_dickson2_yield_census.rs` in this branch.
- Prior run52–54 168-point public signed y^7 cosets, their 56 independently verified horizontal cubic-fibre relations, and the upstream Rust `examples/prime31_coset_cubic_factor.rs` / `pkm_smooth_tower::poly_roots`; no new point arithmetic or rootfinder is proposed.
- Run55's independently enumerated 509-point Dickson base and exact 518,085 pair-sum support.
- Existing *triple-collision* Prime31 work `research/prime31_ic_20261115/native/M3_RANK_EARLY.md` already has rank255/255 at depth9 and a full public-target example; this run is a **different two-summand** exact zero-row/rank and factor-base-union check, not a claim that collision-first index calculus is new.
- Existing overlap: crypto PRs #4692 (public-only SAT/RR9 registered gate), #4812 (14-variable Horner/F3), #4822 (four-variable RR9/Krylov), #4809 (two-summand S3 batch-root extraction), #4819/#4846 (S3 discriminant batch inversion), #4901 (this existing research lane). Separate Fp^2 and genus-4 results are not evidence of an ordinary Prime31 win.
- Literature: Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), 168–182, DOI 10.1016/j.ffa.2018.01.009 (one-relation prime-field method); Yokoyama–Yasuda–Takahashi–Kogure, *Journal of Mathematical Cryptology* 14 (2020), 460–485, DOI 10.1515/jmc-2019-0029 (naive Semaev cost obstruction under statistical assumptions).
- Repository's research-KB MCP unavailable in this runtime; repository code/notes and accessible published sources were used instead. No exhaustive novelty claim.

## Exact arithmetic and independent rank result

For any two signed point pairs with the same sum, the difference of their coefficient vectors is a homogeneous equation in the factor-base discrete logarithms. Trivial `P + (-P) = O` collisions have the **zero** coefficient vector and must *not* inflate rank.

The run55 509-point Dickson2-only support yields:

| Metric | Dickson only | Hybrid Dickson + y^7 |
|---|---:|---:|
| Canonical unsigned base points | 509 | **677** (509 + 168 disjoint x) |
| Signed points | 1,018 | **1,354** |
| Unordered pair records | 518,671 | **917,335** |
| Distinct sums, identity included | 518,085 | **916,469** |
| Inverse-pair records | 509 | **677** |
| Nonidentity pair collision records | 78 | **190** |
| Independent pair-collision rank, over F_n | **13** | **32** |
| Additional independent horizontal cubic-fibre equations | 0 | **56** |
| Combined homogeneous rank | **13** | **88** |
| Residual unknown logarithm coordinates | **496** | **589** |

The 78/190 nonidentity collisions include sign-related duplicates. For the Dickson-only basis they form **13 projective linear-relation classes**, all independent: do not report 78 as the useful rank. For the hybrid, the 32-dimensional pair-collision space and the 56 horizontal-identity space have **zero overlap in rank**: combined rank 88, verified by modular elimination over n.

Every retained C++ pair-sum equality was independently replayed using a separate Python affine EC implementation; all 56 horizontal triples were likewise replayed and checked to be group zero. The modular rank was computed independently from the C++ enumeration using exact mod-n elimination. This is a **verified local arithmetic result**, not signed PCC or native Rust CI.

### Uniform-query limits, not solver timings

The exact uniform *nonzero target* hit rates are `(518085-1)/n = 0.000241255799` and `(916469-1)/n = 0.000426770985` respectively (exclude the identity from both supports). If each randomly shifted target hit supplied **one useful independent new relation**, the optimistic baseline to fill remaining rank would be

- Dickson: `496*n/518084 = 2,055,909.135` trials;
- Hybrid: `589*n/916468 = 1,380,131.312` trials.

These are **conditional best-case rank budgets**, not established expectations of complete solving. Actual useful rank can saturate early, queries can be correlated, one hit may yield multiple decompositions, and the one-relation method can have a different stopping rule. The hybrid reduces this optimistic number of attempts by **32.87%**. Its average relation-oracle invocation must cost **less than 1.48965×** Dickson-only to beat even this limited random-query phase at equal setup/rank assumptions. Naive `B`-proportional probe costs grow by `677/509=1.33006×`, but **the hybrid's two distinct algebraic factor-base charts could cost considerably more**. No public-only S3 or F4 extractor has yet been timed on this union and no speedup is claimed.

### Matched-control check (not strong evidence of special structure)

For 16 preregistered SHA256-seeded *random valid-curve x* bases, each also with 509 unsigned points and all signed lifts, exhaustive independent local two-sum enumeration plus mod-n elimination gave homogeneous rank
`[7,9,8,11,9,13,8,9,9,9,17,8,6,7,13,5]`, average **9.25**, median **9**, range **5–17**. Dickson's rank 13 is *within* that observed range, not evidence of exceptional free-relation concentration. These random bases have no compact Dickson-style membership predicate and are **not** competitor factor bases for the same algebraic solver.

## Exact reproducer and custody

The archived run55 `base.csv` and 168 run53 y^7 points are frozen inputs. New independent scripts:
- `collision_rank.cpp`: complete signed two-sum census and four-index equality receipts, local `g++ -std=c++17 -O3`.
- `verify_collision_rank.py`: separate Python group arithmetic with all 78 nonidentity equalities checked, dense Gauss–Jordan mod n, SHA256 of inputs.
- `build_hybrid_base.py`, `hybrid_census.cpp`, `hybrid_rank.py`: construct 677 distinct points; verify 190 equalities + 56 triple identity equations and rank88.
- `compare_bases.py`: 16 deterministic 509-point random controls, no secret scalars.
- New *original-Rust* checked-in source: `research/prime31_ic_20261115/native/examples/prime31_dickson2_collision_rank.rs`, using existing `Arithmetic` and `Base`, with expected assertions (518085 support, 78/13 collision records/rank, 496 residual coordinates). The focused existing GitHub workflow is extended to compile/run the new Rust example.

Local hybrid public base SHA256: `522747b6cf805c025be4f59d194ec3335a57aff97528fb841cf7c2b4ec2f0162`. Local frozen run55 base SHA256: `fa6a97bb09d2224fb75dfd51aa84fcea0aaa28d4bf08272c2b400827b1ab3f04`.

## Decisions / prioritized next experiments

1. **H56-1: actual hybrid S3 oracle.** Use **existing** native Rust `semaev_projection` and the existing Dickson/`pkm_smooth_tower` membership/rootfind routines; do not develop a C++ attack. Generate fresh public-only nonplanted `aG+bQ` queries and return two *signed* factor-base points, with three labelled branches: Dickson×Dickson, Dickson×coset, coset×coset. Price failed queries, root extraction, group tests and all exceptional curves; compare matched same-runner Dickson-only. Gate: combined oracle mean cost multiplier <1.48965 **and** useful rank gained per query not reduced.
2. **H56-2: scaling/factor-base frontier.** Repeat exact coverage/rank for (B_D\in\{256,509\}) and 0,4,8 y^7 cosets (split-cubic costs included). Vary public coset order/selection on held-out targets. Reject an arm when its fully charged `(expected useful trials × wall cost)` exceeds the prior incumbent.
3. **H56-3: blind F4/RR9.** Return to #4692/#4812/#4822's existing preregistered public-only root extraction and sparse-Krylov matrices; count actual *independent* row increment, not just collision records. No incomplete-degree F4 output may be labelled UNSAT.
4. **H56-4: complete solver gate.** Only after successful public-only extraction, integrate with original Rust factor-base logs, inhomogeneous anchor/target recovery and independent `[k]G=Q` checks for cold 1 and batched 32/128/1024/4096. Fully charge public base generation, precomputation, unsuccessful probes, mod-n LA, fallback and memory. Pair against optimized same-host negation-BSGS, rho, and strongest existing prime-field IC frontier. One free-relation census cannot establish a cheaper complete ECDLP.

**Outcome:** An independently checked 677-point hybrid base has 88 free homogeneous rank and increased exact two-point coverage. The remaining bottleneck is cheap, public-only non-generic extraction; these results do not demonstrate a complete cost advantage.
