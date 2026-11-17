# Higher-degree image-compatible calculus: degree-two fiber gate

## Outcome

The natural degree-two closed-point construction is rejected before relation
collection.  For the cover

```text
E : y^2 + xy = x^3 + 1,
C : v^2 + u^2 v = u^7 + u,
phi(u,v) = (u^3, u v + 1),
```

a generic rational fiber over every requested odd extension has reduced
support degrees `[1,2]`.  The general Jacobian norm sends those components to
`P` and `[2]P`, respectively.  Their sum is therefore only

```text
[3]P = P + [2]P.
```

This relation has verified independent logarithmic rank zero.  If `P=[a]G`
is source-labelled, both component labels already have known coefficients
`a` and `2a`.  If `P` is the unknown target `Q`, the same group identity
reveals no discrete logarithm.  The frozen information gate fails, so the
collector, matrix, individual-log stage, and EKS timing run are not admitted.

This closes the rational-fiber degree-two design.  It does not rule out a
compact growing-degree or adaptive target-local router.

## Exact closed-point evidence

The implementation now exposes an arbitrary-class pushforward certificate.
It factors a Mumford support into monic irreducible closed points, reconstructs
the support, pushes each factor through the residue-field norm, sums the
component images, and compares that sum with the general Jacobian
pushforward.  For a point-class pullback, it additionally checks that every
degree-`d` component maps to `[d]P`.

The deterministic audit includes these fixtures:

| Fixture | Generic support degrees | Projected labels | Purpose |
| --- | --- | --- | --- |
| `n=9,13,19,23,83` | `[1,2]` | `P`, `[2]P` | requested odd-degree cases |
| `n=2` | `[1,1,1]` | `P`, `P`, `P` | positive split control |
| `n=83`, `P=(0,1)` | reduced degree `1`, ramification index `3` | `P` | multiplicity control |
| pinned order-`r` generator | `[1,2]` | distinct nonzero `P`, `[2]P` | target subgroup |

For odd `n`, `2^n` is congruent to `2 mod 3`.  Cubing is a bijection on the
multiplicative group, and the generic support `u^3+x` has one rational root
and one irreducible quadratic factor.  The even-degree control has rational
nontrivial cube roots of unity and splits into three linear factors.  The
audit derives the factorization from the implementation rather than assuming
this parity rule.

On the pinned subgroup,

```text
r = 2417851639230796216685689,
P = (0x477f77103dfad59850800, 0x2fa5e737d542c4e4fd5c3),
```

the native replay checks `[r]P=O`, that the degree-two norm is `[2]P`, and
that this label is neither `O` nor `P`.  Since `r` is odd prime, doubling is
injective on the subgroup.  There is no projected collision to exploit.

## Relation, matrix, and individual-log accounting

The accounting merges closed components that share an elliptic image before
counting factor-base columns or rank.

| Requested measure | Degree-two rational-fiber result |
| --- | ---: |
| distinct projected labels per generic odd fiber | 2 |
| projected collision rate | 0 |
| useful relation probability | 0 |
| verified independent rank per identity | 0 |
| admitted matrix rows / columns | 0 / 0 |
| admitted matrix memory | 0 bytes |
| individual-log success | false |

The zero matrix figures are an early-stop result, not a claim that a completed
collector would consume no resources.  Building that collector is forbidden
by the frozen gate because its only candidate relation is already a group-law
identity.

## Strict crossover gate

All exponents are measured in the actual target order `r`, not the ambient
Jacobian order.  Source rho costs `r^(1/2)`.  Ordinary ambient genus-three
index calculus retains the established approximately `q^(4/3)` relation
collection model.  At `q=2^83`, this is `2^110.666...`, about `70.17`
exponent bits above source rho.  It is not a fallback for the rejected fiber
design.

The separate fixed-bounded-branch counting gate gives `Omega(r)` blind
relation evaluations before linear algebra.  It also cannot beat rho.  Thus:

| Route | Information gate | Complete-cost gate | Finite rho crossover |
| --- | --- | --- | --- |
| rational-fiber degree `1+2` | fail | fail | no |
| projected ordinary ambient relations | not new | fail (`q^(4/3)`) | no |
| fixed bounded-degree branch/router | unresolved labels | fail (`Omega(r)`) | no |
| growing-degree/adaptive target-local router | unclassified | unclassified | unclassified |

No wall-clock or throughput claim is made, and no EKS capacity was spent on a
candidate that failed the exact information test.

## Formal and claim boundary

`ImageClosedDegreeTwo.degree_one_plus_degree_two` proves in an arbitrary
additive monoid that `3 • P = P + 2 • P`.  This formalizes the last group-law
step only.  The Rust certificate separately covers finite-field support
factorization, residue-field norms, reconstruction, and the concrete subgroup
fixture.

The conclusion is scoped: raw multiplicity upstairs cannot be counted as
projected rank, and the obvious degree-two rational-fiber design creates no
new relation.  A future positive result needs a named compact router whose
complete relation search, projected rank, sparse matrix, memory, blind
individual-log descent, and verification model predicts a finite crossover
against strong source rho.

## Sources

- P. Gaudry, [*Index calculus for abelian varieties of small dimension and
  the elliptic curve discrete logarithm problem*](https://doi.org/10.1016/j.jsc.2008.08.005),
  J. Symbolic Computation 44 (2009).  This supplies the general
  dimension-three `q^(4/3)` framework, not an image-compatible router.
- P. Gaudry, E. Thomé, N. Thériault, and C. Diem,
  [*A double large prime variation for small genus hyperelliptic index
  calculus*](https://eprint.iacr.org/2004/153).  This supplies the classical
  large-prime relation framework.
- C. Diem, [*Index calculus with double large prime variation for curves of
  small genus with cyclic class group*](https://arxiv.org/abs/math/0606607).
- [Prior destination-calculus screen](../n83_genus3_image_solver_20261031/DESTINATION_CALCULUS_SCREEN.md).
- [General norm/projector and Prym audit](../n83_jacobian_projector_20261031/RESULTS.md).
- `crypto-autoresearcher/ideas/artifacts/ECDLP-IDEA-002/p1546_projected_smoothness_counting_gate.md`,
  the independent fixed-bounded-branch control.

The direct IACR PDF endpoints returned HTTP 403 in this session.  Primary
texts were inspected through DOI/arXiv metadata and parsed repository/browser
copies; no inaccessible claim was silently promoted to evidence.
