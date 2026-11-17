#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Dense same-residual 2D oracle IC · p=37
Run ID: `run-20261022T0729Z-quartic-2d-dense-p37`. One target `[271]G`. 53,551 exact signed decompositions of the first residual; unanchored rank 52 of 53. The public generator log fixes one column, leaving 52 unknowns. 52 accepted rows from the first 115 decompositions gave full anchored rank and scalar 271. Independent signed-pair and all-log replay passed; non-timing A/A diff empty.

#table(columns: (1.5fr,1.4fr,1.4fr), inset: 4pt,
  [phase], [$F_p$ multiplications], [group operations],
  [model], [859], [0],
  [base], [2,567,679], [0],
  [input], [4,741], [46],
  [eigenvalues], [16,618], [156],
  [orbit map], [1,877,044], [17,524],
  [full pair table], [37,708,944], [388,752],
  [one-query stream], [75,560,753], [779,024],
  [answer checks], [2,004,307], [18,854],
)

Counted totals, previous → dense: 4,048,104,018 → 119,740,945 $F_p$ multiplications; 41,702,654 → 1,204,356 group operations; 53 → 1 residuals. Matrix row operations are a separate unit. This exact group-oracle result does not price polynomial relation finding, hash CPU or a matched rho band. Raw records, commands and source hashes are in `README.md`.

#image("dense_cost.png", width: 100%)
