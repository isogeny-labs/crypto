#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Volcano levels of End(E): prime-power points, isogeny walks, factoring

_curve_traits, branch `claude/curve-traits-power-points`, 2026-10-22. Source
of every number: `docs/curves/traits.json` on `main` (before) and on this
branch (after), tabulated by `tabulate.py` into `levels.json`._

== What ran

`curve_traits build` over the 2112 registry curves and 176 source curves,
with the methods this branch adds:

- *Prime-power point test* (`points.rs`, `scalar_on_power_by_points`):
  Frobenius is the scalar $lambda$ on $E[ell^j]$ only if
  $E[ell^j] subset.eq E(GF(q^m))$, $m = op("ord")(lambda mod ell^j)$; the
  test refutes or confirms each candidate $lambda$ with points of that group
  and Frobenius applied to their coordinates.
- *Quadratic twist for even $k$* in the depth-1 point test, after the
  untwisted test, so no earlier decision changes.
- *Isogeny walks* (`walk.rs`): cycles of horizontal $l$-isogenies,
  $l <= 19$, on $j$-invariants through $Phi_l$. A closed cycle of length $r$
  is a cyclic endomorphism of degree $l^r$ whose conductor (Cornacchia)
  the conductor of End(E) divides, and $r$ is exactly the order of a prime
  above $l$ in the class group of End(E) (computed by composing quadratic
  forms); a walk that does not close bounds that order from below. The
  orders between $ZZ[pi]$ and $cal(O)_K$ that survive decide the levels.
- *Class number 1*: $j(E) = j(cal(O)_K)$ puts $cal(O)_K$ in End(E).
- *Factor hints* for the discriminant cofactors: PARI, GMP-ECM and CADO-NFS
  (C99, C109), every factor proved prime by PARI's `isprime`.

== Results (counts from the two files)

#figure(
  image("open_levels.svg", width: 95%),
  caption: [Open volcano levels by kind, `main` against this branch (all 2288
  curves; counts, no sampling).],
)

#table(
  columns: 3,
  [], [main], [this branch],
  [End(E) established], [1989], [ESTABLISHED],
  [End(E) only bounded], [264], [BOUNDED],
  [$Delta$ not factored], [35], [UNKNOWN],
  [open levels], [312], [OPEN],
)

No level decided on `main` changes value or loses its decision on this
branch (checked level by level). `curve_traits check` reproduces the file.
Build cost on the 4-core container: BUILDCOST.

== What stays open, and why

OPENTEXT

== Scope

Every decision is a proof from exact arithmetic (point certificates,
closed or open walks with simple roots, class-group orders, Cornacchia,
Deuring for class number 1); none is statistical. Unfactored cofactors of
121 to 152 digits are beyond the NFS budget of this container.
