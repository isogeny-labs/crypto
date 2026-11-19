# Prime31 public RR9 norm-resultant/Jacobian front end and fixed-support no-go gate

**2026-11-19 UTC.** Source-specific GitHub CI: **pending** at initial publication; Python algebraic preflight was independently run locally; signed PCC remains `[pcc:unverified]`. This is a public-polynomial component and an exact necessary scaling bound, **not** a cheaper complete non-generic prime-field ECDLP solver.

Prospective registered protocol: [RR9_PUBLIC_GRADIENT_PROTOCOL_20261119.md](RR9_PUBLIC_GRADIENT_PROTOCOL_20261119.md).
Native original-crate example: [prime31_rr9_public_gradient.rs](../native/examples/prime31_rr9_public_gradient.rs). Independent BigInt verifier: [verify_prime31_rr9_public_gradient.py](../native/verify_prime31_rr9_public_gradient.py). Exact GitHub workflow: [.github/workflows/prime31-rr9-public-gradient.yml](../../../.github/workflows/prime31-rr9-public-gradient.yml). Draft PR #4772.

## What is actually computed

On the unchanged ordinary cofactor-one original curve over `F_2147483647`, `a=847514846`, `b=2087444724`, prime subgroup order `n=2147446831`, choose the original first 8 rational-x factor-base points `x=(0,1,2,3,5,6,8,9)`. Compare legacy signed weights `(1,3,5,7,11,13,17,19)` with ternary `(1,3,9,27,81,243,729,2187)`. Both have 64 pairwise-distinct unsigned support abscissae at B8 on this original curve.

For public `R=(r,s)` and *candidate* seven RR9 coefficients `z=(a0,..,a3,v0,v1,v2)`, let

`A(X)=a0+a1 X+a2 X^2+a3 X^3`, `V(X)=X^3+v2 X^2+v1 X+v0`, `D_r(X)=X^2+r X+r^2+a_curve`.

Then the public norm polynomial is

`M_R,z(X)=D_r(X)V(X)^2-(X-r)A(X)^2-2s A(X)V(X)`.

For each signed weighted slot `j`, let `L_j(X)=∏_{b=0}^7(X-x_{jb})` be the **public** unsigned support polynomial and

`H_j(R,z)=Res_X(L_j,M_R,z)=∏_{b=0}^7 M_R,z(x_{jb})`.

The candidate API returns all eight exact `H_j` and their `8×7` Jacobian; it uses no generic EC 4+4 table or curve addition and uses no finite-field inverse in the evaluated predicate. For one fixed `x`, write `U=X-r`, `D=D_r(X)`, `A=A(X)`, `V=V(X)`. The analytic partials are

`∂M/∂a_i = (-2UA-2sV)X^i` for `0≤i≤3`; `∂M/∂v_i = (2DV-2sA)X^i` for `0≤i≤2`.

Prefix/suffix products compute `∂H_j/∂z_k = ∑_b (∂M(x_jb)/∂z_k)∏_{c≠b}M(x_jc)`, without division even at vanishing factors. An independent truncated-dual-number construction tests all 56 derivatives per chart.

**Preflight only, not yet a source-specific CI claim:** 16 planted signed-root fixtures in each of two weight arms (32 total), 32/32 eight-zero-resultant charts, 32/32 Jacobian rank `7/7`, and 32/32 predetermined `a0+1` mutations rejected. The grader uses the known signed leaves to interpolate a chart. No unknown chart or logarithm is recovered from public R by the candidate method.

The full eight-resultant public system is `8` polynomial equations in `7` unknown coefficients, each of degree at most `16`. A dense total-degree-16 monomial space in 7 variables has `C(23,7)=245157` monomials (a ceiling for one polynomial's monomial support, **not a measured F4 matrix rank or a complexity lower bound**). Its local Jacobian rank on planted certificates indicates isolated local solutions, but supplies neither a global solution algorithm nor a bound on solving degree.

## Exact necessary fixed-arity scaling theorem (new falsifier)

Let `G` be any cyclic subgroup of order `n`. Suppose a membership relation consists of `m` independently signed factor-base labels from a fixed base of cardinality `B`, with any predetermined slot weights. Since every one of the `(2B)^m` ordered signed words maps to at most one group element, its representable subset `S⊆G` has size at most `(2B)^m`. Consequently, for a **uniform** public residual `R`,

`Pr[R∈S]=|S|/n ≤ min(1,(2B)^m/n)`.

The inequality holds without randomness or independence assumptions on the word images. For `w` separate windows tested per residual, the union-support bound is at most `w(2B)^m`. For independent uniform shifts with per-query success probability `q`, expected trials `1/q ≥ n/[w(2B)^m]` whenever this bound is nontrivial; algebraic membership cost is optimistically taken as free.

At original Prime31 parameters B=8,m=8, there are exactly `16^8=2^32=4294967296` ordered signed words per window; their exact mean multiplicity over all group elements is `4294967296/2147446831 = 2.000034289...`. Four windows give at most `2^34` possible distinct images. Hence the bound is not restrictive at n≈2^31, and neither proves 100% hit rate nor proves the heuristic independent-image occupancy model.

At **hypothetical** subgroup order approximately `2^256`, the same fixed one-window representation has uniform hit probability at most `2^-224`, and even testing all four such windows per shift gives probability at most `2^-222` and expected at least `2^222` uniform shifts. This would be far worse than `~2^128` generic rho work **for this fixed-width, fixed-B architecture**. It is not an unconditional generic lower bound for every index-calculus or non-generic ECDLP algorithm. To extrapolate, `m log2(2B)` must grow with `log2 n`; that changes the polynomial degrees, elimination complexity and setup cost.

## Prior leads checked to avoid duplication

- **#4692**: Draft shared-coefficient RR9/F4 membership and remainder-tower proposal, still without a source-validated public-only unknown-index extractor on B8. This component is an independently replayable input to that lane, not a replacement solver.
- **#4728**: 70-term determinant / Fourier / zeta transforms and four overlapping B8 windows, full 31-rank/1024-log **generic grader**. It gets its signed indices from group-sum tables.
- **#4770**: 1.262× improvement for ternary signed weights in the same-runner generic-grader 1024-target Rust experiment; not non-generic, and its better support does not prove an algebraic F4 advantage.
- **#4532**: Historical fully charged 1024-log shared-BSGS 170.113 ms on another runner, beating prior IC control; not a same-runner matched comparison here. **#4714** M6 generic table solver and **#4722** formal support census.
- **#4720**: Signed-pair projector, also needs unknown-index discovery upstream.
- `isogeny-labs/crypto-autoresearcher`: SA-04, SA-18 negative RR-norm/determinant screening, and SA-19 Edwards-coordinate limitations. `isogeny-labs/cryptanalysis`: binary Semaev/SAT instrumentation, not a demonstrated Prime31 solver.
- Published prior art: Petit–Kosters–Messeng, *Algebraic Approaches for the ECDLP over Prime Fields*, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, *On the DLP for Prime-Field Elliptic Curves*, FFA 51 (2018), DOI 10.1016/j.ffa.2018.01.009. Shared KB MCP not available, so novelty is not certified.

## Prioritized next falsifiers

1. **Actual public-only selector**: feed only R, canonical B8 support and weights to an F4/XL/Groebner or other algebraic solver for `H_0=...=H_7=0`. Avoid planted z, signed leaves, any generic half-sum table and scalar of R. Test the frozen **46 positive/18 negative B8 natural corpus**; require exact original-curve signed witness replay and track timeouts instead of treating them as negatives. Both legacy and ternary weights.
2. **Cost / algebraic geometry gate**: record target-conditional polynomial sparsity, Jacobian rank and singular branches on roots, F4 degree/memory/time, and whether the rank-seven local charts allow effective Hensel/Newton continuation with an initial chart algorithm that itself is public-only.
3. **End-to-end**: substitute *only after successful gate 1* into the existing B8-window rank31 / 1,024 unknown public-log Rust pipeline; compare cold-1, 64, 1024 on the same runner against strongest available **index calculus, shared BSGS and negation-map rho**. Include all failed shifts, preprocessing, algebraic solving, matrix, verification, memory and signed independent replay.
4. **Nontrivial scaling**: grow `B` and/or `m` on a preregistered ordinary-cofactor-one prime-field ladder. Measure whether the algebraic extraction complexity grows more slowly than the generic rho frontier; no asymptotic claim from a single 31-bit field.

**Conclusion:** first auditable public-polynomial/Jacobian building block and exact architectural scaling falsifier. No discovered unknown indices, no non-generic complete ECDLP solve, no speed advantage established.
