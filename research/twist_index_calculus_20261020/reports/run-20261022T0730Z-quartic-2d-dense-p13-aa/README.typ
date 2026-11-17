#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Dense same-residual 2D oracle IC · p=13 · A/A
Run ID: `run-20261022T0730Z-quartic-2d-dense-p13-aa`. One target `[271]G`. 561 exact signed decompositions of the first residual; unanchored rank 6 of 7. The public generator log fixes one column, leaving 6 unknowns. 6 accepted rows from the first 6 decompositions gave full anchored rank and scalar 271. Independent signed-pair and all-log replay passed; non-timing A/A diff empty.

#table(columns: (1.5fr,1.4fr,1.4fr), inset: 4pt,
  [phase], [$F_p$ multiplications], [group operations],
  [model], [650], [0],
  [base], [216,201], [0],
  [input], [3,560], [35],
  [eigenvalues], [6,478], [62],
  [orbit map], [149,262], [1,422],
  [full pair table], [495,864], [5,112],
  [one-query stream], [1,011,390], [10,430],
  [answer checks], [162,265], [1,548],
)

Counted totals, previous → dense: 8,096,696 → 2,045,670 $F_p$ multiplications; 81,076 → 18,609 group operations; 7 → 1 residuals. Matrix row operations are a separate unit. This exact group-oracle result does not price polynomial relation finding, hash CPU or a matched rho band. Raw records, commands and source hashes are in `README.md`.

#image("dense_cost.png", width: 100%)
