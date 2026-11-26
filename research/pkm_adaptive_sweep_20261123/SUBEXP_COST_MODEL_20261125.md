# Why the degree-32 PKM lead is not subexponential — measured cost exponents

## The measured F4 wall

Same binary, same 13-bit instance (`20261185`), m=2 chain F4, degree bound 8,
one solver call per measurement. The factor base is the interval arm; the
`T = 7` point uses 68 abscissae against 18/26 at T=5/6, but the encoding
diagnostic showed base size changes the per-call cost by only ~2% at fixed T.

| T | Variables / equations | Solving degree | F4 multiply-adds per call | Ratio to previous T |
|---:|---:|---:|---:|---:|
| 5 | 10 / 11 | 6 | 127.4 M | — |
| 6 | 12 / 13 | 7 | 2,002.9 M | **15.72×** |
| 7 | 14 / 15 | 7 | 79,748 M | **39.82×** |

The wall is not a constant 15×: it steepens at T=7 (14 variables). The
isogeny arm is 32.0–32.15× cheaper at equal T (13–15 bits), a constant factor;
it does not change the growth in T.

## What the PKM construction requires

For m=2, the factor base must have `|V| ≈ p^{1/2}` for a constant relation
probability, i.e. `T ≈ 0.5 log2 p`. Substituting the measured per-call law:

| Field bits | Required T | Predicted F4 per call | Exponent in p | Rho exponent |
|---:|---:|---:|---:|---:|
| 32 | 16 | 5.1e25 MA | 2.67 | 0.50 |
| 59 | 29.5 | 2.0e47 MA | 2.66 | 0.50 |
| 192 | 96 | 5.1e153 MA | 2.66 | 0.50 |
| 256 | 128 | 8.2e204 MA | 2.66 | 0.50 |

Even the optimistic T=5→6 law gives exponent ≈2.0; the measured T=6→7 law
gives 2.66. A 32× constant cannot move an exponent above 2 below rho's 0.5.

At fixed T=5 the measured charged cost per cold solve grows 2.93× from 13 to
15 bits, an exponent of **0.78 per bit**, already above rho's 0.50 before any
T growth is charged. The current 13/14/15-bit panels are therefore below rho
in *ratio* only because rho is priced at a much larger constant at these tiny
sizes; they are not a T2 (smaller-exponent) result.

## What a breakthrough would have to supply

The missing object is not another constant-factor map. It is one of:

1. **A T-independent membership/elimination method** for the tower chain —
   cost per system `2^{o(T)}` (ideally `poly(T)`), so that raising T to
   `0.5 log2 p` does not cost `p^{2.6}`. The tower equations are triangular;
   a dedicated triangular/resultant elimination, or a recursive tree solver,
   is the concrete candidate. It must be benchmarked against the T=5/6/7 F4
   baseline (127.4 M / 2.00 B / 79.7 B multiply-adds per call).
2. **A structured relation source or join** with `|V| > p^{1/2}` and a
   sub-`p^{1/2}` way to find `P1 + P2 = Q`. Enumerating V is `2^T = p^{1/2}`,
   so a faster join must exploit the tower structure, not the base as a set.
3. **An m ≥ 3 construction** with a factor base of size `p^{1/m}` and a solver
   whose cost is subexponential in the resulting variable count. The current
   F4 wall makes this worse, not better.

Without (1) or (2), the degree-32 component lead is a well-measured constant
factor on an exponential path that is steeper than rho. It cannot become
subexponential by tuning seeds, bases, or field sizes.

## Claim boundary

This is a cost-model fit plus one new scaling probe (`T=7`, one solver call,
`aux_vars_t7_int68_20261185.json`). It is not a T2 or T3 claim, and measurement
cannot establish T3. The T=7 probe is a single call at 68 abscissae; the
fixed-T base-size insensitivity is taken from the encoding diagnostic, not
re-measured here. No statement about a deployed curve is made.
