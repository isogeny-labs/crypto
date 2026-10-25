| session | arm | method | curve | log2 r | verified | mean S | 95% interval | theory S | S / theory | S / floor | S / reference | lower bound | med wall ms | levels |
|---|---|---|---|---:|---:|---:|---|---:|---:|---:|---:|---|---:|---|
| ECBS1h0c4c69816435 | rho-neg | rho.negation | icv1-fp32-t7319-90da43e9 | 32.00 | 8/8 | 1.014 | [0.549, 1.551] | 0.886 | 1.144 | 1.144 | 1.000 | yes | 15.654 | L2×8 |
| ECBS1h0c4c69816435 | dense-16384 | ic.pipeline | icv1-fp32-t7319-90da43e9 | 32.00 | 8/8 | 4109.036 | [4107.755, 4110.395] | – | – | 4636.550 | 4053.940 | yes | 218588.176 | L1×8 |
| ECBS1h0c4c69816435 | cado-16384 | ic.pipeline | icv1-fp32-t7319-90da43e9 | 32.00 | 8/8 | 4109.036 | [4107.755, 4110.395] | – | – | 4636.550 | 4053.940 | yes | 198547.700 | L1×8 |
