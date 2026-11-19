# Prime31 RR9 two-sign rational-list interpolation: exact rank gate

**19 November 2026 UTC.** Source [PR #4801](https://github.com/isogeny-labs/crypto/pull/4801). Original ordinary cofactor-one prime-field curve only: `p=2^31-1=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, `G=(625037715,2100627251)`. This is a **public-input linear-algebraic obstruction for one proposed extraction mechanism**, not a new complete ECDLP solver or a new solve-time measurement. [pcc:unverified]

## Why this test is nonduplicative

Existing [#4547](https://github.com/isogeny-labs/crypto/pull/4547) proves that a bounded-degree quadratic interpolant through both sign values at each factor-base x only rediscovers the universal elliptic-curve equation. The present test **quantifies the entire weighted-degree polynomial space through D=28, without restricting z-degree to two**, on both original B31 bases and one frozen actual public target. It certifies exactly when a non-universal all-sign interpolation first appears and asks whether that degree could force an eight-leaf relation. It does not alter the nonenumerative extractor owned by [#4692](https://github.com/isogeny-labs/crypto/pull/4692).

The shared research KB tool/CLI and local cargo are unavailable in this runtime. Before implementation, checked the original repository notes [#4547, #4604](https://github.com/isogeny-labs/crypto/pull/4604), [#4539](https://github.com/isogeny-labs/crypto/pull/4539), [#4532](https://github.com/isogeny-labs/crypto/pull/4532), native `prime31_rr9_evaluation.rs` and `lib.rs`, crypto-autoresearcher [#3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301)/[#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305), cryptanalysis [#1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536); PKM, *PKC 2016*, DOI [10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1) and APS, *Finite Fields Appl. 51* (2018), DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009). Bounded audit only; **not a claim of absent prior art**.

## Exact public construction

Freeze `R=G=(r,s)` (nonidentity), and let `(x_i,y_i)` be the 31 original rational factor-base representatives, either first-rational-x31 or Dickson-depth6-31. Both signs are represented by the public slope coordinates

```
z_(i,±) = -(s ± y_i)/(x_i-r)  in Fp.
```

All sampled `x_i != r` and `y_i !=0`. The algebraically **tautological** polynomial

```
Q0(X,Z) = (X-r)Z²+2sZ-(X²+rX+r²+a)
```

vanishes for both sign values at every base x. For the candidate RR9 signed chart `F=(X-r)A(X)+(Y+s)V(X)`, `deg A<=3`, `V` monic cubic, the even stronger identity is

```
V(X)² Q0(X,A(X)/V(X))
 = (X-r)A²+2sAV-(X²+rX+r²+a)V²
 = -M_(A,V,R)(X),
```

where **M is monic degree eight**. This matters: eight roots of the right-hand side do **not** force it to vanish identically. In particular, a naive claim that eight matches force the all-sign interpolant `Q0(X,A/V)` to vanish as a rational function is **false**. Clearing denominators increases the degree. The exact bound for `Q(X,Z)=Σ_{j=0}^L a_j(X)Z^j` after substitution is

```
deg(V^L Q(X,A/V)) <= max_j(deg a_j + 3L),
```

when `deg A,deg V<=3` (cancellation may lower actual degree); it is **not** bounded by the naive weighted degree `max_j(deg a_j+3j)`.

For each `D`, form all monomials `X^i Z^j` with **weight** `i+3j<=D`, and evaluate them at all 62 public signed slope samples. Let `N(D)=Σ_{j=0}^{floor(D/3)}(D-3j+1)`. Since `Q0` has weighted degree 7 with unique leading weighted term `XZ²`, its multiples of total weighted degree <=D form an exactly `N(D-7)`-dimensional subspace of the right kernel, for `D>=7`.

## Deterministic original-curve arithmetic result

The independent finite-field calculation (a separate Python BigInt implementation used only to cross-check ranks, **not** to solve ECDLP) obtained the same exact modular ranks on both native curve bases. The source-bound Rust example is the publication/CI candidate and must pass its own source-specific job before these become CI-verified results.

| Weight D | All monomials N(D) | Signed samples | Exact rank (both bases) | Kernel dimension | Q0 multiples | Extra kernel |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 7 | 15 | 62 | 14 | 1 | 1 | **0** |
| 27 | 145 | 62 | 61 | 84 | 84 | **0** |
| 28 | 155 | 62 | 62 | 93 | 92 | **1** |

At D=27 the entire kernel is exactly `(Q0)` within this weighted degree, since the right-kernel dimension equals the known Q0-multiple dimension. **Consequently no non-Q0 interpolant exists at any smaller weighted degree** for either of these two bases at this specific public target. At D=28 one additional interpolant exists, but the sample count already forces extra kernel dimension: `N(28)-N(21)=63>62`. It cannot be promoted into an unknown-index extractor merely because it exists.

This excludes **all-62-point two-sign interpolation** as a low-degree direct relation extractor in these frozen instances. It does not exclude partial-list interpolation, sign selection without enumeration, higher multiplicities, target-dependent special degeneracies, F4/remainder-tower extraction, or alternative factor bases. No group-sum table is consulted in this certificate.

## Native Rust source and validation boundary

- Candidate native example: [`prime31_rr9_two_sign_wrank.rs`](../native/examples/prime31_rr9_two_sign_wrank.rs). Imports the original `prime31-algebraic-ic::{check_curve,Arithmetic,Base,PRIME31}`. It constructs first-x31 and Dickson-depth6-31 without replacing the group law, asserts every universal-Q0 identity, performs modular elimination, and verifies the exact ranks/dimensions above in focused Rust tests.
- Registered reproducibility gate: [GitHub Actions `prime31-rr9-two-sign-wrank.yml`](../../../.github/workflows/prime31-rr9-two-sign-wrank.yml). Checkout pins original Rust arithmetic source commit `1df714645aa683cf975bc76fcacb9842cb4780fd` and Rust `1.100.0`, overlays only the new example, runs original library tests and focused tests, emits JSON with explicit `group_sum_enumeration_used=false` / `unknown_scalar_recovered=false`, then preserves the raw artifact.
- Separate Python modular computation (not a solver) independently found first-x31 beginning `0,1,2,3,5,6,8,9`, and Dickson depth-6 seed `3` / terminal `933677148` with 31 rational points; all three table rows matched for both bases. Rust CI is a separate verification requirement. No elapsed solve time, full DLP log, or signed PCC verdict results from this computation.

## Prioritized next experimental gates

1. **P0 — nonenumerative extraction, owner #4692:** Reuse the Rust public-only seven-coefficient ideal and its quotient/remainder-tower/F4 code; pass 46 planted positives/18 natural negatives at B8 *without* oracle masks, known leaves, or group-sum half-tables. Test exceptional and repeated-root branches, verify every recovered witness in the independent original group law. Then test B31 natural positive and negative panels. An inconclusive or capped computation stays censored, not negative.
2. **P1 — genuine support-selective decoder:** If trying interpolation again, enforce a partial list / chosen-eight condition **without** enumerating `2^8 binomial(31,8)` signed support choices, and quotient out the universal `Q0` component before claiming relation information. Measure actual recovered coefficients on blinded public R, degree, F4 rows, time, memory, misses, and replay. The above all-62-sign rank gate vetoes unchanged low-degree universal interpolation.
3. **P2 — complete 1,024-target benchmark:** Once P0 yields sufficient independent rows, drop it into existing rank-31 native Rust IC pipeline (#4623/#4728), verify every `[k]G=Q` on untouched prime31 and charge **cold setup, failed shifts, rootfinding, linear algebra, verification and memory**. Run optimized same-runner 2^20-entry shared BSGS (#4532), existing generic-table IC, optimized rho and nearest published prime-field index-calculus control. The historical fully charged 1,024-Q BSGS cohort was **170.113ms** (67,924 KiB RSS); it was a different run from the 3,342.793ms native-affine BSGS and is a cross-run frontier reference, **not** a qualified ratio.
4. **P3 — reproducibility/PCC:** Bind exact candidate source and frozen target corpus to raw artifacts, independent signed mathematical checks and paired A/A noise controls. No novelty/leaderboard claim without solver-stage and complete-solver comparisons.

**Scientific standing:** An exact mathematical no-go for one public-only interpolation family at two original bases and one original public target; implementation and source-specific CI verification tracked on PR #4801. **No new measured cheaper complete non-generic prime-field ECDLP solver is claimed.**
