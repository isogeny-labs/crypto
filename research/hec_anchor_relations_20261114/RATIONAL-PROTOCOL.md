# Higher-pole signed rational interpolation: algebra gate

Frozen before implementation or execution, 2026-11-15. This is an exact
construction gate, not a timing experiment or a novelty claim.

## Sources and motivation

D'Andrea, Krick and Szanto, *Subresultants, Sylvester sums and the rational
interpolation problem*, Theorems 2.2 and 3.1:
https://arxiv.org/html/1211.6895v3 . Their subset formulas and denominator
nonvanishing condition are established prior art. The formula below is a
Vandermonde-weighted specialization with a direct nodewise proof. The research
question is whether signed batches and larger interpolation tuples can reduce
the anchor dimension while preserving useful relation yield and rank.

Sources checked (KB): revision 9141e386254340a9bdba3036f2bb706a, refreshed
2026-11-14T21:44:48Z, lexical only and predating this study. Queries: rational
interpolation Sylvester sums; hyperelliptic Riemann Roch interpolation.
Source source:259e99ea23f0dbeb2eb10fd374f428c9,
crypto:research/notes/ecc2k130/RESEARCH_ECC2K130_WDSAT.md, was opened with metadata
and context. Its norm/incidence solver work motivates testing complete
relations rather than inferring speed from an encoding. The first query gave
irrelevant Sylvester-conjecture hits; it supplies no novelty evidence.

## Construction and proof obligations

For y^2=f(x), deg f=2g+1, choose d>=1 and M=g+2d distinct abscissae xi
with curve ordinates yi. Fix sigma0=1 and let all other signs vary.
Let A=product_i(x-xi), wi=yi/A'(xi), sigmaS=product_(i in S) sigmai,
and WS=product_(i<j in S)(xi-xj)^2 product_(i in S) wi. Define

    b_sigma = sum_(|S|=d-1) WS sigmaS product_(i in S)(x-xi)
    a_sigma = sum_(|T|=d)   WT sigmaT product_(i not in T)(x-xi).

At xi only T containing i contributes to a. Write T=S union {i}.
The identity W_(S union i)=WS wi product_(j in S)(xi-xj)^2 and
product_(j not in S union i)(xi-xj)=A'(xi)/product_(j in S)(xi-xj)
gives a(xi)=sigmai yi b(xi). Terms with i in S vanish in b.
Thus A divides N=a^2-f b^2. Bounds deg a<=g+d and deg b<=d-1 give
deg N<=2g+2d=M+g and deg Q=N/A<=g. The pole orders of a and b*y
have opposite parity, so a nonzero pair cannot have identically zero norm
on this odd-degree model. This gate checks the division independently.

Reject the zero pair and gcd(a,b) of positive degree. A primitive pair has
b(xi)!=0 at every anchor, and b(r)!=0 at every norm root. When Q splits,
orient each residual root r by y=a(r)/b(r), with multiplicity. The function
a-b*y has precisely the selected anchors and residual zeros, so their
degree-zero place sum is zero in the Jacobian. Branch places, repeated roots,
leading-coefficient cancellation and lower residual degrees must be retained.

Each coefficient is a multilinear sign polynomial. A Walsh transform of its
subset coefficient array evaluates all 2^(M-1) global-negation classes using
additions/subtractions after cold subset construction. This gate compares
that transform with direct signed subset summation. It does not omit the
cold construction cost or assert a performance benefit.

## Frozen gate, observables, falsifiers and limits

Implement a std-only Rust standalone test using existing field/polynomial and
Jacobian modules. Independent reference: homogeneous Gaussian elimination of
M interpolation equations in M+1 coefficients. Require projective equality
when nullity=1; for larger nullity retain the case and require both solutions
to satisfy all equations. Reject neither a test nor a tuple to improve counts.

1. Exhaust all 7^4 ordinate assignments at xi=0,1,2,3 over F7 for g2,d1.
   These are arbitrary interpolation data, not claimed curve fixtures.
2. Curve fixtures g=2,3,4; p=101,127; d=1,2,3; first squarefree thread-family
   c in 1..59. Four tuples are cyclic contiguous blocks starting at offsets
   0,3,6,9 in the canonical factor base. Visit every sign class in each tuple.
3. For every class: direct subset vs Walsh; interpolation; exact norm division;
   degree bound; independent Gaussian reference. Count nullity, zero pairs,
   nonprimitive pairs, lower degree and split/nonsplit residuals separately.
4. For every nonzero primitive split residual: verify all root orientations and
   full Jacobian identity including multiplicities. Require at least one such
   check for every (g,p,d), so vacuous correctness fails.
5. Adversarial all-zero ordinates for d2,d3 and duplicate-x rejection are
   explicit controls. Never divide by a zero weight or accept a zero pair.

Any mismatch falsifies the implementation/construction gate. Unit-test scope
only: one local process, <=60 seconds per invocation, <=256MiB target memory,
no timing claim. Retain complete stdout, build command and source hashes.
No full IC run before this gate passes and a separate cold-cost/yield/rank
protocol is frozen. Larger tuple capacity alone is not a rank or yield bound.

## Cost acceptance for a successor

The previous 100-case diagnostic subtracts actual per-case forest and solve
costs before aggregating: its maximum is 4.4482131462x against the fastest
paired control. Even deleting relation, forest and solve work together leaves
median conditional ratios 7.4728x (g2), 5.1353x (g3), 7.8965x (g4).
These are observed fixed-work counterfactuals, not universal lower bounds.
Any 10x cost case must charge setup, target handling, normalization, root
orientation, relation deduplication, recombination, rank and final checking.

