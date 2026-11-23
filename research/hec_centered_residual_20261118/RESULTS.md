# Completing the square in the quadratic residual

All monic quadratics over the four genus-three fields matched the reference on splitting, both roots, order and multiplicity. All 6,912 timed solves verified. Integrated report tuples matched. Median paired speedups exceeded one in both runs on 32 of 48 inputs.

| Genus | Faster twice | Prior / centered, first | Prior / centered, second | Original / centered, first | Original / centered, second |
|---|---:|---:|---:|---:|---:|
| 2 | 11/16 | 0.983–1.066 | 0.973–1.065 | 3.762–4.964 | 3.780–5.125 |
| 3 | 13/16 | 0.997–1.033 | 0.996–1.051 | 3.277–4.818 | 3.162–4.763 |
| 4 | 8/16 | 0.994–1.035 | 0.991–1.016 | 4.585–9.801 | 4.528–9.712 |

The quadratic arm multiplies once to square the center. The prior formula used four field multiplications on a split quadratic with distinct roots. On genus four the finite tail is at most linear and generator harvesting is disabled, so its timing dispersion provides a control for the code layout and shared host. No cell reached tenfold.
