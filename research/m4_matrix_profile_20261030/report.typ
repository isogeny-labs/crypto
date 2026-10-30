#set page(margin: 1.8cm)
#set text(size: 9.5pt)
= m = 4 Macaulay matrix: fitted w and the repriced degree-5 row
#strong[Status.] Offline analysis of 1561 recorded closure rows; the new matrix-profile runs are pending the owner's S3 gate (upload refused by the session network policy). No new solver, no speed claim; rho stays 2^60.81.

#figure(image("w_fit_and_reprice.svg"), caption: [Left: counted word ops vs columns, saturating closures, OLS with intercept per degree. Right: D=5 row in rho-step units vs w; points are the fitted (a, w) with bootstrap 95% intervals.])

== Fitted w (cell-clustered bootstrap, intercept included)
#table(columns: 5, align: (left, right, right, right, right), inset: 5pt,
  [block], [rows / cells], [log2 cols], [w], [95% CI],
  [degree 3, m=3+4], [495 / 37], [7.8 to 13.5], [2.00], [1.83 to 2.07],
  [degree 4, m=3+4], [367 / 18], [9.1 to 18.4], [2.54], [2.42 to 2.60],
  [degree 5, m=3+4], [345 / 12], [10.0 to 16.1], [2.83], [2.59 to 2.92],
  [pooled (degree intercepts)], [1207 / 37], [], [2.34], [2.16 to 2.40],
)
The local slope at the top of the range is 2.7 (degree 4) and 3.2 (degree 5): w rises toward the dense cubic regime. Degree 3 and degree 4 do not overlap.

== D = 5 row, m = 4, bits over rho
#table(columns: 3, align: (left, right, right), inset: 5pt,
  [price], [A: column touches], [B: rho steps],
  [w = 1 floor (existing +10.32)], [+10.32], [+6.95],
  [w = 1.80 (crosses +30)], [+38.45], [+30.02],
  [w = 2.00, a = 0], [+45.71], [+37.27],
  [degree-4 proxy (w 2.54)], [+60.65], [+52.21 (49.5 to 53.7)],
  [degree-5 fit (w 2.83)], [+69.89], [+61.45 (55.8 to 64.0)],
)
== Decision
Above +30 under every measured pricing: the (4,6) cell is a degree curiosity; do not rent memory for it. Structured elimination: the closure matrix has rank 0.6 to 0.75 of its columns (dense, no Krylov gain); the plain XL matrix has rank fraction 0.07 to 0.09 but needs at least one more degree (0 of 16 draws refute at 4). Estimate: a constant-factor lever, not w < 2; confirmation pending the profile runs.
