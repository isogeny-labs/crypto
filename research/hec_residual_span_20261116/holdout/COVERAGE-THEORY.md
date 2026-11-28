# Coverage limits: field size, subgroup size and the genus-4 quotient

The holdout qualifies the stated binomial-family methods on six new fields. It does not establish a generic genus-2/3/4 breakthrough. A field parameter is not a monotone measure of subgroup DLP difficulty: the selected prime orders are101,631,301267,77152909,48781,397 respectively. The point-orderLCM/cofactor/certificate checks validate the actualsourcegroups, notan assumption that ell is comparable with q^g. Modelselection was frozen algebraicfirsteligible, so these smallorders are retained ratherthanreplaced afterseeingthem.

## Explicit genus-4 structural restriction

For C:y²=x⁹+c over Fq with characteristic differentfrom2and3 and c nonzero, the morphism

 pi:C→E, (x,y)↦(z=x³,y), E:y²=z³+c

is a separable degree3cover of a nonsingular genus1curve. It extends across infinity. The induced Jacobian pushforward and pullback satisfy pi_* pi^*=[3] onE: pullback of a divisor counts three points with ramification multiplicity and pushforward returns threetimes theoriginaldivisor. Thus pi_* is surjective, its connectedkernel A has dimension4−1=3, and J(C) is Fq-isogenous to E×A. Indeed the image of pi^* is an ellipticsubvariety, and its intersection with A is finite because it maps underpi_* by[3]; addition therefore gives an isogeny from E×A ontoJ(C). This is a deduction from the explicitcover, not a newattack.

Let D have primeorder ell, ell!=3. If ell>#E(Fq), then pi_*D=0 since itsorder divides both ell and#E. Moreover [3]D lies in the connectedimage of(3id−pi^*pi_*), which is A, so D itself lies inA because3isinvertiblemodell. The same holds for T=kD. The elliptic quotient KILLS this DLP instance; it is not a nonzero ellipticDLPtransfer.

Hasse's bound gives #E(Fq)≤q+1+2sqrt(q). Conservativeintegerupperbounds are210 atq181 and306 atq271. The holdoutorders48781 and397 exceed these bounds. Hence both tested genus-4 sourcegroups are in the three-dimensional kernel factor. They cannot support a claim about an unrestricted, four-dimensional simpleJacobian. This does not automatically produce a genus-3hyperellipticmodel ofA or a faster DLPalgorithm onA.

The factorization S9=Phi3*Phi9 of the automorphism characteristic polynomial is consistent with the2+6Tate-dimension split. Guralnick–Howe's primary result provides the general characteristic-polynomial setting: https://arxiv.org/abs/0804.0578. The explicitcover/norm argument above is the reason for the decomposition here; no abstract citation is used to infer an unconstructed solver.

## Consequence for the research objective

Future scopeclaims must state actualsubgrouporders and curveclass. Largerq alone, a selectedlarge componentratio, or geometricgenus4alone is insufficient. Genericgenus-4coverage needs another justifiedcurveclass/control; strong-subgroupgenus-2/3coverage needs a prospective subgroup-strength criterion, notposthocreplacement of weakcases. The presentholdout is valid positivecorrectness and negativegeneral-speedup evidence for its exactinstances. It leaves the requested10xgenus2+mathematicalobjective open.
