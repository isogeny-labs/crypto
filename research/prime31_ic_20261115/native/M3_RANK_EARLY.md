# Prime31 m=3 collision-first index-calculus control (2026-11-15)

**Status:** exact producer-local arithmetic and one verified full-log experiment **[pcc:unverified]**. The checked-in Rust native example has **not yet been compiled/executed on exact GitHub head**: this connector runtime has no Rust/Cargo, supported EKS command channel, PCC signer or research-KB MCP. It is not a validated win, and it is not an asymptotically non-generic algorithm. Reusable Rust curve arithmetic, the Dickson Base builder and curve certificate are used by the **new Rust example**; an independent local C++/Python control (not a proposed replacement for the Rust engine) generated the numbers below. All cost counts below are deliberately incomplete.

## Reproduction

- Source: [examples/prime31_m3_collision_ic.rs](examples/prime31_m3_collision_ic.rs)
- Frozen producer observations: [M3_RANK_EARLY.results.json](M3_RANK_EARLY.results.json)
- Unit-only Rust compile/test: \`cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m3_collision_ic\`
- Full large-memory native reproduction: \`cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m3_collision_ic\`.
- The focused \`.github/workflows/prime31-algebraic-ic.yml\` compiles/tests the example; the large full replay is **not** automatically launched in the 10-minute CI job, pending admission and memory review.

Input: \`p=2147483647=2^31-1\`; \`E:y²=x³+847514846*x+2087444724\`; **full prime order** \`n=2147446831\` and cofactor 1; \`G=(625037715,2100627251)\`. This is the same ordinary public fixture as the preceding Rust and C++ experiments, not the smooth-order \`y²=x³+x\` positive control. Native \`check_curve\` checks the prime-order certificate, discriminant, and \`[n]G=O\`. Depth-9 Dickson fibre terminal \`375193997\` gives \`B=256\` nonzero-y factor-base points.

The target is the fixed public \`Q=(1547953785,1667575595)\`, derived in a separate generator using SHA-256 of the domain separator \`isogeny-prime31-m3-natural-target-v1\`, reduced to a nonzero scalar, and then multiplied by G. That scalar (\`661068045\`) is held only as an **independent verifier** after the pipeline has recovered the log. No target scalar is passed to the collision enumerator, homogeneous linear algebra, anchor, or target mask generators. The candidate \`R=[a]G+[b]Q\` is natural, not forced to decompose; unsuccessful random masks are charged.

## Algebraic structure: homogeneous relations before the anchor

For **three distinct** factor-base indices \`i<j<k\`, enumerate the 8 signed sums \`±P_i±P_j±P_k\`. A repeated point in the *group* (two different signed triples with the same sum) yields the homogeneous relation

\`(ε_i P_i+ε_j P_j+ε_k P_k)-(ε'_u P_u+ε'_v P_v+ε'_w P_w)=O\`.

Let \`A\` be the coefficient matrix of verified homogeneous collisions over \`F_n\` and \`ell_i=log_G(P_i)\` exist because the group has prime order. Then \`A ell=0\`. If \`rank(A)=B-1\`, a computed nonzero kernel vector \`t\` spans \`ell=λ t\` for some unknown \`λ≠0\`. One **anchor** \`[a]G=Σ c_i P_i\` with \`a≠0\` determines \`λ=a/(c·t)\`. Then a single public-target relation \`[u]G+[v]Q=Σ d_i P_i\`, with \`v≠0\`, recovers

\`log_G(Q)=(λ(d·t)-u)/v mod n\`.

This is a correctness proposition **conditional on** rank, independently replayed group identities and successful anchor/target decompositions. It explains why the previous optimistic full \`B+1\` *target-masked* row-rank requirement is not necessary for this collision-first architecture. It does **not** make generic discovery of those collisions cheap; direct signed-triplet enumeration costs O(B³) group work and O(B³) table memory in the worst case.

The algebraic replacement to investigate is a direct six-summand zero relation (Semaev \`f_6\`) with six factor-base membership constraints, or an all-output multi-decomposition oracle. Finding these rows at a lower *complete* cost would change the result; simply moving the enumeration into a different language does not.

## Producer-local exact measurements

The independent implementation constructed the **same Dickson base**, used separate elliptic-curve point addition, enumerated every signed distinct-index triple, and extracted collision differences. A separate NumPy modular-RREF implementation computed ranks over \`F_n\`; a third pure Python group-arithmetic implementation replayed each basis log and the recovered target. C++ code was a validation instrument only, not the user-facing or checked-in solver.

| Depth | Actual B | Distinct signed m=3 templates | Unique group sums | Collisions | Homogeneous rank |
|---:|---:|---:|---:|---:|---:|
| 8 | 135 | 3,207,960 | 3,205,590 | 2,370 | 126/134 |
| 9 | 256 | 22,108,160 | 21,992,660 | 115,500 | **255/255** |

Depth-8 rank is deficient by **8 beyond** the inevitable one-dimensional log kernel; its 2,370 colliding template rows cannot determine all relative factor-base logs. Depth 9 reaches full homogeneous rank after collision row **3,765** at signed template **4,051,363**—long before scanning all 22.1 million templates. The nullspace is one-dimensional and the anchor fixes its scale. The independent replay checked \`[λt_i]G=P_i\` for **all 256** factor-base points, replayed anchor/target signed relations, and verified \`[661068045]G=Q\`.

**Stop at first full rank** reduces the enumerated bank from 22,108,160 to **4,051,363** signed templates. That bank contains **4,047,598 distinct group sums**; a uniformly sampled nonidentity target hits it with probability about \`0.00188484\` and the exact mean uniform-query wait is \`2147446831/4047598 ≈ 530.55\` trials. Actual independently seeded masks took 573 anchor trials (\`a=2054123301\`) and 372 target trials (\`u=806600916,v=1448900068\`), retaining all misses. The recovered target scalar is \`661068045\`.

| Producer-only EC addition counts | Stop at rank 255 | Full bank |
|---|---:|---:|
| Enumerated signed triples | **4,051,363** | 22,108,160 |
| Base-bank construction additions, excluding factor-base build | **2,033,796** | 11,119,360 |
| Bank + anchor and target masked queries, excluding other phases | **2,094,275** | 11,158,521 |
| Verified target | yes | yes |

The partial operation count is **5.33× lower** than full enumeration on this one fixture, but both are dominated by generic group arithmetic. These are **not total cold costs**: they omit factor-base building, modular linear algebra, extra point verification, memory, time, external tool comparisons and a same-host optimized negation-map rho run. The prior separate BSGS control had roughly 60,524 mean group additions, so even these optimistic partial counts cannot support a cold-advantage claim.

As a diagnostic only, an independent random *sign-orbit occupancy* model predicts about 2,395 collisions at depth 8 and 113,413 at depth 9 (observed 2,370 and 115,500). The closeness is not a proof of algebraic randomness, but there is no large measured collision surplus to exploit from this single fibre. Do not confuse unique reachable targets with independently useful rank; depth 8 is the concrete counterexample.

## Literature and historical lead audit before choosing the experiment

- **PKM 2016**, [DOI:10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1): prior factor-base towers, including subgroup/coset construction. This experiment does **not** originate PKM index calculus.
- **Amadori–Pintore–Sala 2018**, [DOI:10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009): prime-field index calculus with a one-relation reduction. The anchor-and-target idea must be compared with that published lineage; **no novelty is claimed** for the linear-algebra trick.
- **Diem 2011**, [DOI:10.1112/S0010437X10005075](https://doi.org/10.1112/S0010437X10005075): extension-field complexity and specially selected prime-field sequences cannot be transplanted as a universal prime-field advantage.
- [Crypto #4077](https://github.com/isogeny-labs/crypto/pull/4077): whole-call sparse PKM m8 and rank-yield gate \`gamma<3+delta/2\`, plus frozen natural-positive targets.
- [Crypto #4275](https://github.com/isogeny-labs/crypto/pull/4275), [#4091](https://github.com/isogeny-labs/crypto/pull/4091): existing native Rust dense-membership and tower F4 engines. Existing small planted-positive dense wins are **not** natural-positive prime31 cold costs.
- [Crypto #4228](https://github.com/isogeny-labs/crypto/pull/4228): certified paired-S3 preprocessing; compare on the same actual polynomial input, counting certificate work.
- [Crypto #4259](https://github.com/isogeny-labs/crypto/pull/4259): subgroup-base selection was already in PKM §3.3.
- [Crypto #4188](https://github.com/isogeny-labs/crypto/pull/4188) existing two-sum support/cold controls, [crypto-autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301) and [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305): heat map and Semaev-alternatives closures, including earlier six-summand lanes. **Do not present six-summand systems or collision-first algebra as unsearched or novel.**
- [Cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536): concurrent Rust auxiliary-map IC integration; avoid duplicating its mapper or claiming its archived 31-bit replay as a fresh attack. [Crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877): scalar-blind transfer still separately open.

The shared research-KB MCP, wrapper and Conductor/EKS were unavailable in this execution context. This is a selected GitHub/primary-source literature check, not an exhaustive corpus audit.

## Next falsifiable experiments

1. **R31-M3-ALG: six-summand homogeneous collisions.** Feed public \`p31\` and identical depth-8/9 Dickson bases into the *existing Rust F4 engine*, using a correct six-summand zero equation and six membership constraints. Compare dense membership vs tower and the prior paired-S3 preprocessor. For each positive system, independently replay every actual signed six-point zero relation and count **rank increment**, not candidate count; include natural negatives, failed queries, solver timeouts, matrix nnz, extraction and peak memory. A complete algebraic oracle must return rows in less *total charged* work than the enumerated reference to advance. The current packed two-summand tower adapter is not six-variable capable: establish a correct representation or reuse the general \`pkm_smooth_tower.rs\` formulation before benchmark.
2. **R31-M8: all-output sparse index calculus.** Use #4077's frozen test with genuine natural positives and \`gamma,delta\` joint intervals; record failed calls and final scalar recovery. Reuse the nonsplit Dickson fibre since \`v_2(p-1)=1\` on the specified prime. No solver-degree-only proxy qualifies.
3. **R31-COLD: true Rust one-cold comparison.** Execute the native example on a known-allowed backend with all costs counted and the same ordinary prime31 targets as optimized negation rho/BSGS. Follow the repository validation and PCC gates before leaderboard publication. Vary the base, target, and ordering in held-out panels to avoid treating one favourable collision chronology as a scaling law.
4. **R31-REUSE: source compatibility.** Wire any successful dense/tower/sixsum candidate into the **existing** Rust \`solve\` rank and individual-log pipeline; avoid a second standalone C++ attack implementation. Explicitly check the historical 304-lead heat map and current cross-repo PRs before adding any lead to the active atlas.

**Decision:** The new rank-first early stop is a valid measured **producer-local finite-instance reduction within a generic enumerative relation bank**. The decisive unsolved task is to generate the same independent homogeneous rows with an algebraic oracle at lower *complete* cost. Until then, do not describe it as a cheap prime-order ECDLP breakthrough or an independently certified result.
