# Cyclotomic n=83 algebraic membership continuation

Frozen before new `cyclotomic-n83-anf` trials. Input root certificate:
`runs/cyclotomic-n83.json`, retaining its primitive167th root and modulus.

Construct Boolean ANF constraints for `x=z0+z1`, `z0^167=z1^167=1`, and
`x!=0`, the exact C2 subset predicate. Per root use eight83-bit field
vectors: z,z^2,z^4,z^32,z^128 and three product intermediates. Frobenius
image constraints are linear and the four products per root are quadratic.
An additional inverse witness `x*w=1` enforces distinct roots. There are
18 field vectors,1494Boolean variables and1494Boolean equations before
discarding redundant zero equations. Retain every ANF coefficient list,
maximum input degree, monomial counts and a certificate hash.

Check all ANF equations on the first32 recorded C2 witnesses (certificate
order), with assignments independently evaluated through the native wide
field backend. Flip one x bit as a negative control and require rejection
on each of the32. Also test32 repeated-root assignments x=0 and w=0; all
must reject, keeping the exceptional diagonal explicit. Do not tune the
wire layout or sample order using these results.

This supplies an executable multiword algebraic-membership continuation.
It does not run F4 on1494variables, prove a solving-degree bound, compute
subgroup columns or a useful relation rank, or complete the original n83
attack ladder. It is separate from the root-run n23 solver package.
