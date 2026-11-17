# Prime31 public-residual RR9 coefficient support criterion

2026-11-17 UTC · Source PR #4539 · **[pcc:unverified]**

## Concrete change

Extended the existing native Rust `prime31_rr9_evaluation.rs` code from a witness checker that was given eight signed leaves to a *coefficient-only* support predicate, called with `(F,R,H_B)` and **no leaves**. It computes `N_F=U^2-(X^3+aX+b)V^2`, performs exact synthetic division by `X-x(R)`, normalizes to a monic degree-eight `M_F`, and tests **`H_B^8 mod M_F = 0`** using polynomial binary exponentiation. No group additions, generic four-sum lookups or square-root enumeration occur inside this predicate. It is a verification oracle for a supplied `F`, **not** a method for finding `F` from an unknown public target.

## Additional result: seven-variable signed public-target chart

The same existing Rust example now contains `rr9_to_signed_chart` and `rr9_from_signed_chart`. For any nonidentity `R=(r,s)`, the signed equation `F(-R)=0` is equivalent to `U(r)-sV(r)=0`, hence to the unique polynomial quotient `U(X)-sV(X)=(X-r)A(X)` with `deg A<=3`. Therefore
```
F(X,Y) = (X-r) A(X) + (Y+s) V(X),  deg A,deg V <=3.
```
For a true nonidentity eight-leaf relation, `V_3` cannot vanish, so normalize `V_3=1`: precisely **seven** scalar unknowns `(A_0..A_3,V_0..V_2)`. This removes the signed constraint identically rather than asking a polynomial solver to rediscover it. The new tests round-trip each planted F and its deliberately altered same-target F through this chart and reject lower-pole / identity branches. This is a more useful starting system for actual algebraic extraction: `H_B^8 mod M(A,V)=0` has eight coefficient constraints in these seven variables, where `M` is the degree-eight monic x-norm quotient.

### Direct monic residual polynomial: eliminate norm and division

There is a further exact identity. Since `s²=C(r)` and `C(X)-C(r)=(X-r)(X²+rX+r²+a)`, put `D_R(X)=X²+rX+r²+a`. For `F=(X-r)A+(Y+s)V` with monic cubic `V`,

```
N_F(X) = (X-r)[(X-r)A(X)^2 + 2s A(X)V(X) - D_R(X)V(X)^2].
M_(A,V,R)(X) = D_R(X)V(X)^2 - (X-r)A(X)^2 - 2s A(X)V(X).
```

Thus `M` is directly monic of degree eight, requiring **no full x-norm construction or synthetic division**. This makes the prospective elimination system explicit and low-dimensional:

```
unknowns: A0,A1,A2,A3,V0,V1,V2  (V3=1)
equations: all 8 coefficients of H_B(X)^8 mod M_(A,V,R)(X) equal zero.
```

The existing Rust file now implements `rr9_signed_chart_monic_factor` (source commit `4680ec0578eb85138c84c5afdad4f9d4619f851f`) and checks all 64 + six signed-leaf fixtures against the *separately computed full norm quotient*, including six repeated-point divisors. The 70 target-preserving mutations must yield a distinct unsupported factor. The expression's algebraic identity follows by direct expansion; its **numerical Rust gates passed source-specific CI #44039910636** (producer-level only). This is a lower-cost *verification representation*, not evidence of a cheaper complete ECDLP solver.

The chart construction and round-trip are algebraic reductions, **not** an elimination algorithm recovering the seven variables from R. The chart source was introduced in commit `67a3f6302cf2d72eee3b56946c27902625596e67` and checked in source-specific passing CI #44039385332, subsequently extended and rechecked by #44039910636.

The original signed norm interpolator, Hermite jets, determinant gates, actual cofactor-one `p=2147483647` curve, first-x31 and Dickson-depth-six31 bases are reused byte-for-byte outside this addition. The existing GitHub Action uses pinned original native Rust 1.100 source as main's old native package is incomplete.

### Exact equivalence, not a complexity theorem

Write `E: y^2=C(x)=x^3+ax+b` over odd-characteristic `K`, with a finite factor base `B` whose x-coordinates are distinct and rational and whose rational y-coordinates are nonzero, and `H_B=∏_(P∈B)(X-x(P))`. Let `R∈E(K)\{O}`, and `F=U(X)+Y V(X)∈L(9O)` with `V_3≠0`. Put `N_F=U^2-CV^2`.

**Claim (conditional on a supplied F):** `F(-R)=0` and `M_F := N_F / [-V_3²(X-x(R))]` being monic of degree eight and dividing `H_B^8` holds if and only if the other eight zeroes of `F` (with multiplicities) lie among the signed points `±B`. Then their group sum equals `R`. Every eight-point decomposition of `R` over `±B`, including repeated points, produces such an `F` unique up to scalar.

**Proof:** `ord_O(F)=-9` because `V_3≠0`; thus `F` has exactly nine zeroes on the smooth complete curve counted with multiplicities. The degree-nine `x`-norm polynomial has the x-coordinate of every zero of `F` with its exact intersection multiplicity, because `N_F(x)=F(x,y)F(x,-y)`. The signed condition forces one zero to be `-R`. The remaining degree-eight zero divisor has x-polynomial `M_F`. `M_F|H_B^8` forces each remaining zero's x to be one of the finite rational x-coordinates of `B`; its corresponding elliptic point is either `P` or `-P`, with multiplicities handled without a squarefree assumption. The divisor of `F` has sum `O`, hence the eight-point sum is `R`. Conversely, for any effective eight-point divisor `D` of signed base points summing to `R≠O`, Riemann–Roch gives `dim L(9O-D)=1`; its nonzero function has ninth zero `-R`. It cannot have pole order eight, since that would make `D-8O` principal and its sum `R=O`. Therefore `V_3≠0`, and the x-norm factors as required. □

There is no assumption that the eight x-roots are distinct; both `P` and `-P` may occur, and repetitions are governed by local orders. `R=O`, infinity among factors, rational y=0, and low pole-order cases are **rejected**, not automatically treated as negative relations. For the actual p31 curve, odd prime group order excludes rational nonidentity 2-torsion, so each rational factor-base point has nonzero y.

**Degree-of-freedom check:** `L(9O)` is 9-dimensional; quotient by global nonzero scaling and the one independent linear `F(-R)=0` condition leaves **7 projective coefficient coordinates**. The congruence `H_B^8 mod M_F=0` yields eight coefficient equations in those coordinates. The resulting algebraic system can have multiple solutions and high elimination degree. A verifier with a cheap remainder does not make the *extraction* of those seven coefficients cheap.

## Source and negative-test gates

The prospective protocol is `RR9_PUBLIC_NORM_MEMBERSHIP_PROTOCOL_20261117.md`, committed at `104093bfa59f6294d56bc9412992f5442a2e56f5` **before** the Rust implementation `799bfcc7e6567f0a0473e665836c0f4924c530d9`. Test design is fixed:

- 64 actual original-curve supplied-leaf positives (32 per factor base) are required to satisfy the new coefficient-only public norm predicate.
- 64 corresponding mutations `F'=F+X-x(R)` preserve the signed `F'(-R)=0` condition exactly, but are required to fail support divisibility.
- Six repeated-leaf controls (two bases × multiplicity 3/4/8) must pass; six signed-constraint-preserving mutations must fail. The RR9 test pre-existing 128 determinant positive/negative checks, naive diagonal false-positive, Hermite and ninth-jet falsifiers remain unchanged.
- A separate polynomial-only toy test requires `(X-7)^8 | (X-7)^8` but rejects `(X-7)^8 | (X-7)^7`, exercising non-squarefree high multiplicity without group arithmetic.
- Pinned GitHub source-specific Rust CI must compile and pass. **The later source-specific CI #44039910636 passed all asserted 70 positive and 70 negative certificate/monic-factor controls.** Rust compiler and EKS runner are unavailable locally in the current session; the source CI is the execution gate.

No timed comparison is conducted by this test. The status is **new mathematical interface and implemented check; exact-source Rust CI passed, independent signed PCC pending**.

## Exact-source verification completed (producer level; no signed PCC)

**SUCCESS:** [GitHub Actions run #44039910636](https://github.com/isogeny-labs/crypto/actions/runs/44039910636), job `133891681530`, on immutable source `4680ec0578eb85138c84c5afdad4f9d4619f851f`, pinned native Rust 1.100.0 + original source `1df714645aa683cf975bc76fcacb9842cb4780fd`. Log confirms **40/40 library tests**, **5/5 RR9 example tests**, and the complete original-curve program printed **32 true + 32 false determinant decisions for each actual B31 factor base** (128 total). The Rust example's assertions additionally checked **64 distinct and six repeated-point public coefficient support certificates**, **70 same-target altered-polynomial negatives**, **70 signed-chart round-trips**, **70 direct monic factor identities versus the independently computed full x-norm quotient**, and retained duplicate/Hermite/ninth-jet controls; no assertion failed. All fixture classes remain producer-generated, not blinded unknown-scalar discrete logarithms.

[Raw deterministic JSON artifact #14308623652](https://github.com/isogeny-labs/crypto/actions/runs/44039910636/artifacts/14308623652), 13,663-byte ZIP, recorded SHA-256 `40664cacbb8f80ab6d45f20d2c21a9b8688c5312082aa2d0a1d14c781967d7fe`. This is an exact-source CI + self-check result **[pcc:unverified]**, not an independent PCC verdict, peer review, formal proof audit or measured speedup. Prior passing run #44039385332 checked the signed-chart-only revision; the newer run above supersedes it for direct monic-factor assertions.

## Prior-lead and full-cost frontier triage

| Prior line | Established scope / disposition |
| --- | --- |
| Crypto #4466, #4451 | Merged RR9 Hermite/norm and eight-leaf zero-fibre results. Reuse; no novelty claim for Riemann–Roch or polynomial norms. |
| Crypto #4228, #4349, #4188 | Reusable Rust quotient/subresultant/rootfinding and m4 norm/GCD work. Next extraction must use these, not build a new Python ECDLP solver. |
| Crypto #4373; crypto-autoresearcher #3301/#3305 | Censored/negative anchored m8 F4, Semaev-alternative and Vieta/tower trials. No verbatim reruns. |
| Crypto #4484/#4486 | Buffer reuse and linear chord modifications were null/noisy, not a new algorithm. |
| Crypto #4513 (merged) | Complete 1024-Q mixed 4+4 index-calculus-derived Rust solver, but its oracle enumerates half-sums. |
| Crypto #4532 (open) | Same-cohort shared-precomputation BSGS control; **170.113 ms for BSGS m=2^20 versus 2,221.474 ms for mixed IC, a 13.06× generic advantage** on one unisolated Actions runner. This is the now-required batch frontier, producer-only, with BSGS memory material. |

The 1024-Q historical cost figures were **not newly rerun** here. Fresh same-source complete unknown-scalar logs: **0**; fresh cold performance results: **0**. No source here supports a novel non-generic speedup.

Primary literature checked: Amadori–Pintore–Sala 2018 (DOI https://doi.org/10.1016/j.ffa.2018.01.009) and McGuire–Mueller, `A New Index Calculus Algorithm for the Elliptic Curve Discrete Logarithm Problem and Summation Polynomial Evaluation`, https://eprint.iacr.org/2017/1262.pdf. McGuire–Mueller's original §6 complexity discussion shows their random-factor-base variants remain worse than square-root attacks; S9 interpolation itself is not a new hardness break. KB unavailable (MCP namespace not installed; no checkout to invoke wrapper); prior research was read from GitHub PRs, original code and records. No external benchmark claims.

## Next decisive non-generic experiment

1. From public `R`, express `F(-R)=0` to eliminate one coefficient; fix projective scale via `V_3=1` (while retaining a separate low-pole branch). Build the eight coefficient equations of `H_B^8 mod M_F` in seven remaining variables via modular reduction, not expansion of `H_B^8`.
2. Run existing Rust quotient/subresultant/F4/FGLM on B4 and B8 original-curve points, with blind residual inputs. Compare algebraically extracted solution sets against separately enumerated test-only controls including both signs, diagonals, known negatives and infinity. Require ideal equivalence or document precise lost/extraneous components. Record solver degrees, rows, rank, memory, wall, censored failures and all candidate verification.
3. Advance to B31 only if B4/B8 passes without enumerating supports. Require 96+ fresh blinded unknown-scalar complete solutions, rank/relation collection and every cost charged, against tuned rho and shared-precomputed generic BSGS on the same host and targets, with independent PCC checks.

This is a concrete **step toward** an algebraic relation-extraction oracle, not yet a cheaper complete solver.
