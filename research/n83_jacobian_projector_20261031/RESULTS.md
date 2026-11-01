# General Jacobian norm, image projector, and the n=83 Prym

## Outcome

The proposed norm/projector lead is real and implementable, but the Prym
branch is negative for the pinned n=83 target.

The arbitrary-class pushforward is now expressed as a genuine closed-point
norm.  It factors a reduced Mumford support polynomial of degree at most
three, maps linear support directly, evaluates irreducible quadratic or cubic
support in its residue field, and sums the full base-field Frobenius orbit.
The existing 486-class GF(8) group-presentation table remains independent:
table construction now refuses to complete unless every class agrees with the
new norm.

For `e=phi^*phi_*`, the implementation exposes a checked
`e^2=[3]e` certificate.  On declared r-torsion it also exposes
`p=[3^{-1} mod r]e`, checks `p^2=p`, and returns the decomposition

    D = p(D) + (1-p)(D),       phi_*((1-p)(D)) = O.

This supplies the missing image-membership and Prym/kernel mechanism.

The exact n=83 arithmetic then closes the structural question:

    P_Prym,83(X) = X^4 + q^2
                 = (X^2 - 2^42 X + q)(X^2 + 2^42 X + q).

The two elliptic isogeny factors have traces `+2^42` and `-2^42`.
Neither trace equals the source trace `-6151469093347`, and

    gcd(r, q+1-2^42) = 1,
    gcd(r, q+1+2^42) = 1,
    gcd(r, q^2+1)    = 1.

So the Prym shares neither the target prime `r` nor an elliptic isogeny
factor with `E).  There is no hidden r-primary route through the
dimension-two complement.

## Exact derivation

The curve and map descend to GF(2):

    E : y^2 + xy = x^3 + 1,
    C : v^2 + u^2v = u^7 + u.

Direct enumeration in the native audit gives the same point counts for E and
C over the first three extensions:

| Field | #E | #C |
| --- | ---: | ---: |
| GF(2) | 4 | 4 |
| GF(4) | 8 | 8 |
| GF(8) | 4 | 4 |

Thus their Frobenius power sums agree for degrees 1, 2, and 3.  The elliptic
factor is `X^2+X+2`.  Applying Newton identities and the genus-two
functional equation to the remaining four roots gives

    P_Prym,1(X)=X^4+4.

For the degree-83 base change, every Prym eigenvalue alpha is replaced by
`alpha^83`.  Since `alpha^4=-4` and 83 is odd, the resulting four roots
satisfy `beta^4=-2^166=-q^2`; hence `P_Prym,83=X^4+q^2`.

With `q=2^83` and `a=2^42`, `a^2=2q`, and the factorization follows.
The replayable polynomial record is
`IDC1h0ec3d37bcb6795db`; the same expansion is proved by
`Projector.prym_factorization`.

The exact orders are:

| Object | Order |
| --- | ---: |
| trace-`+2^42` factor | 9671406556912635351138305 |
| trace-`-2^42` factor | 9671406556921431444160513 |
| Prym | 93536104789177786765035829293842113257979682750465 |
| target prime `r` | 2417851639230796216685689 |

All three gcd checks against `r` are one.

## What is certified and what is inferred

Exact/native obligations:

- arbitrary split and nonsplit support is handled by one norm path;
- all 486 GF(8) classes are cross-checked against the independent table;
- all 236,196 ordered GF(8) pairs remain the homomorphism test;
- typed pull/push, endomorphism, idempotence, image, kernel, and decomposition
  conditions are explicit return fields;
- the pinned n=83 image generator is pushed by the general norm, not the old
  specialized point formula;
- point counts and integer gcds are exact.

Mathematical inference:

- the kernel identity component is the Prym in the usual cover-Jacobian
  decomposition;
- distinct elliptic Frobenius traces over the same finite field reject an
  isogeny factor shared with the source;
- coprimality of `r` and the Prym order rejects r-primary torsion in the
  complement.

The provider shell disconnected during the first release-test link.  A
non-test `cargo check --lib` had completed before that outage; the interrupted
release run is not counted as a pass.  The branch CI/native replay is the
remaining execution gate.

## The fiber-structured degree-one candidate also collapses

For every odd extension degree `m`, `gcd(3,2^m-1)=1`.  Cubing is therefore
a bijection on `GF(2^m)^*`.  On rational affine cover points the map

    (u,v) |-> (u^3, uv+1)

is injective: equality of images first gives `u=u'`; for nonzero `u` the
second coordinate then gives `v=v'`, while the cover equation has the unique
`u=0` point `v=0`.

Consequently, at all requested toy sizes `n=9,13,19,23` (and at `n=83`)
a projected degree-one factor base has exactly the same distinct cardinality
as its source-point factor base and collision rate zero.  The degree-three
geometric fibers do not create rational fiber collisions at these odd
extensions.  This is an exact formula-level rejection of the simplest
fiber-collision design, not a measured relation-yield result.

An image-closed base made from `phi^*(E(k))` is indexed by elliptic points
and merely transports the source relation space.  A genuinely different
candidate would have to use higher-degree closed points and still pass the
complete relation/rank/matrix/individual-log cost gate.

## Algorithmic consequence

This is not a speedup.  It makes the relevant negative statement sharper.

Because the Prym order is coprime to `r`, an image-closed r-torsion factor
base cannot harvest an independent r-primary component from the Prym.
Ordinary ambient genus-3 relation collection still inherits the prior
approximately `q^(4/3)` collection model, already screened as about 70.17
exponent bits worse than source rho for this case.

The only surviving experiment is narrowly scoped: construct factors in the
transported elliptic image, or project relations with `p`, and measure
distinct projected base size, collision rate, relation probability,
independent rank, matrix dimensions, memory, and individual-log completion.
It must use the actual order `r`.  Stop immediately if the complete model
cannot beat `r^(1/2)` or predict a finite crossover against strong source
rho.

The broad recommendation to “add genus-3 index calculus” should therefore
continue to exclude the standard full-Jacobian algorithm.

## Literature and repository evidence

- D. Lombardo, E. Lorenzo García, C. Ritzenthaler, and J. Sijsling,
  [*Decomposing Jacobians via Galois covers*](https://arxiv.org/abs/2003.07774),
  especially the cover-induced Jacobian/Prym decomposition framework.
- J. Tate,
  [*Endomorphisms of abelian varieties over finite fields*](https://doi.org/10.1007/BF01404549),
  Invent. Math. 2 (1966), 134–144; used for the finite-field isogeny
  criterion.
- [Prior destination-calculus screen](../n83_genus3_image_solver_20261031/DESTINATION_CALCULUS_SCREEN.md),
  which records the full-Jacobian cost rejection.
- [Previous cover decomposition](../ghs_cover_decomposition_20261025/COVER_DECOMPOSITION.md).
- [Previously recorded arbitrary-norm gap](../../problems/binary-cover-arbitrary-jacobian-norm-missing.md).

The IACR-hosted Gaudry and Joux–Vitse PDFs returned HTTP 403 during this
session; no new theorem was inferred from inaccessible text.  Their role in
the prior destination screen remains as recorded there.
