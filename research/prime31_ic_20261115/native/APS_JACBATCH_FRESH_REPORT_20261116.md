# Prime31 APS exact-source Jacobian batch-normalized walk: fresh 24-Q Rust report

**2026-11-16, source-specific Ubuntu 24.04 GitHub Actions Rust 1.100.0 release, [pcc:unverified].**

## Actual outcome

[GitHub Actions run 43854189527](https://github.com/isogeny-labs/crypto/actions/runs/43854189527), job `133254123437`, **passed**. Executed source commit `4278d5d6c081fcb9ceb8eecd76c15bfa3a9338fb`, Rust example blob SHA `4bc34b239c18ccaf36f8bcc5c42baae9537a88ef`. Focused Rust release example unit tests, all reusable F4/given-f correctness gates and `--jacbatch-confirm` passed. The complete developer-execution stdout (timestamps stripped, no case selection or rewriting) is frozen in [APS_JACBATCH_FRESH_CI_20261116.json](APS_JACBATCH_FRESH_CI_20261116.json). Earlier preregistered protocol [APS_JACBATCH_PROTOCOL_20261116.md](APS_JACBATCH_PROTOCOL_20261116.md) was committed at `7a2ff0f2` **before implementation and data**.

| Complete single-target cold Rust cost, summed across 24 new public Q | Affine walk + x-fibre | Jacobian mixed add + 256-batch affine normalization | Jacobian A/A repeat |
| --- | ---: | ---: | ---: |
| Correct, independently replayed unknown scalars | 24/24 | **24/24** | 24/24 |
| Fully cold in-process wall (ms) | 171.151856 | **100.913742** | 98.817670 |
| Paid public labelled source construction (ms) | 101.025610 | **37.137945** | 36.251889 |
| Algebraic S3 pair-quadratics | 803,712 | 803,712 | 803,712 |
| Distinct public target/attempt/accepted-relation transcript | Reference | Exactly identical | Exactly identical |

The candidate's fully cold cost is **0.5896152×** the matched affine control, i.e. **41.04% less runtime / 1.696× faster** within this particular development run. Same-input public source generation is **2.72× faster** (source cost ratio **0.3676092**); absolute difference **63.887665 ms** of paid labelled source work across 24 Q. All **24/24 paired targets** improved. A/A batch-repeat drift is **2.077%** in the aggregate. The predeclared developer gate (correctness 24/24×3, ≤0.90 cost ratio, ≤5% A/A drift, ≥18/24 paired wins, identical full algebraic transcripts) passed. The total wall saving **70.238114 ms** is not identical to the source saving because other full-cold phases and timing noise are included.

These are **source-pinned producer-development observations, not independent PCC-signed measurements, isolated multi-host science, or world-fastest comparisons**. Never mix them numerically with the prior `S3_XFIBRE_MATCHED` 361ms or 384-profile 278ms runs: those used different Q sets and runner/load conditions.

## Rust method and correctness

Original ordinary prime-order curve, not a deliberately weak choice:

    p = 2147483647
    E: y² = x³ + 847514846*x + 2087444724 (mod p)
    n = #E(Fp) = 2147446831 (prime), cofactor 1
    G = (625037715,2100627251)
    source partitions = [384,384,16384], 6-round target cap

Reuse the existing `prime31_algebraic_ic::{Arithmetic, PRIME31}` and published APS-style `R_i=a_i G+b_iQ` one-relation S3 recovery, including Mersenne arithmetic and the previous x-only discriminant factorization `disc_Z S3(X,Y,Z)=16f(X)f(Y)`. The **only** optimization here is generation of *exactly the same* labelled source points.

For a Jacobian coordinate `(X,Y,Z)`, store (x=X/Z^2,y=Y/Z^3). A mixed step adds the public affine table point in projective coordinates, explicitly branching on doubling and cancellation. Instead of an individual modular inversion at each step, the implementation collects at most **256 nonidentity projective points**, computes one prefix-product batch inverse and normalizes every point with `Z_i^{-1}`. The arithmetic is traditional projective elliptic-curve arithmetic plus Montgomery batch inversion; no novel group law or non-generic DLP attack is asserted.

The source PRNG, 64 public steps, labels (a,b), global x-deduplication, bounded retry/identity reseed and exact ordering are unchanged. A deferred transition prevents consumption of an extra PRNG word at batch boundaries. Rust correctness tests independently compare 209 public mixed-add steps, doubling/cancellation, batch normalization, and all labelled points across two smaller public source profiles `[32,48,64]` and `[384,384,2048]`. The complete 24-Q panel then requires identical per-attempt source collision/identity counts, S3 queries, field square roots, candidate signs, algebraic signed relations and recovered scalars. Each of 72 recovered logarithms is checked against the public point using the existing independent caller-side scalar replay; known k never enters the solver.

**Complexity classification:** The complete solver is a faster implementation of the 2018 APS-style one-relation index-calculus *methodology*. The current finite S3 pair search is still **generic meet-in-the-middle equivalent**. The optimization changes cost constants, not the asymptotic ECDLP hardness. It does not beat the strongest external PKM/APS/Groebner frontier or an optimized rho implementation on matched inputs until explicitly tested.

## Reviewed prior work and avoided duplicate dead ends

1. **Complete Rust internal baseline:** #4391's original APS `--unequal-384-confirm`, Mersenne and x-fibre fresh confirmations; new method uses their same exact source and algebraic code. This optimization was not present in the previous registered Rust source gate; it is a standard technique, not a claimed invention.
2. **Non-generic prime-field route:** existing `cryptanalysis/symmetrized_semaev.rs`, #4188 low-arity exact quotient/norm/GCD, merged #4228 and open #4349 certified compact/selected root worker, #4391 `M8_SYMMETRIC_ROOT_RESULT_20261116.md` given-f exhaustive B8 signed-infinity checks. The missing operation is choosing `f(T)\mid H(T)^8` from a natural public Q **without listing candidates**, with exact original-ideal certificates, then a complete logarithm extraction.
3. **Prior negative leads:** #4373 m8 chained-S3 F4 exceeded matrix caps even on positives; #4391 two-anchor m6 six-sum has <4% support ceiling and natural F4 censored; #4383 T=1024 pilot advantage disappeared on a fresh confirmation panel. Do not retune those approaches without a mechanism that changes the obstacle.
4. **Literature:** Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, *Finite Fields and their Applications* 51 (2018), 168–182, DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009); Petit–Kosters–Messeng, *PKC 2016*, DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1); original Semaev 2004. Prior-art review is bounded to available repository notes and publisher records; KB MCP/wrapper unavailable in this runtime. Neither absence of a keyword search hit nor these measurements establishes novelty.

## Ranked next experiments with falsifiable gates

**A. Non-generic selected-root m8/S9 (highest mathematical value).** Use the exact B4/B8 symmetric f validators, implement target-conditioned 3+3+2 Vieta-coordinate root selection from `f(T)|H(T)^8`, and insist on rational roots with explicit certificates in the **original** input ideal and honest infinity/multiplicity handling. Measure B4/B8 positive, natural negative and tampered f before attempting the B31 `C(38,8)=48,903,492` unordered multiset universe. Reject an explicit root catalogue, a hidden group table, or censored F4 as a completed oracle. The target is full cold natural B31 prime31 log recovery and independent useful rank, not given-f validation.

**B. Retune APS partition balance after source arithmetic moved.** Earlier 384/384/16384 was selected with much more expensive affine source generation. Changing the source-to-pair cost ratio shifts the optimal `T_1 T_2` vs `T_3` balance. Predeclare equal expected relation mass and test `[256,256,32768]`, `[320,320,24576]`, `[384,384,16384]`, `[512,512,8192]` on a fresh untouched 24-Q panel, **same Jacobian/x-fibre arithmetic and all-source costs**, with alternate order and A/A. This is purely APS engineering; do not mislabel it a cryptanalytic complexity advance.

**C. Original-curve external controls and independent replay.** Compare this **fully cold** implementation on identical public prime31 targets against matched PKM/APS polynomial-root implementations and the fastest available mature CAS, plus optimized negation-map rho separately as the ECDLP security baseline, with failures and memory included. Execute independent L2/PCC validation before accepting any leaderboard or fastest-world claim. No paid compute was launched in this session; prior monthly budget report showed a binding limit.

## Limitations / merge

The exact source-specific focused Rust Actions run passed; broader repository-wide check, artifact, PCC, review and independent scientific validation must be considered separately. This report is part of existing PR #4391 and does not authorize a manual merge, gate bypass, rewrite or leaderboard promotion.
