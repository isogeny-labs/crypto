# Prime31 x-only Horner/F3 compression and signed witness lifting (2026-11-19)

**Scientific scope:** Exact algebraic reduction and planted-witness soundness tests, **not** public-target-only unknown-index relation extraction and **not** a whole ECDLP speedup. Same fixed ordinary prime-order curve as the earlier Rust Prime31 work. [pcc:unverified].

**Source:** `research/prime31_ic_20261115/native/examples/prime31_horner_xonly_sign_lift.rs`, reused original `prime31_algebraic_ic::{Arithmetic, PRIME31, check_curve}` from `1df714645aa683cf975bc76fcacb9842cb4780fd`. Independent `verify_prime31_horner_xonly_sign_lift.py` implements separate Python-bigint EC arithmetic. Preregistered protocol `HORNER_XONLY_SIGN_LIFT_PROTOCOL_20261119.md` was committed before implementation. CI workflow `.github/workflows/prime31-horner-xy-chain.yml`, source-specific run [#44578723868](https://github.com/isogeny-labs/crypto/actions/runs/44578723868) (pending final gate status at document creation).

## Mathematical result: exact signed x-only chart on finite intermediate states

Let `E: y^2=x^3+ax+b` over odd characteristic, `E(Fp)` have odd prime order, and the signed leaves draw from public finite factor-base points with fixed x roots. We want
`Q=P0+[2]P1+...+[128]P7`.
The previous x/y Horner system used 72 variables and 74 equations on its ordinary affine chart, with 56 low-degree step equations. Define `T7=P7`, `Ti=Pi+[2]T(i+1)`, set `t_i=x(T_i)`, `d_i=x([2]T(i+1))`. Retain only x unknowns.

The polynomial doubling relation
`4(t^3+at+b)d=(t^4-2at^2-8bt+a^2)`
has total degree four. Each addition relation is the standard degree-four Semaev F3 condition
`F3(x_i,d_i,t_i)=0`
where `F3(u,v,r)=(r-u)^2v^2-2((r+u)(ru+a)+2b)v+(ru-a)^2-4b(r+u)`.
Eight leaf constraints `H_i(x_i)=0` have degree eight; the public target supplies `t0=x(Q)`. Since t7=x7, there are exactly **22 variables and 23 equations before public-target substitution**, or **21 variables and 22 equations after**. For comparison, the previous full x/y chart has **72 variables** (70 when terminal x/y are substituted). Thus this reduces the unspecialized scalar variable count by **50 of 72 (69.44%)** and after terminal substitution by **49 of 70 (70%)**. It trades quadratic addition/doubling equations for quartics. Counts are structural, not measured F4 work.

### Conditional soundness and constructive signed recovery
Assume all x coordinates are finite, each input leaf lifts to one of the public factor-base points, and each intermediate Horner suffix is not infinity. Starting from either sign of the final point with x=t7, induct downwards. If `T(i+1)` has x=t(i+1), the rational doubling formula yields exactly `d_i=x([2]T(i+1))`. Its denominator cannot vanish for a finite rational point on an odd-order curve, because y=0 would be 2-torsion. Both the base point and doubled point lie on E(Fp). F3=0 therefore says `t_i` equals the x coordinate of either `P_i+[2]T(i+1)` or `-P_i+[2]T(i+1)` (allowing the finite nonidentity cases). Select the corresponding sign for that public base leaf. At the end `t0=x(Q)`, hence the assembled point is `Q` or `-Q`. If negative, negate all eight leaves simultaneously. Their weighted sum is now exactly the signed public target Q.

The sign-lifting algorithm uses **only recovered x-leaf and x-suffix coordinates, canonical public factor-base points, and public Q**, not planted signs. Its group arithmetic is charged as *post-extraction witness recovery/verification*. This is a conditional mathematical statement, not a generic declaration that all solutions are finite: intermediate infinity cases need an explicit exceptional branch, and repeated/x-matching degeneracies must be handled, not silently deleted. An algebraic solver must enumerate rational roots correctly and reject exceptional artifacts.

## Evidence and limitations

A separate local Python bigint arithmetic replay using the exact original curve, generator and preregistered SplitMix64 seed returned **1,024/1,024 finite planted signed chains**, **0 infinity exceptions**, **0 ambiguous sign assignments**, zero polynomial residuals, and 1,024 recovered signed weighted sums exactly equal to their public Q. This is **an internal mathematical sanity check** before source-specific Rust CI, not independent signed PCC and not a blinded public-only extractor benchmark.

The committed Rust experiment has two focused tests and a release 1,024-case experiment with four B8 windows, poisoned doubling-coordinate and target-x checks, and a full-cohort separate Python affine verifier. The workflow pins original arithmetic and Rust 1.100.0, checks original library tests plus the earlier x/y chart, hashes outputs and uploads raw artifacts. The independent validator checks every generated case and the first 16 saved recovered signed leaves; no generic precomputed group sum tables are used in polynomial checking. Pending CI outcomes must remain pending until verified.

## Prior lead audit and scientific priority

- [#4692](https://github.com/isogeny-labs/crypto/pull/4692): existing nonenumerative RR9/F4/remainder-tower owner and frozen **46 positive/18 negative** public-only B8 gate. Do not duplicate its extraction owner. After Rust validation, try this 21-variable quartic chart as an alternative input to the same solver and compare degrees, learned reducers, fill, peak RSS, negative-tail latency, signs and exact verified target recovery.
- [#4812](https://github.com/isogeny-labs/crypto/pull/4812): 72-variable saturated x/y Horner construction (256 planted fixtures); this report changes the **polynomial encoding**, not relation yield.
- [#4809](https://github.com/isogeny-labs/crypto/pull/4809): F3 two-factor roots are already classical. The exact B31 two-factor support ceiling **1,923** is too sparse for a whole relation pipeline, so don't present this F3 identity as a new rare-arity solver.
- [#4810](https://github.com/isogeny-labs/crypto/pull/4810): Kummer doubling permutes rational curve/twist x separately for odd full group order. This explains the nonzero denominator and prevents false branching interpretations; it does **not** solve a multivariate F4 system.
- [#4807](https://github.com/isogeny-labs/crypto/pull/4807), [#4808](https://github.com/isogeny-labs/crypto/pull/4808): weighted support/rank improves **generic relation grading** but does not provide nonenumerative extraction. This candidate applies to their powers-of-two Horner weights.
- [#4803](https://github.com/isogeny-labs/crypto/pull/4803), [#4801](https://github.com/isogeny-labs/crypto/pull/4801): unweighted support scarcity and all-sign low-degree interpolation obstruction; avoid reopening those branches.
- [#4728](https://github.com/isogeny-labs/crypto/pull/4728), merged [#4532](https://github.com/isogeny-labs/crypto/pull/4532): prior complete rank-31/1,024 target grader and **historical optimized shared BSGS 170.113 ms / 1,024 logs**. That BSGS result is from a different experiment; it is not a paired baseline in this study. Must beat same-runner fully charged IC frontier, optimized rho and shared BSGS to claim complete advantage.
- [#4679](https://github.com/isogeny-labs/crypto/pull/4679): existing Faugère–Lachartre F4 implementation; measure this encoding there instead of building a second F4 solver.
- Repository prior: `research/ic_tree_gluing_20261103`, `research/prime31_ic_20261115/native/M8_UNIFORM_RANK_CEILING_AND_F4_PILOT.md`; external `isogeny-labs/crypto-autoresearcher/experiments/semaev-alternatives-20261105/SUMMARY.md`, `isogeny-labs/cryptanalysis/suite/examples/pkm_tower_pilot.rs`. These contain related small-degree and quotient designs. No originality-by-absence claim; common Semaev and Kummer polynomial identities are known.
- Literature: [Yokoyama et al. (2020)](https://doi.org/10.1515/jmc-2019-0029), rigorous discussion of naive Semaev ideal and square-root-type Gröbner complexity; [PKM prime-field algebraic approach (2016)](https://doi.org/10.1007/978-3-662-49387-8_1). Degrees/variable counts alone cannot establish a fast non-generic ECDLP. The shared research-KB MCP was not accessible in this runtime; GitHub code/PR and web fallback review is bounded, not an exhaustive novelty audit.

## Next experiments, strictly prioritized

1. **Nonenumerative B8 parity:** Build the exact quartic+F3 sparse polynomials using Rust `f4_fp_tower` and submit only public target and public eight-point base, neither signs nor indices, for all frozen 46+/18− inputs. Run 21-variable x-only against 70-variable x/y (with full charts) and older RR9 remainder tower on identical seeds and host. Report all positives, negatives, exceptional cases and time/memory; no generic sum tables or label leaks. An F4 failure is negative evidence.
2. **Rank-31 complete ECDLP gate:** Only if public-only B8 succeeds, integrate into original rank31 relation collector and individual-log recovery; verify all 1,024 [k]G=Q. Cost setup, all failed/random shifts, F4 roots, rank, linear algebra, memory, and target-specific tails on same runner vs native optimized IC, 2^20 shared BSGS, optimized rho for 1, 64, 1024 targets.
3. **Scaling/robustness:** Six growing ordinary prime-order prime-field curves, same success/failure accounting; assess whether F4 solving degree/variables scale better than generic frontier. Do not confuse a 31-bit finite example with asymptotic subexponential ECDLP.

**Status at file creation:** 21-variable algebraic formulation and a local all-positive replay obtained; original-Rust CI and independent verification pending. No measured complete non-generic solver improvement. [pcc:unverified].
