# Rational degree-eight escape: conditional obstruction

Status: mathematical derivation, not a measured speedup or a novelty claim.
No experiment or target selection is performed here. Application to the two
retained genus-three models still requires an explicit irreducibility check.

## Hypotheses and statement

Let C/F_q be y^2=x^7+c, c nonzero, characteristic different from 2 and 7,
q=1 mod 7, and assume x^7+c irreducible. Write J for its canonically
principally polarized Jacobian. There are exactly two F_q-stable subgroups
of J[2] of geometric dimension three over F_2. Both are maximal isotropic,
and both principally polarized quotients are isomorphic over F_q to J.
Consequently successive rational polarized (2,2,2) steps cannot escape
this principally polarized isomorphism class.

## Derivation

Fix a primitive seventh root z in F_q and let s(x,y)=(zx,y). The quotient
C/<s> is rational, with coordinate y. The norm endomorphism is therefore
1+s+...+s^6=0 on J. Define A=s+s^2+s^4 and B=s^3+s^5+s^6.
Integer multiplication in Z[T]/(T^7-1) gives

    (T+T^2+T^4)(T^3+T^5+T^6)=3+T+T^2+...+T^6.

Thus AB=[2] and A+B=[-1]. An automorphism of the curve preserves the
canonical polarization, so its Rosati adjoint is its inverse. Hence A^dag=B,
A^dag A=[2]. Degrees of adjoints agree, so deg A=deg B=8. Since the
characteristic is odd these are separable, their kernels lie in J[2],
and each kernel has dimension three. The polarization identity makes
them maximal isotropic; the induced principal polarization on J/ker A
identifies with the original one on J, and likewise for B. Everything
is defined over F_q.

It remains to exclude additional rational kernels. The seven finite
Weierstrass points form a Frobenius seven-cycle, while infinity is fixed.
Their differences with infinity identify J[2] with the permutation module
F_2^7 modulo its constant vector. Its Frobenius polynomial is

    Phi_7(T)=(T^3+T+1)(T^3+T^2+1) over F_2.

Both factors are irreducible (a cubic without a root in F_2) and distinct.
The module is a direct sum of two nonisomorphic simple three-dimensional
modules. Every Frobenius-stable subspace is a sum of these summands;
there are exactly two of dimension three. They are precisely ker A and
ker B: modulo 2, A(T)=T(T^3+T+1), and B has the reciprocal kernel.
This exhausts all rational maximal isotropic kernels, not merely kernels
represented by partitions into pairs of branch points.

## Literature and boundary

Smith, *Isogenies and the Discrete Logarithm Problem in Jacobians of Genus 3
Hyperelliptic Curves*, sections 3 and 9, explains the faster non-hyperelliptic
transport route and distinguishes tractable kernels from general isotropic
kernels. Section 3, Lemma 1 gives zero tractable kernels for factorization
pattern (7,1), consistent with the elementary fact that a seven-cycle plus
one fixed point preserves no partition into pairs.
https://arxiv.org/html/0806.2995

For polarization, Rosati, degrees and quotient background: Milne,
*Abelian Varieties*, parts I.7, I.13-I.14 and III.
https://www.jmilne.org/math/CourseNotes/AV.pdf

The self-isogeny argument above is a deduction using standard polarization
facts, not a theorem attributed to Smith or Milne. It is not independently
reviewed or formally verified. It excludes neither other degrees nor
extension-defined kernels, other models, or other principal polarizations.
It yields no genus-two or genus-four speed conclusion.

## Next falsifiable gate

Before implementing transport, freeze a Rust qualification on the retained
(p,c)=(113,3),(421,2) models: verify (-c)^((p-1)/7)!=1, the polynomial
identities, and enumerate all Frobenius-invariant dimension-three subspaces
of the six-dimensional 2-torsion module. Require exactly the two stated
kernels. If either model fails the irreducibility premise, retain that
outcome and analyze its actual branch permutation instead of substituting
another curve. Only after this gate consider a different-degree transport
with explicit mapping, target preservation and complete cost case.
