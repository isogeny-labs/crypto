# H116 generator-certificate protocol amendment v2

Amendment `H116-GENERATOR-CHARPOLY-002` is filed before generator search. It
changes only how exact orders are computed. Since the full group order is
already obtained by exhaustive enumeration, the implementation may start with
that order and divide out a prime factor whenever the corresponding smaller
scalar kills the point. [pcc:unverified]

The selected generator orders are then confirmed independently by repeated
addition through the first return to infinity. This produces the same exact
point orders and therefore the same frozen maximum-order and canonical-first
selection as literal repeated addition on every point. [pcc:unverified]

Cells, point ordering, selection, equations, mutations, dropped-generator
control, measurements, acceptance and no-replacement policy are unchanged.
No generator count or coordinate was computed before this amendment.
[pcc:unverified]
