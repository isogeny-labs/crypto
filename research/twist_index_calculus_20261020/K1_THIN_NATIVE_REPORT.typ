#set page(paper: "a4", flipped: true, margin: 14mm)
#set text(size: 8.2pt)
= Native orbit-complete quartic-twist polynomial IC
23 October 2026. Public synthetic one-target p=73/97 panel; 21 preregistered native subset cases, three seeds per curve; Q=[271]G.
The full Markdown report, frozen protocol, raw records, separate audits and run PDFs are in `K1_THIN_NATIVE_REPORT.md` and `K1_THIN_NATIVE_RUNS.md`.
== Mechanism and comparison
One complete L2 twist orbit has 12 physical factors but one folded log column. The k=1 collector peels signed L2 factors and solves S4 on three L0 factors. The full physical base and map are charged even for subsets. Full/candidate below is same-seed counted IC Fp multiplications; IC/rho uses same-target twist24 rho, also in counted Fp multiplications.
#image("k1_thin_native_overview.svg", width: 82%)
== Complete native grid
#table(columns: (1fr, .6fr, .65fr, .8fr, .75fr, 1.1fr, 1fr, .9fr, .65fr), inset: 2.5pt,
[*p/seed*], [*j*], [*rank*], [*query*], [*exact/poly*], [*S4 attempts/fail*], [*IC Fp M*], [*full/IC*], [*audit*],
[73/0], [1], [5], [30], [6/6], [720/0], [445,445,605], [1.994], [pass],
[73/0], [2], [6], [20], [6/6], [960/0], [593,181,104], [1.497], [pass],
[73/0], [3], [7], [15], [7/7], [1,080/0], [666,998,170], [1.332], [pass],
[73/0], [4], [8], [15], [8/8], [1,440/0], [888,203,272], [1.000], [pass],
[73/1], [1], [5], [45], [8/6], [1,080/1], [667,133,505], [1.774], [partial],
[73/1], [2], [6], [28], [9/7], [1,344/1], [829,490,482], [1.427], [partial],
[73/1], [3], [7], [20], [10/8], [1,440/1], [888,451,433], [1.332], [partial],
[73/1], [4], [8], [20], [11/9], [1,920/1], [1,183,617,228], [1.000], [partial],
[73/2], [1], [5], [47], [5/5], [1,128/0], [696,092,141], [3.653], [pass],
[73/2], [2], [6], [45], [6/6], [2,160/0], [1,331,570,891], [1.910], [pass],
[73/2], [3], [7], [43], [11/11], [3,096/0], [1,907,506,701], [1.333], [pass],
[73/2], [4], [8], [43], [15/15], [4,128/0], [2,542,727,892], [1.000], [pass],
[97/0], [1], [4], [310], [4/4], [7,440/0], [4,616,280,829], [2.998], [pass],
[97/0], [2], [5], [130], [8/8], [6,240/0], [3,870,889,536], [3.575], [pass],
[97/0], [3], [6], [310], [16/16], [22,320/0], [13,839,508,536], [1.000], [pass],
[97/1], [1], [4], [302], [4/4], [7,248/0], [4,496,535,068], [0.745], [pass],
[97/1], [2], [5], [75], [5/5], [3,600/0], [2,233,996,578], [1.499], [pass],
[97/1], [3], [6], [75], [6/6], [5,400/0], [3,349,559,070], [1.000], [pass],
[97/2], [1], [4], [182], [4/4], [4,368/0], [2,710,417,878], [2.471], [pass],
[97/2], [2], [5], [150], [5/5], [7,200/0], [4,465,588,225], [1.500], [pass],
[97/2], [3], [6], [150], [6/6], [10,800/0], [6,696,848,851], [1.000], [pass],
)
All 21 runs reach full anchored rank and independently recover 271. The four p=73 seed-one runs miss two real signed relations from one failed S4 system and remain partial relation finders. First/A/A non-timing records, independent audits, six full-base source parities and a mutation rejection pass.
== Claim boundary
Hash/tree and allocator CPU, peak memory, calibrated GAE, isolated ABAB timing and same-input msolve/Magma are missing. Counted field ratios are not priced S or wall-time speed ratios. No matched-rho crossover or deployed-curve claim follows. The complete Markdown table includes separate group counts and IC/rho field-work gaps.
