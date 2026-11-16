# Repeated fixed-base cancellation resolves the tested F97 chart rejection

Two separately frozen gates passed once. V1sourcec6478e372b41/checkpoint20261116T005110Z constructs sections for the previously rejected F97Fermatquartic. V2sourceddc7f2fbeab6/checkpoint20261116T005254Z checks the same candidate formula over fixed primes97,257,509,1009. No scientific failure, rerun or omitted field in these gates; earlier fieldchart rejection remains preserved in its study, not rewritten.

## Mathematical correction

F97discovers O=(4,24), tangent y=22x+33, residual r=x²+11x+6=(x−43)². The repeated fixed divisor is D=2Q with Q=(43,9), disjoint O and Fy(Q)nonzero. Cubic numerator constraints use divisibility by r, including multiplicity, giving the required eight-dimensional space. A repeated residual does not by itself invalidate these RRconstraints.

On a squarefree effective cubic E containing Qonce, old gcd-with-u cancellation removes only one copy. All numerator values and along-curve first derivatives atQvanish; the resulting triple-section vector is zero. Simply removing the admission guard would therefore be unsound.

New candidate computes twoNewtonsteps modulo u³, checks exact curve residual there, computes the common numerator gcd with u², and divides by fullg before reducing modulo u. In the registered control g=r, preserving fixed multiplicity2. Quotient values atQmatch independent second derivatives:

    y' = −Fx/Fy,
    y'' = −(Fxx+2Fxy*y'+Fyy*(y')²)/Fy,
    n/(x−Qx)² atQ = (nxx+2nxy*y'+nyy*(y')²+ny*y'')/2.

All120symmetric tensor evaluations match the root-free polar calculation. Projected vector[84,10,30,13,73,77,5,73]is nonzero; first-order-only vector is zero. This is a known local multiplicity/Hensel correction, not a novel speedup theorem.

## Exact outcomes

| Field | Points | Fixed base branch | Outcome |
|---|---:|---|---|
|97|44|double rational point|pass|
|257|252|split two-point|pass|
|509|520|split two-point|pass|
|1009|1100|split two-point|pass|

Each passes RRrank2/dimension8,multiplicationrank18,diagonalrank112/nullity8, exact coefficient/corruption checks,480determinant and480ordinary split tensor controls,120base-cancellation controls and canonical tangent/line verification. V1RUN.txtand V2run-v2/retain all raw/exclusions; all statuses0/stderr empty. The V2candidate uses the second-order formula for ALL fields; only independent verifier dispatch differs by discovered residual discriminant. No new-prime non-split extension-field case is claimed.

## Formal precision and boundaries

Precision.lean/LEAN.txt/EXIT-LEAN.txt check quotient_precision in pinned Lean4.19/mathlibc44e0c8ee63ca166450922a373c7409c5d26b00b. For a nonzero divisor g with u²=g*v, if g*qa−g*qb=u³*t, then qa−qb is divisible by u. Thus numerator precision modulo u³ suffices for quotient precision modulo u under the explicit divisibility hypotheses. Printed axiomspropext/Quot.sound, no sorry/customaxiom. It does not formalize Hensel convergence, geometric divisor validity or code.

Repeated EFFECTIVE divisor support remains unsupported; this extends repeated FIXED base cancellation on squarefree effective divisors. It does not establish a complete atlas, arbitrary-prime/product-safe arithmetic, subgroup centering, isogeny/DLP portability or timing advantage. The extra Newtonstep/larger modulus can increase arithmetic cost. Full verified cold default remains the earlier F127path; no new-field source DLP was executed. Genus2/4, scaling/charged construction and the mathematical10xobjective remain open; old failed coldcost result unchanged.
