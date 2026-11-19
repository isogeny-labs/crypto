# Prime31 transformed-slot F3: nonsingular target-conditioned chart

**2026-11-19; verified mathematical result, original-Rust source-specific CI pending.** [pcc:unverified]. This is an added correctness/complexity certificate for the existing weighted W8 relation representation in PR #4812, **not** a public-only F4 extractor or a complete-ECDLP advantage.

## Exact theorem

On the unchanged ordinary, cofactor-one Prime31 curve `E/F_2147483647: y²=x³+847514846x+2087444724`, prime order `2147446831`, use four verified disjoint transformed eight-point windows `T_i={x([2^i]P_j)}`. Let `H_i(X)=prod_(u in T_i)(X-u)`. The target-conditioned finite chart is
`H_i(t_i)=0 (i=0..7); S3(t_i,z_(i+1),z_i)=0 (i=0..6); z_7=t_7; z_0=x(Q)`.

There are 14 unknown x-coordinates and 15 equations. **The target-conditioned 15-equation ideal is zero-dimensional and geometrically reduced on its finite ordinary chart.** More specifically, the Jacobian has column rank 14 at every geometric solution, witnessed by the minor formed from the eight `H_i` rows and the six `S3` rows `i=1..6`.

For `S3(x,y,z)=Az²+Bz+C` the exact identity is
`B²-4AC=16 f(x) f(y)`, `f(x)=x³+ax+b`.
At any S3 root,
`(partial_z S3(x,y,z))²=16 f(x)f(y)`.
The factor is nonzero for the original curve's rational ordinary x-coordinates, because a root of f would be rational 2-torsion and the group order is odd. The finite-chart sign-lifting induction puts every geometric suffix x in the same ordinary rational locus. Thus all seven S3 steps are **separable**, including the linear specialization when `x=y`.

Every slot polynomial H_i has eight *distinct* roots; hence each `H_i'(t_i) != 0`. The 14x14 minor is triangular up to block operations and is nonzero:
`det(J_minor)= ± (prod_(i=0)^7 H_i'(t_i)) (prod_(i=1)^6 partial_(z_i) S3(t_i,z_(i+1),z_i))`.
The first 14 equations therefore cut out a finite reduced chart. In a finite product of fields, adjoining the last public-target equation removes components but creates no nilpotents. This proves the claim; see the frozen protocol for the full quantifiers and exceptional-chart limitation.

**New complexity gate:** Before applying the public-Q condition, the 14-polynomial selector/suffix subsystem has at most `8^8 2^6 = 1,073,741,824` geometric finite roots. This is an upper bound, **not** a measured quotient dimension or a lower bound on F4. A 4-byte coordinate stored for every putative root would already be 4 GiB, without indices or algebra. A naive full-quotient-before-Q route is consequently unacceptable; target-coupled elimination remains essential.

## Reproducible check on existing Rust arithmetic

Frozen [protocol](TRANSFORMED_F3_JACOBIAN_PROTOCOL_20261119.md) and new [original-crate Rust example](../native/examples/prime31_transformed_f3_jacobian.rs), [CI workflow](../../../.github/workflows/prime31-transformed-f3-jacobian.yml). Four original windows x 128 planted signed relations, seed `0x4a41434f4249414e`. Registered assertions:
- 512 ordinary finite cases, zero exceptional suffixes;
- 4096 nonzero selector H' values and 3072 nonzero suffix F3 pivots, together 7168 nonzero diagonal factors for 14x14 Jacobian minors;
- 3584 exact S3 identities, nonzero derivatives and discriminant checks;
- original `Arithmetic` and `PRIME31`, 40 baseline tests plus one new focused test;
- no public-only unknown-index extraction and no generic group-sum tables used as an algebraic solver.

**Independent local arithmetic preflight (not GitHub CI):** an independently written affine Python check with a different deterministic seed confirmed 512 ordinary chains, 7168 nonzero pivots, 3072 of the six suffix discriminants, and zero exceptional cases. Its distinct seed is *not* the frozen source-specific Rust cohort; do not combine the two counts or call the source-specific gate passing until CI actually finishes.

## Prior work and handoff

Reviewed before implementation: #4812 original transformed-slot report; #4692 RR9/F4 owner; #4822 sparse Wiedemann; #4816 fixed-unweighted quotient negative; #4809 B2 S3 rootfinding (avoid duplication); #4807/#4808 generic collision/coverage; #4728 complete generic-table Rust control; the Diem 2011 and Kousidis–Wiemers 2019 results on algebraic index calculus and fall degree, plus Semaev and Kosters–Yeo. The shared research-KB MCP was unavailable here; accessible GitHub and primary literature used instead. No novelty-priority claim.

**Next falsifier:** Have the existing #4692 F4 implementation build the 14-var/15-equation *public-only* chart, use the full rank-14 guarantee to check squarefree quotient extraction and failure classifications, and test the exact frozen 46-positive/18-negative B8 panel. Charge all SAT, UNSAT, timeout, memory, and relation failures. Then couple successful extraction to the original rank31/1024 target solver, comparing fully charged same-runner optimized index calculus, BSGS and rho. The historic shared-BSGS 170.113ms/1024 and 1312 W8 attempts imply an *optimistic necessary* <129.66us per attempted residual before any setup, linear algebra or verification charges; this is **not achieved**.
