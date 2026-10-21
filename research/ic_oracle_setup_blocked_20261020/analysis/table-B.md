| session | arm | method | curve | log2 r | verified | mean S | 95% interval | theory S | S / theory | S / floor | S / reference | lower bound | med wall ms | levels |
|---|---|---|---|---:|---:|---:|---|---:|---:|---:|---:|---|---:|---|
| ECBS1h90cd7a0dc213 | rho-neg | rho.negation | icv1-fp32-t7319-90da43e9 | 32.00 | 8/8 | 0.802 | [0.415, 1.214] | 0.886 | 0.905 | 0.905 | 1.000 | yes | 24.832 | L1×1 L2×7 |
| ECBS1h90cd7a0dc213 | scalar-16384 | ic.pipeline | icv1-fp32-t7319-90da43e9 | 32.00 | 8/8 | 4097.226 | [4096.986, 4097.446] | – | – | 4623.224 | 5110.287 | yes | 228405.016 | L1×8 |
| ECBS1h90cd7a0dc213 | batch-16384 | ic.pipeline | icv1-fp32-t7319-90da43e9 | 32.00 | 8/8 | 4097.226 | [4096.986, 4097.446] | – | – | 4623.224 | 5110.287 | yes | 121240.274 | L1×8 |
| ECBS1h90cd7a0dc213 | blocked-16384 | ic.pipeline | icv1-fp32-t7319-90da43e9 | 32.00 | 8/8 | 4097.226 | [4096.986, 4097.446] | – | – | 4623.224 | 5110.287 | yes | 30808.836 | L1×8 |
