# Higher-degree image-compatible Riemann--Roch screen

## Outcome

The complete higher-degree random-label family has no route through the
registered strict gate.  The result is analytic: the protocol required a
complete exponent below `r^(1/2)` before relation measurement, and the exact
screen found no survivor.  No n=9,13,19,23 or n=83 relation-yield/timing run
was performed. [pcc:unverified]

The first new complete linear system is degree five.  Its most direct
fully-split small-base calculus costs `r^(5/3)`; granting two large primes
improves this to `r^(8/5)`.  Allowing nonsplit support and merging all source
components down to only two distinct projected labels still leaves the
optimistic all-large-prime graph at order `r`.  A single projected component
has no independent relation rank and reduces to a generic lookup/collision
search at `r^(1/2)`, equal to the baseline rather than below it.
[pcc:unverified]

At the actual target order, with `log2(r)=81.00000000000092`, the exact model
gives:

| degree-five route | exponent in `r` | leading bits | gap from source rho |
|---|---:|---:|---:|
| five labels, small base | `5/3` | 135.0 | +94.5 |
| five labels, two large primes | `8/5` | 129.6 | +89.1 |
| two labels, optimistic all-large graph | `1` | 81.0 | +40.5 |
| one label, generic lookup | `1/2` | 40.5 | 0.0 |
| source rho | `1/2` | 40.5 | baseline |

The ordinary ambient genus-three control remains `q^(4/3)=2^110.667`, about
70.167 leading bits above source rho.  It is not used as a fallback.
[pcc:unverified]

## Typed construction

```text
X in E[r]
  | phi^*                         degree-three cover, phi_*phi^*=[3]
  v
phi^*(X) in J(C)[r]
  | complete |D|, deg(D)=d>=5    projective dimension d-3
  v
D = sum Q_i                      Q_i are source closed points
  | [4]phi_*(Q_i), merge labels  Prym has no r-primary component
  v
sum lambda(Q_i) = [12]X in E[r]
```

The correspondence and arbitrary closed-point norm are prior certified.  The
new object is the complete degree-`d` effective-representative family for a
fixed class.  Because the genus is three and `d>2g-2`, Riemann--Roch gives

```text
ell(D)=d-2,
dim |D|=d-3,
#|D|(F_q)=(q^(d-2)-1)/(q-1)=Theta(q^(d-3)).
```

For the first new degree, a generic reduced class `(u0,v0)` with
`deg(u0)=3` has an explicit affine chart.  With `k=k1*u+k0`,

```text
V = h+v0+k*u0,
U = (f+h*V+V^2)/u0
  = (f+h*v0+v0^2)/u0 + h*k + k^2*u0.
```

When `k1!=0`, `U` has degree five.  Its two parameters match the complete
degree-five system's projective dimension.  The executable screen also records
the higher one-step ladder `deg(U)=2 deg(k)+3`; from degree seven onward that
polynomial ladder is a proper subfamily, so the cost screen deliberately uses
the larger complete Riemann--Roch family. [pcc:unverified]

## Why the additional parameters do not move the exponent

Let `a` be the number of distinct nonidentity labels left after closed-point
norm, cofactor projection, identity deletion and collision merging.  This is
the matrix arity; the effective degree `d` is not substituted for it.

For a projected base of `B` labels, the registered favorable random-label
model grants containment probability `(B/r)^a`, unit-cost factorization and
norms, unlimited candidate supply, and independent useful rows.  Even with
those grants, collecting `B` rows and sparse linear algebra give

```text
T_rel = r^a/B^(a-1),
T_LA  >= B^2,
B     = r^(a/(a+1)),
T     >= r^(2a/(a+1)).
```

The double-large-prime control grants two unrestricted labels.  Requiring
order `r` accepted graph edges and balancing against `B^2` gives

```text
T_rel = r^(a-1)/B^(a-2),
B     = r^((a-1)/a),
T     >= r^(2-2/a).
```

For completeness, the protocol also grants an all-large-prime random
hypergraph at order `r`; this is more favorable than extrapolating the
two-large-prime formula to arbitrarily many large primes.  It still does not
reach `r^(1/2)`.  At arity one, the sole label equals the right-hand side, so
it creates no matrix equation; matching target-dependent labels against a
known-log table is the usual `B+r/B` collision tradeoff, minimized at
`sqrt(r)`. [pcc:unverified]

Higher-degree closed points have many preimages of each elliptic norm label,
but this does not enlarge the projected matrix.  Under the registered uniform
model, degree-`j` supports give about `B*q^(j-1)/j` preimages of a `B`-label
base while the membership fraction remains `B/r`.  Preimage multiplicity can
change constants and observed collisions; it does not change this family's
exponent without a targeted sublinear norm-preimage algorithm.
[pcc:unverified]

## Executable evidence

- `SCREEN.json`, SHA-256
  `76264656ece3f9bd57f617e32cd5262948d992ccb68f6b31444632a0b1772940`,
  contains exact rational formulas for every degree 5 through 31 and every
  merged arity 1 through `d`, plus the inverse-Cantor ladder and n=83 bit
  conversions.
- `VALIDATION.json`, SHA-256
  `cc139efbffef10c5198923ad3bd9983bf34273c2d3e662927c30c63b6ece0d4d`,
  reports seven passing replay groups: registration, target/rho, complete
  Riemann--Roch grid, inverse-Cantor ladder, n=83 highlights, ambient control
  and advancement decision.
- `sha256sum -c REGISTRATION.sha256` passes for the protocol and requirements.

The replay was written by the producer in this session.  It is a separate
formula implementation, not an independent PCC verifier, and all headline
claims remain `[pcc:unverified]`.

## Scope and next obligation

This rejects complete bounded-degree and growing-degree Riemann--Roch families
whose projected norm labels have only the registered random/generic behavior.
It covers fully split support, nonsplit closed-point support, collision merging,
two large primes, an optimistic all-large-prime control and arity one.  It is
not a universal impossibility theorem for image-compatible calculus or for
the concrete elliptic encoding.

Another run is justified only by a formula-level mechanism that invalidates
the membership count.  The narrow open obligation is a sublinear targeted
norm-preimage construction, or another explicit non-random law on projected
labels, together with its construction, rank, matrix, memory and individual-
log costs.  Absent that mechanism, measuring a larger Riemann--Roch system
would only estimate constants for an exponent already rejected by the frozen
gate.

## Sources

- C. Diem, [*Index calculus with double large prime variation for curves of
  small genus with cyclic class group*](https://arxiv.org/abs/math/0606607).
- C. Diem, [*Index Calculus in Class Groups of Plane Curves of Small
  Degree*](https://eprint.iacr.org/2005/119.pdf).
- V. Shoup, [*Lower Bounds for Discrete Logarithms and Related
  Problems*](https://www.shoup.net/papers/dlbounds1.pdf); used only for the
  generic-label control, not a concrete-group impossibility claim.
- `research/n83_norm_fiber_rr_router_20261118/RESULTS.md` and the prior norm,
  projector, degree-two and destination-calculus screens cited by the frozen
  protocol.

The research KB was unavailable in this runtime, and the 2004/073 and
2004/153 ePrint text endpoints returned anti-bot pages.  No novelty or
literature-exhaustion claim is made from those gaps.
