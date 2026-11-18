# Exact Smith branch gate results [pcc:unverified]

Freeze9949caac151;input hashes pass;sole bounded invocation exits0. All6 original maps pass exact polynomial identities disc(P)=delta4*I². All discriminants have degree32;all square factors I have degree14 and are squarefree. Each I contains the cubic norm root r exactly once, leaving degree11 residual. In raw output this factor is named square_index; it is a square factor, NOT a computed integral-basis index or a ramification divisor. The paired-branch contribution itself has even discriminant valuation.

All6 critical polynomials delta4 are squarefree of degree4 and disjoint from the squarefree cubic r. All D are squarefree of degree2, gcd(F,D) has degree1, and gcd(N,D)=1.

For the two F61 maps:

| Map | Native discriminant comparisons | Irreducible quartic specialization |
|---|---:|---:|
|0|61|t=4|
|1|59|t=11|

All120 comparisons pass. The comparison omits parameters with vanishing quartic leading coefficient, not just zeros of the discriminant. Irreducibility uses degree4 finite-field Frobenius/gcd checks; original sources are retained.

CONNECTEDNESS.md combines these witnesses with previously checked smooth rational points to deduce geometric connectedness, then the generic local sign-permutation argument gives tame ramificationtotal12 and genus3 of the normalized cover for both F61 maps. This is a deduction from the explicit hypotheses and earlier generic chart identification; it has not been independently reviewed or formally verified. It does not compute a smooth model or certify nonhyperellipticity.

Canonical adjoints/model construction and complete source/target divisor transport remain open. The four nonsquare isogeny obstructions are retained. No speedup or complete10x result is claimed. Git custody, noS3polling/no cloud/no merge.
