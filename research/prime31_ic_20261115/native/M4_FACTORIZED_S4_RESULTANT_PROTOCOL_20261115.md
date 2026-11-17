# EXP-R31-L — factorized S4 resultant for prime31 m4 elimination (pre-execution protocol)

Date: 2026-11-15. Standing: proposed, not yet executed, [pcc:unverified]. Scope: Rust-only algebraic oracle optimization on the established ordinary prime-order curve over F_(2^31-1). This is not a complete ECDLP algorithm.

## Read-before-edit and prior lead receipts
- `research/prime31_ic_20261115/native/M4_PAIR_SUPPORT_RESULTANT_20261115.md` and `examples/prime31_s3_quotient_gcd.rs`: existing verified Rust component; reported B8->B16 norm/GCD 1.783 ms -> 106.916 ms at exact-source CI 43747735303. Factor-base precompute reported separately; no natural-useful-rank or cold rho win.
- `M4_NATURAL_YIELD_ARITY_GATE_20261115.md`: exact four-summand coverage B31 618,885 / n=2,147,446,831; any optimized m4 oracle remains limited by a 0.0288196% natural-positive ceiling.
- `native/examples/prime31_m8_balanced_f4_input.rs` and #4077: target-dependent m8 solves are the ultimate bottleneck. #4228 certified quotient/subresultant prototype already investigates direct quotient-algebra elimination, without claiming a whole-ECDLP breakthrough; #4275 dense/tower negative comparison; #4311 single-relation selector proposals and #4338 fast-selector proposals.
- Primary context: [Petit–Kosters–Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1), [Amadori–Pintore–Sala, FFA 2018](https://doi.org/10.1016/j.ffa.2018.01.009). Both already consider algebraic prime-field index calculus. Multiplicativity of resultants is classical; NO new mathematical algorithm or novel complexity theorem is asserted.
- Research-KB MCP not exposed in this runtime; repository wrapper unavailable without a git checkout. Bounded fallback was GitHub primary source, PRs and publisher records. Conductor/EKS may likewise be unavailable; do not claim an independent validation or charged benchmark.

## Exact algebraic hypothesis

Use the original `K(U)=monic(prod_{i<=j} S3(x_i,x_j,U))` and `F(U,V)=S3(U,V,r)`, r=x(R). Write `F=A(U)V²+B(U)V+C(U)` with degrees <=2 in U. For each factor `g_ij(V)=S3(x_i,x_j,V)=d2 V²+d1 V+d0`, define the *padded quadratic resultant*

`T_ij(U)=(A*d0-C*d2)^2-(A*d1-B*d2)*(B*d0-C*d1)`,

a polynomial of degree <=4 in U. It is identical to `Res_V(F,g_ij)` as a Sylvester 4x4 determinant, including projective degree-drop factors. At `U != r`, A=(U-r)^2 is nonzero, so common finite V roots of F and g_ij are correctly detected. For diagonal `i=j`, deg g_ii=1 and the padded resultant includes an extra A(U) factor: this is invertible on any quotient with U-r removed, hence does not change regular gcd support.

The multiplicativity identity over Fp[U] gives

`gcd(K_regular(U), prod_ij T_ij(U) mod K_regular(U))`

the **same monic gcd** as the existing quotient-Horner/norm oracle, provided `K_regular=K/(U-r)` when r is a root of K, and K otherwise. No ordinates or point additions enter the discovery algebra. Handle exceptional U=r using the original finite-root check `gcd(K(V),S3(r,V,r))` and include a U-r factor only if nontrivial. Avoid claiming any root at infinity.

## Predeclared Rust test/measurement

1. Keep the original `m4_first_eliminant` as reference. Implement `m4_first_eliminant_factorized` on the same fixed public target r and abscissae array, using existing prime31 polynomial field ops. No known ordinates, precomputed group-sum table or scalar leak may be used by either algebraic stage.
2. Compare EXACT monic gcd polynomial, exception flag and full final signed relations for fixed planted-positive and public [17]G on B4/B8/B16. Add an exceptional-target r chosen from K's true rational roots, and an unrelated target for a negative case; failures must fail closed.
3. Time each algebraic membership stage with identical K and target; explicitly separate K build, candidate extraction, signed verifier and reference stage, and avoid dividing by a censored or uncompleted reference. Reserve B31 for factorized-only positive/public probes unless the reference has a completion budget.
4. Record factor count, maximal factor degree <=4, deg K and operation-independent count of short-factor multiplications, and timings in Rust JSON. Run Rust-only tests in CI, retrieve exact source/workflow logs if hosted. No Python/C++ new implementation or replay.
5. A meaningful future whole-solver decision still requires genuinely extracted natural m8 relations, rank, final scalar recovery and cold same-host negation-map rho; an m4 stage speedup cannot overcome the exact B31 four-sum yield ceiling.

## Stop conditions
A single mismatched gcd, nonrecovered planted relation, claimed natural win from a planted target, unacknowledged exceptional branch, or CI failure blocks a performance claim. Distinguish source-level theoretical operation-count reduction, executed timings and complete cost.
