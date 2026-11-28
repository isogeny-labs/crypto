# Degree argument: assumptions and mechanical obligations

The proposed bound is still T=2 sum(q_i-1)+2. The fresh audit separates the
general argument, finite certificates and kernel-checked implications. A
producer-owned check or a checked conditional implication is not an independent
signed verdict on the full quantified tower theorem.

The generators are the two full tower chains and the two quadratic Vieta
equations. Terminal values are constants; no field equations are introduced.
Replacing the product Vieta equation using a constant multiple of the sum
equation preserves constructible spaces: both original and replacement
generators have degree2 and the constant row operation is invertible. The
normal form argument must refer to these actual generators, not a radical
ideal or a set of field solutions.

For a fixed complete branch, define the one-coordinate Lagrange projectors.
Every backward sibling fiber must have exactly q_i **distinct field roots**,
not merely one successful path or q_i roots counted with multiplicities.
Their sum is the literal polynomial1. Recursive summation over all paths
therefore gives a literal partition of unity. The highest differing branch
coordinate gives Kronecker evaluation; this supplies linear independence.

The coordinate identity is

    (u_j-a_j)e_j = gamma_j(F_j+(u_(j+1)-a_(j+1))D_j).

Propagate the second term upward through the remaining projectors. Each step
replaces a degree(q_i-1) projector by a denominator of degree at most(q_i-1).
Thus every gate multiple has degree at mostS+1, and the two-chain coordinate
relations lie inV_(2S+1). If degD_i=q_i, this particular degree argument does
not apply. That does not by itself prove a counterexample to another bound;
the runtime rejects this unsupported input. Coprimality ensures a finite
fiber root cannot also be a pole.

Multiply the two Vieta generators by a two-chain projector e. Replacing a
coordinate product costs one extra coordinate multiplication, giving budget
T=2S+2. On a branch, put q=a_0+b_0+t and
K=(a_0-t)(b_0-t)-3t^2-A. The remaining relations are
e(L^2-q) and e(2rL-K). For r!=0 this gives a field block if ell^2=q,
ell=K/(2r), otherwise a discarded projector. For r=0,K!=0 discard the
projector; for r=0,K=0 retain the full algebra F_p[L]/(L^2-q).
In particular q=0 retains a nonzero nilpotent, and nonsquare q retains an
extension-field block even when it has no F_p-valued slope. Replacing these
algebras by field-point sets changes the ideal and is not valid for d_F.

Represent the product of retained blocks by e or e,Le. All representative
degrees are at mostT-1. The quotient map followed by this linear section
defines NF. Kronecker projectors and the basis1,L in quadratic blocks show
the section is injective; the quotient kills each original generator and
therefore the generated ideal. Coordinate action certificates and the quotient
identity give the explicit NF contracts in BoundBridge.lean. The new generic
Lean law proves the remaining monomial induction using the **actual degree**
of p-NF(p), then sums over the support of f. This avoids an invalid induction
over arbitrary sums whose high-degree terms could cancel.

The finite Lean instance uses the original F17 two-level rational map
(X^2+1)/(2X), terminal4. It checks all field roots, denominator nonvanishing,
literal partition and all eight coordinate identities. Separate quotient-ring
and nonsplit controls distinguish nonreduced/extension blocks from point sets.
These are finite instance checks, not all-depth tower contracts.

Repeated tower fibers violate distinct splitting and are rejected. Repeated
abscissae in a target quadratic are different: they require the diagonal
branch Delta=0 and H(s/2)=0. For the prime-order19 curve overF17, the normalized
Velu fiber with terminal12 gives abscissae5,7. The actual repeated-point
relation (5,1)+(5,1)=(6,3) has slope13. Here H=Z^2+5Z+1 and (Z-5)^2 leaves
remainder15Z+10, despite H(5)=0. Requiring repeated-quadratic divisibility
would discard a valid relation on an odd-prime-order curve. Its new native
test and the formal instance checks are separate from timed work.

Empty composition means singleton membership x=c: substitute the constant or
retain the two linear membership equations. Free abscissae without membership
are a different system and are not covered by the depth>=1 hand argument.

Remaining general obligation: instantiate the finite-product quotient/section,
all projector actions and the degree budgets for arbitrary tower depth/degrees
in the formal framework. BoundBridge does not assume the desired last-fall
inequality, but its NF contracts are still hypotheses until that instance is
built. No generic independent signed standing is claimed [pcc:unverified].

## Prime-order specialization does not license changing the general ideal

On an odd-prime-order target group there is no nonidentity rational two-torsion,
so a finite target with ordinate zero is absent. This removes that runtime
branch for those particular targets. It does not license silently radicalizing
the proposed general tower theorem, which was stated for arbitrary target
ordinates and exact generators. The finite nonsplit/nilpotent controls audit
that broader statement; the explicit order19 diagonal relation audits a case
that really occurs on a prime-order curve. These have different purposes.

The hand bound is a constructible-degree statement, not a complexity statement.
For a depth-k quadratic tower it is T=2k+2 with 2k+1 lifted variables after
terminal substitution. The number of monomials up to T is binomial(4k+3,2k+1),
so a small degree alone does not provide a cheap dense linear-algebra algorithm.
This combinatorial count is a structural observation, not an empirical timing
or a claim that a particular last-fall solver necessarily materializes that
entire space. The new experiment measures the explicit evaluator instead.

## Performance-fixture applicability

The measured conjugate-square tower needs M to divide p−1. The chosen word
prime has p−1=29·2^57; NIST P224 has p−1=2^96(2^128−1). Both support the measured
power-of-two sizes. This does not show that the same fixture exists at those
sizes over every prime field. The general rational API accepts other maps,
but finding suitable maps and completely split fibers for another field is
an additional construction/search obligation, not measured here.

The auxiliary Velu map supplies a subset of the field, and that subset is
then lifted on the separately specified target curve. No isogeny between the
auxiliary curve and the odd-prime-order target curve is asserted. In particular,
a rational nontrivial two-torsion kernel on the target itself would contradict
odd prime target group order. The auxiliary/target distinction is essential
for interpreting the prime-order19 positive test and for avoiding a fictitious
DLP transfer claim.
