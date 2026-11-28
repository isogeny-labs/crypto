# Frozen protocol: higher-degree image-compatible Riemann--Roch screen

Frozen on 2026-11-19 before implementation, model execution, or relation-yield
measurement.  This is a cost-model gate for a materially higher-degree family;
it is not an n=83 DLP run and it makes no speedup claim.

## Question and advancement rule

Determine whether complete higher-degree linear systems on the certified
genus-three cover can generate projected relations in the transported
elliptic image with complete expected work below `r^(1/2)`, or at least with a
finite predicted crossover against strong source rho.

The screen advances to implementation or measurement only if one registered
route has all of the following:

1. a typed effective-divisor construction and reconstruction certificate;
2. projected labels in the actual prime-order group, not ambient Jacobian
   labels;
3. a named non-generic mechanism whose relation probability and independent
   rank are not obtained by treating those labels as random group elements;
4. complete setup, collection, large-prime recombination, matrix, memory and
   individual-log costs with exponent strictly below `1/2` in `r`, or a
   concrete finite crossover against the strongest source-rho control.

If every route is at rho parity or worse, stop analytically.  Do not implement
or measure n=9,13,19,23 or n=83 relation collection merely to reconfirm an
exponent failure.

## Curve, transfer and target group

Use the already certified cover

```text
C: v^2 + u^2 v = u^7 + u,
phi(u,v) = (u^3, u v + 1),
E: y^2 + x y = x^3 + 1
```

over `GF(2^n)`.  At n=83 the target prime is

```text
r = 2417851639230796216685689,
log2(r) = 81.00000000000092.
```

The general norm and projector are prior certified infrastructure.  The Prym
order is coprime to `r`, so every matrix label in this protocol is the
cofactor-projected elliptic norm

```text
lambda(Q) = [4] phi_*(Q) in E[r]
```

of a source closed point `Q`.  Equal labels are merged and identity labels are
removed.  A factorization `D=sum Q_i` of an effective representative of a
class `X` must satisfy

```text
sum lambda(Q_i) = [4] phi_*(X).
```

## Complete higher-degree family

Let `X` be a degree-zero class.  For every `d>=5`, effective representatives
`D` with `[D-d*infinity]=X` form a complete linear system of projective
dimension `d-3`: since the genus is three and `d>2g-2`, Riemann--Roch gives
`ell(D)=d-2`.  Thus the family contains
`(q^(d-2)-1)/(q-1)`, asymptotically `q^(d-3)`, representatives per class.
This screens the complete linear system rather than only one polynomial
subfamily.

The first new degree has an explicit affine chart.  For a generic reduced
Mumford class `X=(u0,v0)` with `deg(u0)=3`, let `k=k1*u+k0` and set

```text
V = h + v0 + k*u0,
U = (f + h*V + V^2)/u0,
h=u^2, f=u^7+u.
```

Exact divisibility follows from the Mumford condition.  In characteristic
two,

```text
U = (f+h*v0+v0^2)/u0 + h*k + k^2*u0.
```

For `k1!=0`, `deg(U)=5`; unit-normalizing `U` and reducing `(U,V mod U)`
returns `X`.  The two coefficients of `k` give the `q^2` affine chart expected
from the dimension-two degree-five linear system.  Constant `k` is the prior
degree-four family and remains only as a control.  For `d>5`, the screen uses
the complete Riemann--Roch dimension even where this one-step polynomial
chart is not complete.

## Registered projected-label models

Write `a` for the number of distinct nonidentity projected labels after
factoring, norming and merging a representative.  Closed-point degrees sum to
`d`, but matrix arity is `a`, not `d`.  The screen covers every
`1<=a<=d` and grants the candidate all of these favorable assumptions:

- factorization, norms, projector evaluation and membership tests cost one
  abstract candidate operation;
- the complete linear system supplies as many independent candidates as the
  model requests;
- nonidentity projected labels are uniform and independent in the `r`-group;
- every admitted row is independently useful until matrix rank is full.

These are optimistic screening assumptions, not claims about the actual
image distribution.

### Small-base route

For a projected factor base of `B` labels, a relation of arity `a` is contained
with probability `(B/r)^a`.  Collecting `B` rows costs

```text
T_rel = r^a / B^(a-1),     T_LA >= B^2.
```

Balancing gives

```text
B = r^(a/(a+1)),           T_total >= r^(2a/(a+1)).
```

The full-split degree-five case has `a=5` and exponent `5/3`.

### Double-large-prime route

For `a>=2`, grant relations with two unrestricted projected labels and
`a-2` small-base labels.  The accepted-edge probability is `(B/r)^(a-2)`.
The classical random-label graph route requires order `r` accepted edges to
obtain a linear supply of cycles/tree eliminations, hence

```text
T_rel = r^(a-1) / B^(a-2).
```

Balancing with `B^2` gives

```text
B = r^((a-1)/a),           T_total >= r^(2-2/a).
```

For five rational components this is `r^(8/5)`.  The most favorable
nontrivial closed-point pattern has `a=2`, for which the all-large-prime graph
still needs order `r` edges.

### All-large-prime and arity-one controls

For `a>=2`, also grant an all-large-prime random hypergraph.  Before a linear
2-core/cycle supply appears, its edge and memory scale is order `r`; record
exponent `1` as an intentionally optimistic control rather than assuming that
more large primes preserve the double-large-prime formula.

For `a=1`, the equation says that the single norm label is the known right
hand side.  It supplies no independent matrix row.  Matching such labels from
known and target-dependent classes is exactly a generic table/collision
search: `B+r/B`, minimized at `r^(1/2)`.  It is a BSGS/rho-parity control, not
an index-calculus improvement.

Shoup's generic-group lower bound applies only to routes that expose no
additional structure beyond projected group labels.  Therefore the screen
does not use it to claim an impossibility result for this elliptic curve.  It
uses it as a strict obligation: any survivor must name and cost the
non-generic algebraic property of the norm-label encodings that invalidates
the random/generic controls.  Merely increasing `d`, the Riemann--Roch
dimension, or the number of closed-point preimages is not such a property.

## Closed-point multiplicity check

Degree-`j` closed points have order `q^j/j` possible supports but only `r`
projected labels.  Under the registered uniform model, a `B`-label base has
about `B*q^(j-1)/j` preimages, yet the membership fraction stays `B/r`.
Preimage multiplicity therefore changes sampling constants and collision
counts, not the registered exponents.  A targeted norm-preimage algorithm
that beats enumeration would be a materially different mechanism and is
left as an explicit open obligation, not silently assumed.

## Exact outputs and stop checks

The executable model must use exact rational exponents and emit:

- the Riemann--Roch dimension and candidate-count exponent for each
  `d=5..31`;
- the explicit inverse-Cantor `(degree, parameter dimension)` ladder;
- small-base, double-large-prime and all-large-prime exponents for every
  `a=1..d`, with inapplicable cases marked rather than coerced;
- n=83 leading exponent bits and gaps against `0.5*log2(r)` for the
  degree-five full-split route, the most favorable `a=2` graph route, and the
  `a=1` generic control;
- a survivor only for an exponent strictly below `1/2` with a named
  non-generic mechanism.  Equality does not advance.

No elapsed-time, throughput, relation-yield or rank measurement is authorized
if the survivor set is empty.

## Controls and claim boundary

- Prior degree-four exhaustive n=9 result: checks the random-label model at
  one toy scale; it is not extrapolated as a proof.
- Ordinary full-Jacobian genus-three calculus: retained as the screened
  `q^(4/3)` ambient control, not relabelled as image-compatible.
- Source rho and image BSGS/rho: actual-order controls at exponent `1/2`.
- The complete-linear-system count, closed-point free representation and
  large-prime method come from the cited literature; projected-label
  uniformity and random-hypergraph behavior remain registered assumptions.

The result may reject this higher-degree random-label Riemann--Roch family. It
must not be phrased as a proof that no image-compatible calculus, targeted
norm-preimage construction, or elliptic-encoding attack can exist.

## Sources checked

- C. Diem, [*Index calculus with double large prime variation for curves of
  small genus with cyclic class group*](https://arxiv.org/abs/math/0606607),
  especially the full-class-group sampling, factor-base, large-prime tree and
  `q^(2-2/g)` analysis.
- C. Diem, [*Index Calculus in Class Groups of Plane Curves of Small
  Degree*](https://eprint.iacr.org/2005/119.pdf), for free closed-point
  divisors, bounded-degree Riemann--Roch computation and plane-model relation
  generation.
- V. Shoup, [*Lower Bounds for Discrete Logarithms and Related
  Problems*](https://www.shoup.net/papers/dlbounds1.pdf), for the scoped
  generic-group `Omega(sqrt(r))` control.
- `research/n83_norm_fiber_rr_router_20261118/RESULTS.md`,
  `research/n83_jacobian_projector_20261031/RESULTS.md`,
  `research/n83_image_closed_degree2_20261101/RESULTS.md`, and
  `research/n83_genus3_image_solver_20261031/DESTINATION_CALCULUS_SCREEN.md`.

The shared research KB was unavailable: no KB MCP tools are attached and the
repository wrapper `scripts/research-kb` is absent.  The search fallback was
the repository record plus the primary sources above.  The ePrint text pages
for 2004/073 and 2004/153 returned an anti-bot page, so their theorem text was
not imported anew.
