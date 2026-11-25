# Aux-variable count drives F4 cost; the isogeny factor is a constant — 2026-11-25

**This document replaces an earlier version of the same file** whose headline
claimed the 32× was "the interval arm's bit-encoding". The prediction in that
version — that the ratio should step up by an order of magnitude once the
matched base crossed 2^5 — was tested and **failed**. What follows is the
corrected reading, with the failed prediction kept visible because the way it
failed is the informative part.

One binary (`8e654529…`, commit `cc7d6141f2`), one instance
(`icv1-fp13-tm13-9e2d11f6`, seed 20261185), `m = 2` chain F4, degree bound 8.

## The four measurements

| Run | Base | Aux vars / summand | MA/call | Max rows | nnz/row | D | Relations |
|:--|---:|---:|---:|---:|---:|---:|---:|
| interval, `bits=5` | 18 | 5 | 127.4M | 1493 | 189.9 | 6 | 219 |
| isogeny `t=5` | 18 | 5 | **3.97M** | 821 | 54.7 | 5 | 208 |
| interval, `bits=6` | 18 | 6 | 1956.8M | 6037 | 465.5 | 7 | 219 |
| interval, `bits=6` | 26 | 6 | 2002.9M | 5544 | 444.2 | 7 | 15 |
| isogeny `t=6` | 26 | 6 | **55.9M** | 1711 | 116.0 | 5 | 19 |

## What the numbers say

1. **Auxiliary-variable count sets the cost, base size does not.** At `bits=6`
   the interval arm costs 1956.8M per call on 18 abscissae and 2002.9M on 26 —
   a 2 % difference for a 44 % larger base. Going from 5 to 6 aux variables
   multiplies it by 15.4×. The isogeny arm moves the same way: 3.97M → 55.9M
   (14.1×) for the same +2 variables. Both systems are almost exactly as
   sensitive to aux count as each other.

2. **At equal aux count the isogeny is 32–36× cheaper, and that is the whole
   effect.** T = 5: 127.4/3.97 = 32.1×. T = 6: 2002.9/55.9 = **35.8×**. The
   factor is stable because the mechanism is stable: the interval's rows carry
   3.5–3.8× more non-zeros (189.9 vs 54.7; 444.2 vs 116.0) in a matrix with
   1.8–3.2× more rows, and resolves at one higher degree (6–7 vs 5).

3. **The failed prediction.** If the 32× were purely an encoding-width effect,
   a 26-point base paired with `t=6` should have shown ~10²–10³×. It showed
   35.8×, because raising the base size forced the isogeny's tower to `t = 6`
   too — the two arms scale together in aux count. The earlier doc's
   "encoding term" framing was too strong: encoding width matters, but it
   matters equally on both sides, so it cannot explain the ratio.

So the honest mechanism sentence is: *the isogeny chain and the bit-window
interval respond identically to auxiliary-variable count, and at equal count the
isogeny's Macaulay rows are ~3.5× sparser and its matrix smaller, which yields
a constant 32–36× per-system saving.* That is the "banded quadratic chain
versus dense bit-coupled S₃" reading from `INSTRUMENTATION_F4_DEGREE32_20261125.md`,
now with a matched-aux-count control behind it.

## Consequence for scaling

Because the ratio is set by matching T rather than by absolute base size, the
13-, 14- and 15-bit panels all landing at 32× is not a coincidence of window
width — they all used T = 5. A rung that needs T = 6 on both sides should land
near 36×, which is what this probe shows. The factor is therefore expected to
stay in the low tens, not to explode, as size grows.

Stores: `encoding_width_bits5_20261185.json`,
`encoding_width_bits6_20261185.json`, `aux_vars_t6_int26_20261185.json`,
`aux_vars_t6_cand26_20261185.json`.
