# EXP-R31-L — implemented factorized S4 / quartic product-gcd oracle on prime31

**Date:** 2026-11-15. **Standing:** source-implemented, exact-head native execution **unconfirmed as of check-in**, [pcc:unverified]. **Language:** Rust only for all new arithmetic, tests, benchmark harness and CI. This is an algebraic index-calculus component improvement, **not** a complete ECDLP solver or new prime-field asymptotic algorithm.

## Implementation / checked-in execution contract

- Source: [`examples/prime31_s3_quotient_gcd.rs`](examples/prime31_s3_quotient_gcd.rs) with `m4_first_eliminant_factorized(xs,k,t)`, `m4_factorized_scale`, a new independent exact-eliminant equality test and CLI `--m4-factorized`.
- Standalone **Rust-only** exact-head development workflow: [`prime31-factorized-s4-rust.yml`](https://github.com/isogeny-labs/crypto/blob/research/norm-branch-rank-pilot-20261115/.github/workflows/prime31-factorized-s4-rust.yml). It checks the reused Rust library, targeted algebraic parity tests and matched stage costs. The first observed workflow run [43763707585](https://github.com/isogeny-labs/crypto/actions/runs/43763707585) was **queued** when inspected; queued != passed. Follow latest-head actions for authoritative results. No standalone host/CPU, root-crate, PCC, EKS or cold ECDLP measurement has been claimed.
- Preserve the original quotient/Horner implementation unchanged for paired reference. Compare exact *monic gcd polynomials*, not merely Boolean positivity; B4/B8/B16 run positive/public/exceptional targets. The exceptional `U=t` check is unchanged and the fast branch fails closed if the removed factor is repeated.
- B31 fast-only arm avoids reporting a speedup against an uncompleted old solver. On the planted four-point target it also factors the gcd, reconstructs pair indices and verifies a full signed relation **after** algebraic discovery. On the unrelated public `[17]G` it measures target-dependent negative membership. Three target categories at each B; run complete-or-fail, with no censored time promoted as a win.

## Mathematical mechanism: an exact cost reduction for this implementation

Work with the same ordinary cofactor-one curve

```
Fp: p = 2^31 - 1 = 2147483647
a = 847514846, b = 2087444724
#E(Fp) = n = 2147446831 (prime), G = (625037715,2100627251)
```

Let `K(Z)=monic(prod_(i<=j) S3(xi,xj,Z))`, with `D=deg K=B²`. The old membership stage computes

`gcd(K(U), Res_Z(S3(U,Z,t),K(Z)))`

by reducing the full degree-D polynomial `K(Z)` in a quadratic quotient over `Fp[U]/K(U)`. With the existing naive polynomial multiplication/remainder, this is a degree-D Horner loop containing degree-D modular products: an `O(D³)=O(B^6)` field-operation route.

**New algebraic route:** `K(Z)` is already given as a product of `M=B(B+1)/2` S3 factors. Because resultants are multiplicative in their second polynomial, compute each short resultant separately. Write

`S3(U,Z,t)=A(U) Z²+B(U) Z+C(U)`, with `deg_U A,B,C <=2`, and `S3(xi,xj,Z)=d2 Z²+d1 Z+d0`. Then

`T_ij(U)=(A*d0-C*d2)^2-(A*d1-B*d2)*(B*d0-C*d1).`

This is a **degree-at-most-four** polynomial in U, constructed with existing prime31 polynomial operations. Accumulate `prod_ij T_ij(U) mod K_regular(U)` using only short-factor multiplications, and take `gcd(K_regular, product)`. The original `K` normalization contributes only a nonzero scalar, irrelevant to the gcd. There is no point addition, known ordinate or group-sum table in this algebraic discovery routine.

For `i=j`, the original `S3(xi,xi,Z)` has degree one in Z. Padding it to degree two in the 4x4 Sylvester determinant introduces the extra factor `A(U)=(U-t)^2`, which is a **unit modulo K_regular**. If `t` is a K-root, the implementation divides off `U-t` before taking the regular gcd and separately checks a finite common root in `K(Z),S3(t,t,Z)`; a repeated special root fails closed. Thus the regular gcd is identical to the old oracle's result on the registered squarefree factor-base support, including diagonal factors.

The new stage's naive arithmetic now multiplies M polynomials of degree <=4 modulo degree D, costing `O(M*D + D²)=O(B^4)` field work including a classical Euclidean gcd, plus `O(B^4)` pair-support construction. This is a reduction in the **implemented component's operation bound**, not an asymptotic ECDLP complexity claim.

| B | D=deg K | Short S4 factors M | Target algebraic stage |
|---:|---:|---:|---|
| 4 | 16 | 10 | Paired old/new |
| 8 | 64 | 36 | Paired old/new |
| 16 | 256 | 136 | Paired old/new |
| 31 | 961 | 496 | Fast-only; full planted witness |

## Previously observed measurements (historical; NOT from this new source)

The prior exact-source Rust development workflow [43747735303](https://github.com/isogeny-labs/crypto/actions/runs/43747735303) reported the old target-dependent norm/GCD at B4/B8/B16 as **0.057/1.783/106.916 ms** on a planted target. That comparison explicitly excluded cold base construction and final recovery; it was a single-run stage figure. The new branch cannot be called faster until the new Rust-only workflow completes and its exact-head stdout is inspected. Even a fast B31 m4 oracle cannot escape the independent cardinality ceiling: the canonical B31 factor base covers only 618,885 of 2,147,446,831 group points with at most four signed summands, or approximately 0.02882% of uniform public targets. Do not turn this into a claimed complete ECDLP advantage.

## Prior lead assessment

- [#4228](https://github.com/isogeny-labs/crypto/pull/4228): certified quotient/subresultant and early elimination. The new factorized product is a testable low-cost front end to that work, but its math is classical and overlapping scope must be checked before further integration.
- [#4275](https://github.com/isogeny-labs/crypto/pull/4275): dense factor-base F4 gains versus tower still lost to generic enumerative controls. The present optimization specifically reduces high-degree quotient arithmetic instead of re-running its membership contest.
- [#4077](https://github.com/isogeny-labs/crypto/pull/4077) and [balanced m8 Rust source](examples/prime31_m8_balanced_f4_input.rs): full-call m8 algebraic cost, large F4 matrices and rank remain the decisive algorithmic obstacle.
- [#4311](https://github.com/isogeny-labs/crypto/pull/4311) / [#4338](https://github.com/isogeny-labs/crypto/pull/4338): one-relation/fast-selector proposals do not yet supply an implemented, non-generic, label-free fast selector; do not relabel generic meet-in-the-middle as algebraic IC.
- [y-hybrid work](XY_COORDINATE_PROJECTION_PILOT_20261115.md): y-only creates rational cubic lifting ambiguity; oriented x/y merits a stage comparison but has not improved natural four-term support.
- [#3877](https://github.com/isogeny-labs/crypto/pull/3877): chained Jacobian transfers remain a distinct path until an independently valid computable arbitrary-point transfer and full-cost win exist.
- Foundational external literature: [Petit–Kosters–Messeng (2016)](https://doi.org/10.1007/978-3-662-49387-8_1), [Amadori–Pintore–Sala (2018)](https://doi.org/10.1016/j.ffa.2018.01.009). Resultants and prime-field Semaev relations are not novel. The research-KB MCP/CLI was unavailable; the audit was bounded to current GitHub source, lead PRs, and publisher records.

## Next falsifiable experiments (Rust only)

1. **Close the immediate exact-head execution gate:** complete the targeted Rust-only workflow, retain stdout and source SHA, classify compile failure/timeout/success correctly and independently replay polynomial equality on B4/B8/B16; include B31 successful planted signed extraction and unrelated `[17]G` control before advertising a measured component improvement. Store complete, censored or failed outcomes.
2. **Natural-public m4 negative control:** record 32-64 deterministic sampled `aG+bQ` inputs with scalars withheld from discovery; count successes and per-target cost. Because the exact 0.02882% B31 ceiling already rules out a cheap all-m4 cold solver, use this only to detect false positives and characterize cost.
3. **Escalate to m6/m8 algebraically, not by point tables:** use the optimized short-resultant arithmetic within the balanced m8 Rust S4/S3 input and #4228 certified quotient engine; track S4 elimination degree, max matrix, real target-dependent relations, exception handling and incremental projective rank. Do not assume a degree-B⁴ m4 support polynomial can be materialized cheaply.
4. **Whole-call acceptance:** at least 12 held-out arbitrary prime31 Q, full relation acquisition, sparse linear solve, individual log and `[k]G=Q` checks, all cold setup + failed queries + memory, matched optimized Rust negation-map rho/BSGS and independent PCC. Cross-check another ordinary cofactor-one prime-field curve before extrapolating.

**Conclusion at present:** a theoretically cheaper, exact algebraic **component** has been implemented in Rust and checked in. Actual compiled equality and timing receipts are pending; the target-dependent higher-arity index-calculus solver remains an open mathematical/engineering problem.
