# Ordinary endomorphism conductors in the retained crossing

The exact prime is ell=33555419 (integer bit length26). The independently
recomputed field/order/trace values give
`t²−4p = −19 ell²` and `gcd(t,p)=1`; both endpoints are ordinary. Thus their
geometric endomorphism orders lie between the Frobenius order of conductor ell
and the maximal order of fundamental discriminant−19. Since ell is prime, the
only possible endomorphism conductors are1 and ell. [pcc:unverified]

PARI independently computes the class polynomial `H_{−19}(X)=X+884736` and
class number1. The source j is its root, so that ordinary source has maximal
endomorphism order. The actual returned quotient equals the retained target
equation, whose j is not that root. It therefore cannot have maximal order;
its conductor is ell. This class-polynomial/order-sandwich argument identifies
the actual endpoint conductors, rather than identifying either conductor with
the Frobenius conductor by assumption. [pcc:unverified]

The complete native evaluator checks a nonzero exact prime-order auxiliary
kernel and covers every half-kernel representative before emitting the quotient
and point images. The quadratic-twist conjugation is defined over Fp even
though the isomorphisms use Fp². Its x/y formula follows by conjugating the
normalized map `(f(x),y f'(x))` through `(x,y) -> (d x,d sqrt(d)y)`.
Small square/nonsquare controls use direct PARI kernel-polynomial maps; large
point images and normalized dual compositions are separate implementation
checks. This document is a mathematical derivation with executable arithmetic
certificates, not a compiled formal proof. [pcc:unverified]

Primary ordinary-order references previously reviewed for this same family:
[Galbraith, introduction and section2](https://antsmath.org/ANTSXVI/papers/Galbraith.pdf)
and [Galbraith–Gilchrist–Robert, section2.1](https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/articles/climbing_volcanoes.pdf).
The retained constructor is `conductor_gap_cm_orbits_20261015/native/CONSTRUCTOR.md`.
The exact primary-source excerpts were reviewed in the previous fixture-order
audit; KB retrieval is unavailable here. No novelty or record claim is made.
