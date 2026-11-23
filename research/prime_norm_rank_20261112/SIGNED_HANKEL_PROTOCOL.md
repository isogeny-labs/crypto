# Public signed Hankel compression: prospective proof/control contract

2026-11-13. Continue the original general-prime-field sub-exponential ECDLP
objective without weakening the complete exp(o(log r)) bit-cost criterion.
This iteration tests a public representation and update rule, not a speed claim.

Let D=2a>=4, G=product_i(X-x_i), distinct public x_i and chosen nonzero public
branches z_i with z_i²=f(x_i). Put w_i=1/product_(l!=i)(x_i-x_l),
M(j,k)=sum_i w_i*z_i*x_i^(j+k), j,k<a-1. For B of degree<=a-2, the candidate
is A=(sB modG), where s is the public interpolant of z_i. Predicted equivalence:
M*coeff(B)=0 iff degA<=a iff a bounded A/B realizes those selected branches.
With the square-root support and nonzero norm, the prior direct certificate
then gives N=cG. Unlike the earlier moment matrix, M explicitly depends on
public branches; do not identify it with W-base jets without a map.

Prove the central degree test rather than assume a parity-check oracle:
for R of degree<D, degR<=a iff all top remainder moments
coeff_(D-1)((R X^j) modG) vanish for j<a-1. Complementary monomials detect a
forbidden highest coefficient. Derive barycentric moment evaluation from the
actual product nodes using pinned Mathlib Lagrange.coeff_eq_sum, not a supplied
successful F/H or abstract matrix of assumed rank. Derive B coefficient
linearity and the literal Hankel matrix-vector identity. A branch update at t
changes M by delta*w_t*v_t*v_t^T with v_t=(1,x_t,...); a sign flip has delta=-2z_t
[pcc:unverified].

Local native development controls only: same six public F17 branches as the
parent contract, f=X³+2X+2, a=3; positive assignment and only y13 flipped.
Check actual barycentric weights, Hankel entries, full six-by-six kernel versus
two-by-two kernel equivalence, interpolation from recovered B, degree bounds,
original public rows and full norm. Check every one-coordinate flip's rank-one
entry formula and determinant update against independently computed Leibniz
determinants. Include a wrong weight / coefficient perturbation negative
control that must fail the literal comparison. Do not run a random sign search,
time a solver, infer yield, or extrapolate runtime from these fixed tests.
No paid/EKS attack/scaling job; any later such panel requires live admission.

Source freeze and exact commands/exits retained. Lean source without holes or
custom axioms, standard-foundation audit and fresh kernel replay. Preserve all
failed attempts and every prior immutable artifact. Source-bound unsigned
claims, editable diagram, readable report/PDF and PR accompany results.

Sources checked (KB unavailable: MCP absent, wrapper no executable): existing
NormPublicJets / NormQuotientMoment and pinned Mathlib Lagrange interpolation;
Zeh-Gentner-Augot https://arxiv.org/abs/1110.3898 equation3 and Hankel/key-equation
setup; Cheng https://arxiv.org/abs/cs/0507026 sections3/4; Li-Wan-Zhang
https://arxiv.org/abs/1501.01138 Proposition4.1 and dense-set assumptions;
Iwata https://doi.org/10.4230/LIPIcs.ISAAC.2017.1 exact parity definition;
Mahalanobis https://arxiv.org/abs/2607.09814v2 introduction (fixed-defect success
cost explicitly retained). These do not establish the required search bound.
