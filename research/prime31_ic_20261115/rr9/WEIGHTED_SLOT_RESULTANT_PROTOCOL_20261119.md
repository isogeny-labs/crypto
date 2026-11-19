# Frozen protocol — weighted W8 slot-root resultant gate (2026-11-19)

## Mathematical question and registration
The fixed-squarefree RR9 equation M8=H_B does not represent the weighted signed eight-slot language. Test an exact public **slot membership** algebraic replacement, without claiming it solves the additional target-conditioned RR9 equations.

Let T_i={x([2^i]P_j):j in W} for slots i=0,...,7 and each eight-point factor-base window W of the original ordinary cofactor-one Prime31 curve (p=2147483647, a=847514846, b=2087444724, group order n=2147446831). Since signs do not affect x, each T_i has at most eight distinct field elements. Use the original Rust `prime31_algebraic_ic::{Arithmetic,check_curve,PRIME31}`; do not create replacement curve arithmetic. Rebuild first 31 increasing ordinary x points using the existing library, windows indices 0..7, 8..15, 16..23, 23..30.

For each i define a *publicly computable* monic degree-eight polynomial H_i(X)=product_(t in T_i)(X-t). For a candidate monic degree-eight M(X)=X^8+c7 X^7+...+c0 define the degree-eight-in-coefficients equation
`R_i(M):=Res_X(H_i,M)=product_(t in T_i)M(t)=0`.
Prove the precise implication **conditional on pairwise disjoint T_i and each having eight distinct roots**:

`(for all i, R_i(M)=0) <=> (there are unique distinct t_i in T_i for i=0..7 with M(X)=product_i(X-t_i))`.

This is a statement about unsigned x-root choices, NOT signed group-sum target Q or ECDLP extraction. There are 8^8 possible monic M for the disjoint-set case, prior to target conditioning. No discriminant saturation is needed because degree eight plus eight disjoint sets forces eight simple Fp roots.

## Original-curve disjointness and deterministic controls
First compute every T_i from **original** `Arithmetic::scalar`, check eight distinct roots per slot and pairwise disjointness among all 64 transformed abscissas in each of four windows. Do not silently assume this holds on other curves or windows.

Then seed SplitMix64 with 0x534c_4f54_5f52_4553. In each window generate 128 signed eight-leaf words, independently choosing the factor-base index in each slot from 0..7 (repeats allowed) and sign from the next PRNG low bit. Form the signed weighted EC sum Q with the original Arithmetic, and monic M from the eight transformed x-roots. Assert all eight R_i(M) vanish. Construct a matched negative M' by replacing *only* slot-0 chosen transformed root with an unused transformed root from slot 1 (choose (index1+1)%8); all other seven roots unchanged. Assert R_0(M') != 0 and R_i(M')=0 for i=1..7. Assert both M and M' are degree-eight monic with pairwise distinct roots **only when a positive's within-slot selections did not collide cross-slot** (guaranteed by disjointness). Assert weighted Q on curve, and [n]G=O via check_curve.

Primary frozen gates: four windows; 64 unique transformed abscissas/window; 512 planted positive certificates; 512 matched negative certificates, each failing exactly slot 0; 4096 zero positive resultants and 512 nonzero negative R_0. Record first-case per-window selected indices, signs, M and M' coefficient arrays and resultant nonzero flags. Use one independent Python BigInt affine implementation to reproduce the four windows, all 512 cases and all resultant tests, but don't call it independent signed PCC. Include source SHA, protocol SHA and CI proof. No generic table inside candidate, no hidden witness passed to any claimed extractor (this is a planted identity/correctness control, *not* such an extractor).

## Predeclared interpretation and cost gate
The eight R_i constraints use eight coefficients c0..c7, replacing 8 leaf x variables plus 7 auxiliary chain x variables for *slot membership only*. They do not by themselves encode Q; do not compare an eight-variable membership system to a fourteen-variable **complete public-target** chart as if the two were equivalent. The target-conditioned RR9 quotient/remainder equations and actual elimination cost remain separate unimplemented requirements, owned by PR #4692. The historical 1,312-attempt/170.113ms BSGS sensitivity implies an optimistic <129.66 microsecond average budget per attempted weighted public residual; requires same-runner reproduction. If overlaps exist or a frozen gate fails, report the full counterexample instead of an exact reduction.

Prior lead boundaries: #4812 x-only Horner; #4816 unweighted fixed-M8 quotient; #4692 weighted public-only RR9 owner; #4728 complete generic-grader Rust solver; #4532 fastest historical shared BSGS; #4807/4808 weighted grading yield. Relevant prior art: Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, FFA 2018, DOI 10.1016/j.ffa.2018.01.009; Kudo–Yokota–Takahashi–Yasuda, CANS 2018, DOI 10.1007/978-3-030-00434-7_19. No novelty priority claim. [pcc:unverified]

## Registered extension before Jacobian code

The eight product-of-linear-form resultant equations are a complete intersection with exactly `8^8=16,777,216` complex/algebraic-closure solutions when all T_i have eight elements and are pairwise disjoint; each corresponds to a distinct monic M, and every solution is in F_p. At every solution, the Jacobian determinant in coefficient variables is `det(J) = (product_i product_(t in T_i\{t_i}) M(t)) * (product_(i<j)(t_j-t_i))`, up to the standard Vandermonde ordering convention. All factors are nonzero. Hence the membership ideal is zero-dimensional, reduced/radical, and geometrically split, *but has a huge 16.8M-dimensional Fp quotient*. This supplies a falsifiable warning against first computing the full membership quotient before coupling the target; it does not bound a target-conditioned RR9/F4 system.

Secondary diagnostics: check all 512 planted Jacobian determinants nonzero in the original Rust and independent Python code, and record one determinant per window from the first fixture. No change to the original primary gates or frozen seed. Do not present Jacobian nonsingularity as a fast solver.
