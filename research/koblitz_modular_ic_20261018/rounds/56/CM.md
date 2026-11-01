# R56: complete A divisor, actual infinity points and rational uniformizers

## Source and exact chart

Use the retained source field L of degree16 over the base
V²+z²V+z+z³=0,with actual T=lambda. Infinity56-02checks the entire
source polynomial under h=1/z,W=h²V,s=h4lambda and all coordinate products:

    W²+W=h+h³,
    R=(1+z)V+z³, b=zR(R+1), c=z(R+1), A=1+c.

Both base infinity points Q0,Q1 are rational and smooth,with W residues0/1
and uniformizerh. Exact telescoping plus Hensel gives their unique formal
roots;128jets decide every leading numerator coefficient used here.

| Base function | ord_Q0 | ord_Q1 |
|---|---:|---:|
| R | -3 | 2 |
| b | -7 | 1 |
| c | -4 | -1 |
| A | -4 | -1 |
| Delta | -37 | -1 |

In particular,b has a boundary zero atQ1. Registered b denominators require
that branch even when the finite affine chart treats b as a unit.

## All source points above infinity

The first cover faces were repeated eighthpowers;they did not establish
ramification. Refine56-01does one further translation at each residue a,a²
inF4(a²+a+1=0),with full reconstruction of the original degree16polynomial
moduloh128. Retained precisions are120aboveQ0 and72aboveQ1.

The final coefficient valuations at degrees0,1,8,16 are(6,4,0,8)aboveQ0
and(5,4,0,8)aboveQ1. The complete lower hull has positive-root segments
of lengths1and7,with rootvalues2and4/7 aboveQ0,1and4/7 aboveQ1.
The remaining8roots have negative valuation in the translated chart and
belong to the other residue cluster. The 7segment has full reduced
denominator7;after monic normalization overF4((h)),the reviewed separable
Newton theorem gives an irreducible totallyramified degree7factor. The
length1segment gives a linear factor. Both residue conjugates pass full
coefficientwise Frobenius compatibility. Each pair descends to one closed
point overF2 with residuefieldF4.

Consequently there are exactly two closed points above each rational base
infinity point. Let R07,R01 lie overQ0 and R17,R11 overQ1,the second index
being the ramification index. Every residue degree is2,and each complete
fiber accounts for2*7+2*1=16. No further infinity point is omitted.

Check56-01freshly reconstructs allfourserialized localcharts,checks every
Newton secant inequality independently of the hull algorithm,and verifies
the exact coordinate identities and full degree accounting. Coefficients
unknown beyondprecision cannot change these hulls:all decisive vertices
are below the retainedprecision and every omitted coefficient has strictly
higher order. Separability is checked before applying the theorem.

## Explicit source uniformizers

The following are actual F2 rational coordinate formulas,not just predicted
valuation types:

    alpha0=h8*lambda²+(h4+h5)*lambda+1+h+h²,
    alpha1=h²*lambda²+(h+h²)*lambda+1+h².

In each ramified chart the full polynomial identity gives

    alpha_i=h*t*(1+h+h*t),  ord(h)=7, ord(t)=4.

The last factor has residue1,so ord(alpha_i)=11. Thus

    sigma0=alpha0²/h³ at R07,
    sigma1=alpha1²/h³ at R17

are explicit rational uniformizers,with order22-21=1. At the unramified
points R01,R11,h is a uniformizer. The normalized completed local rings
therefore have residuefieldF4 and are F4[[sigma0]],F4[[h]],F4[[sigma1]],
F4[[h]],respectively. This uses standard complete equicharacteristic DVR
expansion;it is not an assertion that the smaller coordinate-image subring
is a monomial ring or that its saturation/proper embedding is resolved.
Orders7and11 give concrete inputs for that next completed-subring check.

## Complete principal divisor and prescribed poles

R55identified B/AB as four reduced degree20fields P1,...,P4. This already
makes the local orders at these zeros precise without requiring global
normality of every finite coordinate chart:localizing at each zero makes
(A) the maximal ideal of a Noetherian local domain. That local ring is a
DVR with uniformizer A,by Stacks00PD,and its normalization adds no branches.
B is a finite order in the actual source field,so every finite zero on the
normalization lies over this quotient. A has only poles above infinity.

The complete divisor on the smooth projective source normalization is

    div(A)=P1+P2+P3+P4 -28R07-4R01-7R17-R11.

Zero degree is4*20=80,pole degree is2*(28+4+7+1)=80. These coefficients
use actual base orders multiplied by the checked ramification indices.
The initial source Newton data also gives exactly the same pole divisor
for lambda:original s=h4lambda has valuation0aboveQ0 and3aboveQ1.

Unit56-01evaluates actual lambda,V in the entire R55degree80quotient,
checks c=1,the actual lambda border relation,and full Bezout products for
lambda andb units. Both gcd checks pass and a changed inverse is detected.
It follows that lambda/A has exactly simple poles at all four Pi and no
infinity poles. It belongs to the actual prescribed space L(P1+...+P4).
Multiplication by A identifies that space with L((A)_infinity). This gives
a concrete complete-space construction target for the level77map search.

## Formal and evidence boundaries

Four authored Lean implications compile and pass separate audit,zero failing
and zero trusted:jet order stability,chart scaling,pole pullback,and the
full-denominator e/f conclusion. Actual source arrays,Hensel errorbounds,
Newton factor theorem,separability/localfield interpretation,and DVR/curve
normalization facts are external premises. PROOF_MAP.md matches each claim.
The global modular target identities and intended11action are separate.

Six terminal workflows:five accepted and one rejected. Infinity56-01failed
indexed-list mapping after its baseprefix;its late marker is excluded.
Infinity56-02maps the whole list first and passes allrequiredchecks. Native
integerdivision and redefinition warnings are retained non-error diagnostics.
All35executed historical source/protocol/driver/auditor pins match Git.
Native limits4GiBvirtual/1200s/oneSingularFLINTthread,seed56;Lean4.34.1,
mathlibd13f23b,-j1/-M4096,separate1200compile/audit,no virtualcap.

## References and next construction

Reviewed retained R50/R51complete RRsource,R52actualfield/R53coordinates,
R55completezeroquotient;[Stacks04GM](https://stacks.math.columbia.edu/tag/04GM)
for Hensel,[Stacks00PD](https://stacks.math.columbia.edu/tag/00PD) for the
principal-maximal-ideal DVR criterion,[Stacks0C1B](https://stacks.math.columbia.edu/tag/0C1B)
for valuation/ramification semantics,and
[Garrett,Newton polygons,pages1–2](https://www-users.cse.umn.edu/~garrett/m/number_theory/newton_polygon.pdf)
for separable root counts and the full-denominator conclusion. The facts
about this source are checked/derived here,not quotations from those sources.

Match the exact R27divisors of the complete68-section L(D0),D0=7D_A+D_B,
to these actual infinitypoints. Once (A)_infinity≤D0 is certified,impose
all degree48local constraints inside that complete space to construct the
full L((A)_infinity),rather than another coefficient box. Compute all32RR
basis/nu expansions using the explicituniformizers;then test source-bound
modular maps with descent/denominator/intended11action checks. Preserve all
properpolarization,separateincidencesaturation,node187,reverseclass and
larger112/991lanes. NEXT.md records their continuation.
