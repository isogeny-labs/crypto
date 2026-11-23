# Tables (kappa at genus 3) [pcc:unverified]

Single host, single thread, wall clock, no PMU. Medians of the trials; A/A = |median(even) - median(odd)| / median.


## p = 65537  (r = 35204231349877, 46 bits, mulmod branch: f64-Barrett; 16 trials)

IC run: trials 582790, la_mul 52,122,811 (wiedemann 52,122,807), la_add 553,133,264, core 3181x3110, nnz after SGE 44,638; cantor_mul 132,214,399, cantor_inv 3,008,739. Load average (1 min) over trials: 0.34 to 0.94.

| quantity | median ns | min | max | A/A spread |
|---|---|---|---|---|
| t_F in situ (per fmul+10 finv, Cantor) | 5.554 | 4.332 | 6.758 | 1.8% |
| t_F bare, throughput | 1.895 | 1.467 | 2.085 | 2.1% |
| t_F bare, dependent chain | 5.619 | 4.394 | 5.992 | 1.1% |
| t_R in situ (LA wall / la_mul) | 99.63 | 90.42 | 107.2 | 2.3% |
| t_R bare mulmod+addm | 4.257 | 3.304 | 6.692 | 0.2% |
| t_R bare mulmod, throughput | 4.904 | 3.823 | 6.722 | 0.7% |
| t_R bare mulmod, dependent chain | 16 | 12.46 | 18.29 | 0.2% |
| LA wall / la_add (info) | 9.388 | 8.521 | 10.1 | 2.3% |
| bare F_p add | 1.257 | 0.9817 | 1.332 | 0.4% |
| bare mod-r add | 1.25 | 0.9784 | 1.307 | 0.1% |
| LA wall total (ns) | 5.193e+09 | 4.713e+09 | 5.589e+09 | 2.3% |

| kappa | value |
|---|---|
| in situ: t_R / t_F | 17.9 (per-trial 15.6 to 23.7; A/A 4.5%) |
| bare matched: (mulmod+addm) / F_p mul throughput | 2.25 |
| bare mulmod throughput / F_p mul throughput | 2.59 |
| bare dependent chains | 2.85 |
| in-situ t_R over bare F_p mul throughput | 52.6 |
| add: mod-r / F_p | 0.994 |

## p = 131101  (r = 562757240242147, 49 bits, mulmod branch: f64-Barrett; 16 trials)

IC run: trials 1397302, la_mul 148,300,903 (wiedemann 148,300,901), la_add 1,639,444,399, core 5115x5018, nnz after SGE 77,861; cantor_mul 317,779,886, cantor_inv 7,229,952. Load average (1 min) over trials: 0.99 to 1.06.

| quantity | median ns | min | max | A/A spread |
|---|---|---|---|---|
| t_F in situ (per fmul+10 finv, Cantor) | 6.76 | 5.25 | 8.615 | 10.6% |
| t_F bare, throughput | 2.39 | 1.871 | 2.574 | 7.5% |
| t_F bare, dependent chain | 5.977 | 4.715 | 6.275 | 5.7% |
| t_R in situ (LA wall / la_mul) | 114.9 | 107 | 125 | 0.5% |
| t_R bare mulmod+addm | 4.245 | 3.848 | 8.155 | 8.6% |
| t_R bare mulmod, throughput | 4.869 | 4.059 | 8.165 | 3.0% |
| t_R bare mulmod, dependent chain | 15.94 | 12.55 | 18.74 | 1.0% |
| LA wall / la_add (info) | 10.39 | 9.675 | 11.31 | 0.5% |
| bare F_p add | 1.265 | 0.9999 | 1.359 | 1.1% |
| bare mod-r add | 1.262 | 1.003 | 1.357 | 1.2% |
| LA wall total (ns) | 1.704e+10 | 1.586e+10 | 1.854e+10 | 0.5% |

| kappa | value |
|---|---|
| in situ: t_R / t_F | 17 (per-trial 13.6 to 22.8; A/A 10.3%) |
| bare matched: (mulmod+addm) / F_p mul throughput | 1.78 |
| bare mulmod throughput / F_p mul throughput | 2.04 |
| bare dependent chains | 2.67 |
| in-situ t_R over bare F_p mul throughput | 48.1 |
| add: mod-r / F_p | 0.998 |

## p = 262147  (r = 4511942408507159, 53 bits, mulmod branch: u128 %; 16 trials)

IC run: trials 3373332, la_mul 426,192,389 (wiedemann 426,192,385), la_add 4,951,428,437, core 8162x8037, nnz after SGE 138,100; cantor_mul 768,769,059, cantor_inv 17,487,509. Load average (1 min) over trials: 1.00 to 1.08.

| quantity | median ns | min | max | A/A spread |
|---|---|---|---|---|
| t_F in situ (per fmul+10 finv, Cantor) | 6.792 | 5.292 | 8.701 | 6.5% |
| t_F bare, throughput | 2.872 | 2.226 | 3.027 | 0.8% |
| t_F bare, dependent chain | 6.396 | 5 | 6.72 | 0.5% |
| t_R in situ (LA wall / la_mul) | 107.1 | 103.4 | 114.1 | 0.8% |
| t_R bare mulmod+addm | 5.159 | 3.76 | 6.563 | 3.4% |
| t_R bare mulmod, throughput | 3.266 | 2.517 | 4.122 | 8.9% |
| t_R bare mulmod, dependent chain | 6.893 | 5.496 | 7.145 | 0.7% |
| LA wall / la_add (info) | 9.222 | 8.901 | 9.818 | 0.8% |
| bare F_p add | 1.267 | 1.154 | 1.39 | 0.7% |
| bare mod-r add | 1.268 | 1.069 | 1.348 | 0.7% |
| LA wall total (ns) | 4.566e+10 | 4.407e+10 | 4.861e+10 | 0.8% |

| kappa | value |
|---|---|
| in situ: t_R / t_F | 15.8 (per-trial 12.7 to 19.9; A/A 6.2%) |
| bare matched: (mulmod+addm) / F_p mul throughput | 1.8 |
| bare mulmod throughput / F_p mul throughput | 1.14 |
| bare dependent chains | 1.08 |
| in-situ t_R over bare F_p mul throughput | 37.3 |
| add: mod-r / F_p | 1 |
