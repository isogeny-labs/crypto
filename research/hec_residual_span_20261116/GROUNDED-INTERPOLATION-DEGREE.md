# Grounded interpolation finisher: exact degree, rank and cost obligations

[pcc:unverified]. Author-derived classical algebraic screen, extending the generic nullity-window result and earlier genus2 onecore promotion trial. No new experiment, new mathematical priority or tenfold result. Applies to smooth odd-characteristic monic y²=f(x), deg f=2g+1, with one rational point infinity.

## 1. Correct construction for every genus under study

Choose g+1 distinct abscissas xi of grounded rational points Pi=(xi,yi), choosing actual signed ordinates. Let U=product_i(X-xi) and V be the unique degree<=g polynomial with V(xi)=yi. Every factor of U divides f-V², so

    H=(f-V²)/U

is a polynomial. Its degree is EXACTLY g and it is monic: deg(V²)<=2g, while f is monic of degree2g+1 and U is monic of degreeg+1. If H splits completely, each residual point is (r,V(r)) with the multiplicity of r in H, and

    sum(input point classes)+sum(residual point classes)=0

in J(Fp), where every point class subtracts infinity. Root overlap with an input adds multiplicity, not a duplicate row. No extra source or target coefficient is inserted by fiat; the forest's affine expressions for those points carry the target dependence.

|genus|interpolated input points|degree V at most|residual degree|
|---|---:|---:|---:|
|2|3|2|2|
|3|4|3|3|
|4|5|4|4|

The earlier three-point genus2 promotion experiment therefore does not automatically provide a quadratic-tail construction in genus3/4. Cubic/quartic residual splitting and residual grounding must be priced. If an output point is ungrounded, the relation may be valid but is outside the restricted scalar-signature finisher. It must be handled by a separately specified full-root method or counted as ineligible, not treated as a free log.

## 2. Fixed-space Riemann–Roch condition count

For a FIXED effective pole divisor E of degree n>=2g-1, Riemann–Roch gives l(E)=n-g+1, since deg(K-E)<0 and thus l(K-E)=0. After k INDEPENDENT prescribed zero conditions, the vector space has dimension n-g+1-k. A nonzero function requires k<=n-g. If its actual poles have full degree n, the remaining zero divisor after those k zeros has degree n-k>=g.

Thus a residual of degree s<g requires at least g-s excess zero conditions beyond the freely available projective dimension n-g. They can be met only through dependence/special choices, a search for compatible supports, a change of pole divisor or another mechanism that is itself accounted for. For s=2 the excess is g-2, consistent with PENCIL-PARAMETER-DEFICIT.md. Changing to a larger FIXED pole divisor does not remove this nonspecial dimension count.

This is not a universal complexity lower bound. If the zeros are dependent, the space is special (outside the stated degree condition), the actual pole order is smaller, or the pole divisor itself varies, the assumptions must be re-evaluated. Each such construction needs an explicit incidence/dimension analysis and charged search. A bare parameter count is not a proof of absence of all fast relation methods.

For E=n*infinity on the odd model, x has poleorder2 and y poleorder2g+1. Below2g+1 only polynomials in x occur. Their nonbranch zeros occur in involution pairs and cancel after signed projection. Branch contributions are the already-implied 2-torsion relations (projected log0 for oddell), not new generic label information. This exception must not be used to advertise a useful low-degree principal relation from an arbitrary all-grounded support.

## 3. Exact one-dimensional finishing equation

Suppose the CURRENT small system B*S=d is consistent with nullity1 and unresolved target. Write S=S0+w*theta, with w_k!=0. A grounded point has an affine expression h_i(S)+c_i from the forest. Hence its log is alpha_i+beta_i*theta. For a fullygrounded interpolation relation with signed multiplicities mi,

    A=sum_i mi*alpha_i,
    C=sum_i mi*beta_i,
    A+C*theta=0.

If C!=0, theta=-A/C and k=S0_k+w_k*theta, followed by native kD=T. If C=0, consistency requires A=0 and this relation adds no target information. This is an exact criterion only because all residual and input points are grounded. Free-root coordinates cannot be suppressed.

Computing S0,w and the alpha/beta values is a real part of the algorithm. The existing rank diagnostic did not construct them. A sparse forest insertion with the existing small solver can serve as an exact reference, but its cost cannot simultaneously be omitted from the candidate and used as a free quotient oracle.

## 4. Explicit arithmetic schedule worth qualifying

U can be built by successive linear-factor multiplication. For each xi, obtain Li=U/(X-xi) by synthetic division. Evaluate di=Li(xi); distinct xi makes every di nonzero. Batch-invert all di with the retained inverse routine, then V=sum_i yi*di^(-1)*Li. Form f-V², divide exactly by U, split degreeg H with the qualified native oracle, and evaluate signed residual ordinates. Record ALL failed splits and ungrounded residuals. Independent polynomial and original-group checks are needed on every admitted trial in qualification.

This shares one batch inversion across interpolation denominators, but still has polynomial multiplication/division, degreeg factoring, image/tag assembly and point-selection work. A 64-trial or other bounded schedule is not admitted here: no empirical success probability, rank distribution or complete-cost case has been established for it. Under a uniform random-polynomial heuristic the split probability would be about1/g!, but these interpolated residuals are not proved uniform; do not use that heuristic as a coverage or speed guarantee.

## 5. Decision and falsifiers before execution

Do not port the genus2 quadratic-tail premise to genus3/4. A proposed all-genus finisher must explicitly use degreeg residuals, or prove how it satisfies the extra g-2 conditions for quadratic residuals. Compare its whole cold cost to both staticforest and public-normalization controls, including the prefix needed to reachrho1. The existing windows are mostly late; an attractive finishing component is not a tenfold whole-method result.

Correctness falsifiers include nonzero remainder in U|(f-V²), wrong residual degree, lost multiplicity/sign, a residual point treatedgrounded without a valid forest expression, a zero signature classifiedindependent, disagreementwith full coefficient/target-rank reference, or failure of native target equality. Coverage failures and unresolved bounds mustremain visible ratherthan be repaired by postoutcome point/cut selection.

## Sources and scope

Primary context checked in full HTML: Falcone–Filippone, Mumford representation and Riemann Roch space of a divisor on a hyperelliptic curve, arXiv2303.08441v1, sections1–2, https://arxiv.org/html/2303.08441v1 . It gives Mumford/interpolation/divisor reduction and explicit Riemann–Roch bases. The construction/condition-count/finishing argument above is derived here; no new theorem attribution or independent review is claimed. Local prior sources: promotion-interpolation/RESULTS.md, PENCIL-PARAMETER-DEFICIT.md, GROUNDED-DUAL-SCREEN.md, native-nullity-window/RESULTS.md and native-public-constraints/RESULTS.md. Fullgenus2+10xgoal remainsactive/unachieved. Gitcustody; no S3/cloud/merge.
