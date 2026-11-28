# Two-center power images for sign recovery

T-1804, 2026-11-07. Derived construction and producer software checks, not independently signed evidence. [pcc:unverified]

## Question and relation to earlier work

Can nonlinear scalar images reduce destination state without the earlier 2^J subset table or degree-r indicator? The existing quotient-indicator extractor excludes fixed small linear summaries with exact linear updates/readout. This module changes the unknown to a power image and retains polynomial state for that image. It does not make the original scalar indicator low-degree.

Let r>=5 be a caller-certified prime, m>=2 divide r-1, d=(r-1)/m, and zeta have exact order m in F_r. Inputs are canonical and r<=u64::MAX/2. The code checks exact order in m multiplications, bounded by max_orbit before work, and bounds d+1 by max_degree before allocation. No factorization of r-1 or primitive-root search is hidden in production: m and zeta are supplied certificates and their validity is checked. Tests search for zeta only in reference code.

## Nonlinear transport of a character constraint (derived)

For center c, put y=k+c and z=y^m. For nonzero b,

    product_{j=0}^{m-1}(y-b*zeta^j) = y^m-b^m
    product_{j=0}^{m-1}chi_r(k+c-b*zeta^j) = chi_r(z-b^m).

Proof: zeta's powers are the m distinct roots of T^m-1. Both monic degree-m polynomials in the first line have precisely the roots b*zeta^j; hence they agree. Multiplicativity of the quadratic character, including zero, proves the second line. No sign factor is omitted: the factors are y minus the roots. The source queries are the public points Q+[c-b*zeta^j]P. There is no supplied point [k^m]P.

Zero source output identifies k=b*zeta^j-c and is accepted only after scalar-to-point verification. The source adapter validates Q before any group arithmetic. The source remains the earlier standalone projected construction: p>3, prime subgroup r>=7, r!=p, gcd(r,p-1)=1 and 2^v2(p-1)<=4, with its existing geometry cap and caller primality obligations. The destination construction itself has no p restriction beyond its F_r assumptions.

## Smaller exact candidate state (derived)

The image of y->y^m is {0} union the d-th roots of unity. Initialize the monic squarefree polynomial

    G_c(Z) = Z*(Z^d-1).

Squarefreeness follows because d is nonzero in F_r and the nonzero roots are simple. Its d+1 roots are all possible z. After a folded sign s!=0, replace G_c by

    monic_gcd(G_c, (Z-b^m)^((r-1)/2) mod G_c - s).

For s=0 use gcd(G_c,Z-b^m). Induction shows this retains exactly the image values consistent with all folded observations. The imported capped half-GCD driver and modular-power implementation perform the update. Repeated and contradictory observations are meaningful. The count is the degree and the singleton is minus the constant coefficient when degree one. No subset table, dense scalar indicator or scalar-label access appears in production.

**Restriction:** available shifts are b^m, not arbitrary elements of F_r. Different b may repeat the same folded constraint. The implementation deliberately preserves the bounded deterministic b=1,...,B schedule; it assumes neither independence nor logarithmic query separation. It can return unresolved. A compressed representation alone does not imply efficient extraction.

## Two-center fiber extraction (derived)

When two distinct centers c0,c1 give singleton images z0,z1, compute

    H(X) = monic_gcd((X+c0)^m-z0, (X+c1)^m-z1).

Every valid k is a root of H. Each individual fiber splits over F_r; if one z is zero its fiber polynomial has a repeated root, but the other image is nonzero because centers differ. Thus their common-root multiplicities are one. The gcd represents exactly the simultaneous fiber, including zero-image cases. A degree-one gcd gives k=-H[0]. The public adapter verifies [k]P=Q; otherwise it reports unresolved fiber rather than enumerating it. Empty state signals inconsistency.

For m=2 there is a useful unconditional extraction fact: subtracting the two equations gives

    2*(c1-c0)*k = z1-z0-(c1^2-c0^2).

The coefficient is nonzero, so any consistent pair has exactly one scalar. This does not prove the bounded folded query schedule reaches singleton images. For m>2 the joint map need not be injective; the checks retain actual ambiguous fibers. A subsequent construction can add centers or use residual source information, but this module does neither after a two-center ambiguity.

## Cost model and implementation boundaries

Let S(r) be the cost of one existing source query, with source-field arithmetic separate from scalar-field arithmetic. Calibration is paid once by PublicRecovery::new. Each completed folded update uses m source evaluations and m shifted public points, plus O(m) scalar multiplications for orbit offsets. This initial implementation computes each [offset]P by double-and-add: O(m log r) source-group additions per update. The initial input-validation query costs one additional source evaluation. Early zero output pays only the executed prefix; all outer calls, scalar-multiplication invocations, group additions (including final verification), and completed updates are counted. Internal geometry and field operations are not replaced by those counters: their earlier S(r) cost remains charged.

Two root states have degree at most d+1, i.e. d+2 coefficients each. Update modular powering costs O(M(d+1) log r) field operations, and half-GCD costs O(M(d+1) log(d+1)) field operations plus the backend's exponentiation-based inversions. Fiber construction uses binary polynomial powering to degree m, with safe O(M(m) log m) work; fiber gcd has the analogous half-GCD charge. Native inversions contribute conservatively O((d+m) log r) modular products per relevant gcd; do not count them as unit-cost modular products. Conservative polynomial workspace is O(d log(d+1)+m log(m+1)), plus O(m) orbit offsets and the source workspace. Scalar-only root coefficient storage is O(d+m).

A divisor m near sqrt(r) makes d+m about sqrt(r), but requires that divisor and increases each folded query to m*S(r). No theorem here accelerates that source batch. On the original r479 fixture, the tested m2 state starts at degree240, whereas the earlier consecutive-three-sign state starts at at most60 on the frozen20: this is a larger initial destination state on that fixture. The purpose of that fixture is public-API composition, not a favorable comparison. For r97,m8 the image degree is13; the direct scalar domain has97 elements. These are derived dimensions, not timings. Large m also restricts the available image shifts and can impair separation. Selecting m from a presumed square-root divisor would not cover all primes.

## Exact software checks (producer evidence)

validation-01 records two passing native test methods, 157 imported methods filtered. No experiment/timing panel or cloud job was launched.

- Primes 7,13,17,31,41,97; every divisor m in2..8 and every k: 5,382 orbit-product identities, 5,382 complete image-root set comparisons against direct filtering, and 897 two-center fiber comparisons against explicit simultaneous powers. This includes zero images, zero signs, empty and repeated constraints. All quadratic fibers are linear. There are202 retained ambiguous higher-power fibers.
- Public point composition: all13 inputs on E/F7: y²=x³+3, P=(1,2), r13, m3,zeta3; and the original frozen20 points on E/F509: y²=x³+2x+10, P=(3,68), r479,m2,zeta478. All33 return the reference scalar.20 use fiber gcd and13 use zero-source or identity termination.725 source calls total, including initial validation. These combined counts are coverage, not a per-curve performance comparison. The full4,563-case cohort is unchanged.
- Controls reject false root order, orbit/degree cap excess, invalid signs/offsets, equal centers, invalid/noncanonical public points; contradictory signs make empty state; a one-call budget returns unresolved. Production dependency check excludes scalar reference sources; scalar labels and exhaustive finite-field references occur only in tests.

The validation receipt binds30 source/input dependencies and command/output hashes. Native source runs with overflow checks. Warnings are preserved. No failed numerical check preceded this passing receipt.

## Formal proof map

ProductMap.lean: PowerFold.map_product proves that a multiplicative map preserving one commutes with finite list products. Its explicit hypotheses are exactly preservation of the unit and of each binary product. Lean4.34.1, commit5045d0056413266e57c625dcd7c365b10e377c52; imports Init only. #print axioms is retained. This is solely the homomorphism bridge, not a formal proof of the root-of-unity factorization, finite-field image description, gcd extraction, source character theorem, or complexity.

Full formalization requires finite-field and polynomial definitions and their roots/gcd infrastructure. The available crypto-lean checkout178c03f46d9347eb92da61a9307eff3d396e3e4f has no built Mathlib package in its formal project; its declaration pins mathlib d13f23b723b8a846827a245b89c10fc7d3f11612. Those dependencies were not built or downloaded. The full written proofs above remain unformalized. No proof bypass or custom axiom is used for the partial lemma.

## Sources and next construction

Sources checked (KB): server unavailable and scripts/research-kb has no executable. Local CONTINUATION sections lifted-sign-moments, quotient-indicator-extractor and triple-log-derivative; existing half-GCD RootSet and public Source. NIST DLMF27.9 https://dlmf.nist.gov/27.9 provides the quadratic character conventions. Cheon and Kim, A new approach to the discrete logarithm problem with auxiliary inputs (2016), introduction and section2, https://doi.org/10.1112/S1461157015000303, explicitly requires auxiliary powers and uses collision lists. Its small polynomial image is relevant prior art; its auxiliary-input or collision algorithm is not implemented here, and its complexity is not transferred. The first attempted Cheon2010 PDF fetch timed out. No novelty claim is made.

Next: seek a source-native evaluation of the orbit character product cheaper than m separate geometry queries, while studying separation on the restricted shift set. Test additional centers against the retained ambiguous higher-power fibers before claiming general extraction. These are concrete missing constructions, not implied by the image degree reduction.
