# Is the S_m chain F4-decidable at m >= 5?

Raw rows: `chain_diag.jsonl`. One system per cell, `p = 3907` (12-bit),
`prime-isogeny:t,seed=0,aux=ordinary`, `pkm-semaev:engine=f4,mode=chain`,
`--max-trials 1`, 60 s solver budget. `decided = (budget_exceeded == 0)`.

| m | t | vars m·t+(m−2) | \|V\| | degree bound | decided | D | max cols | max nnz | wall |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 5 | 2 | 13 | 4 | 8 | **yes** (refuted) | 7 | 5 551 | 2.0 M | 0.3 s |
| 4 | 3 | 14 | 8 | 8 | **yes** (refuted) | 7 | 9 454 | 33.0 M | 3.8 s |
| 5 | 3 | 18 | 8 | 8 | no | — | 85 442 | 344 M | 466 s |
| 5 | 3 | 18 | 8 | 10 | no | — | 65 051 | 40 M | 291 s |
| 5 | 3 | 18 | 8 | 12 | no | — | 85 442 | 84 M | 379 s |
| 6 | 3 | 22 | 8 | 8 | no | — | — | — | >1800 s |

Two findings.

**1. The boundary is the variable count, not `m`, and not the degree bound.**
`m = 5, t = 2` (13 variables) and `m = 4, t = 3` (14) are refuted at degree 7
in under 4 s. `m = 5, t = 3` (18) never decides — at degree bound 8, 10 or 12
alike, so it is not a bound setting — and the engine grinds through 40–344 M
non-zeros. `m = 6, t = 3` (22) did not finish one system in 30 min; in the
§6 run all 34 of its systems were `budget_exceeded`. Feasible here: ~14
variables. Not feasible: 18.

**2. At the paper's density condition the variable count is `log2 p`,
independently of `m`.** With `deg L = 2^t ≈ p^{1/m}`,

    t ≈ log2(p)/m   ⇒   m·t ≈ log2 p,

and the system carries `m·t` tower variables plus `m−2` chain auxiliaries. Our
own rows obey it: `m·t = 18` at `p = 2^18`, `12` at `p = 2^12`. The `m = 5, 6`
rungs of §6 sit at 15 and 18 variables on a 12-bit field only because `2^t`
cannot hit `p^{1/5} = 5.2` or `p^{1/6} = 4.0`; the nearest power of two
overshoots, and that overshoot alone is what pushed them past the ceiling.

So moving work from `t` to `m` does not shrink the system: it keeps it at
`log2 p` tower variables. For P-192 that is 192, against a measured ceiling of
~14. At `m = 6` the target degree `2^32` needs `t = 32`, i.e. 192 tower
variables.

**The one lever that does move the count is the smoothness bound `B`.** A
smooth divisor of `2^t·∏p_i` with primes up to `B` needs only
`n0 ≈ log2(p)/(m·log2 B)` isogeny levels, so

    m·n0 ≈ log2(p)/log2(B).

`B = 2` (the paper's Table 5, and every run in this record) gives `log2 p`;
`B = 13` gives `log2 p / 3.7 ≈ 52` for P-192; `B = 2^20` gives about 10. That
buys variables at the price of equation degree — each Vélu step is rational of
degree `p_i`, so the equations become degree `B` — and nobody, including the
paper, has measured `B > 2`. That is the axis the next experiment belongs on,
not `m`.
