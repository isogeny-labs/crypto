# EXP-R31-RR9-PUBLIC-NORM — exact public-only coefficient certificate

**Prospective scope, 2026-11-17 UTC.** Real ordinary cofactor-one curve `p=2147483647`, `E:y^2=x^3+847514846x+2087444724`, prime group order `n=2147446831`. The reused native Rust `prime31_rr9_evaluation.rs` remains the implementation. No caller scalar, known leaves, group addition or generic half-sum tables are inputs to the proposed *membership predicate*; planted leaves remain only in an outside fixture builder. **No coefficient search, non-generic ECDLP solver or speedup is claimed.** [pcc:unverified]

## Exact statement to test

Let `B` consist of rational affine representatives with pairwise distinct x-coordinates and `H_B(X)=product_(P in B)(X-x(P))`. For `R != O`, a nonzero `F=U(X)+Y V(X)` with `deg U<=4`, `deg V<=3`, `V_3!=0`, define
`N_F=U^2-(X^3+aX+b)V^2`, and (provided `F(-R)=0`)
`M_F=N_F/(-V_3^2 (X-x(R)))`, a monic degree-eight polynomial.
Then

`F(-R)=0 and M_F | H_B^8`

is **equivalent to** the remaining eight zeroes (counted with multiplicity) of `F` being signed points from `B`. Consequently they sum to `R`. Conversely every eight-leaf signed decomposition of nonidentity `R` determines such `F` uniquely up to scale by Riemann–Roch. Both orientations and repeated points are retained; `M_F` is **not** a distinct-root-only constraint.

Proof idea: `F` has exactly nine finite zeroes since its pole order at `O` is nine. The x-norm records their x-coordinates including multiplicities. The rational support of each root of `H_B` is `{P,-P}`. `F(-R)=0` chooses the ninth signed point; removing one copy leaves eight base-supported zeroes. Their sum is `R` because the divisor of a rational function sums to `O`. The converse uses the one-dimensional `L(9O-D)` for any effective degree-eight signed divisor `D`, and `R != O` forces pole order nine. Infinity, noncanonical input and `V_3=0` require separate branches, not acceptance.

This tests **membership of supplied coefficients F**, not how to find them from R. The previous RR9 norm equality tested an already supplied eight-leaf factorization; the new public predicate is a stricter *interface separation* for future nonenumerative elimination.

## Source-exact deterministic controls (no timing claims)

1. Use existing 32+32 planted distinct positive certificates on the two native B31 bases. Construct `H_B` from public x-coordinates alone; then the public predicate receives only `(F,R,H_B)`. Assert its synthetic division has zero remainder, monic quotient degree 8, and `H_B^8 mod M_F = 0` using modular polynomial exponentiation.
2. For every positive, change `F` by adding `X-x(R)` to `U`: this **preserves F(-R)=0** but should break base support. Assert false, counting any unexpected true as a scientific collision needing analysis, not silently skipping it.
3. Retain all 128 original-curve determinant positive/negative decisions, naive duplicate false-positive, Hermite multiplicities `3,4,8` and ninth-jet falsifier. On all six repeated-leaf originals, public membership should pass and the constrained mutation should fail.
4. Unit-test polynomial modular remainder and `H^8` divisibility on repeated-root toy polynomials. The implementation must not call elliptic group arithmetic or enumerate candidate leaves inside membership.
5. Existing pinned Rust 1.100 workflow on pull_request is the expected executable verification. Any compile or test failure stays a failure, is fixed in a distinct commit and never reported as a pass.

## Prior sources and contrast

- Merged crypto #4466: RR9/Hermite signed norm interpolation; #4451: zero-fibre obstruction; #4228/#4349: quotient/rootfinding; #4188: earlier m4 norm/GCD; #4373: censored anchored m8 F4. No new norm novelty claim.
- Merged #4513 and open #4532: complete reusable mixed-base 1024-Q Rust IC, and matched shared BSGS generic control **170.113ms versus IC 2221.474ms** on one unisolated producer-run [pcc:unverified]. This is the correct batch-cost frontier, not rho-only. BSGS reference has 2^20 baby entries and material memory.
- crypto-autoresearcher #3301/#3305 and crypto #4484/#4486: negative/censored nonenumerative and minor APS-optimization leads; do not repeat.
- Primary literature: Amadori–Pintore–Sala (Finite Fields and Their Applications 51, 2018, DOI 10.1016/j.ffa.2018.01.009); McGuire–Mueller (ePrint 2017/1262, sections 2–6) has non-Gröbner prime-field summation-polynomial alternatives and explicitly estimates complexity worse than square-root group methods under its random-base model. Elliptic Riemann–Roch and polynomial norms are classical.
- KB MCP not exposed in this tool runtime and no repository checkout/executable wrapper here; fallback was bounded repo PR, primary-source and session-log search. Conductor and Rust toolchain also unavailable in this runtime (no executable `cargo`).

## Next real extraction gate

Eliminate the seven remaining projective coefficient degrees of freedom after the linear condition `F(-R)=0`, **without enumerating signed supports**. The `H_B^8 mod M_F=0` condition gives eight explicit polynomial remainders in the coefficients; verify exact fibre equality at B4/B8 against independently enumerated *test only* solutions, including signed collisions and infinity branches. Instrument F4/FGLM, root isolation, saturation, rank, memory and all failed calls. Only after success attempt complete hidden-Q logs on B31 and frozen same-host cold comparisons with shared BSGS and tuned rho. No performance inference from a membership check alone.

## Extension B — signed public-residual seven-coordinate chart (frozen before implementation)

For a nonidentity public `R=(r,s)`, the condition `F(-R)=0` is exactly `U(r)-sV(r)=0`. Polynomial division gives a **unique** `A` of degree at most three satisfying `U(X)-sV(X)=(X-r)A(X)`. Hence

`F(X,Y)=(X-r)A(X)+(Y+s)V(X)`,

with eight coefficients `A_0..A_3,V_0..V_3`. All relevant eight-leaf nonidentity solutions have `V_3!=0`; set `V_3=1` projectively, leaving precisely **seven** affine unknowns. This chart makes the signed orientation condition automatic, and the nonenumerative extraction target is the eight remainder equations in these seven variables.

**New source-exact falsifier, no timed performance claim:** reconstruct `F` from this chart for all 64 already planted nonzero-R coefficients and all six repeated-point certificates; assert equality up to the unique `V_3` scaling, and that reconstructed `F(-R)=0`. The changed-support `F'=F+(X-r)` must also remain on this signed chart, so a negative test cannot pass by merely violating `F(-R)=0`. Reject `R=O` and `V_3=0`; require test assertions for both. No generic ECDLP enumeration, new solver or speedup claim.

## Extension C — direct monic eight-factor polynomial, no norm division (frozen before source)

Write `R=(r,s)`, `C(X)=X^3+aX+b`, `D_R(X)=X^2+rX+r^2+a`. Since `s^2=C(r)`, `C(X)-s^2=(X-r)D_R(X)`. On the signed chart `F=(X-r)A+(Y+s)V`, its x-norm is identically
```
N_F(X)=(X-r)[(X-r) A(X)^2 + 2s A(X)V(X) - D_R(X)V(X)^2].
```
Therefore, after `V_3=1`, the *monic degree-eight* residual factor polynomial is directly
```
M_(A,V,R)(X) = D_R(X)V(X)^2 - (X-r)A(X)^2 - 2s A(X)V(X).
```
This uses polynomial multiplication and subtraction only; no synthetic norm division, no supplied leaves and no elliptic group arithmetic. Its leading coefficient is one because `deg V=3`, `deg A<=3`. It makes extraction's eight congruences explicitly `H_B^8 mod M_(A,V,R)=0` in the seven normalized chart coefficients. This is an exact classical algebraic simplification, **not** a faster discrete-log solver.

**Precommitted tests:** on the same 64 distinct and six repeated positive producer fixtures, compute `M_direct` and assert coefficient equality to the quotient of the full norm `N_F/[-V_3²(X-r)]`, after scaling `F` to `V_3=1`. Assert monic degree eight, `H_B^8 mod M_direct=0`, and target-preserving mutation changes the monic polynomial and fails divisibility. Reject invalid point / V3=0. Preserve the old independently computed norm and Hermite controls. Do not run a new timing experiment or make a speedup claim from polynomial formula simplification.
