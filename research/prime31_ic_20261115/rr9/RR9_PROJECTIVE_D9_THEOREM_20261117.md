# A degree-nine projective separator for the B8 RR9 public coefficient ideal

**Date:** 2026-11-17 UTC. **PR:** #4550 (stacked on #4549). **Validation:** mathematical argument plus proposed source-specific Rust replay; producer-only [pcc:unverified]. **This is NOT an ECDLP speed result or a support-free extractor.**

## 1. Setup and two exact lemmas

Work over any field K of odd characteristic, including K=F_(2^31-1), whose short-Weierstrass cubic f(x)=x³+ax+b is squarefree. Fix R=(r,s) on y²=f(x) and distinct factor-base x_i ≠r with nonzero y_i. Let H_B(X)=prod_{i=1}^8(X-x_i), monic degree 8, and define A(X)=sum_{j=0}³ a_jX^j, V(X)=X³+sum_{j=0}²v_jX^j. The existing public-only RR9 system consists of the eight coefficient equations q_0=...=q_7=0 in seven unknowns expressing

    M(X) := (X²+rX+r²+a)V(X)² - (X-r)A(X)² - 2sA(X)V(X) = H_B(X).

All q_i are affine quadrics. Homogenize them to degree-two polynomials Q_i in eight variables (a0,a1,a2,a3,v0,v1,v2,h).

**Lemma A — no solutions at infinity, over algebraic closure.** At h=0 the leading homogeneous quadratic parts of all eight q_i together say that

    (X²+rX+r²+a) U(X)² -(X-r)A(X)² -2sA(X)U(X)=0,

where U=v0+v1X+v2X². The cubic identity

    (X-r)(X²+rX+r²+a)+s² = f(X)

makes the same polynomial identity

    f(X) U(X)² = ((X-r)A(X)+sU(X))².

If U≠0, the squarefree odd-degree cubic f would be a square in the rational function field over the algebraic closure of K, impossible because its simple roots have odd valuation. Hence U=0 and then A=0. Therefore **all Q_i have no nontrivial simultaneous zero with h=0**. This argument is unconditional on the public R beyond R lying on the nonsingular curve, and independent of the sign oracle.

**Lemma B — exact degree-nine linear-algebra membership criterion.** Let S=K[a0,...,v2,h]. The Q_i form a homogeneous system of eight quadrics. For a projective-root-free system, the ideal has height eight; in the Cohen–Macaulay ring S this is a homogeneous regular sequence. Thus

    Hilb(S/(Q0,...,Q7),t) = (1-t²)^8/(1-t)^8 = (1+t)^8.

Its degree-nine quotient component is zero. Accordingly, the degree-nine Macaulay matrix consisting of rows mQ_i, all homogeneous monomials m of degree seven, has **full column rank 11,440**. Equivalently h^9 admits a degree-nine algebraic unit certificate after dehomogenization.

Conversely, if some affine coefficient chart solution c∈Kbar^7 exists, evaluation of all degree-nine monomials at (c,h=1) yields a nonzero right-null vector of that matrix. The h^9 coordinate equals one, so its rank is **strictly less than 11,440**.

Combining Lemma A with Lemma B gives, for every public residual R in the non-exceptional B8 branch,

    an affine RR9 chart solution over Kbar EXISTS
        ↔ rank(Macaulay_degree_9) < 11,440,
    an affine RR9 chart solution over Kbar DOES NOT EXIST
        ↔ rank(Macaulay_degree_9) = 11,440.

This is an **exact** algebraic distinction at D9, unlike the inconclusive degree-five and degree-six ranks. It is a specialization of classical resultant/regular-sequence theory, **not** a new asymptotically fast index-calculus algorithm. It is not an unconditional criterion for all eight-leaf signed supports: the chosen normalization V_3=1 excludes exceptional chart points.

**Signed-leaf soundness.** The exact identity

    -(x_i-r)M(x_i) =
      [(x_i-r)A(x_i)+(s+y_i)V(x_i)]
      [(x_i-r)A(x_i)+(s-y_i)V(x_i)]

forces a sign at every factor-base x_i when M=H_B. Because H_B has simple roots, y_i≠0, and x_i≠r, the two factors cannot both vanish at any x_i. The resulting rational RR9 function has zeros at eight signed E(K) base points and at -R, so the elliptic group sum of the eight signed leaves is R. For the two frozen off-support negatives, a chart solution over Kbar would therefore imply a forbidden sign combination among the same 256 K-rational possibilities; hence there is no solution even over Kbar, and the D9 matrix is theoretically full rank for each. **This is a mathematical consequence, not a measured Rust D9 rank.**

## 2. Why #4548 and #4549 could not decide the negative cases

The generic complete intersection has Hilbert dimensions C(8,5)=56 and C(8,6)=28, matching all three measured cases at D5/D6. However a genuine positive affine solution prevents the eight homogenized quadrics from being a regular sequence. No projective-root-free regular-sequence Hilbert calculation can prove negative from low-degree rank agreement. The first guaranteed separation is by D9, where the generic complete-intersection Hilbert function is zero. Earlier D7/D8 divergences are possible but **not claimed**. D7/D8 generic quotient dimensions are 8 and 1 respectively.

D9 has C(16,7)=11,440 homogeneous columns and 8*C(14,7)=27,456 row generators. A naive full dense pivot store needs about 1.047 GiB (11,440²×8 bytes), before working allocation. Avoid blind dense escalation.

## 3. Native Rust positive-witness scope

The append-only source \`research/prime31_ic_20261115/native/examples/prime31_rr9_public_macaulay.rs\` exposes \`projective_degree_nine_witness_controls()\`, using the unchanged original arithmetic. A separate Rust example tests the *positive direction only*: eight test-oracle signed leaves for frozen R=(1911579686,548820096) are interpolated into the seven chart coefficients, each q_i is checked, and every one of 27,456 **sparse D9 rows** is dotted against the 11,440-component evaluation vector. The h^9 value is 1. This is a constructive finite-field rank-**deficiency** witness, **not** a sparse D9 full-rank algorithm and not a public-only extraction routine. The sign mask and coefficients are test-only outputs, never inputs to a candidate extraction worker. Negative D9 rank intentionally remains null in the result schema.

Actual p=2^31-1 curve: a=847514846, b=2087444724, cofactor one subgroup order n=2147446831. Frozen positive R and two off-support negatives are unchanged from #4549. The prior full-cost producer 1,024-target comparison on this curve remains cached BSGS **170.113ms**, complete mixed-base IC **2,221.474ms** (#4532); not remeasured here.

## 4. Next experiment with falsifiable gates

1. Implement an actual **public-R-only sparse D9 rank / unit-certificate** algorithm in the existing pinned Rust sparse-F4 / quotient stack, not dense elimination. Positive must be rank-deficient and yield an algebraic chart witness; both fixed negatives must be full rank with a replayable unit certificate. Consider projected Krylov/Berlekamp algorithms only when the exact rank claim is preserved; randomized failures are inconclusive, not proof.
2. Test the conditional 7-of-8 quotient strategy: if seven equations form a zero-dimensional quotient with a compact multiplication representation (generic Bezout degree 128), test multiplication by the eighth. Measure saturation and infinity components explicitly. **128 is a generic Bezout bound, not measured RR9 quotient rank.**
3. Extend to B31, all repeated-root and exceptional-chart branches, then complete blinded ECDLP for unknown Q with setup/misses, orientation, field operations, elimination, relation-rank and memory charged. Compare against fully shared generic BSGS and strongest valid rho; a pure finite-support/resultant selector is not evidence of a non-generic solver advantage.

## 5. Prior sources (bounded)
- Repository: #4539 coefficient chart; #4541 radical factor support; #4545 signed-fibre; #4546 exceptional x=r; #4547 supplied-anchor generic-equivalence; #4548 Macaulay D5; #4549 D6/Koszul; #4373 F4; #4228/#4349 quotient; #4513 full IC; #4532 matched BSGS.
- Cross-repo: isogeny-labs/crypto-autoresearcher #3301 lead atlas, #3305 Semaev alternatives; prior cryptanalysis/index-calculus work as cited in parent records. No verified new cross-repo solver claim.
- Literature: Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, FFA 2018, DOI 10.1016/j.ffa.2018.01.009; Kudo et al., CANS 2018, DOI 10.1007/978-3-030-00434-7_19; Macaulay2 [resultants documentation](https://macaulay2.com/doc/Macaulay2-1.22/share/doc/Macaulay2/Resultants/html/_resultant.html).
- Research-KB MCP and repository wrapper unavailable in this connector-only runtime; fallback was exact live GitHub files and public primary/publication links. Novelty unestablished.

**Status:** degree-nine mathematical separator proved at the stated algebraic scope; exact-source Rust PASS/FAIL, artifact and measured bounded witness wall are to be recorded separately when CI completes. No complete non-generic solver has been measured.