#set page(paper: "a4", margin: 1.6cm, footer: context align(center, text(size: 7pt, fill: gray)[EXPLORATORY, POST HOC deep dive: no verdict is changed, no speedup is claimed. Page #counter(page).display()]))
#set text(size: 9pt)
#set table(stroke: 0.4pt + gray, inset: 3pt)
#show table: set text(size: 7pt)
#show image: set image(width: 100%)
= Deep dive on H-T and H-M: carry-up cost decomposition and the Weierstrass-model effect on IC
<deep-dive-on-h-t-and-h-m-carry-up-cost-decomposition-and-the-weierstrass-model-effect-on-ic>
#strong[EXPLORATORY, POST HOC.] Written 2026-11-06, after the preregistered analysis and while the independent validation of the frozen snapshot is running. Study `research/volcano_ecdlp_census_followup_20261023/`. Nothing in this document changes a preregistered verdict, a frozen result or `analysis/results.json`, and no statement here is a speedup claim. Numbers come from `results_deepdive.json` (`deepdive.py`, seed 20261106, 10,000 replicates unless stated) over the committed extracts, census files and, for the IC mechanism, the 12 M session archives streamed through `tar`. Unpriced native work is priced at 0, 0.1 and 1 GAE per operation (ANALYSIS\_PLAN s1).

== Summary (one page)
<summary-one-page>
#strong[Preregistered verdicts, unchanged, quoted from `REPORT.md`:]

#strong[H-T (carry up to the crater, solve with `rho-cubic`): SUPPORTED.] Pooled over 63 source vertices (vertex-weighted, A3.2), T \= 0.5946 \[0.5684, 0.6220\] at 0.0 GAE, 0.5917 \[0.5655, 0.6193\] at 0.1 GAE, 0.5781 \[0.5512, 0.6063\] at 1.0 GAE per Velu point map. The control T0 (same transport, same crater curve, `rho-neg`) \= 0.9672 \[0.9227, 1.0145\], 0.9672 \[0.9223, 1.0149\], 0.9670 \[0.9204, 1.0164\], whose CI includes 1 at all three prices. Theory 1/sqrt(3) \= 0.5774; the transport term is 0.21% of the numerator at 1 GAE. Measured same-curve automorphism gain in S (`rho-cubic` / `rho-neg` on the carried instances): 0.6148 \[0.5895, 0.6420\]. This is a measured end-to-end ratio of a known composition, about 0.58x in ops\_gae, not a new speedup; and it is relative to `rho-neg` at the source, which is the strongest rho available on every (j !\= 0) source curve (erratum F3).

#strong[H-M (does IC cost depend on the Weierstrass model?): INCONCLUSIVE by the frozen-seed count, on the Monte Carlo boundary with 'supported for IC' (erratum F1).] `ic` is significant (Holm, alpha 0.05) on 5 of 12 chosen curves; `rho-neg` on 0 and `bsgs` on 0. Supported would need 6 of 12 for `ic` and at most 1 for each of the others; refuted would need at most 1 for `ic`. \[Withdrawn in the erratum (F1): the sentence 'IC’s model dependence is therefore present on some curves and absent on others: it is neither a general property nor noise' is not supported; a precise evaluation of the same test gives 6 of 12, 'supported for IC', so the verdict is not robust either way.\] The census’s V3 crater values (IC 57 / 67 / 73) are quoted only as the motivating observation.

Erratum F1 of `REPORT.md` (quoted, last sentence of the erratum entry): #strong[The H-M verdict therefore sits on the Monte Carlo boundary between inconclusive and supported for IC and is not robust either way.]

#strong[Exploratory findings (all post hoc; n and intervals in the sections):]

#align(center)[#table(
  columns: 3,
  align: (col, row) => (right,auto,auto,).at(col),
  inset: 6pt,
  [\#], [finding], [key numbers (n)],
  [1],
  [T factors exactly as T \= F x T0; the automorphism factor F carries the gain, the control T0 is consistent with 1],
  [at 1 GAE: F 0.598 \[0.571, 0.626\], T0 0.967 \[0.921, 1.015\], T 0.578 \[0.551, 0.606\]; n \= 63 vertices],
  [2],
  [the ~6% gap to 1/sqrt3 is a charged additive part (ops minus canonicalisations \= 371 +- 8) that does not shrink; the iteration count itself matches theory],
  [charged ratio 0.615 \[0.589, 0.642\]; canonicalisation ratio 0.578 \[0.551, 0.607\]; predicted 0.614; n \= 3064 pairs],
  [3],
  [per-vertex T\_v is homogeneous: no trend with level; 2 of 63 CIs include 1],
  [Cochran Q 65.4 on 62 df, p \= 0.36; slope 0.9966 per level, p \= 0.74],
  [4],
  [break-even price of a Velu point map is about 177 GAE (charged ops) and has no finite value under the plan’s same-price pricing; transport share 0.21% at 1 GAE],
  [model extrapolation (not measured): 4.32e-09 at 64 bits, 5.46e-38 at 256],
  [5],
  [H-T is a known composition: the volcano preserves r, the crater adds the order-6 automorphism for the price of a few point maps],
  [no new method; see section 5],
  [6],
  [significant IC curves (frozen seed): V5:14, V6:2, V6:99, V6:214, V6:223; V6:98 is the sixth],
  [no V5-vs-V6 or level pattern (Fisher p \= 0.58); n \= 12 curves],
  [7],
  [paired/blocked tests agree with the unpaired ones (blocking gains nothing); pooled across 12 curves the IC model effect is strong, rho-neg and bsgs show none],
  [sum of F 33.6 vs null mean 12.0, p \<\= 5e-05; rho-neg p \= 0.83; n \= 6,144 rows per arm],
  [8],
  [max/min of model means is biased up by noise alone (null median 1.13-1.18); excess over null p95 is visible for IC only],
  [IC above null p95 on 9 of 12 curves, rho-neg on 0, bsgs on 0],
  [9],
  [IC cost tracks relation yield, which tracks how many factor-base relation sources fall in the subgroup on each model; the Legendre count (factor-base length) does not],
  [log trials per relation vs log H: r \= -0.84; log S vs log H: r \= -0.70, p \= 0.0001; n \= 96 models],
  [10],
  [Q1’s curve-effect test and H-M do not contradict: Q1’s within-vertex spread already contains the model effect, which Q1’s own data show (V6, 648 vertices x 3 models)],
  [pooled F 1.36, permutation p \= 0.0005; between-model CV 8.8% \[7.2, 10.2\]],
  [11],
  [design recommendation: about 96 rows per model (1.5x the 64 used) gives 80% power for \>\= 6 of 12 under moment-shrunk truth],
  [simulated, 1000 studies per point],
)
]

#pagebreak()
== 1. H-T decomposition (question 1)
<h-t-decomposition-question-1>
#emph[Exploratory, post hoc. n \= 63 source vertices (levels 1-6 with 1/2/4/8/16/32 vertices), 3,064 (target, round) pairs per arm; vertex-weighted ratios of means as in A3.2; cluster bootstrap, 10,000 replicates, seed 20261106, 95% percentile CI.]

With C the vertex-averaged priced `ops_gae`, M the mean number of Velu point maps, p the price per point map and N\_v the source rows:

- F \= (C(rho-cubic, Tcr) + p M) / (C(rho-neg, Tcr) + p M): the crater factor, the same transport in both terms.
- T0 \= (C(rho-neg, Tcr) + p M) / C(rho-neg, N): the control of the plan.
- T \= F x T0 holds exactly (maximum error 1.1e-16 at every price, overall and per level).

#align(center)[#table(
  columns: 5,
  align: (col, row) => (auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [price per point map], [F (crater factor)], [T0 (control)], [T (verdict quantity)], [transport share of T numerator],
  [0 GAE],
  [0.615 \[0.589, 0.642\]],
  [0.967 \[0.923, 1.013\]],
  [0.595 \[0.569, 0.622\]],
  [0.000%],
  [0.1 GAE],
  [0.612 \[0.586, 0.639\]],
  [0.967 \[0.922, 1.013\]],
  [0.592 \[0.566, 0.619\]],
  [0.036%],
  [1 GAE],
  [0.598 \[0.571, 0.626\]],
  [0.967 \[0.921, 1.015\]],
  [0.578 \[0.551, 0.606\]],
  [0.207%],
)
]

Per level at 1 GAE (descriptive; the plan’s verdict is the overall row only):

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,right,auto,auto,auto,).at(col),
  inset: 6pt,
  [level], [vertices], [F], [T0], [T],
  [1],
  [1],
  [0.609 \[0.574, 0.646\]],
  [0.956 \[0.902, 1.014\]],
  [0.582 \[0.548, 0.617\]],
  [2],
  [2],
  [0.580 \[0.546, 0.615\]],
  [1.005 \[0.944, 1.075\]],
  [0.583 \[0.547, 0.622\]],
  [3],
  [4],
  [0.555 \[0.520, 0.594\]],
  [1.015 \[0.953, 1.086\]],
  [0.563 \[0.528, 0.600\]],
  [4],
  [8],
  [0.608 \[0.561, 0.662\]],
  [1.007 \[0.937, 1.087\]],
  [0.612 \[0.558, 0.672\]],
  [5],
  [16],
  [0.595 \[0.553, 0.641\]],
  [1.007 \[0.917, 1.103\]],
  [0.599 \[0.549, 0.652\]],
  [6],
  [32],
  [0.603 \[0.558, 0.653\]],
  [0.931 \[0.856, 1.010\]],
  [0.562 \[0.519, 0.608\]],
  [all],
  [63],
  [0.598 \[0.571, 0.626\]],
  [0.967 \[0.921, 1.015\]],
  [0.578 \[0.551, 0.606\]],
)
]

#strong[Which factor carries the gain.] F does. In logs at 1 GAE, log T \= -0.5480 splits into -0.5152 from the pure automorphism ratio (rho-cubic over rho-neg on the crater rows, uncharged work priced), +0.0008 from transport, and -0.0336 from the control: 94% of the log-gain is the automorphism, 6% is T0 sitting below 1. #strong[T0 is consistent with 1]: its CI 0.967 \[0.921, 1.015\] includes 1 at all three prices, as it should, because the cost of rho depends on the group order r and the volcano preserves r. Its 3% shortfall is consistent with sampling noise: the bootstrap variance of log T is shared between the numerator and the denominator draws (var log F \= 5.4e-04, var log T0 \= 6.3e-04, covariance -3.0e-04), so the T0 draw does about as much to the width of T’s CI as F does. Level 6 alone has T0 \= 0.931 (CI upper end 1.010), which pulls its T to 0.562, about level 3’s 0.563, the lowest.

#figure([#image("figures/fig1_T_decomposition_by_level.png")],
  caption: [
    fig1: H-T decomposed (exploratory, post hoc). Source `results_deepdive.json`, key `H-T_decomposition`; SVG beside the PNG.
  ]
)

== 2. Gap to the 1/sqrt3 theory (question 2)
<gap-to-the-1sqrt3-theory-question-2>
#emph[Exploratory, post hoc. Tcr pairs: n \= 3064 (63 vertices); crater positive-control pairs (D, N, ctl pooled): n \= 4000. CIs: vertex-cluster bootstrap (Tcr) and pair bootstrap (crater), 10,000 replicates.]

Theory: with equivalence classes of order 6 against 2, rho needs sqrt(pi r / 12) against sqrt(pi r / 4) iterations, ratio 1/sqrt3 \= 0.5774. Measured charged-ops ratio rho-cubic / rho-neg: #strong[0.615 \[0.589, 0.642\]] on the carried instances and #strong[0.6113 \[0.5992, 0.6236\]] on the crater (the report’s 0.6113). Both CIs exclude 1/sqrt3: the gap is +0.037 (6.5%) and +0.034 (5.9%).

#strong[What explains it, derived from the recorded fields.] Every V5 rho row records `ops_gae` (charged) and `canonicalisations_uncharged`. Their difference is:

- 371.1 +- 8.2 operations (min 340, max 394), #strong[identical in the rho-cubic and rho-neg rows of a pair in 100% of the 3064 pairs], and uncorrelated with the walk length (correlation +0.003 for rho-neg, -0.010 for rho-cubic). It is an additive part that belongs to the instance, not to the walk: the automorphism does not shrink it. (What it is, setup or collision handling, is not derivable from the CSV; only that it does not scale with the walk.)
- The canonicalisation count, the natural proxy for walk iterations, has the ratio #strong[0.578 \[0.551, 0.607\]] on Tcr and 0.5751 \[0.5624, 0.5882\] on the crater: both include 1/sqrt3 \= 0.5774.
- If the iterations scale by exactly 1/sqrt3 and the additive part stays, the charged ratio would be (theta C + f) / (C + f) \= #strong[0.6139] on Tcr (measured 0.6148) and 0.6133 \[0.6128, 0.6138\] on the crater (measured 0.6113). The gap is therefore accounted for by the additive part, with nothing left over that the data can resolve.

Absolute levels: canonicalisations / sqrt(r) are 0.972 (rho-neg) and 0.555 (rho-cubic) against the idealised sqrt(pi/4) \= 0.886 and sqrt(pi/12) \= 0.512: both arms sit 9.6% and 8.6% above the formula. That excess (distinguished-point delay, fruitless cycles, collision detection) is equal in the two arms within noise, so it cancels in the ratio; the data #strong[cannot separate] its causes, and in particular cannot say whether rho-cubic loses more to fruitless cycles than rho-neg.

Distribution shape (Tcr pairs, n \= 3,064 each):

#align(center)[#table(
  columns: 6,
  align: (col, row) => (auto,right,right,right,right,right,).at(col),
  inset: 6pt,
  [quantity], [mean], [median], [CV], [median/mean], [skew],
  [S rho-cubic],
  [0.6464],
  [0.6124],
  [0.449],
  [0.947],
  [0.79],
  [S rho-neg],
  [1.062],
  [1.015],
  [0.457],
  [0.955],
  [0.65],
  [canonicalisations rho-cubic],
  [2268],
  [2130],
  [0.522],
  [0.939],
  [0.79],
  [canonicalisations rho-neg],
  [3966],
  [3775],
  [0.500],
  [0.952],
  [0.65],
  [Rayleigh reference],
  [],
  [],
  [0.523],
  [0.939],
  [],
)
]

The canonicalisation counts have the CV and median/mean of a Rayleigh-like collision time in both arms (CV 0.50-0.52 against 0.523), so rho-cubic’s walk length is the same shape as rho-neg’s, scaled. Quantile ratios cubic / neg (5, 25, 50, 75, 95, 99%): charged S 0.690, 0.616, 0.603, 0.606, 0.597, 0.601; canonicalisations 0.591, 0.555, 0.564, 0.576, 0.577, 0.586. The charged ratio is highest at the low quantiles (the additive part is a larger share of a short solve), the canonicalisation ratio is flat near 0.56-0.59 (row-bootstrap CIs in fig8, n \= 1,000 replicates).

#strong[Not charged for rho-cubic]: the class-representative work per iteration (up to six images) is counted in `canonicalisations_uncharged` for both arms and priced at the same p for both. A canonicalisation under the order-6 action is plausibly dearer than a sign choice; a per-arm price was not measured, so T at 0.1 and 1 GAE treats them alike.

#figure([#image("figures/fig8_gap_to_theory.png")],
  caption: [
    fig8: gap to theory. Source `results_deepdive.json`, keys `H-T_gap`, `crater_rows`.
  ]
)

== 3. Heterogeneity across the 63 vertices (question 3)
<heterogeneity-across-the-63-vertices-question-3>
#emph[Exploratory, post hoc. n \= 63 vertices; per-vertex CIs are the preregistered pair-bootstrap CIs in `analysis/results.json` (descriptive there), reused here.]

T\_v at 1 GAE ranges 0.43 to 0.89. The per-vertex CI includes 1 for #strong[2 of 63 vertices at 0 GAE, 2 at 0.1 and 2 at 1 GAE] (vertices 50, 57 at 1 GAE; largest upper end 1.22); their T\_v are 50: 0.89 \[0.65, 1.22\], 57: 0.72 \[0.46, 1.10\] (8 targets x few rounds each). The control T0\_v includes 1 for 58 of 63.

Heterogeneity beyond sampling noise is not detected: Cochran’s Q on log T\_v \= 65.4 on 62 df, p \= 0.36, I2 \= 5% (standard errors from the percentile CI widths, approximate). Trend with source level, which here equals path length to the crater (M\_v \= 2 x level): weighted slope of log T\_v \= -0.0034 per level (ratio 0.9966), permutation p \= 0.74 (10,000 shuffles); Spearman rho \= -0.21, permutation p \= 0.10 (2,000 shuffles). Level means of T\_v at 1 GAE: L1 0.582, L2 0.583, L3 0.563, L4 0.614, L5 0.603, L6 0.568. No trend, no outlier beyond what 63 noisy ratios give.

#figure([#image("figures/fig2_T_v_forest.png")],
  caption: [
    fig2: per-vertex T\_v at 1 GAE with 95% CIs, grey diamonds the control T0\_v. Source `analysis/results.json` `H-T.by_vertex`.
  ]
)

== 4. Break-even price and transport share (question 4)
<break-even-price-and-transport-share-question-4>
#emph[Exploratory, post hoc. n \= 63 vertices. Break-even prices are point values per vertex (no CI); pooled transport shares carry the bootstrap CI of section 1.]

p\* is the price per Velu point map at which T \= 1: p\* \= (C(rho-neg, N\_v) - C(rho-cubic, Tcr\_v)) / M\_v. It depends on what C includes:

- #block[
  #set enum(numbering: "(a)", start: 1)
  + charged ops only: per vertex min 33, median 165, max 901 GAE per point map; #strong[pooled 177 GAE]. By level 1-6: 901, 437, 304, 200, 167, 159.
  ]

- #block[
  #set enum(numbering: "(a)", start: 2)
  + every uncharged operation priced at 1 GAE: min 65, median 330, max 1803; pooled 353.
  ]

- #block[
  #set enum(numbering: "(a)", start: 3)
  + the plan’s definition, the same p on the uncharged counters and on the point maps: #strong[no finite p\* exists for any of the 63 vertices]. Raising p adds more to the rho-neg denominator (about 3,900 canonicalisations) than to the rho-cubic numerator (about 2,300) and the transport term (2 to 12), so T falls with p, from 0.5946 at 0 to 0.5781 at 1.
  ]

A Velu point map at an order-2 isogeny costs a handful of field operations, so the study’s upper price of 1 GAE is already conservative, and the break-even is two to three orders of magnitude above it.

Transport share of the T numerator, measured, at 1 GAE (95% CI): level 1 0.040% \[0.038, 0.042\]; level 2 0.082% \[0.078, 0.086\]; level 3 0.128% \[0.122, 0.134\]; level 4 0.159% \[0.149, 0.170\]; level 5 0.201% \[0.189, 0.213\]; level 6 0.246% \[0.232, 0.260\]; overall 0.207% \[0.200, 0.214\]. It grows linearly with depth (2 point maps per level) and stays below 0.25%.

#strong[MODEL EXTRAPOLATION, not a measurement.] Assumptions: solve cost \= c sqrt(r) GAE with c \= 0.646 (the vertex-averaged rho-cubic S on Tcr; the additive part scales too); transport \= 2 x depth point maps priced at p, independent of bit size; depth scenarios: the measured mean depth (5.1), 6 held fixed, and 6 x bits / 26. Not modelled: finding the path to the crater, whether a j \= 0 volcano of that depth exists at that size, cofactors, and the class machinery at that size.

#align(center)[#table(
  columns: 6,
  align: (col, row) => (right,auto,right,right,right,right,).at(col),
  inset: 6pt,
  [bits of r], [depth scenario], [depth], [transport GAE (p \= 1)], [projected transport share, p \= 0.1], [p \= 1],
  [26 (measured)],
  [measured mean depth],
  [5.1],
  [10],
  [1.92e-04],
  [1.92e-03],
  [26 (measured)],
  [depth 6],
  [6.0],
  [12],
  [2.27e-04],
  [2.26e-03],
  [26 (measured)],
  [depth 6\*bits/26],
  [6.0],
  [12],
  [2.27e-04],
  [2.26e-03],
  [64],
  [measured mean depth],
  [5.1],
  [10],
  [3.67e-10],
  [3.67e-09],
  [64],
  [depth 6],
  [6.0],
  [12],
  [4.32e-10],
  [4.32e-09],
  [64],
  [depth 6\*bits/26],
  [14.8],
  [30],
  [1.06e-09],
  [1.06e-08],
  [128],
  [measured mean depth],
  [5.1],
  [10],
  [8.55e-20],
  [8.55e-19],
  [128],
  [depth 6],
  [6.0],
  [12],
  [1.01e-19],
  [1.01e-18],
  [128],
  [depth 6\*bits/26],
  [29.5],
  [59],
  [4.95e-19],
  [4.95e-18],
  [256],
  [measured mean depth],
  [5.1],
  [10],
  [4.63e-39],
  [4.63e-38],
  [256],
  [depth 6],
  [6.0],
  [12],
  [5.46e-39],
  [5.46e-38],
  [256],
  [depth 6\*bits/26],
  [59.1],
  [118],
  [5.37e-38],
  [5.37e-37],
)
]

The share falls like 2^(-bits/2). The projection says transport stays negligible if the model holds; it does not say a crater of such depth exists at cryptographic size, nor that the path can be found cheaply, and it is not evidence.

#figure([#image("figures/fig3_breakeven_and_extrapolation.png")],
  caption: [
    fig3: (a) and (b) measured, (c) model extrapolation. Source `results_deepdive.json`, key `H-T_vertices`.
  ]
)

== 5. What H-T means (question 5)
<what-h-t-means-question-5>
An l-isogeny between two curves over the same prime field preserves the group order up to the isogeny’s kernel: the volcano’s curves all have the same order, so the same prime r in the same size subgroup, and Pollard rho on any of them costs the same, which is what the control T0 \= 0.97 (CI includes 1) shows. The j \= 0 crater, though, has an extra automorphism of order 6 that the other curves lack; rho can quotient by it and run on classes of size 6 instead of 2, which saves a factor 1/sqrt3, about 0.58. A Velu isogeny is a rational map of known formula, so carrying the instance (G, Q) up to the crater costs two point maps per level: 2 to 12 maps, well under 1% of a 26-bit solve even at 1 GAE each. H-T measures that composition end to end and finds 0.58 of the cost of solving where the instance was born with rho-neg, at 63 vertices.

It is a known composition, the census’s A2.3 transport-then-solve with the crater’s automorphism as the destination gain, not a new method: the gain is the j \= 0 automorphism and the isogeny is only the road to it. It is relative to rho-neg at the source (the strongest rho available on a curve with j not 0 or 1728), and it is a statement about this 26-bit family, where an isogeny path to a j \= 0 crater of depth 1 to 6 exists. It says nothing about curves that have no such path, and nothing about index calculus, which is 53 to 141 times the matched rho in this study (REPORT section 9).

== 6. Which five curves, and where the near misses are (question 6)
<which-five-curves-and-where-the-near-misses-are-question-6>
#emph[Exploratory, post hoc. n \= 12 curves, 8 models x 64 (target, round) rows per curve and arm; the table is the frozen-seed result of `analysis/results.json` (permutation p with 10,000 shuffles, seed 20261024, Holm over the 12 curves).]

#align(center)[#table(
  columns: 8,
  align: (col, row) => (auto,right,right,right,right,auto,right,right,).at(col),
  inset: 6pt,
  [curve], [level], [ic F], [ic p\_perm], [ic p\_Holm (frozen)], [significant], [rows: unpaired Holm, 20,000 shuffles, new seed], [paired Holm],
  [V5:0],
  [0],
  [1.95],
  [0.0629],
  [0.2516],
  [no],
  [0.2372],
  [0.2758],
  [V5:14],
  [4],
  [3.21],
  [0.0014],
  [0.0140],
  [yes],
  [0.0235],
  [0.0215],
  [V5:19],
  [5],
  [1.77],
  [0.0929],
  [0.2787],
  [no],
  [0.2725],
  [0.2791],
  [V5:44],
  [6],
  [0.77],
  [0.6136],
  [0.8623],
  [no],
  [0.8616],
  [0.7867],
  [V6:2],
  [0],
  [3.22],
  [0.0025],
  [0.0225],
  [yes],
  [0.0238],
  [0.0215],
  [V6:43],
  [2],
  [2.21],
  [0.0322],
  [0.1610],
  [no],
  [0.1612],
  [0.2020],
  [V6:98],
  [0],
  [2.81],
  [0.0073],
  [0.0511],
  [no],
  [0.0434],
  [0.0493],
  [V6:99],
  [1],
  [5.23],
  [0.0001],
  [0.0012],
  [yes],
  [0.0006],
  [0.0006],
  [V6:214],
  [1],
  [5.91],
  [0.0001],
  [0.0012],
  [yes],
  [0.0006],
  [0.0006],
  [V6:223],
  [3],
  [3.21],
  [0.0028],
  [0.0225],
  [yes],
  [0.0238],
  [0.0215],
  [V6:262],
  [2],
  [1.00],
  [0.4312],
  [0.8623],
  [no],
  [0.8616],
  [0.7867],
  [V6:599],
  [4],
  [2.33],
  [0.0242],
  [0.1452],
  [no],
  [0.1488],
  [0.1458],
)
]

The five frozen-seed curves are V5:14 (level 4), V6:2 (0), V6:99 (1), V6:214 (1) and V6:223 (3). Near misses: V6:98 (Holm 0.0511, the sixth), then V6:43 (0.161), V6:599 (0.145), V5:19 (0.279), V5:0 (0.252). #strong[No clustering]: V5 has 1 of 4 curves significant and V6 4 of 8 (Fisher exact p \= 0.58); by level (frozen run), level 0 has 1 of 3 (V6:2; V5:0 and V6:98 not), level 1 has 2 of 2, level 2 has 0 of 2, level 3 has 1 of 1, level 4 has 1 of 2 (V5:14 yes, V6:599 no), levels 5 and 6 have 0 of 1 each. Most of the non-significant curves still have F above the null mean of about 1 (V5:0 1.95, V5:19 1.77, V6:43 2.21, V6:599 2.33, V6:98 2.81); only V6:262 (1.00) and V5:44 (0.77) do not. That is why the pooled test of section 7 is the better instrument.

== 7. Paired and pooled tests (question 7), and the V6:98 boundary
<paired-and-pooled-tests-question-7-and-the-v698-boundary>
#emph[Exploratory, post hoc; seed 20261106. n \= 12 curves x 8 models x 64 blocks per arm. The frozen reproduction through the committed `hypothesis_M` matched `results.json` exactly (F and p for all 36 curve-arm cells).]

#strong[The boundary.] The frozen rule reads 5 of 12 as inconclusive; erratum F1 says the 6th curve, V6:98, sits on the Monte Carlo boundary. Here is how far:

- V6:98’s raw permutation p is 0.00703 +- 0.00008 at 1,000,000 shuffles (running estimates by shuffle count: 1,000: 0.0060, 10,000: 0.0081, 100,000: 0.0076, 1,000,000: 0.0070); the parametric F(7, 504) gives 0.00699. Holm’s multiplier at its rank is 7, so the adjusted p is about 7 x 0.00703 \= 0.0492, just under 0.05.
- At 200,000 shuffles per curve the unpaired Holm p is 0.0496 and the unpaired count is 6 of 12; the paired count is 6 of 12 (V6:98 Holm 0.0488).
- 100 independent whole-table reruns with 10,000 shuffles each (the frozen setting, different seeds): unpaired design 6 of 12 in 57 runs and 5 of 12 in 43, V6:98 Holm quantiles 2.5/50/97.5%: 0.0395, 0.0483, 0.0613; paired design 6 of 12 in 71 runs and 5 in 29.

So the preregistered statistic landed on 5 on the one stream it used (43% of independent 10,000-shuffle streams do the same): with an exact evaluation of the same rule the verdict would read 6 of 12, rho-neg 0, bsgs 0, supported for IC. This agrees with erratum F1; it does not change the reported verdict, and it does not rest on any other curve being borderline (at the high-precision setting the sixth curve is at 0.0496 and the seventh, V6:43, at 0.157).

#figure([#image("figures/fig7_V698_boundary.png")],
  caption: [
    fig7: V6:98 against shuffle count and across independent 10,000-shuffle runs. Source `results_deepdive.json`, `H-M.boundary`.
  ]
)

#strong[Paired (blocked) test.] The 8 models of a curve solve the same 64 (target, round) instances (BSGS gives identical cost in 100% of blocks), so a blocked permutation (shuffle model labels inside each block) is available. It buys nothing here: the residual mean square after removing blocks is 0.998 of the within-model mean square for IC, and the block share of variance is 0.9% (instance difficulty does not carry across models). Per curve the paired and unpaired p’s differ by Monte Carlo and small shifts only (fig4); both designs give #strong[6 of 12] for IC at 20,000 shuffles and 0 for rho-neg; for bsgs every block is identical, so F is 0 (unpaired) and the blocked statistic is undefined.

#figure([#image("figures/fig4_HM_paired_vs_unpaired.png")],
  caption: [
    fig4: per-curve Holm p, unpaired against paired, 20,000 shuffles per curve. Source `results_deepdive.json`, `H-M.per_curve`.
  ]
)

#strong[Pooled across the 12 curves] (the per-curve permutations are independent, so the null is the sum of independent permutation F values; Fisher’s combination uses each curve’s permutation ranks; 20,000 shuffles, so p cannot go below 5e-5):

#align(center)[#table(
  columns: 7,
  align: (col, row) => (auto,auto,right,right,right,right,right,).at(col),
  inset: 6pt,
  [arm], [design], [sum of F (12 curves)], [null mean], [null 95th pct], [p (sum of F)], [Fisher p],
  [ic],
  [unpaired],
  [33.6],
  [12.0],
  [15.3],
  [5.0e-05],
  [5.0e-05],
  [ic],
  [paired],
  [33.7],
  [12.1],
  [15.3],
  [5.0e-05],
  [5.0e-05],
  [rho-neg],
  [unpaired],
  [10.2],
  [12.0],
  [15.3],
  [8.3e-01],
  [8.5e-01],
  [rho-neg],
  [paired],
  [10.3],
  [12.0],
  [15.2],
  [8.3e-01],
  [8.4e-01],
  [bsgs],
  [unpaired],
  [0.0],
  [12.0],
  [15.3],
  [1.0e+00],
  [1.0e+00],
  [bsgs],
  [paired],
  [undefined],
  [],
  [],
  [],
  [],
  [ic, excluding the 5 frozen-significant curves (n \= 7)],
  [unpaired],
  [12.8],
  [7.0],
  [9.5],
  [4.0e-04],
  [6.0e-04],
  [ic, excluding the 5 frozen-significant curves (n \= 7)],
  [paired],
  [12.8],
  [7.0],
  [9.6],
  [4.5e-04],
  [5.5e-04],
)
]

The evidence for #emph[some] model effect on IC is strong overall (mean F 2.80 per curve against a null mean of about 1.0), and it does not rest on the five significant curves. rho-neg is below its null (mean F 0.85; p \= 0.83) and bsgs is exactly 0. The pooled test was chosen after the data; it is a post hoc strengthening of the question, not a replacement for the preregistered per-curve rule.

== 8. Effect-size calibration of max/min (question 8): a methodological finding
<effect-size-calibration-of-maxmin-question-8-a-methodological-finding>
#emph[Exploratory, post hoc. n \= 12 curves per arm, 8 model means each of 64 rows. Null: model labels permuted across the 512 rows of a curve, 20,000 shuffles, seed 20261106.]

`REPORT.md` section 8 gives, per curve and arm, max/min of the eight model means with a percentile bootstrap CI (rows resampled within model). That ratio is biased upward by noise alone: even when no model effect exists, the largest of eight noisy means over the smallest is above 1. Under the permutation null its #strong[median is 1.16 to 1.18 for rho-neg and 1.13 to 1.18 for IC] (95th percentile 1.26 to 1.29). The reported bootstrap CIs inherit the same bias: 12 of 12 rho-neg CIs have a lower end above 1 although rho-neg has no significant curve, and for BSGS, where the models carry identical instances and cost, the observed ratio is exactly 1.000 but all 12 CIs lie above 1 (about \[1.03, 1.11\]) and 12 of 12 exclude their own point value. The bootstrap CI of a max/min is a CI of a biased statistic, not of the model effect.

#align(center)[#table(
  columns: 6,
  align: (col, row) => (auto,auto,auto,auto,right,right,).at(col),
  inset: 6pt,
  [arm], [observed max/min, range over 12 curves], [null median, range], [null 95th pct, range], [curves above null 95th pct], [above null 99th pct],
  [ic],
  [1.118 to 1.577],
  [1.130 to 1.182],
  [1.206 to 1.291],
  [9 of 12],
  [4 of 12],
  [rho-neg],
  [1.099 to 1.259],
  [1.164 to 1.183],
  [1.259 to 1.294],
  [0 of 12],
  [0 of 12],
  [bsgs],
  [1.000 to 1.000],
  [1.064 to 1.066],
  [1.099 to 1.104],
  [0 of 12],
  [0 of 12],
)
]

Read against the null, #strong[rho-neg’s observed 1.10-1.26 is entirely null-sized (0 of 12 above the null 95th percentile)], so the percentile CIs in REPORT section 8 should not be read as showing a rho-neg model effect, and IC’s observed ratio exceeds the null 95th percentile on 9 of 12 curves (4 above the 99th percentile; about 0.6 would be expected by chance): the 'excess over null median' for IC runs from 0.99 (no excess) to 1.35 (V6:214). The unpaired null for bsgs is not a fair benchmark because BSGS values are identical across models by construction (observed 1.000 against a null median 1.06): that cell is an illustration of the bias, not a test. Methodologically, an effect size here should be quoted as observed / null median or as the between-model coefficient of variation (method of moments: IC 7.2% pooled over the 12 curves, rho-neg 1.7%, bsgs 0), not as a raw max/min with a bootstrap CI. I did not edit `REPORT.md`.

#figure([#image("figures/fig5_null_calibrated_maxmin.png")],
  caption: [
    fig5: observed max/min of model means (dots), the reported bootstrap CI (grey) and the 95th percentile under model-label permutation (bars). Source `results_deepdive.json`, `H-M.maxmin_calibration`.
  ]
)

== 9. Mechanism: why IC depends on the model (question 9)
<mechanism-why-ic-depends-on-the-model-question-9>
#emph[Exploratory, post hoc. n \= 96 models (12 curves x 8), 64 ic rows each. IC features come from the committed extracts; relation counters come from streaming `records.jsonl` of the 12 M session archives (no unpacking). The factor base and the subgroup count H were rebuilt from the census coefficients in 30 lines of exact modular arithmetic (not an ecbench measurement) and checked against the recorded `abscissae_scanned` of all 96 models: all 96 match.]

What `prime-abscissa:size=64` is, from `src/cryptanalysis/ic_boundary.rs` (`prime_factor_base`): the 64 smallest abscissae x \= 1, 2, 3, … for which x^3 + ax + b is a square, both signs. 'Prime' means prime field; the base is not prime-valued x. An isomorphism (x, y) -\> (u^2 x, u^3 y) moves which small x lie on the curve, so each model has a different factor base. Each IC solve stops after about 19 relations on average and spends almost all its cost in the relation phase: S tracks `trials` (log S against log trials per relation: r \= 0.85 within curve).

Findings, within-curve (each curve’s models centred, so curve and cofactor effects are removed), permutation p from 10,000 shuffles of model labels inside a curve:

#align(center)[#table(
  columns: 4,
  align: (col, row) => (auto,auto,right,right,).at(col),
  inset: 6pt,
  [per-model feature], [against], [r], [p],
  [log H (relation sources inside the subgroup)],
  [log trials per relation],
  [-0.841],
  [0.0001],
  [log H],
  [log mean S (ic)],
  [-0.699],
  [0.0001],
  [log H],
  [log mean S (rho-neg), negative control],
  [+0.093],
  [0.3705],
  [k0 + 0.5 (factor-base columns inside the subgroup)],
  [log mean S (ic)],
  [-0.147],
  [0.1005],
  [npair + 0.5 (same-coset column pairs)],
  [log mean S (ic)],
  [-0.090],
  [0.2665],
  [Legendre symbols to build the base (\= abscissae scanned)],
  [log mean S (ic)],
  [-0.016],
  [0.89],
  [model coefficient u / p],
  [log mean S (ic)],
  [-0.041],
  [0.71],
  [model coefficient a / p],
  [log mean S (ic)],
  [-0.063],
  [0.58],
  [model coefficient b / p],
  [log mean S (ic)],
  [+0.007],
  [0.95],
  [model coefficient gx / p],
  [log mean S (ic)],
  [-0.004],
  [0.97],
  [model coefficient gy / p],
  [log mean S (ic)],
  [-0.074],
  [0.48],
  [model index 0-7],
  [log mean S (ic)],
  [+0.283],
  [0.009 (one of 13 uncorrected tests; Bonferroni 0.12)],
)
]

H \= 2 k0 + 4 C(k0, 2) + 2 npair counts the distinct elements +-P\_i and +-P\_i +- P\_j (P in the 64 factor-base points) that lie in the subgroup of order r. A relation needs a target combination equal to such an element, so the per-trial yield is expected to be proportional to H / r (a counting argument; the data below test its consequence). H varies by model (for example V5:0 1140-1358, V5:14 1142-1664, V5:19 1188-1578, V5:44 1664-1954), because which small x are on the curve, and in which coset of the subgroup (cofactors 4, 47), changes with the model. Across the 96 models the within-curve correlation between log H and log trials per relation is #strong[-0.84] and with log mean S #strong[-0.70] (within-curve regression slope -0.57, R2 0.49; capped below 1 by row noise on 64-row means). The negative control, rho-neg’s cost on the same models, does not correlate (r \= +0.09, p \= 0.37). The factor-base length (Legendre count) is not the mechanism (r \= -0.02), nor are the model coefficients.

Per curve (n \= 8 models each; Spearman rank correlation of H with the model’s trials per relation and with its mean S; negative means higher H, cheaper IC):

#align(center)[#table(
  columns: 4,
  align: (col, row) => (auto,right,right,right,).at(col),
  inset: 6pt,
  [curve], [cofactor], [rho(H, trials per relation)], [rho(H, S ic)],
  [V5:0],
  [4],
  [+0.49],
  [+0.22],
  [V5:14],
  [4],
  [-0.57],
  [-0.57],
  [V5:19],
  [4],
  [-0.86],
  [-0.69],
  [V5:44],
  [4],
  [-0.12],
  [-0.57],
  [V6:2],
  [47],
  [-0.90],
  [-0.86],
  [V6:214],
  [47],
  [-0.98],
  [-0.98],
  [V6:223],
  [47],
  [-0.84],
  [-0.53],
  [V6:262],
  [47],
  [-0.86],
  [-0.71],
  [V6:43],
  [47],
  [-0.90],
  [-0.62],
  [V6:599],
  [47],
  [-0.86],
  [-0.55],
  [V6:98],
  [47],
  [-0.97],
  [-0.87],
  [V6:99],
  [47],
  [-0.91],
  [-0.81],
)
]

The association is negative on 11 of 12 curves for trials per relation and 11 of 12 for S. It is strongest on V6 (cofactor 47; within-curve r \= -0.96 for trials per relation and -0.77 for S, n \= 64 models) and weaker on V5 (cofactor 4; -0.36 and -0.47, n \= 32; the V5 crater V5:0 has the wrong sign). Neither k0 nor npair alone reaches significance (table above); only their combination H does. The log scale is used so that V5’s larger absolute H range does not dominate the pooled within-curve correlation; the rank-based per-curve table shows the pattern without it. That the predictor’s form matters is a reason to fix it before testing it again.

This supplies the mechanism that REPORT section 12 declined to claim: the x-coordinate dependence of the factor base acts through how many relation sources fall in the subgroup. It is a measured association on 96 models with a derived predictor chosen after seeing the relation counters, so it is exploratory; a follow-up should fix H before the run. Phase counters other than trials, relations and `abscissae_scanned` (linear algebra rank, row operations) were not used.

#figure([#image("figures/fig11_ic_yield_mechanism.png")],
  caption: [
    fig11: within-curve log trials per relation (left) and log mean S (right) against log H. Source `results_deepdive.json`, `mechanism`.
  ]
)

#figure([#image("figures/fig6_IC_model_means_significant_curves.png")],
  caption: [
    fig6: IC model means (+-1.96 SE, n \= 64 rows each) on the five frozen-significant curves and V6:98; brackets under the model index give H.
  ]
)

== 10. Reconciling with Q1’s curve-effect result (question 10)
<reconciling-with-q1s-curve-effect-result-question-10>
#emph[Exploratory, post hoc. n \= 648 V6 vertices, each with its D curve and two ctl re-scalings (three isomorphic models, the same instances); rows per model 8 to 56 depending on level.]

The two tests differ in design and in what varies. Q1’s curve-effect test asks whether the D curves of different vertices spread more (variance of the D means within level) than the three isomorphic copies of one vertex (within-vertex variance of {D, ctl1, ctl2}); the ratio was F \= 0.99 for IC (p 0.42), and erratum F6 notes that its per-curve flags cannot fire at 648 curves (smallest attainable Holm p about 0.33). H-M asks whether the 8 models of one curve differ. They address different questions: Q1 compares #emph[curves] against a yardstick built from #emph[models], and that yardstick (variance of the three model means, 196.4 for IC) already contains whatever model effect exists. H-M measures the yardstick itself.

Q1’s data can be asked the H-M question directly: within each vertex, shuffle the three model labels (unpaired, and inside (target, round) blocks), pooled over vertices:

#align(center)[#table(
  columns: 6,
  align: (col, row) => (auto,right,right,right,auto,auto,).at(col),
  inset: 6pt,
  [arm], [pooled F (3 models within vertex)], [permutation p, unpaired], [blocked], [between-model CV (95% CI, vertex bootstrap)], [within-vertex variance of model means, observed vs row-noise expectation],
  [ic],
  [1.36],
  [0.0005],
  [0.0005],
  [8.8% \[7.2, 10.2\]],
  [196.4 vs 148.1],
  [rho-neg],
  [0.99],
  [0.6597],
  [0.5997],
  [0.0% \[0.0, 3.8\]],
  [0.03958 vs 0.04044],
  [bsgs],
  [0 (identical across models)],
  [1.0],
  [1.0],
  [0],
  [0],
)
]

On Q1’s own data the IC model effect is present (p at the 2,000-shuffle floor, 5e-4), with a between-model CV of 8.8% \[7.2, 10.2\], in line with the 7.2% estimated from the 12 M curves; rho-neg shows none (F 0.99). Of the within-vertex variance of IC model means (196.4), about 148 is row noise and the remaining 48 is model effect; the between-vertex variance of D means was 194.4, which is the same size, so #strong[vertex-to-vertex differences in IC cost are not distinguishable from the model effect plus row noise]. That is what Q1 reports, and it is consistent with H-M: no contradiction. The reading to avoid is 'a curve does not differ from its re-scalings', which suggests the re-scalings are interchangeable; for IC they are not (CV about 9%), and Q1 could not see it because it was built to detect curve effects, not model effects. Q1’s rho-neg and bsgs conclusions are unaffected.

#figure([#image("figures/fig10_model_effect_by_arm.png")],
  caption: [
    fig10: between-model CV by arm, V6 Q1 data (3 models, 648 vertices) and H-M (8 models, 12 curves). Source `results_deepdive.json`, `V6_model_effect` and `H-M.variance_components`.
  ]
)

== 11. Power for a follow-up (question 11)
<power-for-a-follow-up-question-11>
#emph[Exploratory, post hoc design recommendation, not a result. 1000 simulated studies per point, seed 20261106.]

Simulation: for each of the 12 curves, true model means are the grand mean plus the observed deviations shrunk by sqrt(max(0, 1 - 1/F)) (so that the simulated between-model mean square matches the observed one in expectation), each model’s 64 observed rows are rescaled to its true mean and resampled with replacement n times, parametric F(7, 8n-8) p-values, Holm over the 12 curves, outcome \= ic significant on at least 6 of 12. 'Observed' uses the unshrunk means (optimistic); 'null' sets all model means equal.

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,right,right,right,right,).at(col),
  inset: 6pt,
  [rows per model], [P(\>\= 6 of 12), shrunk], [mean significant, shrunk], [P(\>\= 6), observed means], [P(\>\= 6), null],
  [64],
  [0.340],
  [5.0],
  [0.949],
  [0.000],
  [80],
  [0.667],
  [6.2],
  [0.998],
  [0.000],
  [96],
  [0.881],
  [7.1],
  [0.999],
  [0.000],
  [112],
  [0.963],
  [7.9],
  [1.000],
  [0.000],
  [128],
  [0.987],
  [8.4],
  [1.000],
  [0.000],
  [192],
  [1.000],
  [9.5],
  [1.000],
  [0.000],
  [256],
  [1.000],
  [9.9],
  [1.000],
  [0.000],
  [384],
  [1.000],
  [10.1],
  [1.000],
  [0.000],
  [512],
  [1.000],
  [10.0],
  [1.000],
  [0.000],
)
]

At the 64 rows used, the shrunk scenario has power 0.34 for \>\= 6 of 12 and expects 5.0 significant curves, which matches the observed 5 or 6: the study sat on the boundary because its power for the 6-of-12 rule was about one third. #strong[96 rows per model] (for example 16 targets x 6 rounds) reaches 80% under the shrunk truth; 64 under the optimistic one. The null scenario never reaches 6 (mean 0.05 false positives of 12), so the rule keeps its size. A follow-up could also fix H (section 9) beforehand and test the yield prediction directly, which needs far fewer rows.

#figure([#image("figures/fig9_power.png")],
  caption: [
    fig9: power of the preregistered H-M rule against rows per model. Source `results_deepdive.json`, `H-M_power`.
  ]
)

== Limits
<limits>
- Everything here is exploratory and post hoc; the analyses were chosen after the frozen results were known, several tests are uncorrected across this document, and no number changes a verdict. The H-M pooled test and the yield mechanism in particular are hypotheses for a follow-up.
- Section 2: the additive part and the canonicalisation proxy are read from recorded counters; their meaning (setup, or something else) is not derivable from them. Canonicalisations are assumed to count walk iterations.
- Section 3: per-vertex SEs for Q come from percentile CI widths and are approximate; 8 targets per vertex.
- Section 4: the extrapolation is a model with stated assumptions; the existence and path-finding cost of deep j \= 0 volcanoes at cryptographic size are not modelled.
- Section 7: permutation p-values have Monte Carlo error (stated); the frozen verdict was computed with one stream and is kept.
- Section 9: H is a derived predictor with weights set by counting elements, chosen after the relation counters were seen; the factor base is rebuilt in Python (checked against the 96 recorded scan counts) and is not an ecbench measurement; relation counters come from the M archives only.
- Section 10: Q1’s rows per model are small (8 to 56); the pooled F uses all 648 vertices and the 2,000-shuffle permutation has a floor of 5e-4.
- Section 11: the simulation resamples the 64 observed rows per model, so it cannot be heavier-tailed than the data; it assumes the 12 curves are representative.
- One shared VM; validator running concurrently; seeds fixed at 20261106.

== Files
<files>
`deepdive.py` (reads only committed inputs, writes `results_deepdive.json` with the sha256 of every input), `build_md.py` (lays this file out from the JSON), `build_pdf.sh`, `figures/` (SVG + PNG), `DEEPDIVE.md`, `deepdive.pdf`. Copies of the PDF, SVGs and PNGs are in `benchmarks/20261106-volcano-HT-HM-deepdive/` (no `benchmarks/README.md` row: exploratory, no validation). Run `python3 deepdive.py` (about 8 minutes on 4 vCPUs), then `python3 build_md.py` and `./build_pdf.sh`.
