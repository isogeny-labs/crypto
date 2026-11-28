#set page(paper: "a4", margin: 20mm)
#set text(size: 10pt)
= R86: affine lambda on the diamond overlap

The frozen source-coordinate check identifies `b` and `c` on the inverse
quadratic chart. The diamond parameter `k=b+(1+c)c` satisfies
`z k=(1+z)c²` modulo the source equation. The resulting update is

`lambda1 = [lambda+(1+z)²g]/(z²g)`, with `g=1+z+z²+V`.

#image("affine86.svg", width: 100%)

Singular checked two original-coordinate identities, the `k` reduction,
and the complete lambda cross-product modulo the quadratic equation.
Both deliberately corrupted controls produced nonzero remainders. The
accepted run used the committed R86 source, one thread, and its four
hash-pinned donor files. Lean separately proves the characteristic-two
`k` simplification and its cleared lambda cross-product.

The next calculation substitutes the update into the degree-16 relative
equation, then transports all 32 R50 RR basis generators with pole bounds
and full module lifts in both directions.
