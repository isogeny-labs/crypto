# Auxiliary two-isogeny chain: software qualification

2026-11-25. Implement the published auxiliary-curve construction separately
from the pinned Dickson driver. No scientific solver panel or timing run is
part of this component; the corrected-output plan remains unchanged.

## Sources and scope

Sources checked (KB): status and two queries for supersingular smooth-p+1
two-isogeny chains and coset/pole conditions return wrapper exit127; the exact
receipt is retained with the qualification evidence. The fallback review is
Petit–Kosters–Messeng, [PKC2016, Algorithm1 and Section3.4/Lemma1](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
the existing study's torus construction, and the official
[Sage isogeny reference](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_curve_isogeny.html).
The literature already supplies the auxiliary-isogeny factor-base mechanism.
This component provides a small native implementation, not a novelty claim.

Use only the public fixture primes7,31,127,8191. All are3 modulo4 and p+1 is a
power of2. On E': y²=x³+x, pairing x with -x cancels the quadratic-character
sum, giving p+1 rational points. Only (0,0) is a nonidentity rational point
of order2; the finite abelian2-group is therefore cyclic. A proposed generator
must pass [p+1]G=O and [(p+1)/2]G≠O. A deterministic generator search, if
used by a later experiment, is explicit setup work and reports its search
counts. Tests use the tiny7/31 fixtures and fixed known points; they do not
run an auxiliary-base selection panel at127 or8191.

## Representation and algebra

Represent y²=x³+A₂x²+A₄x+A₆ over an allowed prime, with validated points and
an explicit point at infinity. Reject singular curves. For a rational
order-two point (r,0), write z=x-r, a=3r+A₂ and b=3r²+2A₂r+A₄. Then

\[
 u=z+a+b/z,\qquad v=y(1-b/z^2),\qquad
 E_{next}:v^2=u^3-2au^2+(a^2-4b)u.
\]

Send O and (r,0) to O. Nonsingularity implies b≠0 and a²-4b≠0.
The identity (u-a)²-4b=(z-b/z)² gives the target equation. These are
standard degree-two isogeny coordinates after translation; no checked Lean
formalization is claimed. The point at infinity and exceptional denominators
are part of the API, not discarded affine cases.

Starting with full-order G, at step i use the order-two point obtained from
the current image of G and its known order. Map G forward and halve that
order. Do not assume the next kernel point still has x=0. Permit depth1..6
only, and require the image of G to retain order at least4. The accumulated
kernel is H=<[(p+1)/2^depth]G>.

Maintain a composed x-map N/D, initially X/1. For the next step put Z=N-rD;
replace N by Z²+aZD+bD² and D by ZD. These degrees are2^depth and2^depth-1.
At a zero of the new denominator, either D=0 and N≠0, or Z=0 and bD²≠0;
thus the recurrence introduces no common numerator/denominator root.

For a rational anchor alpha, reject image O or a point of order2. Let chi
be the image x-coordinate. Return both the chain and the monic polynomial
N-chi D, together with the x-coordinates of alpha+H. No two points of this
coset share x: otherwise they are negatives and their image is2-torsion.
The degree2^depth polynomial therefore has precisely those distinct roots.
Denominator zeros cannot enter this finite-image fiber. This is the standard
coset argument in the cited paper, with explicit pole handling.

Target-curve lifting is separate: retain only roots with nonzero square
right-hand side on the chosen target curve, use one canonical y sign, and
record the resulting base. Do not assume every auxiliary root lifts or that
the target base size equals the isogeny degree.

## Software controls and accounting

Create a dependency-free Rust library in auxiliary-isogeny/, leaving every
qualified Dickson source/input byte unchanged. Use bounded native unit tests
for known7/31 point orders, addition, the degree-two formula and poles, full
group-homomorphism controls on F7, changing kernel x along a chain, equality
of chained/dense evaluation, coset product equality, target lifting and
invalid prime/curve/generator/depth/anchor inputs. Include a denominator-pole
negative control and a corrupted dense coefficient control. These are
software regressions, not solver-performance observations.

Run the unit suite and library release build once, preserving any failed
output before a correction. Do not rerun unrelated Dickson suites. Publish
source, commands, logs, build identity, scoped unsigned evidence, diagram,
inspected report PDF and reproduction instructions. A later experiment must
charge generator search, chain construction, dense composition, coset
enumeration, target lifting and any serialization or reference setup. This
library is not yet connected to RR5 relation collection or linear algebra.
