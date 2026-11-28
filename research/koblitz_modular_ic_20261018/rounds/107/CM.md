# R107: normalized x=1 branch data of the degree-112 source

R107 substitutes R106's two exact base series into all 29 terms of the
actual R51 monic lambda equation, through `s^32`. Let `alpha` satisfy
`alpha^2+alpha+1=0` over F4 and put `u=l+alpha`. The resulting
coefficient orders in F4[[s]] are all **strictly below 32**:

| Base place | u^0 | u^1 | u^4 | u^8 | u^16 |
| --- | ---: | ---: | ---: | ---: | ---: |
| y=0, v_s(t)=3 | 28 | 23 | 23 | 0 | 0 |
| y=1, v_s(t)=4 | 27 | 23 | 23 | 0 | 0 |

The leading F4 scalar of u^0 is `alpha` in both rows; those of the
other displayed coefficients are 1. The x=1 fiber reconstructs
`u^16+u^8`. Thus all points relevant to the Newton lower hull are
known exactly at the frozen precision. The lower edges near `u=0` are

    y=0: (0,28)--(1,23), slope -5; (1,23)--(8,0), slope -23/7;
    y=1: (0,27)--(1,23), slope -4; (1,23)--(8,0), slope -23/7.

The point `(4,23)` lies strictly above the second edge in each case.
The horizontal edge `(8,0)--(16,0)` accounts for roots near the other
F4 root `alpha+1`; it is not a positive-valuation edge at `alpha`.
The [Newton polygon theorem](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf)
gives one degree-1 factor and one degree-7 factor near each F4 root;
the latter is irreducible because `gcd(23,7)=1`. Frobenius conjugates
the factors near `alpha` and `alpha+1`. Over F2((s)), each base place
therefore has two normalized closed places, of local degrees 2 and 14,
both with residue F4 and lambda ramification indices 1 and 7.

R105/R106 give the base ramification indices 3 and 4 over `t=x+1`.
Hence the **degree-112 source field** has four normalized closed places
above x=1, all of residue degree 2, with t-ramification indices
`3,21,4,28`. The local-degree sum is
`2*(3+21)+2*(4+28)=48+64=112`, agreeing with R101's source-order
fiber lengths while now resolving their branch decomposition. This is
a field-completion/normalization result; an explicit integral basis and
the N51 valuations remain separate work.

The committed producer is SHA-bound to the R51 donor, R105 parser and
R106 series. The accepted run exited zero, a fresh process reproduced
`result.json` byte-for-byte, and an altered base hash was rejected.
An independent generalized-Hasse-jet composition recomputed every
lambda coefficient, the F4 translation, all orders and leading scalars.
Lean 4.34.1 checked the exact hull inequalities, coprimality, and
degree/ramification arithmetic; the Newton/Hensel factorization theorem
is the cited mathematical step, not formalized in this Lean file.

Raw producer/replay SHA-256:
`193db048167c2fd0e349a1af992095b008e8ea3425e9ee4243120d0c38f90e70`.
Result JSON SHA-256:
`485c48de9128cf7d7939d5c73cfb3c7dd738592b93144cab45f9a1171c8b031d`.
Next construct the degree-1 F4 branch series and a uniformizer for the
degree-7 factors, then evaluate the actual N51 coordinate at all four
places. The other finite supports and infinity follow.

Sources: R51 donor, R63 degree-112 field bridge, R101 F4 residue,
R105/R106 base completions, [Newton theorem](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf),
and [Stacks Henselian factorization](https://stacks.math.columbia.edu/tag/04GE).
