# Prime31 transformed-slot Semaev F3 chain — target-coupled 14-variable chart and sparse 7-variable alternative

**Research result (November 19, 2026).** Continuing existing [#4812](https://github.com/isogeny-labs/crypto/pull/4812) on the unchanged ordinary cofactor-one `E(F_2147483647)`, group order `2147446831`. Frozen [protocol](TRANSFORMED_SLOT_F3_PROTOCOL_20261119.md) was published before the candidate Rust source. [pcc:unverified]. The existing **public-only extractor** remains [#4692](https://github.com/isogeny-labs/crypto/pull/4692); this report does not claim a completed F4 solve, new ECDLP logarithms, or a complete-solver speed advantage.

## Exact reformulation

For one eight-point public factor-base window, let `T_i={x([2^i]P_j):j=0..7}` and `H_i(X)=product_(t in T_i)(X-t)`, for `i=0..7`. Let `L_i=[2^i]S_i` be the unknown signed transformed leaf, `z_i=x(L_i+...+L_7)` and `t_i=x(L_i)`. The desired public target constrains `z_0=x(Q)`. The **ordinary finite chart** is
```
H_i(t_i)=0,                        i=0,...,7,
S3(t_i,z_(i+1),z_i)=0,             i=0,...,6,
z_7=t_7; z_0=x(Q) public.
```
There are exactly **14 unknown coordinates** (`t_0..t_7,z_1..z_6`), **15 equations**, and maximum **total degree eight** (eight degree-8 selector equations, seven degree-4 Semaev F3 equations). The previous same-size rationally eliminated Horner chart had seven degree-10 equations; the old quartic Horner chart carried 21 variables. Here `[2^i]P_j` is precomputed **once per public slot**: no intermediate scalar-doubling variables or denominator clearing. This is a more favorable *representation* for an F4 shootout, not a demonstrated F4 speedup. In particular the degree-8 leaf polynomials remain an elimination bottleneck.

**Soundness of the finite chart.** For ordinary rational x-coordinates on this odd-order curve, `S3(x,y,z)=0` means that some sign choices give an elliptic group sum with x-coordinate z (unless the sum is infinity, which has no finite x). Starting at `z_7=t_7`, back-substitute signs to construct the signed transformed leaves, then globally negate them if needed to match the public Q rather than -Q. This uses only the candidate polynomial solution and publicly available transformed factors. Exceptionally vanishing partial sums require an additional projective chart; the finite-chart claim does not silently include them.

**Eliminate the seven first leaves symbolically.** Define
```
K_i(U,V) := Res_X(H_i(X), S3(X,U,V))
          = product_(t in T_i) S3(t,U,V),       i=0,...,6.
```
The alternative **7-variable / 8-equation chart** is `K_i(z_(i+1),z_i)=0` for `i=0..6` plus `H_7(z_7)=0` with `z_0=x(Q)` fixed. Its maximal total degree is **32**. Each `S3(t,U,V)` contains at most nine monomials with bidegree at most (2,2), so `K_i` has bidegree at most (16,16) and **at most 289 monomials**. The product equations factor explicitly into eight quadratic-in-each-variable components; they should be kept factored until the elimination backend requires expansion.

**Input-polynomial sparsity:** direct expansion of one generic S3 uses 13 nonzero monomials at these curve parameters; each H_i has eight or nine nonzero univariate coefficients. The four original windows therefore present **162, 163, 163, and 163 total monomial occurrences** across the 15 input equations, independently counted in Python. By contrast the earlier degree-10 eliminated Horner S3 equation was reported to have 67 terms per link (seven links), before adding its eight selectors. This is an *input-size* reduction, not evidence about F4 fill, last-fall degree, or total solving time.

The 7-variable system is triangular but **not** automatically cheap: conditioning on a suffix coordinate, `K_i` can have up to 16 possible next x-values. Naive sequential branching across seven levels can reach `16^7=268,435,456` partial assignments before applying the final eight-leaf test, and should NOT be mistaken for a fast public target extractor. This is an upper branch cap, not a lower bound on F4, resultant-tree, or batched rootfinding methods.

## Preregistered independent preflight (Python BigInt, not Rust CI)

The independent local affine EC implementation used the published original curve, generator, four original B8 windows, SplitMix64 seed `0x54524e5333463301`, and 128 planted signed weighted relations per window, including repeated base selections. The checker regenerated all points without reading Rust-produced witnesses.

| Frozen check | Independent local outcome |
|---|---:|
| Planted finite signed relations | **512/512** |
| Exceptional partial sums | **0/512** |
| Valid selector equations `H_i=0` | **4,096/4,096** |
| Semaev F3 identities | **3,584/3,584** |
| Product `K_i=0` identities | **3,584/3,584** |
| Public target x+1 mutations rejected on unchanged suffix | **512/512** |
| Independent sign-lift / full group identity | **512/512** |
| Generic group-sum table calls during candidate polynomial evaluation | **0** |
| Public-only unknown-index extraction / new ECDLP logs | **0 / 0** |

For the four original windows, exact independent expansion of 28 bivariate `K_i` polynomials gave **26 at 289 monomials**, one at 285 and one at 288. The two shortened supports occur at window 0 slots 0 and 1; all 26 remaining polynomials have 289 nonzero coefficients in the original field. Each has top bidegree term `U^16 V^16` with coefficient one. **This measures algebraic sparsity, not elapsed F4 solving time.**

The original-crate Rust candidate is [`prime31_transformed_slot_f3.rs`](../native/examples/prime31_transformed_slot_f3.rs); [independent verifier](../native/verify_prime31_transformed_slot_f3.py), [source-specific workflow](https://github.com/isogeny-labs/crypto/blob/automation/prime31-horner-xy-chain-20261119/.github/workflows/prime31-transformed-slot-f3.yml). It pins the original `Arithmetic`/`PRIME31` implementation at commit `1df714645aa683cf975bc76fcacb9842cb4780fd`. The GitHub Actions run is a distinct gate: **do not infer original-Rust PASS from Python preflight**. Independently signed PCC remains unverified.

## Prior lead and novelty boundary

- [#4812](https://github.com/isogeny-labs/crypto/pull/4812): prior 21-variable/degree-4 explicit Horner, 14-variable/degree-10 eliminated Horner, and eight slot-root resultants. The present direct transformed-slot system is algebraically equivalent to that target-constrained relation language, but reduces the *maximum step degree* without increasing its 14-variable count.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) and [#4822](https://github.com/isogeny-labs/crypto/pull/4822): live public-only RR9/F4 and sparse-Wiedemann extraction owners. Do not copy their engines.
- [#4816](https://github.com/isogeny-labs/crypto/pull/4816): complete fixed-*unweighted* quotient negative baseline; it cannot invalidate the **weighted** 8-slot language.
- [#4728](https://github.com/isogeny-labs/crypto/pull/4728): complete rank-31/1,024-Q original Rust grader, but it uses forbidden generic four-sum tables, so its completeness cannot be ascribed to algebraic F4 extraction.
- Merged [#4532](https://github.com/isogeny-labs/crypto/pull/4532): strongest historical matched shared BSGS **170.113 ms for 1,024 logs**. With 1,312 previous relation attempts, the entire extractor could consume at most **129.66 μs/attempt** under an impossibly generous zero-other-cost assumption; same-machine fresh controls required.
- `crypto-autoresearcher/ideas/mythos-20261109-relations.md` previously examined resultant towers and warned about joint solving costs; `cryptanalysis/suite/src/cryptanalysis/prime_orbit_index_calculus.rs` contains automorphism/generic factor-base work. This is **not** a novelty or asymptotic improvement claim.
- Primary external background: [Kousidis–Wiemers 2019](https://doi.org/10.1515/jmc-2017-0022) on first fall degree and [Kosters–Yeo 2015](https://arxiv.org/abs/1503.08001) on summation polynomial caveats. See also [complexity bounds on Semaev's naive IC](https://doi.org/10.1515/jmc-2019-0029). Shared research-KB MCP was inaccessible in this runtime; searched accessible GitHub sources and literature instead.


## Original-source Rust CI and independent verification — November 19, 22:38 UTC

[GitHub Actions source-specific push run #44630431346](https://github.com/isogeny-labs/crypto/actions/runs/44630431346) on candidate commit `5a0942768fd81be66cb0bb4db38f04d7200fb8a6` finished **every original-Rust and independent replay job step successfully**; [artifact #14580424253](https://github.com/isogeny-labs/crypto/actions/runs/44630431346/artifacts/14580424253) is available. The actual original Rust 1.100.0 library passed **40/40**, candidate example **1/1**. Original Rust produced a JSON cohort whose strict registered `jq` gate checked **512/512 finite relations, 4,096 H_i zero equations, 3,584 S3, 3,584 factored K_i, 512 poisoned-target rejections, 512 post-solve sign reconstructions, and zero generic sum-table calls**. Separately authored Python BigInt affine arithmetic reproduced **all 512 source cases**, their certificates, all K coefficient term counts and mutated controls.

- Original JSON SHA256: `8efc9f1a2d5d98fe3f5d2b46f844ae6a81150e80b702361017e80d029d544cdb`.
- Independent verifier JSON SHA256: `be00dd6de9dcb67914982d7cdbc39f0986e5477fb9c55e3b0cac7e893c32523a`.
- Uploaded artifact ZIP SHA256: `e478f23840168d661a306a1cbf665bcd8f80a759cae4e6863ad2088717c3325f`.

An earlier sparse-checkout CI attempt **failed during compilation of the unchanged original library**, because `f4_fp_tower.rs` and `sig_fp_tower.rs` were omitted from the checkout, not because the new S3 experiment failed. Corrected the sparse checkout at commit `5a094276`; retain the failed run [#44629961677](https://github.com/isogeny-labs/crypto/actions/runs/44629961677) as an auditable precursor. This is a mathematical and Rust-original-source validation, **not** an independently signed PCC validation, public-only F4 solution, or complete ECDLP speedup.

## Registered next falsifier

Have the current #4692 F4 solver load **all four algebraic systems** for identical residuals: 21-variable quartic Horner, 14-variable degree-10 eliminated Horner, the new 14-variable selector/S3, and the 7-variable degree-32 product chart. Freeze variable order, elimination degree, memory cap and full 46-positive/18-negative B8 custody. For each query require recovered unknown original factor indices, signed points and `[d]G=Q` downstream, **without hidden signs, planted witness data or generic sum-table accesses**. Report SAT, certified UNSAT, timeout, false positives, F4 matrix nonzeros, peak RSS, operations, and 1/64/1024 target setup amortization. Only if the new chart passes should it replace the generic grader in #4728 and be tested against matched BSGS, rho, and the best current index-calculus implementation on the same runner.

**Current gate:** mathematical encoding and independent planted-case preflight are positive; native Rust source-specific CI and *actual public-only F4 extraction* are separately required for a verified solver performance claim.
