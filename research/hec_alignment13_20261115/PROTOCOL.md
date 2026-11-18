# Target Kummer centering and nonsplit evaluation gate

Correctness only; no source alignment, input isogeny lift, timing or10xclaim.
All prior cold records, failed V2 and successful V3 controls remain unchanged.

Primary Tian4.2.3 https://arxiv.org/html/2007.03172#S4.SS2.SSS3 centers the
triple map at O+delta with2delta=K. Prior target section basis supplies exact
functions. This gate removes the group-reference offset before attempting
source-coordinate alignment. KB lexical query 'Kummer projective alignment
theta characteristic half canonical finite field genus three good functions'
on revision9141e386254340a9bdba3036f2bb706a returned genus2pointers, no direct
new result; absence is not novelty. Read primary Tian and current Git sources.

Pinned target arithmetic: copy hec_transport13/reused.rs from7117af90f867;
no change to its group law. Good section basis/cancellation copied from same
head with build returning its basis. Prior basis/norm/cancellation gates run
as registered positive controls, not a new timing cohort.

Use existing Q1/F127, five supplied rational points(18,52),(28,86),(43,125),
(101,43),(34,113), no rational-point enumeration. O=(18,52). Verify tangent
line y=107x+31 cuts K=2O+(101,43)+(34,113) by exact polynomial division,
residual roots and nonzero residual at O. Jacobian reference Dinf is selected
by the unchanged group law from supplied points. Define

    H=2Dinf-2O-K=2Dinf-4O-R1-R2.

Construct from point differences, check[ell]H=0 for ell1028893, set
beta=[(ell+1)/2]H and verify2beta=H. Then delta=Dinf-O-beta is a rational
degree2class with2delta=K. For a targetclass X, take E representing X-beta
relative to Dinf. The section's j(E)=E-O-delta equals X, proving centering.
Failure of odd-subgroup or double-beta checks is retained unsupported/failure,
not repaired by choosing a different half from expected data.

Extract an affine degree3graph directly from packed H0(3K-E) annihilator:
columns for1,x,x² must be independent. Solve columns x³ and y in that basis
to obtain monic u and quadratic v. Reconstruct ALL10columns, roundtrip the
result through class_from_conditions and require identical canonical QClass.
Reject unsupported projection/infinity charts; section API also rejects
repeated u, non-curve residues and F_y-nonunits. Apply prior Hensel/base-divisor
normalization, without rational or extension point scanning.

Set A=2*((28,86)-O), require nonzero and[ell]A=0. Test EXACT n1..32 on X=nA
and-X. Keep every exclusion. Require at least16pairs with both chart values,
all projectively equal under negation; at least1irreducible and1nonsplit
actual curve divisor; corrupted-coordinate control rejected. Classify degree3
u by degree gcd(u,x^127-x):0irreducible,1linear/quadratic,3fully split, after
squarefree check. Curve equation and roundtrip distinguish these from the
prior arbitrary-algebra tests. Re-evaluate available uncentered X/-X and
require at least1negative control disagreement. Do not use a matching result
to select beta or alter the fixed cohort.

Maximum600seconds aggregate guard. Commit worker/protocol/fingerprints and
checkpoint before one execution to RUN.txt/RUN.stderr/EXIT.txt. Preserve failed
output; no rerun without committed amendment. No secret target scalar is used.
Shared arithmetic implies no independent implementation/validation claim.
Remaining source-target alignment, actual inverse solving, chart completion,
full cost, and separate genus2/4 obligations stay open.
