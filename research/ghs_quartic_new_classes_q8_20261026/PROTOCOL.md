# GF(8) class-C plane-quartic extension

Frozen 26 October 2026 before the new draw. Conductor T-5424.

The [previous GF(8) census](../ghs_binary_cubic_isogeny_20261026/REPORT.md)
identified four leading class-C source orders with explicit smooth
quartics. The exact source census leaves six other orders with no direct-B
model, subgroup cofactor at most four, and prime subgroup at least 127:
`478,482,524,548,554,556`. These orders had no quartic witness in the
previous 1,024-draw sample. Existing methods include
[Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf),
[Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf),
[Tian's cubic isogeny construction](https://arxiv.org/pdf/2012.07173),
and [Milio's Jacobian isogeny algorithms](https://www.ams.org/mcom/2020-89-323/S0025-5718-2019-03486-5/).
The first two handle the known direct route; the latter two motivate a
plane-quartic transfer search but do not by themselves give an evaluated
map for these ordinary binary sources.

## Frozen search

- Conway `k=GF(8)`, `k2=GF(64)`, `K=GF(512)` and the same 15 ordered
  quartic monomials as the previous screen.
- Read the prior 1,022-model normalized elliptic census with its source
  SHA-256. Assert the six selected order classes and their prime/cofactor
  gates from that census.
- Draw 20,000 monic quartics with coefficients `[1] + 14` independent
  uniform integers in `0..7`, seed `2026102642` from Python Random.
- Retain exact tallies at the successive gates `#C(k)=9`,
  `#C(k2)=65`, smoothness, integral `t=(513-#C(K))/3`, source-order
  match, and selected six-order match. Keep the first quartic witness
  per selected order, all coefficients and exact point counts.
- Independently recount `#C(k)`, `#C(k2)`, `#C(K)` for selected
  witnesses and recheck smoothness, Weil polynomial, subgroup prime,
  and absence of a direct-B source at the same order.

The output is an exact structural isogeny-class witness. It is not an
evaluated subgroup map, novelty proof, or complete GHS-vs-rho result.
