# Prime31 RR9: exact signed-cancellation coverage and weighted-oracle handoff

**2026-11-19 UTC.** Candidate PR [#4803](https://github.com/isogeny-labs/crypto/pull/4803), protocol [registered before native source](RR9_SIGN_CANCELLATION_COVERAGE_PROTOCOL_20261119.md). **Scope:** necessary relation-yield obstruction and original-curve grading enumeration, *not* a public-only algebraic extractor, a complete ECDLP solver or a solve-time improvement. [pcc:unverified]

## The distinct problem classes cannot be interchanged

- The degree-5 public RR9 Macaulay study [#4548](https://github.com/isogeny-labs/crypto/pull/4548) sets `M(X)=H_B(X)` for one **fixed set of eight distinct factor-base x values**. There are only **256** possible unordered signed EC sums for this stratum, because permutation of the eight leaves leaves their sum unchanged. A polynomial-system solution for a public residual outside those 256 cannot exist, regardless of F4 or Macaulay optimization.
- If all **eight unweighted leaves may repeat** from the same eight-x base, there are at most `C(16+8-1,8)=490314` unordered multisets of sixteen signed points. Cancellation of `P+(-P)=O` tightens this limit further to **187137** distinct net signed coefficient vectors and hence at most 187137 target sums.
- The rank31/1024-log four-window W8 solver [#4728](https://github.com/isogeny-labs/crypto/pull/4728) uses eight **position-weighted factor-base slots**. Order now matters: the earlier exact mean ordered-witness count `16^8/n=2.000034...` belongs to this *different system*, and the above unweighted ceiling must **not** be used to exclude the weighted method. Its polynomial extractor must encode the slot-specific weighted resultants with repeat and sign handling, not just substitute `H_B` into the earlier squarefree Macaulay code.

## Proof of the sharper signed-cancellation ceiling

Let `P_1,...,P_d` be d distinct factor-base points with freely permitted signs. An unordered eight-leaf signed sum is `R=sum_{i=1}^d k_i P_i`, where `k_i=(#positive P_i)-(#negative P_i)`. Necessarily `||k||_1<=8`, with `||k||_1 ≡ 8 mod 2`. Conversely every such integer vector is realizable with eight leaves by padding `sum_i |k_i|` leaves with `(8-||k||_1)/2` inverse pairs of `P_1,-P_1`. Hence the achievable signed *coefficient vectors* are exactly those integer vectors.

The number with exactly j nonzero coordinates and L1 norm r is `binom(d,j) 2^j binom(r-1,j-1)`: choose nonzero positions, choose signs and compose r into j positive parts. Summing even radii r=0,2,4,6,8 gives:

| L1 radius | Distinct coefficient vectors, d=8 |
|---:|---:|
| 0 | 1 |
| 2 | 128 |
| 4 | 2,816 |
| 6 | 27,008 |
| 8 | 157,184 |
| **Total** | **187,137** |

The map `k -> sum k_iP_i` may identify multiple vectors, so **187137 is an unconditional exact combinatorial upper bound** on unweighted B8 group support, not a group-law generic lower bound. This sharpens the 490314 signed-unordered-multiset count. The fixed squarefree-eight-x subclass has only `2^8=256` sign choices.

## Actual original-curve numerical control

**Unchanged fixture:** `p=2147483647=2^31-1`, `a=847514846`, `b=2087444724`, full group prime order `n=2147446831`, `G=(625037715,2100627251)`, cofactor one. First rational-x eight canonical points:

```
(x,y): (0,309380359), (1,749117756), (2,51658786),
       (3,166645833), (5,457995111), (6,865603774),
       (8,849737830), (9,548858265).
```

A separately authored Python BigInt elliptic-curve enumeration, independent of the candidate Rust source, generated every admissible integer vector and used the actual curve group law to determine group-point equality. **Producer-local exact arithmetic observations (to be source-confirmed by GitHub CI):**

| Scope | Vectors enumerated | Distinct actual group sums | Uniform group target hit fraction |
|---|---:|---:|---:|
| Eight distinct fixed x, one sign each | 256 | 256 | `256/2147446831 ≈ 1.1921e-7` |
| Eight unweighted signed x, repetitions allowed | 187,137 | **187,137** | `187137/2147446831 ≈ 8.7144e-5` |

The observed injectivity of these 187137 net vectors is specific to this fixed curve and chosen eight base x's; it is **not claimed universally**. Both rows are exact finite-census results if reproduced by the original Rust source, not probabilistic fits. The Rust example uses only existing `Arithmetic`, `check_curve` and `PRIME31`; its signed-vector and 256-word censuses are **generic enumeration grading controls**, completely disjoint from any proposed nonenumerative extractor. It records the actual group additions and elapsed grading wall time, which must **not** be interpreted as a candidate or solver timing.

For a uniform random public residual R in the full order-n group, the exact probability of a relation from these fixed supports equals the distinct-support cardinality divided by n. On the historical 1219 training+online shifted requests, independent uniform-shift assumptions would give an **expected** number of at most `1219*187137/n ≈ 0.10623` unweighted hits, or `1219*256/n ≈ 0.0001453` full-squarefree hits. This is not a statement about adversarially selected, witness-planted or position-weighted targets. Even granting all other IC costs for free, neither fixed-support unweighted design can generate the needed independent rows from these numbers of natural uniform queries.

## Prior leads checked and actual decision

- [#4716](https://github.com/isogeny-labs/crypto/pull/4716) already proves a shared unsigned-base multiset ceiling; the present proof additionally **quotients signed inverse-pair cancellation** and measures a precise eight-x unweighted support. It does not claim #4716's theorem is wrong.
- [#4717](https://github.com/isogeny-labs/crypto/pull/4717) treats RR9 repeated-root multiplicity strata; [#4548](https://github.com/isogeny-labs/crypto/pull/4548) is specifically the fixed-squarefree `M=H_B` pilot (degree-5 inconclusive). [#4547](https://github.com/isogeny-labs/crypto/pull/4547) addresses seven-supplied-anchor generic equivalence; [#4801](https://github.com/isogeny-labs/crypto/pull/4801) addresses a **different** all-sign rational-list rank obstruction. Those lanes are not repeated here.
- [#4728](https://github.com/isogeny-labs/crypto/pull/4728), [#4774](https://github.com/isogeny-labs/crypto/pull/4774), [#4793](https://github.com/isogeny-labs/crypto/pull/4793), [#4532](https://github.com/isogeny-labs/crypto/pull/4532) document the complete *generic grader* rank31 batch and the strong optimized shared-BSGS comparison. Do not transfer any time ratio from these to the current group-support census.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns the genuine unknown-index **position-weighted RR9/F4/remainder-tower** effort; coordinate with it, do not independently build a duplicate F4 extractor. Relevant cross-repo `crypto-autoresearcher` SA-04/SA-18 negative determinants, `cryptanalysis` prime-field Semaev experiments. Shared research KB service and local Rust toolchain unavailable in this connector runtime; bounded GitHub-primary-source audit substituted, not evidence of absolute novelty.
- Publications: Petit–Kosters–Messeng, *PKC 2016*, DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1), and Amadori–Pintore–Sala, *Finite Fields and Their Applications* 51 (2018), DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009). Both are relevant prime-field algebraic IC comparators; the current vector census is not a new index-calculus attack.

## Prioritized next experiments (distinct falsifiable gates)

1. **P0 weighted algebraic equivalence (#4692 owner):** on the *same* frozen weighted B8 46-positive/18-negative public residual corpus, implement or continue exact **eight slot-specific weighted-resultant constraints**, with sign orientation, repeat roots and exceptions. Prove equal complete rational solution sets to an independently held-out generic grader on feasible small panels. Forbid reading its masks, x-index choice, scalar or EC group-sum tables during candidate extraction. Persist positive/negative/undetermined classification.
2. **P1 natural unknown-index extraction:** run owner F4/tower/SAT variants on the frozen B8 panel with exact field-operation accounting, Gröbner degree, all censored/failed query costs, real root reconstruction and signed original-curve verification. The 2026 Macaulay degree-5 fixed-squarefree pilot is *not* eligible as a proxy for W8 natural-positive yield.
3. **P2 full prime31 rank and solving:** only after P0/P1 pass, replace W8's generic grader in the existing Rust rank31/1024-target batch, charge all setup, failed shifts, rank defects, linear algebra, memory, recovery and checks. Compare first to same-ideal published PKM/APS-style algebraic frontier, then in a paired run to optimized shared `2^20`-baby BSGS and optimized negation-map rho. Test one, 64 and 1024 logs.
4. **P3 growing-curve regime:** replicate on non-deliberately-weak ordinary prime-order/cofactor-one curves across a field-size ladder; separately test whether per-relation gain survives complete cold costs. Require independent signed PCC and source-bound artifacts before claiming a whole-solver advantage.

**Status:** A sharper exact mathematical support bound and independently reproduced original-curve group census. Source-specific native Rust CI **passed** and independently authored Python arithmetic agreed. No complete non-generic ECDLP speed advantage is claimed. No complete non-generic ECDLP speed advantage has been established here.

## Source-specific GitHub Actions closeout — 2026-11-19 UTC

[Exact-source CI #44506856237](https://github.com/isogeny-labs/crypto/actions/runs/44506856237) **PASS** (job #135500604170), Rust 1.100.0 on ubuntu-24.04, pinned original native arithmetic `1df714645aa683cf975bc76fcacb9842cb4780fd`, candidate workflow/source commit `806c13399d93043b354e6eefb90cf162c83d4dbf`. Original library tests **40/40**, focused new example tests **2/2**, release binary output JSON schema assert PASS, upload success. The report-only follow-up does not change the tested Rust example/workflow.

Machine-readable source-bound observations:
- Signed net vectors enumerated **187137/187137**, distinct original-curve group points **187137**. This specific original eight-x base has no collisions among admissible signed net vectors.
- Fixed squarefree eight-x sign sums **256/256** distinct and all contained in the full unweighted support.
- Group additions in grading precomputation, net-vector traversal and squarefree check: **223480** (the prior independently written Python enumeration counted 353480 recursive group-add calls including zero choices; **these are different accounting definitions**).
- Grading-only release execution wall: **43,509,259 ns = 43.509259 ms** on the GitHub runner. It does not perform algebraic extraction, rank31 recovery or any unknown ECDLP and must not be compared to complete-solver timings.
- Raw JSON SHA-256 `74c3adf7937f0ef71f7e4627b2365fe7e9361cc121cb1a6f4ebc239642be54e1`; [artifact #14519549353](https://github.com/isogeny-labs/crypto/actions/runs/44506856237/artifacts/14519549353), upload ZIP digest `bcf1ecf8c4fd9e4235ec24284acad6e915986dce27d897f0274f03bdcd01dfbd`.
- Global repository unrelated checks may fail on inherited missing artifact-store files or curve registry drift; these do not invalidate the isolated source-specific example. Independent third-party signed PCC is still **[pcc:unverified]**.
