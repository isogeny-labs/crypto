# Exact prime m2 original-top Hilbert calculation

2026-10-28. This is a frozen-method design for exact computations on six
retained inputs: original fixed-base planted/natural and paired baseline/spike40
planted/natural. No new target, point base, polynomial or preprocessing is used.
The wider n83 m2..9 FFD/LFD and elite-base study remains open.

## Literature and mathematical derivation

Primary sources read before implementation:

- Ghorbani, Khosrovshahi, Maysoori and Mohammad-Noori, *Inclusion Matrices and
  Chains*, https://arxiv.org/pdf/0709.3144, Corollary 3 and its proof, page 10.
  For t<=k<=n-t, the integral inclusion matrix W(t,k) is unimodularly equivalent
  to a diagonal matrix with entries C(k-j,t-j), multiplicity C(n,j)-C(n,j-1).
- Phuong and Tran, *A new proof of Stanley's theorem on the strong Lefschetz
  property*, https://arxiv.org/pdf/2211.13548, Theorem 2.7 and proof, pages 5-6.
  The sum of variables in the quadratic monomial complete intersection has
  maximal-rank powers in characteristic zero or greater than the variable count.
  This is a separate cross-check on the large-characteristic specialization.
- Retained TOP_DESIGN.md, top.rs and TOP_FORMAT.json establish the saved input
  presentation; their frozen bytes will not be changed by this phase.

The new native checker matches the ENTIRE source against the saved m2 S3 plus
54 Boolean field equations, including their exact coefficients and target.
The original highest S3 form is x0^2*x1^2; the field tops are v_i^2.
Nonzero selector weights yield an invertible diagonal change, checked by
recorded modular inverses. Thus the graded quotient is

    Q = (A tensor B) / (L_A^2 L_B^2),
    A = Fp[a_1,...,a_27]/(a_i^2), B = Fp[b_1,...,b_27]/(b_i^2),
    L_A = sum a_i, L_B = sum b_i.

The squares here are monomial ideal generators, not Boolean reduction of the
original input before taking its highest forms. No affine consequences are added.

In A_i, squarefree monomials are a basis of size C(27,i). Multiplication by L_A^2
from degree i-2 has matrix twice the subset inclusion matrix. For i>=2 put
(t,k)=(i-2,i) when i-2<=27-i, otherwise complement and transpose to
(t,k)=(27-i,29-i). Both obey t<=k<=27-t. The native certificate records every
2*C(k-j,2), its multiplicity C(27,j)-C(27,j-1), and its residue-nonzero test.
The rank r_i is the sum of multiplicities with nonzero entry; r_0=r_1=0.
Here p>27, so all entries are nonzero and the ranks equal
min(C(27,i-2),C(27,i)). This last equality is checked, not an assumption about
small characteristic.

In bidegree (i,j), multiplication by L_A^2 L_B^2 is the tensor product of these
maps. Its image has dimension r_i*r_j. Distinct bidegrees form a direct sum,
so the exact Hilbert coefficient is

    h_d = sum_(i+j=d) [C(27,i)*C(27,j) - r_i*r_j].

Every term, all degree coefficients through 54 and the terminal zero at 55
are retained. The first zero coefficient is the stated degree of regularity
under the original-top graded-vanishing convention. It is not a solving degree,
last fall degree, affine final-basis degree, or attack-cost measurement.
The source does not hard-code the resulting degree.

The field hypothesis is also checked: a full Lucas primality certificate uses
the complete factorization of p-1, trial-proven small factors, and a unit whose
order is p-1. Fermat residue 1 and nonunit exponents for every distinct prime
factor force order p-1. A composite p has phi(p)<p-1, a contradiction. Native
addition/doubling modular multiplication avoids overflowing u128 at this p.

## Controls, execution, limits and retention

Development and runtime controls independently expand multiplication matrices
and perform modular Gaussian elimination for n=2..8, all degrees, p=2,3,5,101.
Small p must include nonmaximal ranks; in p=2 the squared linear form is zero.
Full product matrices for n=2..4, p=2,3,101 check the bidegree convolution against
explicit total-degree images. The two code paths share only basic modular
arithmetic and binomial dimensions. They do not constitute independent review.
A complete-source negative control must reject omitted equations.

Six cases and six fresh-process deterministic replays; one thread, 2 GiB virtual
memory, 120 seconds each producer/replay, no retries. No statistical or clock
comparison. The existing-host fallback remains tied to the earlier retained
EKS shared-bucket denial; the fresh overview found no duplicate running job.
No IAM or capacity is changed. All files, logs, failures and partial outputs
are uploaded to the approved shared bucket and preserved by content hashes.
Protocol/source commits and a source snapshot are uploaded before production.

## Formalization map

HilbertArithmetic.lean checks only the finite large-characteristic rank-formula
arithmetic and the position of its first zero by kernel-checked `decide`.
The subset-inclusion rank theorem, invertible coordinate substitution and
polynomial-quotient/tensor-image bridge are NOT formalized here. They have the
written proof above and cited primary theorem. Do not describe the whole
regularity certificate as Lean-proven. No custom axiom or native_decide is used.
