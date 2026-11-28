# Order-three automorphism: many short rows, no log solve

As a positive control for structured relation collection, replace the
selected `a=−3` curve by the `j=0` comparison curve

`E_CM/F_p: y²=x³+40`, `p=2^61−1`.

The verified order-4,950 root subgroup contains
`ζ=z^1650=1669582390241348315`, a nontrivial cube root of unity.
The automorphism `ψ(x,y)=(ζx,y)` preserves the curve and the root
x-subgroup. Every lift-valid orbit

`(x,y), (ζx,y), (ζ²x,y)`

lies on a horizontal line and therefore sums to the identity. The
[dependency-free native scan](native/examples/cm_orbits.rs) finds exactly
**2,433 lift-valid columns in 811 disjoint orbits**. Its 811 three-term
rows have exact matrix rank 811 over any coefficient field because their
supports are disjoint, leaving 1,622 formal unknowns. To reach rank
`B−1` from these rows alone would require 1,621 additional independent
rows. The count and rank are direct combinatorics, not a heuristic.

The [native Hasse-interval search](native/examples/quotient_order.rs) gives

`#E_CM(F_p)=2305843008054377925`
`          =3·5²·7³·1753·51131959441`.

The largest factor `q=51131959441` was independently checked prime. On
the `q`-subgroup, the native executable verifies that `ψ` acts as the
known scalar `λ=39764776730`, with `λ²+λ+1=0 (mod q)`, on **all 811
nonzero projected orbit representatives**. The two scalar-endomorphism
rows per orbit reduce three column logs to one; the horizontal-line row
is a linear combination of these known identities. No relation between
different orbit representatives or target descent follows from them.

This finite comparison curve is also a weak ECDLP benchmark for the
requested exponent: generic square-root search on its largest prime
subgroup has scale `√q≈226,123` group steps, already below
`p^(1/3)≈1.32 million`. The abundant automorphism rows therefore cannot
be presented as a cube-root index-calculus breakthrough. Extra
automorphisms and their scalar action are standard CM/GLV structure; the
contribution here is a reproducible rank control that distinguishes
relation count from useful independent information on the chosen base.
The [selected-trace obstruction](cm_isogeny_obstruction.md) further
shows that no `j=0` or `j=1728` curve lies in the selected curve's
`F_p`-isogeny class, so an isogeny chain cannot import these extra
automorphisms into the original ECDLP instance.

Reproduce:

```sh
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example cm_orbits
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example quotient_order
```
