#set page(paper: "a4", margin: 1.65cm, numbering: "1")
#set text(font: "Libertinus Serif", size: 9.2pt)
#set heading(numbering: "1.")
#set table(stroke: 0.4pt, inset: 5pt)

= Projector-only adaptive image routing for the n=83 cover

_2026-11-18 · Exact kernel-coset audit and formula gate_

The general Jacobian norm and normalized image projector work as designed,
but the projector alone is not a new relation router.  For

$ E: y^2 + x y = x^3 + 1, quad
  C: v^2 + u^2 v = u^7 + u, quad
  phi(u,v) = (u^3, u v + 1), $

write $p=[3^(-1) mod r] phi^* phi_*$.  If $K$ lies in the pushforward
kernel, then

$ p(D + K) = p(D). $

The Lean proof is axiom-free and bound to claim `PCC-20261118-81b046`
`[pcc:PCC-20261118-81b046]`.  Projection keeps the elliptic norm label and erases
the class-specific Prym/kernel residue.

#figure(
  image("figures/router-gate.svg", width: 100%),
  caption: [The projector is a correct image map, but every projector-only relation route either returns the source relation module, retains ambient collection cost, or loses the kernel-cancellation condition.],
)

== Exhaustive finite control

The native audit reconstructs all 486 classes of the existing `GF(8)`,
`a=1`, `b=3` fixture.  Its general norm has six labels with 81 preimages
apiece.  Every one of the $486^2$ ordered pairs is then tested both before and
after projection.

#table(
  columns: (2.8fr, 1fr),
  [Exact population], [Count],
  [Ambient Jacobian classes], [486],
  [Elliptic norm labels], [6],
  [Kernel / norm-fiber size], [81],
  [Ordered pairs], [236,196],
  [Homomorphism pairs / failures], [236,196 / 0],
  [Projected-zero pairs], [39,366],
  [Exact Jacobian-zero pairs], [486],
  [False projected cancellations], [38,880],
  [Exact lift fraction, conditional on projected zero], [$1/81$],
)

For a surjective finite-group homomorphism $N:J arrow E$ with kernel $K$,
the projected-zero ordered-pair population is the product of the group and
kernel orders, while exactly one ordered inverse pair per class sums to zero
in $J$.  The fixture gives
$486 dot 81=39,366$ and $486$ respectively.  A nonzero kernel class paired
with the identity is the explicit negative control.

These deterministic counts are recorded in `certificate.json`, SHA-256
`cf4643eb...ab49b45`.  They are an implementation and ambiguity control, not
an estimate of n=83 relation probability, and have no independent replay
(`[pcc:unverified]`).

== n=83 image replay

For

$ r = 2417851639230796216685689, quad
  3^(-1) mod r = 1611901092820530811123793, $

the certificate checks $[r]P=O$, the general norm identity
$phi_* phi^*(P)=[3]P$, normalized-projector idempotence, and that the
projector fixes the pinned pullback with zero complementary component.

Because $gcd(3,r)=1$, $[3]$ is injective on order-$r$ torsion.  Thus
$phi^*$ is injective there: image-closed relations correspond exactly to the
relations among the selected source points.  Pullback and projection do not
create an additional independent relation coordinate.

== Router classification

#table(
  columns: (1.7fr, 3.8fr),
  [Route], [Gate result],
  [Project first], [The factor base is in the transported image; its relation module is the selected source relation module.],
  [Project after exact ambient collection], [The projected relation is valid, but finding the exact ambient relation is not made cheaper by the projector.],
  [Accept only the projected relation], [It certifies only that the ambient sum is in the kernel.  A Prym/kernel cancellation witness is missing.],
  [Grow reduced support degree], [Reduced genus-three Mumford support is capped at degree three.  Genuine growth needs non-reduced effective divisors, Riemann--Roch data, or another representation.],
)

The candidate therefore stops at the formula/information gate.  Relation
probability at n=83 is not estimated; matrix dimensions, memory, and blind
individual logarithms are not admitted; no EKS timing run is justified.

== Remaining constructive lead

The result is narrow.  It does not reject every higher-degree or adaptive
image-compatible calculus.  A broader design remains open only if it adds a
mechanism not contained in the projector itself:

- a concrete non-reduced or Riemann--Roch divisor representation;
- source labels obtained without already solving their logarithms, or a
  controlled method for cancelling Prym residues;
- useful-relation probability and independent rank after label merging;
- matrix, memory, blind individual-log, and verification costs in the actual
  order $r$;
- a finite crossover against the strong source rho control.

The next experiment should instantiate that oracle on `n=9,13,19,23` and
close its cost model before n=83 timing.  “Project more ambient relations” is
not such an oracle.

== Sources and scope

- Diem, _Index calculus with double large prime variation for curves of small
  genus with cyclic class group_,
  #link("https://arxiv.org/abs/math/0606607")[arXiv:math/0606607].
- Lombardo, Lorenzo García, Ritzenthaler, Sijsling, _Decomposing Jacobians via
  Galois covers_,
  #link("https://arxiv.org/abs/2003.07774")[arXiv:2003.07774].
- Repository studies `n83_jacobian_projector_20261031`,
  `n83_image_closed_degree2_20261101`, and
  `n83_genus3_image_solver_20261031`.

The research-KB and the relevant vault-backed lead records were unavailable
as readable text.  The accessible primary papers and committed repository
evidence are the documented fallback.  This report makes no wall-clock,
throughput, attack, security-level, or performance-advantage claim.
