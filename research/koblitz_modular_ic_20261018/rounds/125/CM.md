# R125: N51 on the simple x=0 places; exact seven-place cancellation

R125 propagates all 15 exact R124 N51 coefficient series through the branch-specific F4 lambda translations from R119–R122, with a cutoff at every binomial coefficient. On each final correction coordinate it evaluates the simple and ramification-7 leading roots using their normalized weights. Both conjugate F4 roots agree.

| base place | simple-place N51 order | seven-place candidate order | seven-place tied correction powers |
|---|---:|---:|---|
| y0/e1 | **-30** | -210 | 0, 7 |
| y0/e3 | **-21** | -147 | 0, 7 |
| y1/e1 | **0** | 0 | 0, 7 |
| y1/e2 | **0** | 0 | 0, 7 |

The simple-place orders are certified: each has a unique weighted minimum at correction power 0, F4 leading scalar 1, and no precision competitor. On each ramification-7 place the powers 0 and 7 tie with leading scalars 1 and 1, hence cancel exactly. None of the seven-place candidate orders is an N51 valuation. This isolates the remaining valuation work to lifting the degree-seven local correction past its leading root. The simple places contribute pole orders 30 and 21 at the two y0 base places and unit order at the two y1 base places.

The source/protocol were committed before data. An independent binomial expansion using an explicit F4 multiplication table reproduced all 15 final coefficient support hashes, orders and cutoffs on both conjugate roots at all four base places. Byte-identical replay, changed-R124-input rejection, F4 conjugacy, and Lean 4.34.1 finite weighted-minimum and tie checks passed. The first negative-control harness attempt omitted five required fixtures and failed with a missing-file error; the isolated complete-fixture rerun rejected the changed input as intended. The PDF was rendered and visually inspected. Lean checks the finite weighted assertions, not the entire series substitution. No cloud spend.

Result SHA-256: `c9606bafa00f11756f7c4a84d0738033fb9337e8ea7028933e4ba7902fc3b795`. Replay: `python3 research/koblitz_modular_ic_20261018/rounds/125/branch125.py <output-directory>`. Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/125/verify125.py`.

Sources: R119–R124 published local expansions; [Stacks Henselian lifting](https://stacks.math.columbia.edu/tag/04GE) and [Stoll's Newton polygon notes](https://www.mathe2.uni-bayreuth.de/stoll/teaching/pAdicAnalysis-WS2015/Skript-pAdicAnalysis-pub-print.pdf).
