# R123: complete normalized branch census above x=0

The pinned R116 base series have four F2-rational x=0 places with x-orders `1,3,1,2`. R119–R122 give, near each of the two conjugate F4 lambda residues, an exact translated Newton polygon whose positive edges have horizontal lengths 1 and 7. In every case the leading coefficients at powers 1 and 8 are 1. The seven-edge initial relation is `W+W^8=W(W^7+1)=0`; its nonzero root `W=1` is simple in characteristic 2. The length-one edge is linear with nonzero leading constant. Hensel lifting over the appropriate series field gives one degree-one factor and one degree-seven factor over F4((s)): the slope denominator 7 forces the latter's ramification degree to be 7, and its total degree is 7. The two F4 residue roots are Frobenius-conjugate, so these factors pair into two places over F2((s)), each of residue degree 2, with relative ramification 1 and 7.

| F2 base place | base x-order | normalized x-orders | residue degrees | x-divisor degree |
|---|---:|---|---|---:|
| y0/e1 | 1 | 1, 7 | 2, 2 | 16 |
| y0/e3 | 3 | 3, 21 | 2, 2 | 48 |
| y1/e1 | 1 | 1, 7 | 2, 2 | 16 |
| y1/e2 | 2 | 2, 14 | 2, 2 | 32 |

There are eight normalized places over F2 above x=0, and the divisor degree of x above zero is `16+48+16+32=112`, matching the degree-112 field. The final nonpositive-correction edges in each translated polynomial account for the other F4 residue cluster; the two clusters exhaust the degree-16 lambda fiber over each base place. This is a complete local branch count, with N51 valuations still to be evaluated on these places.

The source/protocol were committed before data. Every input is SHA-pinned; the source checks both conjugate roots, exact cutoffs, edge lengths, slopes, leading coefficients, residue relation, and degree total. Independent verification recomputes the factors and divisor degree from the pinned rows. Changed-input rejection, byte-identical replay, Lean 4.34.1 finite degree arithmetic, and a visually inspected PDF passed. Lean proves the finite `2*(1+7)` and degree-112 arithmetic; the Hensel and Newton-polygon theorem applications are stated above rather than formalized in Lean. The first changed-input control used undedicated `/tmp` numeric directories; it was replaced by an isolated `TemporaryDirectory` control, which passed. No accepted-run producer failure or cloud spend.

Result SHA-256: `1435f4fbc2d1b04d628b5adc326916ed1f3a1406c24271348aaffe857e0b3fe3`. Replay: `python3 research/koblitz_modular_ic_20261018/rounds/123/census123.py <output-directory>`. Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/123/verify123.py`.

Sources: R116 and R119–R122 result files; [Stacks Henselian local rings](https://stacks.math.columbia.edu/tag/04GE) and [Stoll's Newton polygon notes](https://www.mathe2.uni-bayreuth.de/stoll/teaching/pAdicAnalysis-WS2015/Skript-pAdicAnalysis-pub-print.pdf).
