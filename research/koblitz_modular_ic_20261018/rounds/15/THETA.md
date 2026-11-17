# Actual theta-class descent and the 3lambda target

G14 supplies four principally polarized quotients q:B→A over F2, with
B an ordinary trace-zero4fold times a supersingular elliptic companion,
and q of degree19². The actual ordinary Hermitian matrix H and Cartier
character action C are retained in G15. The companion is not the ordinary
elliptic curve of the other Koblitz sign.

## A symmetric split representative

Over F32, let Theta0=O_E(0), T=(0,1) the unique nonzero geometric2-torsion
point and w²+w+2=0, with w=Frob on E0 and w=-Frob on E1. For alpha=a+bw,

 alpha^*Theta0 = Norm(alpha)*Theta0 + P_(ab*T)

as a divisor class. To see this, sum the geometric kernel with its
inseparable multiplicity. If a is even the inseparable multiplicity is
even, so its2-torsion sum vanishes. If a is odd, alpha is separable and
Norm(alpha)≡a(a+b) mod2. Its kernel has a unique nontrivial2-primary
cyclic component exactly when b is odd; its sum is T in that case and0
otherwise. This proves the formula for every a,b, not just the finite
controls. G15 exhausts all30 coefficient pairs in[-5,5] with norm a power
of2≤16 on each curve over F16, verifies kernel cardinality including
inseparability, and checks the weighted sum.

Represent H by diagonal Hii*Theta0 plus, for each i<j, the cross bundle
(pr_i+[H_ij]pr_j)^*Theta0-pr_i^*Theta0-([H_ij]pr_j)^*Theta0.
It is symmetric. The product with19*O_S(0) on the supersingular companion
is a symmetric split representative L of q^*lambda. G15 derives its
Frobenius bias by restriction to the four axes using the formula above.
This is a Pic0 calculation; it does not construct rational functions
representing the isomorphisms.

## Actual source descent, not dual-coordinate descent

The q5 coefficient rotation is R(a0,a1,a2,a3)=(-sum a,a0,a1,a2).
G15 checks R^5=I, R^dagger H R=H and its integral8x8 Frobenius binding.
The dual Kummer coordinate rotation is different and is not substituted.
On geometric2-torsion of Pic0 the action is P=R^transpose modulo2.
The actual bias bits are[7,2,7,2]; the unique correction bits are
[15,9,15,9], where bit i is the coefficient of T on axis i. All16 possible
drifts are solved uniquely. Lean checks P*c+d=c and the two-sided
inverse (I-P)^-1=P+P³ for each actual graph.

Twisting L by the displayed2-torsion class produces a symmetric
Frobenius-invariant representative. All its factors and endomorphisms
are defined over F32. Choose an isomorphism for the F32/F2 generator.
Its5-fold cocycle is a scalar invariant under the generator, thus in
F2^×={1}; consequently it is coherent and the bundle descends to B/F2.
The supersingular companion contributes no geometric2-torsion drift.
This argument descends a line bundle, not only a polarization morphism.

## Descent through the 19-kernel

[Moonen, Theta groups8.10–8.12](https://www.math.ru.nl/~bmoonen/BookAV/ThetaGr.pdf)
identifies descent with a theta-group lift. Isotropy gives geometric
lifts, but rational lifts require an additional argument. The lifts form
a torsor under Gamma^D, with geometric points (Z/19)². Frobenius acts
affinely x↦C*x+b on this torsor. The actual C matrices are
[[17,18],[2,0]] and [[2,18],[2,0]]; (I-C)^-1 is respectively
[[4,15],[8,12]] and [[1,18],[2,18]]. G15 exhausts every361 drift and361
character choice per graph; Lean checks both matrix inverse identities.
Thus every possible affine drift has exactly one rational lift. The
universal additive fixed-point lemma is formalized in Lean.

The symmetric involution preserves the unique rational lift, so the
resulting ample principal bundle Theta on A/F2 is symmetric. Equivalently,
its difference from [-1]^*Theta lies in Gamma^D(F2)=0. Amplitude and the
principal type follow from the frozen polarized quotient and q^*Theta=L.
These scheme/theta-group arguments are written; finite certificates
alone are not claimed to formalize them.

## Consequence and open implementation

Ample Riemann–Roch gives h0(3Theta)=3^5=243 and
h0(q^*(3Theta))=19²*243=87723. The third power of an ample line bundle
on an abelian variety is very ample, giving A→P242 of degree5!*3^5=29160.
See [Genestier–Ngo2.2.1–2.2.3](https://www.math.uchicago.edu/~ngo/Shimura.pdf).
Relative to G14's canonical6lambda construction the section count is
smaller by exactly32. This is a section-count ratio, not a runtime ratio.

This specifies the actual corrected bundle class and finite descent
obstructions. Constructing bases of sections, theta-group action matrices,
F2 invariant sections, a saturated homogeneous ideal and its identification
with the existing affine quotient remain open. The target P242 is not an
explicit embedding yet. Full h-saturation is not implied by this result.
