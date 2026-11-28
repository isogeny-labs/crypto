# Prime31 two-prefix RR9 hybrid elimination: prospective gate — 2026-11-19 UTC

## Fixed original curve and source
Reuse `research/prime31_ic_20261115/native` Rust arithmetic pinned to `1df714645aa683cf975bc76fcacb9842cb4780fd`, original nonsingular ordinary cofactor-one E/Fp: p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251). No replacement or deliberately weak curve.

This is a non-generic **algebraic component**, not an end-to-end non-generic ECDLP solver. The candidate hybrid gets **two supplied signed support points**, so an eventual public-only implementation must account for finding/trying the unknown prefixes. A grader may use planted witness indices only for validation. No generic 4+4 group-sum hash table in the candidate evaluator.

## Mathematical contract
For public R=(r,s) and F=(X-r)A(X)+(Y+s)V(X), A=a0+a1 X+a2 X^2+a3 X^3, V=X^3+v2 X^2+v1 X+v0, choose two *distinct* signed support points Q0=(x0,y0),Q1=(x1,y1) with xi != r. Let alpha_i=(yi+s)/(xi-r) and choose five free variables z=(a2,a3,v0,v1,v2). Define C_i=-alpha_i (xi^3+v0+v1 xi+v2 xi^2)-a2 xi^2-a3 xi^3; a1=(C1-C0)/(x1-x0), a0=C0-a1 x0. Then F(Q0)=F(Q1)=0 exactly, for every z, and every original 7-coefficient solution of those two linear equations arises uniquely.

For each of the remaining six slots j=2..7, public unsigned support x_(j,b), b=0..7, define M_R,z(x)= (x^2+r x+r^2+a_curve)V(x)^2-(x-r)A(x)^2-2s A(x)V(x) and H_j(z)=product_b M_R,z(x_(j,b)). They are six equations of degree at most 16 in five variables. Use the exact affine chain-rule Jacobian and inversion-free prefix/suffix products for H. Separately replay values and derivatives over prime-field BigInts. No claim that solving H=0 is fast or that a zero H uniquely selects a genuine signed relation.

## Registered fixtures and falsifiers
- First eight rational-x original-curve factor points (x=0,1,2,3,5,6,8,9); 8 slots with legacy weights (1,3,5,7,11,13,17,19) versus ternary (1,3,9,27,81,243,729,2187). All 64 unsigned support x in each arm must be distinct.
- Exactly 16 deterministic planted signed words per arm, with index=(3t+7j+j*j) mod 8, sign positive iff (t+3j) mod 4 < 2; original EC addition constructs public R for grader. Recover full chart using all eight leaves **only in the grader**, then feed just (R,Q0,Q1,z_free,public support) to the hybrid.
- Exact reconstruction must match all seven grader coefficients, all eight F(Qi)=0, six H_j=0, full column 6x5 Jacobian rank 5, and a2+1 predetermined mutation must spoil at least one H_j. Any exception fails.
- Report degree-16 dense monomial-size ceiling C(16+7-t,7-t) for t=0..4 known-prefix anchors, branches (2B)^t with B=8, and branch-times-monomial pressure; **this is not an F4 complexity measurement or lower bound**.
- Run 40 original Rust library tests, focused example tests, and an independent Python arithmetic replay under GitHub Actions using Rust 1.100.0; archive source-bound raw JSON. Do not quote timings as a whole-solver speedup. Signed PCC independent attestation pending.

## Decision gate and comparators
If t=2 shrinks algebraic complexity enough to pay for 256 prefix guesses, implement a real *public-only* selector or measured F4/XL recovery on the frozen 46-positive/18-negative B8 corpus, before integrating rank31/1024 target solve. If it does not, prioritize no-prefix public H_j and a genuinely better algebraic solver in #4692/#4772. Charge 256 branch attempts and every field inverse and residual solve. The reference is strongest matched index calculus and same-runner negation BSGS/rho; prior #4770 9.128s complete generic-grader vs old 11.516s (1024 targets) is not a non-generic win.

## Bounded prior-art audit
Checked main source/AGENTS/ACTIVE, #4692 (public RR9/F4), #4728 (four-prefix known-leaf 7->3 and generic 31-rank), #4770 (ternary-weight generic table control), #4772 (public seven-variable gradient and scaling), #4548 (public Macaulay), #4652 (negative y-ranked factor base), cryptanalysis prime-field auxiliary IC and crypto-autoresearcher SA-04/SA-18 norm/determinant negatives and SA-19 coordinate model. Published Petit–Kosters–Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala (FFA 2018, DOI 10.1016/j.ffa.2018.01.009) are established prior art. Shared research-KB MCP/CLI and EKS unavailable here; no novelty clearance claimed.
