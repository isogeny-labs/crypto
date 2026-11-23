# Paired Kummer relations: information and model admission [pcc:unverified]

This screen concerns the original mixed-term genus2/3/4 panel, not the binomial families in the earlier kummer-screen. It is a mathematical admission/cost screen, not an implemented Kummer solver, novelty claim or speed measurement. Independent review remains open.

## Two different operations must not share one price

For genus2, the generic biquadratic map from K(P),K(Q) describes the unordered pair K(P+Q),K(P-Q). In contrast differential xADD takes K(P),K(Q),K(P-Q) and returns K(P+Q). The supplied difference is one of the two outputs of the full pair map. Thus a cheap xADD call does not create two previously unavailable points. In a fixed-step chain K(T+jD), the previous point supplies the difference and one fresh point is generated per call. It may still be efficient, but a claimed free doubling of candidates is invalid.

Recovering the pair from two inputs requires the actual generic biquadratic evaluation, pair extraction, orientation and exception handling; those costs cannot be replaced by split-fast xADD counts. The two Kummer outputs are not negatives of one another in general and may yield useful distinct rows after signed decomposition. Global sign copies of one complete row are dependent.

## Exact relation information

Let P=aD+bT and let a shift Q have factor-base expression q. If signed decompositions d_plus,d_minus of P+Q,P-Q are found, then

    (d_plus-q) L - b*k = a,
    (d_minus+q) L - b*k = a.

Their difference is the homogeneous factor-base constraint

    (d_plus-d_minus-2q)L=0.

Their sum retains the same anchor direction(a,b). If q is unnormalized/unknown in the current linear system, the pair does not supply two independent source/target coefficient directions just by being a pair. It can supply a genuinely useful factor-base constraint. If q is already normalized by other rows, those normalization rows must be included in any rank/cost argument.

For a public shift Q=cD+dT, the anchors instead have coefficient pairs (a+c,b+d),(a-c,b-d). Their determinant is

    2(bc-ad).

Thus Q=D and b!=0 produce distinct directions; the difference of the two decomposition rows is a nonhomogeneous normalization of2D. This is not excluded by the earlier complete-automorphism-copy span argument. It is also not a guarantee of new rank after existing relations: exact quotient signatures/rank must still be checked. Over odd ell, replacing the two augmented rows by sum and difference is an invertible row operation.

The unknown sign of a Kummer lift is not a second valid signed relation. For example in F17 a true relation with D=1,T=5 and decomposed R=6 satisfies6-1-5=0; changing only the decomposition sign gives-6-1-5!=0. Negating the entire row remains valid and dependent. Signed recovery or an explicitly charged sign-resolution construction is necessary before linear insertion.

## Admission on the actual original curves

For odd-degree y²=F(x), infinity is a fixed branch point. If squarefree F has r irreducible factors, the Frobenius-fixed2torsion has dimension r-1 over F2. Proof: branch-point2torsion is even subsets of the2g+2branch points modulo complement. Infinity being fixed rules out Frobenius sending a subset to its complement. Frobenius-invariant subsets are unions of the r+1cycles; the even condition removes one dimension because the infinity cycle is odd, and complement removes another. Hence fixed dimension(r+1)-2=r-1.

The native gate recomputes factor degrees on all11original models, compares the earlier kernel census and enumerates the entire2gdimensional module's fixed vectors. For genus2 a split-fast model requiring all16rational2torsion points cannot be imported if this dimension is below4. General Kummer models remain available; do not confuse this admission failure with exclusion of all Kummer arithmetic.

There is a stronger isogeny shortcut test without computing #J. The known source has a nonzero point of odd prime order ell, so ell divides #J(Fp). The integer Hasse upper bound U=(ceil(sqrt p)+1)^(2g) is valid. If 2^(2g)*ell>U, then #J cannot also be divisible by2^(2g). Full rational2torsion would force that divisibility. Every Fp-isogenous abelian variety has the same Frobenius polynomial and rational point count, so it cannot acquire full rational2torsion either. The gate checks the prime-order point and inequality on each original model. A quadratic twist alone has the same action on2torsion because[-1] acts trivially there. This does not exclude every extension-field/twist-composition construction; those require their own target transport and cost case.

## Costs and higher genus

The published general-Kummer recovery formula with both relevant neighbors available has cost71M+8S+8constant-multiplications+35additions+1inverse; obtaining a missing neighbor costs additional xADD. Its generic-coordinate premises and exceptional/theta cases must be checked. This is a published circuit count, not a measured cost on our sources or a universal lower bound. It cannot be silently replaced by fast-split counts.

For genus3, Mueller's Proposition4.1 rules out the direct analogous biquadratic forms for a generic hyperelliptic Jacobian in the paper's embedding/premises. It is not an instance-specific proof for every original model or an exclusion of other embeddings/higher-degree formulas. No genus4 circuit follows from the genus2 formulas. The abstract coefficient determinant works for any odd-prime-order abelian group, but a cheap geometric circuit for each genus remains an additional obligation.

## Concrete successor falsifier

Do not implement a full paired solver until a source-specific circuit is written and counted: projection/precomputation, full pair versus supplied-difference state, exceptional cases, orientation/signed recovery, smoothness/factorization, all actual rows/target-rank arrival, subgroup elimination, public validation and original-target verification. Q=D can provide a normalization bridge, but a fixed-difference chain provides only one fresh Kummer point per xADD. Demonstrate a complete predicted cost at most one tenth of the qualified normalized comparator before calling it a10xcandidate. Fast-model admission or cost failure must be retained without replacing the original source curve.

## Primary sources and prior work

- Mueller, Explicit Kummer surface formulas for arbitrary characteristic, section5/Proposition5.1: generic unordered pair and biquadratic forms. https://www.math.uni-hamburg.de/home/js.mueller/general_kummer.pdf
- Gaudry, Fast genus2 arithmetic based on Theta functions, introduction: the stated fast theta formulas require full rational2torsion. https://www.degruyterbrill.com/document/doi/10.1515/JMC.2007.012/pdf
- Chung-Costello-Smith, Fast, uniform, and compact scalar multiplication, sections6.3-6.5: projection, recovery premises/count and general-versus-fast distinction. https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf
- Mueller, Explicit Kummer varieties of hyperelliptic Jacobian threefolds, section4/Proposition4.1: generic genus3 biquadratic obstruction in the specified coordinates. https://doi.org/10.1112/S1461157014000126
- Standard finite-field isogeny/Frobenius and Hasse facts: Milne, Abelian Varieties, https://www.jmilne.org/math/CourseNotes/AV.pdf ; same characteristic polynomial for isogenous finite-field varieties is also stated in Finite group subschemes of abelian varieties over finite fields, https://doi.org/10.1016/j.ffa.2014.04.001 . The subgroup-divisibility admission argument above is our deduction.

Local prior art: kummer-screen/SCREEN.md covers different retained families, ANCHOR-TRANSPORT-SCREEN.md covers complete symmetry copies, GROUNDED-DUAL-SCREEN.md gives exact target quotient/rank criteria. None supplies the missing paired circuit/cost proof. KB lexical search was used as a novelty/context screen; its records are not independent proofs.
