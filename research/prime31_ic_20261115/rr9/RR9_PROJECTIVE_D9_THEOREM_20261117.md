# A degree-nine projective separator for the B8 RR9 public coefficient ideal

**Date:** 2026-11-17 UTC. **PR:** #4550 (stacked on #4549). **Validation:** mathematical argument and actual passing exact-source native Rust 1.100 CI; producer-only [pcc:unverified]. **This is NOT an ECDLP speed result or a support-free extractor.**

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

## 6. Completed exact-source native Rust test, source and artifact

- **PASS** [GitHub Actions #44104421359](https://github.com/isogeny-labs/crypto/actions/runs/44104421359), PR event, source \`32d6ba5489dbf69f0fa2bc98556fff315a23a80f\`: pinned 1.100.0 original library and bounded new example tests, release build, full raw JSON validation.
- [Raw artifact #14337801335](https://github.com/isogeny-labs/crypto/actions/runs/44104421359/artifacts/14337801335), archive SHA-256 **12f74c30d341c53def98c226bbf149eee6c9eb0a711d2b2082776d9c269e7f3e**, independently checked against downloaded 822-byte ZIP.
- Original-curve positive **test-only chart coefficients** (455835358, 518652820, 754716693, 932383545, 62510441, 1884059825, 1142049404) passed all eight q_i, direct M=H_B, and independent original-group signed sum R=(1911579686,548820096).
- Degree-nine **positive evaluation null certificate** passed **27,456/27,456 sparse row dot products** comprising **240,240 term products**; its h^9 coordinate is 1. Therefore the actual positive D9 matrix has rank at most 11,439, regardless of elimination algorithm. The negative D9 rank **was not computed** and remains JSON null.
- Charged wall for bounded witness-only construction and checking: **6,846,317 ns (6.846317 ms)** on the GitHub runner. This number is **not** the runtime of a public-residual-only extractor, a unit-certificate solver, a relation generator or a complete ECDLP algorithm. Public-only nonenumerative extraction=false and complete unknown logarithms recovered=0 in raw JSON.
- All statuses remain producer [pcc:unverified], no signed independent PCC or measured generic-frontier improvement.


## 7. Cross-repository anti-duplication receipt (verified live)

- [crypto-autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301) is the 2026-11-14 prime-field IC lead heat-map fold: **39 disposition changes**, 14 proof/cost obstructions and 17 toy-scale exhaustions with explicit revisit triggers; a generic pair-table oracle was deliberately left open as a **generic constant** lead, not a novel IC asymptotic mechanism. This supports rejecting sign-support enumeration as a non-generic extraction claim.
- [crypto-autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) landed 28 preregistered Semaev-alternatives experiments and explicit ERRATA 8–9 reversing a previously overstated SA-23 gain. Its revised atlas keeps shared-coefficient membership open, but requires independently audited closed leads and complete total-cost controls. This D9 rank criterion is a **possible exact membership gate**, not an already recovered public support or an acceleration claim.
- [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) already builds **matched Rust prime-field auxiliary-base vs same-base-direct vs random-base IC** controls at p=2^31−1. That campaign explicitly separates fixed-prefix yield from complete unknown-log recovery and corrects phase accounting; no demonstrated full solver advantage can be imported. A future B31 experiment should **reuse that same-base/random-base comparison design** instead of reopening base-selection novelty claims.
- [crypto #4373](https://github.com/isogeny-labs/crypto/pull/4373) capped eight **m8 chained-S3 F4 arms** as oversize/undetermined; simply changing S3 pairings without changing elimination structure cannot be promoted. D9 is a new *mathematically exact* gate but still requires a sparse rank implementation that actually beats this prior cost barrier.

This is a bounded review of **named PRs**, not a 4-repository exhaustive audit or a claim that the shared research-KB was searched.
**Status:** The exact D9 separation theorem is proved for the stated B8 chart and a concrete p31 positive-null certificate is replayed in Rust with passing CI. The next missing executable step is public-only sparse negative full-rank/unit-certificate extraction; B31 full ECDLP and any speed advantage are not established.