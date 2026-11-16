# Degree-eight rational and two quadratic branch poles

The source frozen at `e77997882dd9` completed all four carrier/source
representative cells for `B=LQ_iQ_j` and `B=L³Q_iQ_j`. Every cell represented
3,858,261,660 logical numerator triples using exact bitsets, performed
10,800 direct bitset controls, and checked every point-filter survivor by
exact polynomial division and squarehood.

| carrier | source | point survivors | exact identities | CPU s |
|---:|---:|---:|---:|---:|
| 32280 | 68 | 6,223 | 0 | 64.084 |
| 32280 | 84 | 6,384 | 0 | 64.148 |
| 35349 | 68 | 6,345 | 0 | 63.975 |
| 35349 | 84 | 6,262 | 0 | 64.157 |

The four source-bound receipts are named
`degree8_two_poles_f<carrier>_e<source>.json` in this directory.
All have 10 pole pairs, 20 denominator shapes, 540 branch assignments,
and zero exact square-polynomial identities. The checked coordinate and
Frobenius transformations extend this shape-restricted result to the 120
recorded carrier/source equation pairs. Other degree-eight denominators
remain open, including nonbranch square factors and three quadratic branch
poles. This is a finite exclusion, not a new vulnerable family.
[pcc:unverified]
