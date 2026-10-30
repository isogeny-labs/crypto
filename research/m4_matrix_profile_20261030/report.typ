#set page(margin: 1.8cm)
#set text(size: 9.5pt)
= m = 4 Macaulay matrix: fitted w and the repriced degree-5 row
#strong[Status.] Fitted w from 1561 recorded closure rows, plus 34 new matrix-profile runs (m = 4, five cells, degrees 3 and 4). The profiler's word ops equal the recorded ones on 29 of 29 comparable rows. S3 gate run by the user (session network policy refused it). No new solver, no speed claim; rho stays 2^60.81.

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
== Matrix profile (degree 4, measured)
#figure(image("matrix_profile.svg"), caption: [Left: closure rank over columns (satisfiable 0.68 to 1.00; unsatisfiable collapse to rank 1) against plain XL rank (0.09 to 0.13). Middle: share of columns that are subspace-only monomials, by degree block. Right: Krylov cost of the plain XL matrix equals the dense Macaulay reduction and is 340 to 2500 times below the closure, which is what refutes.])
#table(columns: 6, align: (left, right, right, right, right, right), inset: 4pt,
  [cell (n,l,N)], [cols], [closure rank / cols], [XL rank / cols], [rho], [closure vs base ops],
  [(11,5,42)], [124314], [0.73 to 0.77], [0.087], [52 to 56], [3.2e11 vs 2.4e8],
  [(12,5,44)], [149905], [0.68 to 0.76], [0.086], [59 to 61], [2.9e11 vs 3.6e8],
  [(13,4,42)], [124314], [1.000], [0.102], [54 to 56], [6.1e11 vs 2.9e8],
  [(12,3,36)], [66712], [1.000], [0.131], [40], [5.1e10 vs 1.0e8],
)
Subspace-only columns are 22% of degree-2 columns and 1.6% to 4.3% of degree-4 columns: almost every column touches an intermediate-point coordinate. The closure is over 99.7% of the work.

== Structured elimination, priced at the real row (n = 131, N = 398)
#table(columns: 4, align: (left, right, right, right), inset: 5pt,
  [plain-XL degree D'], [Krylov word ops], [row, bits over rho (conv -8.4)], [no conversion],
  [6 (measured one-degree lag)], [2^78.0 to 2^78.7], [+42.8 to +43.5], [+51.2 to +51.9],
  [5 (needs XL = closure degree; 0 of 136 support it)], [2^64.8 to 2^65.5], [+29.6 to +30.3], [+38.0 to +38.7],
)
At the measured sizes the Krylov cost equals the dense Macaulay reduction (ratio 0.8 to 1.0); a structured solve lowers the row from +61.5 to about +43 through a constant, not an exponent below 2.

== Decision
Above +30 under every measured pricing, including the structured route (about +43): the (4,6) cell is a degree curiosity; do not rent memory for it. The Krylov route reaches the +30 line only if plain XL refuted at the closure's own degree, which 0 of 136 committed draws support.