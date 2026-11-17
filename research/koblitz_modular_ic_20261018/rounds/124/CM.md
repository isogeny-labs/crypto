# R124: exact first N51 leading-form cancellation at x=0

All 105 rational terms of the pinned R51 N51 equation were evaluated on the four R116 order-640 base series and combined by lambda power before taking coefficient orders. Every numerator and denominator is visible: the largest denominator s-orders are 54, 162, 54 and 108 on y0/e1, y0/e3, y1/e1 and y1/e2. No source-precision lower bound competes with the first weighted minimum.

| base place | first lambda pole order k | candidate N51 s-order | tied lambda powers | leading sum at both F4 roots |
|---|---:|---:|---|---:|
| y0/e1 | 10 | -53 | S | 0 |
| y0/e3 | 7 | -44 | S | 0 |
| y1/e1 | 0 | -23 | S | 0 |
| y1/e2 | 0 | -23 | S | 0 |

Here `S={0,1,3,4,6,7,8,10,11,13,14}`. Its bit polynomial is exactly `sum(T^j,j in S)=(1+T+T^2)^7` over F2. Consequently the first N51 form vanishes at both conjugate roots of `T^2+T+1` on every base place. These are **exact cancellations**, so none of the candidate orders in the table is an N51 valuation. The seventh-power structure gives a focused next calculation: evaluate the source coefficient series after the branch-specific lambda corrections from R119–R122, where the correction's seventh power may compete with higher s-order terms.

The source/protocol were committed before data. A separate polynomial long-division evaluator reproduced every one of the 15 coefficient supports and cutoffs on all four places, including the tied sets and F4 sums. A changed R116 input failed the hash gate; fresh replay is byte-identical. Lean 4.34.1 proves the four finite weighted-minimum and tie sets, both F4 cancellations, and the seventh-power bit-polynomial identity using result-bound constants. Lean does not formalize the 105 rational substitutions or the valuation theorem. The one-page PDF was rendered and visually inspected. No accepted-run failure or cloud spend.

Result SHA-256: `2bff55ee15f77c64046d06eb6f3c5d1b0f4de5e213fed2eaff9cb83977a35b69`. Replay: `python3 research/koblitz_modular_ic_20261018/rounds/124/norm124.py <output-directory>`. Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/124/verify124.py`.

Sources: R51 donor, R102 parser, R109 x=1 N51 method, R116 base series, R118 lambda edges and R123 branch census; [Stacks normal-curve DVR criterion](https://stacks.math.columbia.edu/tag/0B8Y), [Stoll's Newton polygon notes](https://www.mathe2.uni-bayreuth.de/stoll/teaching/pAdicAnalysis-WS2015/Skript-pAdicAnalysis-pub-print.pdf).
