# A research lead: a cheap isogeny step before Tian's cubic descent

**Current results:** [all four research tracks](FOUR_TRACK_RESULTS.md), with
[derivations and scope](FOUR_TRACK_METHOD.md). A certified p=157 example forces
two degree-three steps in total from two nonprepared sources. A p=109 CM audit
shows different coverage at the same endomorphism order. The transfer now
handles obstruction fallback and more exceptional divisors, and completes
actual target logarithms with line and double-large-prime solvers. Complete
attacks remain slower than source Pollard rho. One extra step after preparation,
universal exceptional-chart handling, and novelty remain open.

The [previous polynomial-inversion results](NEXT_RESULTS.md) are historical
stages, with independently retained outputs and limitations.

The earlier [276-quotient construction results](QUOTIENT_RESULTS.md) and
[their derivation](QUOTIENT_METHOD.md) are retained as historical stages.

The sections below document the earlier eligibility stage and its motivation;
their then-outstanding implementation milestones are superseded by the current
results linked above.

The promising next project is to turn Tian's supplied-kernel construction into
an automatic, measured pipeline for prime-order cubic-extension curves, with
explicit accounting for quotients that fail to produce a usable Jacobian.

## The proposed pipeline

For an ordinary curve E/F_(p^3), p >= 5, with large prime order r:

1. Check p = 1 mod 3 and r = 4 mod 9.
2. Factor the third division polynomial over F_(p^3).
3. If it has four rational roots, the required scalar action on E[3] is already
   present. If it has only one, take its unique rational 3-isogeny E -> E'.
4. On E', construct the candidate rational (3,3,3)-kernels of its cubic Weil
   restriction. The independent finite computation below finds 12 satisfying
   the coordinate-transversality condition.
5. Reconstruct the principally polarized quotients and test which are usable
   genus-three Jacobians over F_p. This stage is implemented and independently
   certified on the 23 newly eligible small-field inputs; see the current results.
6. Transfer the DLP and benchmark a real genus-three index-calculus solver,
   including preprocessing, construction, relation generation, and linear algebra.

A degree-three isogeny preserves an r-order subgroup when r != 3:
phi(Q) = m phi(P) for Q = mP. It therefore supplies a valid preliminary reduction.

The screen applies specifically to the auxiliary prime 3 and the stated
transverse-kernel condition. Failing it does not rule out auxiliary primes
5,7,..., different polarization constructions, or other ECDLP attacks.

## Why the cheap arithmetic screen works

Let K=F_(p^3), Q=p^3, and T be the quadratic twist of E over K. Then

    #T(K) = 2(Q+1)-r.

The favorable torsion condition is that Q-Frobenius acts as -I on E[3]. This is
equivalent to T having full rational 3-torsion. Necessarily 9 divides #T(K), and
the Weil pairing forces p = 1 mod 3. Since p^3 = 1 mod 9 in this case, the order
condition simplifies to r = 4 mod 9.

Conversely, suppose these congruences hold. If T already has full rational
3-torsion, stop. Otherwise its 3-primary rational group is cyclic. Divisibility
of #T(K) by 9 supplies a rational point P9 of order nine. Quotient T by its unique
rational subgroup of order three, using phi_T:T->T'. The point phi_T(P9) has
order three. The kernel of the dual isogeny is another rational order-three
subgroup because mu_3 is contained in K. They are distinct, since

    dual(phi_T)(phi_T(P9)) = 3 P9 != 0.

Thus T' has full rational 3-torsion. Twisting the isogeny back gives a rational
3-isogeny E->E' on whose 3-torsion Frobenius acts as -I. The group order remains r.

This elementary argument justifies the preprocessing step; it does not certify
that the subsequent polarized quotient is a Jacobian over the desired field.
It uses standard isogeny and torsion facts and is not asserted to be a new theorem.

## Exact torsion-module calculation

Write A for Q-Frobenius on E[3]. After compatible identifications of the three
conjugate 2-dimensional torsion spaces, base-field Frobenius on the Weil
restriction can be modeled by

    F(v0,v1,v2) = (A v2, v0, v1).

Its alternating pairing has blocks J, pJ, p^2J, where
J = [[0,1],[-1,0]]. The code checks F is a p-symplectic similitude.

There are 33,880 three-dimensional subspaces of F_3^6 and 1,120 Lagrangians.
`kernel_screen.py` enumerates all of them for all eight relevant conjugacy-class
models of A. Coordinate transversality means that a candidate intersects none
of the three coordinate 2-planes nontrivially.

| p mod 3 | trace(A) mod 3 | A type | Stable Lagrangians | Transverse | Compatible with r prime > 3? |
|---|---|---|---:|---:|---|
| 1 | 0 | nonscalar | 0 | 0 | yes |
| 1 | 1 | nonscalar Jordan | 1 | 0 | yes |
| 1 | 1 | scalar -I | 16 | 12 | yes |
| 1 | 2 | nonscalar Jordan | 1 | 0 | no |
| 1 | 2 | scalar I | 16 | 12 | no |
| 2 | 0 | distinct eigenvalues | 4 | 2 | no |
| 2 | 1 | irreducible | 0 | 0 | yes |
| 2 | 2 | irreducible | 0 | 0 | yes |

For prime-order input curves, the favorable model is therefore precisely the
scalar -I case. A characteristic polynomial alone does not distinguish this
case from a nonscalar Jordan block: the actual torsion module matters.

For the nonscalar Jordan case, F has one length-six Jordan block, and its unique
stable three-dimensional subspace is a coordinate-product kernel. For scalar
-I, there are two length-three blocks and 12 transverse candidates. All 12
matrix bases are exported in `results/kernel_classification.json`.

**Transverse is a necessary screen here, not a sufficient Jacobian certificate.**
One must still handle polarized decomposability, the finite-field Jacobian
descent/Serre obstruction, and hyperelliptic versus non-hyperelliptic targets.

## What was actually tested

With SageMath 10.6 and seed 20261025:

- 282 distinct (j-invariant, prime order) pairs over cubic fields were sampled
  at p=5,7,11,13,17,19,23,29,31,37,41,43. Curves descending to F_p and
  supersingular curves were excluded.
- There were 24 accepted curves per characteristic except p=5, where the
  20,000-draw cap produced 18. These exploratory counts are not a population
  density estimate or a complete enumeration. Frobenius conjugates and curves
  in the same isogeny class can both occur.
- 12 curves already had the required scalar 3-torsion action.
- 23 more reached it through one 3-isogeny, for 35 candidate curves in total.
- The 282 curves represent 203 sampled K-isogeny classes. Direct candidates
  occurred in 10 of those classes; preprocessing reached candidates in 20.
- Every sampled result matched the arithmetic screen. The order, nonzero
  subgroup image, and scalar relation were checked on each newly reached case.
- The original eligibility probe reconstructed no new quotient. The continuation
  constructs all 12 quotients on each of the 23 newly reached inputs. General
  target-image inversion and an end-to-end attack benchmark remain unfinished.

| p = 1 mod 3 | Accepted curves | Direct candidates | After at most one step |
|---|---:|---:|---:|
| 7 | 24 | 0 | 3 |
| 13 | 24 | 1 | 4 |
| 19 | 24 | 2 | 7 |
| 31 | 24 | 1 | 6 |
| 37 | 24 | 5 | 9 |
| 43 | 24 | 3 | 6 |

Example: over F_(7^3) with z^3+6z^2+4=0,

    E: y^2 = x^3 + (4z^2+3z+3)x + (4z^2+5z+2)

has prime order 373. Its twist order is 315, divisible by nine. Its third
division polynomial has factor degrees [1,3]. The unique rational 3-isogeny
leads to

    E': y^2 = x^3 + (3z^2+6z+4)x + (4z^2+5z+2),

whose third division polynomial splits into four linear factors. Both source
and image recover the same sampled scalar, 218. The continuation now constructs
12 quartics for this example; 11 give unobstructed Jacobians, all of prime order
373. The new quotient point map is also evaluated modulo sign.

## A composed Jacobian reduction was also checked

Starting from Tian's published curve E0 over F_(523^3), the experiment follows
two 3-isogenies E0->E1->E2, choosing E2 with nonscalar 3-torsion action. The new
input is

    E2: y^2 = x^3 + (241w^2+483w+333)x + (182w^2+31w+408),
    w^3+2=0.

It has the same prime group order r=143069629, but fails the direct transverse
3-kernel screen. One dual step reaches a scalar-action neighbor; a second
dual step reaches the already constructed published quotient.

The chain of dual isogenies has degree nine. On points obtained by the forward
chain, it multiplies the published inputs by nine. Evaluating all six published
rational functions verifies that the final Jacobian images are exactly 9S and
9T, for the published S,T, and still satisfy 9T=1987(9S). The image subgroup
order is unchanged. The scalar 1987 is recovered on the actual F_523 Jacobian
within the explicitly stated interval [0,4095].

This establishes correctness of one composed reduction on a different input
curve. It reuses the published quotient, and is not a general new quotient
constructor. Its bounded target BSGS is not comparable to the full-order source
BSGS and establishes no speedup.

## Prior work and novelty boundary

- [Galbraith–Hess–Smart, Extending the GHS Weil Descent Attack (2002)](https://eprint.iacr.org/2001/054)
  already uses isogenies to move to a curve admitting an effective descent.
  Isogeny preprocessing itself is not new.
- [Tian, Cover Attacks for Elliptic Curves over Cubic Extension Fields (2023)](https://doi.org/10.1007/s00145-023-09474-2)
  supplies the relevant (ell,ell,ell)-isogeny/Jacobian attack framework. The
  accessible 2020 preprint assumes an appropriate rational isotropic kernel.
  The full journal text is subscription-only in the access checked here; this
  limits any claim about what the final version does not contain.
- [Sutherland, Isogeny volcanoes (2013)](https://arxiv.org/abs/1208.5370)
  covers the classical graph/endomorphism-ring structure underlying this kind
  of preprocessing.
- [Hashimoto–Nuida, Ordinary 3-Isogeny Graphs ... (September 2026)](https://arxiv.org/abs/2609.11037)
  is recent adjacent work on explicit 3-isogenies, twisted Hessian curves and
  volcano structure. It is another reason not to present the torsion/isogeny
  observations alone as a priority claim.

Targeted searches found these overlaps but do not establish the absence of
earlier work on the exact combination. **No novelty or priority certification
is claimed.** The potentially new project is a complete classification and
practical construction/coverage study for the resulting prime-order Tian
quotients, with this small candidate set used to bound the search.

## Concrete next milestone and rejection criteria

The generic quotient construction and obstruction checks are now implemented
for the 12 transverse kernels on all 23 newly eligible curves. The next
implementation milestone is general inversion of quotient Weil coordinates
to target divisor classes, consistent sign recovery, and an actual target
index-calculus benchmark including all preprocessing costs.

This lead becomes substantially stronger if a useful proportion produce
non-hyperelliptic genus-three Jacobians at negligible cost relative to the
target DLP. It weakens if the quotients are mostly decomposable, require an
unhelpful twist, or cannot be constructed cheaply. Existing target solver costs
must be measured end to end before claiming a security reduction in practice.

A subsequent extension is q=p^2: the same cubic-module argument then concerns
prime-order curves over F_(p^6) and Jacobians over F_(p^2). Combining these new
quotients with decomposition methods would require its own analysis; no
improved exponent for that hybrid is asserted here.

## Reproduce

From the suite root, using SageMath 10.6:

```sh
python3 research_lead/kernel_screen.py
sage -python research_lead/isogeny_probe.py --samples 24 --seed 20261025
sage -python research_lead/composed_fixture.py
```

The first script is pure Python. The other two use the existing `common.py`,
`tian.py`, and `tian_data.json`. They perform local computations without network.
Raw records are in `research_lead/results/`.
