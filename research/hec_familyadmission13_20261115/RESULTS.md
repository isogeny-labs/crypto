# Elliptic decomposition excludes the portability family from generic genus-three scaling

Source5e5e52cabbd6, pre-run checkpoint20261116T005753Z. One once-only audit passed, exit0/stderr empty. No DLP or timing executed. This is an applicability correction, not a speedup or novelty claim. Prior section/tensor correctness results remain valid.

## Exact quotient argument

On C:x⁴+y⁴=1, the three sign involutions have genus1quotients

    E0:s²=1−t⁴ via (t,s)=(y,x²),
    E0:s²=1−t⁴ via (t,s)=(x,y²),
    E1:s²=t⁴+1 via (t,s)=(x/y,1/y²).

The equations are checked at every affine rational point where defined. These maps extend between smooth projective curves; characteristic is odd and the quartic quotient equations are separable. The quotient by the full sign group is the conic U²+V²=1. Pullbacks of regular elliptic differentials dt/s are −x dx/y³, y dx/y³ and dx/y³, respectively. They form the three independent canonical differential directions of the smooth plane quartic. Hence the induced map between E0×E0×E1and J_C has full differential rank and is an isogeny overFp. This is a readable standard geometric argument, not established solely by the code or a fullLeanformalization. Relevant primary context: [Fité–LorenzoGarcía–Sutherland2018](https://link.springer.com/article/10.1007/s40687-018-0162-0). Explicit maps above fix the field of definition for these exact curves; arbitrarytwists may have different rational decompositions.

## Registered numerical audit

Elliptic orders use Legendre sums and correct smoothprojective pointsatinfinity. Ccounts are independent exhaustive affine-plus-infinity enumeration. Every field satisfies #C=p+1−2t0−t1. This trace check is NOT an independent determination of the whole zeta polynomial; #J below is DERIVED from the isogeny and elliptic orders.

| p | #C, counted | #E0, counted | #E1, counted | #J, derived | Largest prime divisor |
|---|---:|---:|---:|---:|---:|
|97|44|80|80|512000|5|
|257|252|256|256|16777216|2|
|509|520|520|500|135200000|13|
|1009|1100|1040|1040|1124864000|13|

Exact factors are printed in RUN.txt. Quotient equation/differential substitution checks pass at all applicable enumerated points. Over any of these fields, every prime divisor of #Jdivides an elliptic factor and is at most p+1+2sqrt(p). Consequently this family cannot supply ell~q³subgroups for the intended genericgenus3scalingcomparison. No observed section speed/cost on this family should be promoted to such a DLPclaim. This is not a claim that every non-hyperelliptic quartic decomposes.

## Actual transfer instance remains distinct

The previously executed source has p127,N2057786=2*1028893, from hec_coldfiber13_20261115/run-v2/COLDPREP.stdout and public scalar/subgroup verification. A proper positive-dimensional factor of a threefold overF127has dimension at most2; the general abelian Weilbound gives order≤(sqrt127+1)^4<13^4=28561. Its order cannot be divisible by1028893. Thus an F127isogenous decomposition into lower-dimensional factors is incompatible with the established order. This does not prove absolute simplicity after extending the field. The Fermatfamily audit neither invalidates nor extends the actual F127transfer result.

## Formal and research boundary

PrimeBound.lean checks the elementary conditional statement: a prime dividing a*a*bwith positive a,b≤Bmust be≤B. Same pinned Lean4.19/mathlib environment as priorproofs; inspect LEAN.txt/EXIT-LEAN.txt. This formalizes arithmetic inference only, not quotient geometry, pointcounts or abelian isogeny.

Decision: keep Fermatcohort as section/cancellation controls only. Do not use it for claimed generic10xcrossover. A future performancecohort needs subgroup order certificates, explicit same-target transfer and baseline, admitting actual source/kernel construction costs. Prefer parameterizing the existing simple F127source family or another explicitly justified source with a certified large prime subgroup. Known Smith2-isogeny transfer is an alternative literature track, not a construction executed here. Genus2/4and the mathematical10xobjective remain open. Prior failed coldcost comparison unchanged.
