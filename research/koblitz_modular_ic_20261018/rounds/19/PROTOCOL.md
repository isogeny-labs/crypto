# Round19 preregistration

Client date22 October2026 America/Los_Angeles; execution23 October UTC.
Parent a2b8f27be5fb3faac9362430ef43245b3bce819d. Earlier rounds are frozen.
Rust performs arithmetic, search, result assembly and hashes. Python is used
only for text editing, existing repository tests, plotting and PDF inspection.
This round makes no IC cost assessment.

## Controlled-pole level77 maps

CM23 searches x=N/q^2, y=M/q^3 on the actual CM17 incidence chart, for
q in [1,b,c,1+c,b+(1+c)c]. Enumerate all32^5 numerators N in the F32-span
of [1,b,c,lambda,nu]. M ranges over the F32-span of the15 monomials of
total degree at most2 in b,c,lambda,nu. For each elliptic sign use all ten
CM17 points plus the eight independently constructed CM22 holdouts.
At a point with q=0 skip that constraint and record the skip. Reject x
without a target y before solving the full F2-linear system for all75
coefficient bits of M. Enumerate all solutions if the nullity is at most20;
otherwise retain the complete affine solution space and mark enumeration
incomplete. Never promote a finite fit to a function-field map.

The pole bound is intrinsic: let D be the sum of the positive pole divisors
of b,c,lambda,nu on the smooth proper normalization. N has poles bounded by
D and M by2D, hence x and y have poles bounded by D+2(q)_0 and
2D+3(q)_0 respectively. These bounds do not compute cusp multiplicities,
identify a prescribed cuspidal Riemann--Roch space or yield a factor map.
Positive controls recover both signs of the identity on a synthetic
elliptic chart using the same linear solver. Bound20 minutes per q,
32^5 x forms per sign, at most5 denominator families and2 signs.

## Mixed F2 invariant sections

G22 constructs a small, explicit system on an auxiliary mixed bundle on
the rotation-compatible scalar cover. On E^4 put Q5=-(Q1+Q2+Q3+Q4).
The five rational functions x(Qi) are permuted by the augmentation5-cycle;
their elementary symmetric functions e0,...,e5 share pole divisor bounded
by 2 sum_i [Qi=O]. Pull back by multiplication105, verifying the orbit,
coefficient Frobenius, and invariance under actual105-torsion generators
from G17. This yields sections under the written divisor/descent bridge.
It does not identify this auxiliary mixed bundle with3L or construct the
243 sections of the requested polarized quotient. Check function values
on16 valid tuples per sign in F2^168, retaining all pole rejections.
Cap512 tuples,40 torsion translates per tuple,20 minutes. Prove the
generic symmetric-function invariance and actual orbit in Lean.

## Four Q28 models

Q29 first profiles the actual source cones for nodes95,97,123 in each
graph using the existing exact univariate normalization, with degree32,
4096 terms and100000 memoized nodes. If an actual higher-degree
univariate coordinate is available, construct a resultant relation for a
lower-degree supported node and that coordinate. Bound lower degree12,
2^12 determinant states and50000 terms. Verify the resultant by exact
univariate substitution before using its homogenization as a separator.
Export the source DAG, coefficients and selected F2 boundary coefficient.
The structural elimination implication must be proved or its missing
formal bridge stated. Retain any failed resultant attempt.

If Q28 is excluded, add only the proved radical boundary consequence and
rerun all Q13--Q17 constraints plus the ten existing source repairs, using
100000 visits per query and1000000 per graph. Retain every new survivor;
whole saturation remains open without an equality certificate.

## Evidence and proofs

Preserve command logs, bounded failures, exact outputs, mathematical notes,
proof map, deterministic raw/validation archives and byte-identical replay.
Every new named Lean result is kernel checked and its axioms audited.
The geometric normalization, divisor and descent bridges remain explicit
where not formalized. Refresh the canonical frontier, report, editable SVG,
150dpi PNG and an inspected PDF. Commit and update the existing ready PR.

## Q29 coupled amendment, after profiling and before resultant arithmetic

The univariate attempt fails its own trigger: nodes95/97 already have the
maximum pure-x degree9, and123 is not univariate. Preserve that failure in
q29-source-profiles.json. Gate123 is y times a degree8 x-polynomial S,
while95 is a degree9 polynomial P. On the actual ordinary elliptic input
its norm is V^2+x S(x)V+S(x)^2(x^3+a x^2+1). This has x-degree19.
Compute its norm in F360[U,V][x]/(P(x)-U), using a9x9 multiplication
matrix and determinant subset recurrence. Verify exact substitution
U=P(x), V=y S(x) after reducing y^2+xy=x^3+a x^2+1. Check that the
ordinary projective homogenization has unique top-total-degree term
c U^19 with c nonzero. This would force U=0 at the reduced boundary and
exclude Q28. It remains a coupled source elimination result, not complete
saturation. Bound4096 determinant states,50000 terms,20 minutes.

## Q31/Q32 amendment, after Q30 and before family arithmetic

Q29 succeeds, while Q30 retains support[120,122], the degree8 pure-x
source polynomial and its constant translate. Compute all distinct actual
pure-x polynomials of degree1 through8 on input0, compare each with the
actual degree9 polynomial at95, and export their monic norm resultants.
A d-by-d determinant (d<=8) in F360[U,V] gives total degree9 and only
U^9 at its leading boundary. Verify every relation by independent exact
univariate substitution. Deduplicate identical polynomials while retaining
all source-node aliases. All such lower-degree coordinates then vanish
at the reduced boundary, independent of the value of node95. Retain source
coefficients, memoized DAGs, all resultants and actual Q30 bindings.
Cap256 distinct polynomials per graph,50000 terms and20 minutes. Q32 adds
only these radical consequences and reruns the full retained constraints
with the prior visit caps; new survivors remain unclassified.

## G23 amendment, after G22's auxiliary-degree obstruction

The auxiliary2(I+J) polarization cannot reach the requested quotient.
Test whether each of the two actual G16 Hermitian matrices H is exactly
sum_(j=0)^4 (ell R^j)^dagger(ell R^j), for an integral CM row ell.
The ring is Z[w], w²+w+2=0, conjugation w->-1-w; R is the actual
augmentation cycle. Enumerate all390625 rows with each of the eight
integer CM coefficients in[-2,2], first retaining the original[-1,1]
subbox count. Check the full matrix, with a trace filter only as a valid
necessary condition. Bound20 minutes and record every match or the full
bounded null. A match is a polarization identity, not the canonical
symmetric Pic2 bundle identification or an invariant19-kernel basis.
If there is a match, evaluate its five pullback points and orbit sections
on the retained G22 tuples, preserving poles; add a finite Lean identity
certificate. Otherwise retain this search as a negative bounded lead.

## G24 amendment, after the bounded G23 null

G23 has no match for either H; preserve its null and trace counts. Test an
exact transfer-lattice explanation modulo5 before generalizing that null.
For the actual augmentation R modulo5, v=(1,1,1,1) satisfies Rv=v and
sum_j R^j=0. Consequently every transfer sum_j(R^j)^t M R^j annihilates
v. Verify that the actual real coefficient matrices of H0,H1 and3H0,3H1
do not annihilate v. If confirmed, this excludes all integral orbit-sum
Gram decompositions (including sums of orbits) in the written CM basis,
without a coefficient cap. It does not exclude arbitrary invariant mixed
line bundles, fractional pullbacks or general sections. Export all actual
matrices and the finite obstruction, and prove the structural transfer
implication plus actual certificates in Lean. Bound matrices4 and1000000
exact modular operations,20 minutes.

Client context update: the client now supplies23 October2026 as its date.
The original22 October preregistration header is retained as historical
context; the final ledger and report use the supplied23 October date.

## G25 amendment: clear the transfer obstruction on the actual scalar cover

G24 concerns primitive H and3H. It does not exclude3*105² H on G21's
stable scalar cover, which is divisible by5. Inspection of the recorded
C_CM columns shows that rows0,1 of C divide by5; the last two factor
weights315,855 divide by5. Test the positive transfer seed consisting of
rows C0/5,C1/5,C2,C3 with weights45,315,63,171. Its5-orbit Gram sum
should equal3*105² H. Verify the full actual CM matrix before using it.

If the matrix identity holds, evaluate elementary symmetric orbit
coefficients of both x and y on each group of five pullback points.
Together with1 these give41 rational sections with common divisor
sum_i weight_i sum_j(ell_i R^j)^*[O]. All weights are at least3.
Check rotation and twisted coefficient Frobenius on the16 retained G22
split tuples per sign, retaining every pole rejection. Test the24 actual
small-prime torsion translations on the first valid tuple per sign;
record noninvariance rather than asserting a quotient map. Bound20 maps,
32 tuples,48 translate diagnostics and20 minutes. Add actual weighted
Gram/orbit/weight Lean certificates. The polarization identity does not
match the canonical symmetric Pic2 class or compute theta kernel lifts.

Clarify G22's dimension obstruction using the actual companion factor:
G16's companion line bundle has degree57, not3. The full auxiliary
product therefore has section dimension57*80*105^8, which must also be
checked nondivisible by361 before applying the full degree361 gluing
obstruction. Preserve the initial ordinary-only G22 artifact and rerun
with this explicit companion check. This does not alter its point values.

## Recovery and control wording, before the evidence freeze

After the23 October Spot migration the saved complete audit has732
checked theorems, zero failures and zero trusted results. No audit or
mathematical experiment is repeated merely because its process ended.
The approved handoff, managed Diem state, EKS inventory, original PR head
and local Conductor are reconciled before the remaining freeze/replay.
No EKS job or generator is submitted. Original bounds and acceptance
criteria remain unchanged.

"Both signs of the identity" in the positive control means the identity
map and its elliptic conjugate on the a=0 synthetic chart. It is not a
known-solution solver control for both curve coefficients a=0,1. All64
actual constant survivors across both coefficients are independently
checked by Lean. The report now states the control's exact coverage.
