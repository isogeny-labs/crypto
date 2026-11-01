#set page(paper: "a4", margin: 1.7cm, numbering: "1")
#set text(font: "Libertinus Serif", size: 9.2pt)
#set heading(numbering: "1.")
#set table(stroke: 0.4pt, inset: 5pt)

= Degree-two image-closed fiber calculus for the n=83 cover

_2026-11-01 · Exact closed-point audit and strict source-rho gate_

The first natural higher-degree image-compatible design is rejected before
relation collection.  For the cubic cover

$ E: y^2 + x y = x^3 + 1, quad
  C: v^2 + u^2 v = u^7 + u, quad
  phi(u,v) = (u^3, u v + 1), $

every requested odd-degree generic rational fiber has one degree-one and one
degree-two closed component.  The general Jacobian norm maps them to $P$ and
$[2]P$.  Their projected relation is therefore only
$[3]P = P + [2]P$, a scalar-multiplication identity with independent
logarithmic rank zero.

#figure(
  image("figures/degree-two-gate.svg", width: 100%),
  caption: [The exact component norm fails the information gate, so the collector and EKS timing stage are stopped.],
)

== Exact evidence

#table(
  columns: (1.3fr, 1fr, 1.4fr, 2.2fr),
  [Fixture], [Degrees], [Projected labels], [Control or conclusion],
  [`n=9,13,19,23,83`], [`[1,2]`], [$P, [2]P$], [Requested odd-degree cases],
  [`n=2`], [`[1,1,1]`], [$P,P,P$], [Positive split control],
  [`n=83`, `(0,1)`], [reduced 1], [$P$], [Ramification index 3 retained separately],
  [Pinned order-$r$ point], [`[1,2]`], [distinct $P,[2]P$], [Target subgroup check],
)

The native certificate factors each reduced Mumford support, reconstructs it,
pushes every irreducible factor using the residue-field norm, sums the
component images, and compares the result with the general arbitrary-class
pushforward.  For each point-class pullback it checks that a degree-$d$
component maps to $[d]P$.

For the pinned generator

$ r = 2417851639230796216685689, $

the replay checks $[r]P=O$, the quadratic component maps to $[2]P$, and this
label is neither $O$ nor $P$.  Because $r$ is odd prime, doubling is injective
on the subgroup.  The apparent extra closed point creates no projected
collision or compression.

The even-degree split case catches an implementation that always reports a
`1+2` factorization.  The exceptional point catches confusion between reduced
support degree and scheme-theoretic ramification multiplicity.

== Information and complete-cost gates

#table(
  columns: (2.5fr, 1fr),
  [Measure after projected-label merging], [Result],
  [Distinct labels per generic odd fiber], [2],
  [Projected collision rate], [0],
  [Useful relation probability], [0],
  [Independent rank per identity], [0],
  [Admitted matrix rows / columns], [0 / 0],
  [Admitted matrix memory], [0 bytes],
  [Individual-log success], [false],
)

If $P=[a]G$ is source-labelled, the two component labels already have scalars
$a$ and $2a$.  If $P$ is the unknown target $Q$, the identity
$[3]Q=Q+[2]Q$ reveals no logarithm.  This is why the zero matrix entries mean
“early stop,” not a zero-cost completed attack.

All asymptotic accounting uses the actual order $r$.  Source rho costs
$r^(1/2)$.  Projecting ordinary ambient genus-three relations retains the
approximately $q^(4/3)$ collection scale: at $q=2^83$ its exponent is
$110.666...$ bits, about $70.17$ bits above source rho.  The independent
fixed-bounded-branch gate is $Omega(r)$ before linear algebra.  Neither route
predicts a finite crossover, and neither is admitted as a fallback.

#pagebreak()
== Scope and remaining obligation

The rejection is deliberately narrow.  It closes the degree-two component of
a rational fiber and prevents raw upstairs multiplicity from being counted as
projected rank.  It does not prove that every higher-degree image-compatible
calculus is impossible.

The remaining unclassified object is a compact growing-degree or adaptive
target-local router.  Before implementation it must name and cost:

- the target-local relation search and its useful probability;
- distinct projected columns and verified independent rank;
- sparse matrix dimensions, memory, and solve work;
- blind individual-log descent and final verification;
- a finite crossover against the strong source rho implementation.

The Lean theorem `degree_one_plus_degree_two` proves the final group identity
in any additive monoid.  It does not formalize the finite-field factorization
or Rust norm path; those are covered by the native certificate and controls.

== Sources

- Gaudry, _Index calculus for abelian varieties of small dimension and the
  elliptic curve discrete logarithm problem_, JSC 44 (2009),
  #link("https://doi.org/10.1016/j.jsc.2008.08.005")[DOI].
- Gaudry, Thomé, Thériault, Diem, _A double large prime variation for small
  genus hyperelliptic index calculus_,
  #link("https://eprint.iacr.org/2004/153")[ePrint 2004/153].
- Diem, _Index calculus with double large prime variation for curves of small
  genus with cyclic class group_,
  #link("https://arxiv.org/abs/math/0606607")[arXiv:math/0606607].
- Repository screens `n83_genus3_image_solver_20261031`,
  `n83_jacobian_projector_20261031`, and the P1546 projected-smoothness gate.

This report makes no wall-clock, throughput, attack, security, or speedup
claim.  The frozen gate rejected the candidate before any EKS timing run.
