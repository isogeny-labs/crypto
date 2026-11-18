# Exact planted full-system regularity: 197 to 14

For the unchanged fixed planted m2 system on P-256 base FB1h16882188194e,
the native certificate gives **D_dense=197, D_chain=14**. D is ideal-module
Castelnuovo--Mumford regularity of the actual generator homogenization,
with no affine-linear preprocessing. This is a producer certificate, not
independent signed or Lean admission. Claims5ad714/1cb5fb [pcc:unverified].

The baseline/candidate are the preregistered GCB-dense/GCB-chain. Their
equations, point sets, target, degree4 group circuit and target line did not
change. Dense has4 affine variables and4 membership equations; chain has18
variables and102 membership equations. Each has one group equation and H.
Every added chain coordinate reconstructs uniquely, with no ordinate/sign
cover or lost point. Both have the same ordered solution set after projection.

## Finite input and complete relation reference

Each one-point algebra has288 distinct rational points. Frozen H-free
graded border/rank certificates prove its exact reduced vanishing ideal
over the algebraic closure, with a graded free K[H] basis B. Dense input
quotient regularity is97, chain5. This phase independently constructs the
dense degree-compatible point basis for interpolation and reproduces the
cached chain certificate's hash. It does not change the original dense CI
presentation or preprocess the group equation using known relations.

The earlier exhaustive reference checks all82944 ordered tuples. Its
planted solutions are exactly (193,262) and (262,193). These are two
distinct ordered points but the same index-calculus row. Both are replayed
again with GMP/OpenSSL. The first input's x-coordinate separates the two
solutions; its nonzero difference and inverse are recorded. Thus their
reduced homogeneous coordinate ring C has Hilbert numerator1+t over1-t.

## Explicit annihilator certificate

Let E_{p,i}=B_i(p). Exact elimination solves E c_a=delta_a and E c_b=delta_b
for a=193,b=262; the resulting indicator polynomials are checked on EVERY
one-point input. In the two-point tensor basis, the accepted-tuple indicator
vectors are v0=c_a tensor c_b and v1=c_b tensor c_a. They are independent.

For degree cutoff n, a combination alpha*v0+beta*v1 is represented in
degree n exactly when its coefficients at every basis monomial of total
degree>n vanish. The coefficient matrix has only two columns. The verifier
checks its exact tail rank: rank2 by a nonzero2x2 determinant, rank1 by a
nonzero row AND zero determinants with every remaining high-degree row,
rank0 by checking all high-degree coefficients are zero. This certifies
all cutoffs through the maximum tensor-basis degree, after which it is zero.

The kernel dimensions and explicit kernel combinations give a graded free
K[H] basis of Ann_A(f), because f vanishes precisely on the two accepted
tuples and A is the exact reduced point algebra. Each reported generator
is checked to have its stated degree and zero coefficients above it; its
combinations span every cutoff's kernel. Independence and these dimension
checks prove the generators are complete, not merely annihilating examples.

Observed generator degrees [pcc:unverified]:

| Arm | Input quotient r | Ann free generator degrees | Ann Hilbert numerator |
|---|---:|---|---|
| Dense |194|193,194|t^193+t^194|
| Chain |10|10,10|2t^10|

These numbers are computed from interpolation/tail ranks. The dense CI
duality prediction is checked only AFTER the computation, not supplied as
its input. The full tensor coefficients are reproducible from the emitted
one-point vectors, with a canonical stream hash; no relation truth table
is used to build or preprocess equations.

## From the annihilator to exact regularity

The actual homogeneous multiplication sequence gives

    Hilb(A/(f),t)=(1-t^4) Hilb(A,t)+t^4 Hilb(Ann_A(f),t).

Saturating the quotient in H gives C, since all geometric input points are
rational and the complete reference has exactly those two solutions. The
kernel T of A/(f)->C is finite H-torsion. The verifier subtracts
Hilb(C)=(1+t)/(1-t), checks that the difference is a polynomial with
nonnegative integer coefficients, and emits that complete polynomial.

For clarity, this uses the same standard CM regularity as the frozen
protocol: reg(M)=max_i(end H^i_m(M)+i). Here M is finite over K[H], so
H-torsion equals irrelevant-ideal torsion. The exact sequence with C gives
H^0_m(M)=T and H^1_m(M)=H^1_m(C); higher local cohomology vanishes. C has
regularity1. Consequently

    reg(M)=max(end T,1),     D(J)=reg(S/J)+1.

This is an equality, not a leading-ideal estimate. It retains the original
generators' finite H-torsion rather than replacing them by saturation.
The native coefficients give [pcc:unverified]:

| Arm | Last nonzero T degree | Quotient regularity | Full ideal regularity |
|---|---:|---:|---:|
| Dense |196|196|197|
| Chain |13|13|14|

Primary source for the standard local-cohomology definition and its
equivalence with graded Betti regularity: Marc Chardin, Some Results and
Questions on Castelnuovo--Mumford Regularity, sections1--2, read before this
proof writeup:
https://webusers.imj-prg.fr/~marc.chardin/publications/textes/24.survey-reg.pdf
Filtered point-algebra prior art remains linked in ANN_M2_PROTOCOL.md.
These classical arguments are reused; no novelty claim.

## Validation and scope

Primary/fresh bounded native invocations both exit0 with byte-identical
four JSONs. Seven corrupt controls rejected: missing accepted tuple, and
altered indicator/rank/ideal-shift in each arm. All288 point evaluations,
all82944 tensor rows per arm and complete tail-rank conditions are covered.
There is no discarded probabilistic branch. API counts are certificate
setup/check work, not measured field-equivalent solving operations.

This closes the planned exact planted m2 algebraic qualification locally.
It complements the earlier unplanted198to14 result. It does not itself
establish exact higher-m degrees, an affine F4 degree reduction, a runtime
gain or independent signed/Lean admission. The m2--7 scope stays active.
