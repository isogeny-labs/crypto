# Monic-in-t S3 quotient: an exact alternative to generic F4 for pair factor-base relations

## PRIOR-ART CORRECTION — REQUIRED comparison with existing original Rust (2026-11-15)

A repository code-search audit found an **already working S3 quotient/GCD abscissa oracle**, [`research/prime31_ic_20261115/native/examples/prime31_s3_quotient_gcd.rs`](../../prime31_ic_20261115/native/examples/prime31_s3_quotient_gcd.rs), documented and CI-passed in [S3_QUOTIENT_GCD_ORACLE_20261115.md](../../prime31_ic_20261115/native/S3_QUOTIENT_GCD_ORACLE_20261115.md), [original Rust CI #43741710811](https://github.com/isogeny-labs/crypto/actions/runs/43741710811), PR #4188. That earlier method regards S3(X,Y,z) as a quadratic in Y over Fp[X]/h(X), performs target-dependent monic normalization away from X=z, takes polynomial remainder and a GCD, then independently lifts signed relations. It already handles exceptional target-abscissa collisions and produced native (not fully charged) **B31 ~0.45–0.49 ms** and **B135 ~28–29 ms** algebraic stages. Prior project work also tested m4 quotient chains and public targets.

**Consequence:** an S3 quotient/GCD method, polynomial factorization, and prime31 Rust implementation are *not new*. This new symmetric `t=x1*x2` monic quotient representation removes a leading-coefficient inversion across finite s (a coordinate/presentation distinction), while the balanced rank-four factor tree is a proposed fast symbolic-construction refinement. Both remain unproven as improvements over the existing Rust method. **Compare to the source-bound November 15 GCD incumbent, not merely to F4/ordered Semaev**, before further engineering or novelty claims. The older oracle includes signed-group validation; this PR's current independent prototype checks only x-pairs and is therefore less complete. If matched candidate performance is not superior, reuse the incumbent and avoid maintaining parallel solvers.


Date 2026-11-22. Status **mathematical derivation and independent small-instance modular arithmetic check; native implementation and speed unresolved.** This is a **different** elimination from #4922's reusable Macaulay pivot projection. Priority/novelty not established. Do not represent it as an asymptotic breakthrough.

## Core observation

For `E/Fp: y²=x³+ax+b`, a **fixed** public target abscissa `z` and pair invariants `s=x1+x2, t=x1*x2`, Semaev's equation is

```
F_z(s,t) = t² + q_z(s)*t + r_z(s) = 0
q_z(s) = -4 z² - 2 s z - 2 a
r_z(s) = s² z² - (2 a z + 4 b) s + a² - 4 b z.
```

Critically, **the coefficient of t² is 1** for every z, so the quotient `R_z=Fp[s,t]/(F_z)` is always a free `Fp[s]`-module of rank two with basis `1,t`; no inverse of a target-dependent leading coefficient is needed.

Let `h(X)=prod_{w in W}(X-w)`, `P_k=x1^k+x2^k`, and the earlier exact trace moments `H_j = sum_{k=0}^B h_k P_(k+j)`, j=0,1. These are the **radical** pair-image equations. In `R_z`, compute by linear recurrence

```
P0=2, P1=s,
Pk=s*P_(k-1)-t*P_(k-2),
t*(u(s)+v(s)t) = -r_z(s)*v(s) + (u(s)-q_z(s)*v(s))*t.
```

Thus reduce each moment *during its construction*, without constructing an F4 Macaulay matrix:

```
H0 ≡ b0(s)+a0(s)t,   H1 ≡ b1(s)+a1(s)t (mod F_z).
```

The coefficients `a_i,b_i` are exact univariate polynomials over Fp. The recurrence has O(B) steps and O(B²) elementary coefficient work with ordinary dense polynomial arrays (not O(B^4)); faster univariate arithmetic may improve this, subject to measurement.

## Exact univariate elimination with complete exceptional branches

For any common zero (s,t) of H0,H1,F, the two univariate conditions hold:

```
D(s) = a0(s)*b1(s)-a1(s)*b0(s) = 0,
N(s) = b0(s)^2 - q_z(s)*a0(s)*b0(s) + r_z(s)*a0(s)^2 = 0.
```

Therefore every genuine rational solution s is a root of `C(s)=gcd(D,N)`. This is an **exact necessary projection** (not an iff in isolation). Recover all roots of C over Fp using finite-field polynomial rootfinding. For each such s:

- If a0(s) != 0, set `t=-b0/a0`.
- Otherwise if b0(s) != 0, reject.
- Otherwise if a1(s) != 0, set `t=-b1/a1`.
- Otherwise if b1(s) != 0, reject.
- If both remainders are identically zero at s, solve the **monic quadratic** `F_z(s,t)=0` for rational t.

In every branch, verify **all three original polynomials** `H0,H1,F`. Then independently recover `x1,x2` as roots of `X²-sX+t`, verify both are rational and in W, lift actual curve points, enumerate valid signs and verify the signed group equation (with infinity and 2-torsion branches). If D=N=0 identically as polynomials (possible pathological specialization), treat the query as **censored/fallback**; do not pretend a finite projection exists. This procedure is complete on ordinary finite charts provided rational-root extraction succeeds and the stated exceptional branches are implemented.

The scheme is especially attractive as a *non-table* pair-root oracle and a building block for pairwise m4 chains. However a pair-join oracle with even ~linear work in B is **not** enough by itself to establish a sub-rho m4 cold exponent; a measured sub-B target-query strategy, authentic rank and amortized setup are still required. Fast univariate factoring is not free. The previous radical DRL, #4922 sharing, #4901 real RHS and #4920 signed-pair controls remain required baselines.

## Independent arithmetic spot-check (not original Rust)

A standalone Python modular-polynomial prototype (kept out of the performance path) checked the exact quotient/remainder identities on the unchanged Prime31 curve for W={x(G),x(2G)}. With z=x(G), x(2G) and x(3G), the polynomial gcd C has degree one and includes exactly the relevant pair-sums among W. With z=x(10G), C=1 and there is no candidate. Explicit substitution into all remainder equations and F_z confirmed each genuine pair. This is not a representative yield experiment, not blind unknown-index recovery, and is not native Rust benchmark evidence.

## Admission gate

Implement the rank-two quotient recurrence and root extractor **in original Rust**; compare on the same B3..32 Prime31 public-Q targets to matched original F4, September DRL, #4922 reuse and #4920 table/original root finders. Record all construction, gcd, root factoring, sign/lift checking, retries and memory. Add positive and negative small-field exhaustive cases; include diagonal x1=x2 and intermediate-infinity group cases. If the non-table root extractor doesn't beat those controls, classify the negative result and stop that branch. **No speedup or priority claim here. [pcc:unverified]**


## Candidate improvement: divide-and-conquer factor multiplication in the rank-four algebra (unimplemented)

The linear Newton recurrence as currently implemented stores all `P_k` and costs `O(B²)` prime-field coefficient arithmetic to construct `H0,H1` for one target. There is a stronger algebraic route to **quasi-linear symbolic construction**, provided fast univariate multiplication is available. Work over the rank-four `Fp[s]`-algebra

```
A_z = Fp[s,t,X] / (t²+q_z(s)t+r_z(s), X²-sX+t).
```

It is free over `Fp[s]` on `1,t,X,tX`: both relations are monic in their respective extension variables, so there is no bad-leading-coefficient specialization. Multiply the factor-base polynomial **as a product of linear factors in this algebra**, using a balanced subproduct tree:

```
h(X) = PRODUCT_(w in W) (X-w) ≡ U(s,t)+V(s,t)X in A_z.
H0 = 2U + s V,
H1 = s U + (s²-2t)V.
```

Each `U,V` has the same rank-two `1,t` coordinates as the earlier oracle. For two factors represented by `(U,V)` and `(U',V')`, their product is

```
U_new = U*U' - t*V*V'
V_new = U*V' + U'*V + s*V*V'
```

where multiplications in `Fp[s,t]/(F_z)` use `t²=-q_z(s)t-r_z(s)`. A product node with `L` factors has univariate coefficient degree `O(L)`; fixed-rank ring products require only a constant number of univariate polynomial multiplications of degree `O(L)`. A balanced tree of B leaves consequently uses `O(M(B) log B)` field operations for the **symbolic H0/H1 construction**, where M(B) is the cost of degree-B polynomial multiplication. With near-linear M(B), this is `O~(B)` instead of the direct `O(B²)` recurrence. This is a mathematical conditional complexity bound; the existing Rust prototype does **not yet** implement a fast polynomial multiplication backend, and no measurements are asserted.

This is a meaningful **component asymptotic** improvement over the naive coefficient recurrence, not a non-generic ECDLP exponent improvement. The gcd, extraction/factorization, group-lift and target-rank costs remain, and merely processing B factor-base elements costs Omega(B) per fresh target unless preprocessing/shared structure avoids a full scan. In the optimistic four-summand relation-count model, an O~(B) per-target oracle with quadratic LA only *matches* the generic rho exponent 1/2 rather than beating it. A credible ECDLP exponent gain requires an amortized, sub-B per-shift oracle or a structural way to reuse target-independent work. As an additional check, compare the old Newton method and this tree on **identical W,z** by requiring identical four univariate coefficients and independently crosschecking all recovered S3 pairs before benchmarking.
