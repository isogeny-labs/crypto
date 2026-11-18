# Prime31 weighted-M6: exact unavoidable collision quotient

2026-11-18 UTC · [PR #4722](https://github.com/isogeny-labs/crypto/pull/4722) · [frozen protocol](PRIME31_M6_FORMAL_COLLISION_PROTOCOL_20261118.md) · [original-crate Rust example](examples/prime31_m6_formal_quotient.rs) · [independent BigInt verifier](verify_prime31_m6_formal_quotient.py).

**Scientific classification:** exact finite integer-combinatorial support bound and original-curve planted point identity. Not a fresh algebraic relation extractor, 1,024-log solve, measured ECDLP speedup or performance comparison. Producer-side evidence is `[pcc:unverified]` until independent signed validation. **Source-specific GitHub Actions passed:** [run #44336386449](https://github.com/isogeny-labs/crypto/actions/runs/44336386449), pinned Rust 1.100.0 and original native source `1df714645aa683cf975bc76fcacb9842cb4780fd`: 40/40 reused Rust library tests, 2/2 new quotient tests, full original-curve native JSON, and separately authored Python BigInt verification of all 203 partitions per weight profile, B2/B3/B4/B8 exhaustive coefficient vectors, original-curve planted EC equality and a rejected one-count mutation. [Raw source-bound artifact](https://github.com/isogeny-labs/crypto/actions/runs/44336386449/artifacts/14442695390) (artifact ID 14442695390). This remains producer-side evidence; independent signed PCC validation is not complete.

## New result: relation-hit ceiling is strictly below the word bound

Fix the unchanged ordinary cofactor-one Prime31 curve:
`p=2147483647; E:y²=x³+847514846x+2087444724; n=2147446831`. Keep the first 31 rational-abscissa base points with canonical positive ordinates and original six public weights `(1,3,5,7,11,13)`.

A six-slot **word** `(i_1,...,i_6)` has a formal coefficient vector

```
c_t(i) = sum_{j : i_j=t} w_j, for t in 1,...,B.
```

The actual EC sum is `sum_t [c_t(i)] P_t`. Words with the same integer vector necessarily have the same group sum, independently of curve structure. The converse need not hold. Therefore the exact number `F_B` of distinct formal vectors is an **upper bound** on EC support.

Let `S_k(w)` be the set of distinct *multisets* of block-weight sums obtained from set partitions of six **labeled** slots into k nonempty blocks. For a multiset `s` let `m_s(v)` be the multiplicity of value v. Then

```
F_B(w) = sum_{k=1}^{min(B,6)} sum_{s in S_k(w)}
         B! / ((B-k)! * product_v m_s(v)!).
```

**Proof:** A word determines a nonempty set of k distinct base indices, and each used index has a positive coefficient equal to the weight sum of the subset of slots assigned to that index. The multiset of k coefficients belongs to `S_k(w)`. Conversely, choosing k distinct base indices and assigning the coefficient multiset to them determines one formal vector. Equal coefficient values are indistinguishable, giving the denominator `product_v m_s(v)!`. Distinct coefficient multisets and distinct base-index sets give disjoint formal vectors. Summing gives an exact count. This is pure finite combinatorics and does not assume independent EC word images.

### Exact B31 count, original six weights

| Support size k | Distinct block-sum multisets | Formal coefficient vectors |
|---:|---:|---:|
| 1 | 1 | 31 |
| 2 | 19 | 17,205 |
| 3 | 75 | 1,928,355 |
| 4 | 65 | 46,442,340 |
| 5 | 15 | 305,839,800 |
| 6 | 1 | 530,122,320 |
| **Total** | | **884,350,051** |

The 203 set partitions yield 176 unique sum patterns distributed as `(1,19,75,65,15,1)`, whereas their raw Bell-number counts are `(1,31,90,65,15,1)`. The prior loose word bound was `31^6=887,503,681`. Exactly **3,153,630** ordered words are redundant when only distinct formal coefficient vectors are counted.

Thus for a truly uniform residual R on the original group,

```
Pr[R in six-slot support] <= 884350051 / 2147446831
                         = 0.4118146434331905.
```

The previous `31^6/n` ceiling was `0.4132831920158493`. The new upper bound is **0.146855 percentage points** stricter; it is not an exact success probability. For i.i.d. uniform random residuals, the expected number of trials for 1,024 positive outcomes is bounded below by `1024*n/F_31 = 2486.55558...` (ceil **2,487**), not the previous ceil 2,478. This expectation says nothing deterministic about one seeded corpus, and additional rank constraints can only raise the actual required work.

An explicit original-curve collision certificate needs no logarithms: since `1+7=3+5=8`, words `(0,1,1,0,2,3)` and `(1,0,0,1,2,3)` both represent `[8]P_0+[8]P_1+[11]P_2+[13]P_3`. The native code checks their **actual group sums**, not just integer vectors. The independent checker uses separately written affine EC arithmetic and checks that equality again.

### Weight-design decision

Two alternatives have all 64 subset sums of their six weights distinct:

- `(1,2,4,8,16,32)` — binary;
- `(1,3,7,15,31,63)` — odd, superincreasing.

When subset sums are unique, every formal coefficient identifies exactly the slots it contains, so every word has a different formal coefficient vector: `F_B=B^6`. For B31, the **maximum possible improvement in this formal support ceiling**, even with ideal alternative weights, is only `3,153,630/887,503,681 = 0.355337%`. Under the same uniform model, no fixed six-positive-slot weight choice can beat the earlier universal `31^6/n = 41.3283%` upper bound. Actual elliptic-curve collisions and query costs can erase any formal gain. This is a **pre-screen to avoid spending an expensive experiment on weight tuning**, not a claim that the alternate weights perform better.

### Fully independent small-B controls

The Rust producer enumerates 203 restricted-growth set partitions and exact `u128` falling factorials. The independent Python verifier instead partitions **bit masks by their least member**, exhaustively enumerates all coefficient vectors for B2/B3/B4/B8 and uses separately implemented group-law arithmetic on the original curve. Frozen exact counts:

| B | Ordered six-slot words | Original-weight formal vectors |
|---:|---:|---:|
| 2 | 64 | 39 |
| 3 | 729 | 543 |
| 4 | 4,096 | 3,418 |
| 8 | 262,144 | 249,348 |
| 16 | 16,777,216 | 16,559,176 |
| 31 | 887,503,681 | 884,350,051 |

The full Rust program checks `check_curve(PRIME31)`, lifts the canonical B31 and checks the planted equality in the actual group. The verifier rejects a one-count mutation of the B31 headline. Counts are mathematical enumerations, not measurements of solver throughput or relation recovery.

## Lead audit and next discriminating experiments

**Already explored:** [#4714](https://github.com/isogeny-labs/crypto/pull/4714) has a complete 1,024-target original-curve weighted-M6 Rust solver with generic 3+3 sum tables; in matched runs it loses to shared BSGS by approximately 21× and to the mixed-IC control by about 1.8×. [#4722](https://github.com/isogeny-labs/crypto/pull/4722) proves the original 31^6/n ceiling. [#4503](https://github.com/isogeny-labs/crypto/pull/4503) covers separate mixed signed-base formal support. [#4692](https://github.com/isogeny-labs/crypto/pull/4692) is the RR9/F4 target-conditioned extractor lane. [#4717](https://github.com/isogeny-labs/crypto/pull/4717) covers low-power root-multiplicity strata; [#4720](https://github.com/isogeny-labs/crypto/pull/4720) provides the signed two-leaf projector; [#4701](https://github.com/isogeny-labs/crypto/pull/4701) optimizes isogeny inversion. Cross-repo audit also located [crypto-autoresearcher #3261](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3261) (SA-23 negative-control work), [#3281](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3281) (cover follow-ups), and [cryptanalysis #1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766) (cold single-target certification). The extraordinary prime-field subexponential claim in [#4698](https://github.com/isogeny-labs/crypto/pull/4698) is a separate unbenchmarked, unsigned paper proof; it is not the Prime31 complete-solver result.

**Relevant prior mathematics:** [Semaev (2004) and Gaudry (2009), overview](https://isogenylabs.io/cryptanalysis/summation-polynomials.html), [Yokoyama et al. 2020](https://doi.org/10.1515/jmc-2019-0029), [APS prime-field variant, 2018](https://doi.org/10.1016/j.ffa.2018.01.009). This weighted coefficient-vector counting identity is presented as elementary and not as a new general index-calculus attack. Research-KB and Conductor were unavailable through this connector session; scope is explicit GitHub code/PR searches and these papers.

**Priority 1 — RR9/F4 genuinely nonenumerative:** On public-only B4/B8 positive, negative, vertical and multiple-fibre targets, demand recovered **signed** leaves with `generic_3sum_table_count=0` and `generic_4sum_table_count=0`. An independent group-law grader must recover every witness on planted fixtures, distinguish no-solution from timeouts, and reject altered public targets. Start with B4/B8 because a supplied divisor-support predicate is not an unknown-divisor extractor.

**Priority 2 — fresh B31 original-curve recovery:** Carry only natural B31 algebraic relations into the existing Rust rank-31 matrix; solve 1,024 blinded unknown logarithms with no hidden scalar at relation generation and no generic fallback. Preserve all failed trials, repeated roots and censoring. Do not change the curve or artificially smooth its group order.

**Priority 3 — decisive same-host cost panel:** Compare complete cold one-target and 1,024-target costs, including F4/RR9 setup, coefficient extraction, field arithmetic, all failures, linear algebra, verification, memory, optimized mixed IC, shared BSGS, and negation-map rho. Original matched BSGS reached 144.049 ms for 1,024 public targets, so a new 3,766-query design has an *optimistic* ceiling around 38.25 microseconds per query **before** setup; this is a target not a performance estimate.

**Decision:** Stop generic M6 weight retuning as a primary research lane. Even eliminating every unavoidable formal collision can move its coverage bound only 0.15 percentage points. Spend compute on algebraic relation discovery, independent proof scrutiny and complete-cost comparisons instead.
