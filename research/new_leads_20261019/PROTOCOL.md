# Cubic odd-torsion module screen — frozen protocol

Question: which small symplectic modules compatible with a cubic Weil
restriction admit Frobenius-stable maximal isotropic subspaces?

This is a finite linear-algebra experiment, not an elliptic-curve search,
isogeny construction, DLP solve, or speed measurement. No actual curve or
subgroup identity is assigned to these abstract modules.

For ell in {3,5}, enumerate every three-dimensional subspace of F_ell^6
once via reduced row-echelon bases. Retain exactly those isotropic for
the standard alternating form with coordinate pairs (0,1),(2,3),(4,5).
The expected complete populations are 1,120 and 19,656 respectively.

For every nonzero q residue and every t residue, let A=diag(1,q),
B=[[0,-q^3],[1,t]], and F(v0,v1,v2)=(B A^-2 v2,A v0,A v1).
Also test B=lambda I whenever lambda^2=q^3 and 2lambda=t modulo ell.
The scalar and companion cases must be separate: the trace and determinant
do not determine the repeated-root torsion module. Exclude ell dividing q.
These are specified model modules, not a claim to classify all symplectic
conjugacy classes or all elliptic Weil restrictions.

Check F's alternating-form multiplier q on all pairs of basis vectors.
Count F-stable isotropic subspaces by RREF membership and independently by
Gaussian rank of [L;F(L)] for every candidate. Require agreement, and
independently check each retained first witness by a rank routine. Identity-F
is the positive control: every enumerated Lagrangian must survive. Verify
that a non-isotropic rank-three coordinate subspace is rejected.

Output exact per-module counts and first basis witnesses as JSON lines,
plus CSV counts. Include q,t,B type, total subspaces tested, and all control
outcomes. No Monte Carlo uncertainty applies to these exhaustive finite sets.
Retain any failed assertions and exit codes. Compile offline with rustc;
the main crate is not needed. Commit this protocol and source before running.

Interpretation: a zero rejects this specific module/ell combination; a
positive count is only a necessary kernel-screen hit. It does not establish
realizability by a named elliptic curve, a rational non-product polarization,
a smooth quartic over F_q, an executable transfer, subgroup preservation,
or a faster destination solver. Before promotion require actual torsion
coordinates, pairing and Frobenius witnesses, quotient/descent checks, exact
source subgroup, and complete cold matched solving costs.

Execution envelope: one CPU, at most 1 GiB RAM, 300 seconds; no cloud dispatch.
This is a correctness screen; no performance ratios or qualified timings.
