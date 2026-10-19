| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-t64801-082475e4 | 31.08 | ic-a2-1024 | 4/4 | 34.160 | 36.584 | 58.405 | — |
| icv1-fp32-t64801-082475e4 | 31.08 | ic-a2-128 | 4/4 | 95.187 | 114.135 | 284.668 | — |
| icv1-fp32-t64801-082475e4 | 31.08 | ic-a2-16 | 4/4 | 1047.515 | 1255.919 | 3131.555 | — |
| icv1-fp32-t64801-082475e4 | 31.08 | ic-a2-256 | 4/4 | 50.669 | 60.514 | 149.123 | — |
| icv1-fp32-t64801-082475e4 | 31.08 | ic-a2-32 | 4/4 | 681.735 | 817.581 | 2040.190 | — |
| icv1-fp32-t64801-082475e4 | 31.08 | ic-a2-512 | 4/4 | 26.621 | 30.835 | 68.756 | — |
| icv1-fp32-t64801-082475e4 | 31.08 | ic-a2-64 | 4/4 | 196.288 | 235.473 | 588.142 | — |
| icv1-fp32-t64801-082475e4 | 31.08 | ic-a2-8 | 1/4 | 2016.791 | 2416.183 | 6010.709 | — |
| icv1-fp32-t64801-082475e4 | 31.08 | rho-neg | 4/4 | 0.751 | 0.825 | 1.494 | — |
| icv1-fp32-t55395-81841389 | 31.62 | ic-a2-1024 | 4/4 | 34.862 | 38.188 | 68.125 | — |
| icv1-fp32-t55395-81841389 | 31.62 | ic-a2-128 | 4/4 | 113.926 | 136.633 | 340.998 | — |
| icv1-fp32-t55395-81841389 | 31.62 | ic-a2-16 | 4/4 | 1187.048 | 1423.250 | 3549.068 | — |
| icv1-fp32-t55395-81841389 | 31.62 | ic-a2-256 | 4/4 | 60.289 | 72.107 | 178.472 | — |
| icv1-fp32-t55395-81841389 | 31.62 | ic-a2-32 | 4/4 | 660.767 | 792.553 | 1978.628 | — |
| icv1-fp32-t55395-81841389 | 31.62 | ic-a2-512 | 4/4 | 33.963 | 39.836 | 92.695 | — |
| icv1-fp32-t55395-81841389 | 31.62 | ic-a2-64 | 4/4 | 301.127 | 361.251 | 902.372 | — |
| icv1-fp32-t55395-81841389 | 31.62 | ic-a2-8 | 0/4 | — | — | — | — |
| icv1-fp32-t55395-81841389 | 31.62 | rho-neg | 4/4 | 1.414 | 1.554 | 2.820 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-t64801-082475e4 | ic-a2-1024 vs ic-a2-128 | ic-a2-1024 < ic-a2-128 |
| icv1-fp32-t64801-082475e4 | ic-a2-1024 vs ic-a2-16 | ic-a2-1024 < ic-a2-16 |
| icv1-fp32-t64801-082475e4 | ic-a2-1024 vs ic-a2-256 | ic-a2-1024 < ic-a2-256 |
| icv1-fp32-t64801-082475e4 | ic-a2-1024 vs ic-a2-32 | ic-a2-1024 < ic-a2-32 |
| icv1-fp32-t64801-082475e4 | ic-a2-1024 vs ic-a2-512 | unresolved inside the band |
| icv1-fp32-t64801-082475e4 | ic-a2-1024 vs ic-a2-64 | ic-a2-1024 < ic-a2-64 |
| icv1-fp32-t64801-082475e4 | ic-a2-1024 vs ic-a2-8 | incomplete |
| icv1-fp32-t64801-082475e4 | ic-a2-1024 vs rho-neg | rho-neg < ic-a2-1024 |
| icv1-fp32-t64801-082475e4 | ic-a2-128 vs ic-a2-16 | ic-a2-128 < ic-a2-16 |
| icv1-fp32-t64801-082475e4 | ic-a2-128 vs ic-a2-256 | ic-a2-256 < ic-a2-128 |
| icv1-fp32-t64801-082475e4 | ic-a2-128 vs ic-a2-32 | ic-a2-128 < ic-a2-32 |
| icv1-fp32-t64801-082475e4 | ic-a2-128 vs ic-a2-512 | ic-a2-512 < ic-a2-128 |
| icv1-fp32-t64801-082475e4 | ic-a2-128 vs ic-a2-64 | ic-a2-128 < ic-a2-64 |
| icv1-fp32-t64801-082475e4 | ic-a2-128 vs ic-a2-8 | incomplete |
| icv1-fp32-t64801-082475e4 | ic-a2-128 vs rho-neg | rho-neg < ic-a2-128 |
| icv1-fp32-t64801-082475e4 | ic-a2-16 vs ic-a2-256 | ic-a2-256 < ic-a2-16 |
| icv1-fp32-t64801-082475e4 | ic-a2-16 vs ic-a2-32 | ic-a2-32 < ic-a2-16 |
| icv1-fp32-t64801-082475e4 | ic-a2-16 vs ic-a2-512 | ic-a2-512 < ic-a2-16 |
| icv1-fp32-t64801-082475e4 | ic-a2-16 vs ic-a2-64 | ic-a2-64 < ic-a2-16 |
| icv1-fp32-t64801-082475e4 | ic-a2-16 vs ic-a2-8 | incomplete |
| icv1-fp32-t64801-082475e4 | ic-a2-16 vs rho-neg | rho-neg < ic-a2-16 |
| icv1-fp32-t64801-082475e4 | ic-a2-256 vs ic-a2-32 | ic-a2-256 < ic-a2-32 |
| icv1-fp32-t64801-082475e4 | ic-a2-256 vs ic-a2-512 | ic-a2-512 < ic-a2-256 |
| icv1-fp32-t64801-082475e4 | ic-a2-256 vs ic-a2-64 | ic-a2-256 < ic-a2-64 |
| icv1-fp32-t64801-082475e4 | ic-a2-256 vs ic-a2-8 | incomplete |
| icv1-fp32-t64801-082475e4 | ic-a2-256 vs rho-neg | rho-neg < ic-a2-256 |
| icv1-fp32-t64801-082475e4 | ic-a2-32 vs ic-a2-512 | ic-a2-512 < ic-a2-32 |
| icv1-fp32-t64801-082475e4 | ic-a2-32 vs ic-a2-64 | ic-a2-64 < ic-a2-32 |
| icv1-fp32-t64801-082475e4 | ic-a2-32 vs ic-a2-8 | incomplete |
| icv1-fp32-t64801-082475e4 | ic-a2-32 vs rho-neg | rho-neg < ic-a2-32 |
| icv1-fp32-t64801-082475e4 | ic-a2-512 vs ic-a2-64 | ic-a2-512 < ic-a2-64 |
| icv1-fp32-t64801-082475e4 | ic-a2-512 vs ic-a2-8 | incomplete |
| icv1-fp32-t64801-082475e4 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-t64801-082475e4 | ic-a2-64 vs ic-a2-8 | incomplete |
| icv1-fp32-t64801-082475e4 | ic-a2-64 vs rho-neg | rho-neg < ic-a2-64 |
| icv1-fp32-t64801-082475e4 | ic-a2-8 vs rho-neg | incomplete |
| icv1-fp32-t55395-81841389 | ic-a2-1024 vs ic-a2-128 | ic-a2-1024 < ic-a2-128 |
| icv1-fp32-t55395-81841389 | ic-a2-1024 vs ic-a2-16 | ic-a2-1024 < ic-a2-16 |
| icv1-fp32-t55395-81841389 | ic-a2-1024 vs ic-a2-256 | ic-a2-1024 < ic-a2-256 |
| icv1-fp32-t55395-81841389 | ic-a2-1024 vs ic-a2-32 | ic-a2-1024 < ic-a2-32 |
| icv1-fp32-t55395-81841389 | ic-a2-1024 vs ic-a2-512 | unresolved inside the band |
| icv1-fp32-t55395-81841389 | ic-a2-1024 vs ic-a2-64 | ic-a2-1024 < ic-a2-64 |
| icv1-fp32-t55395-81841389 | ic-a2-1024 vs ic-a2-8 | incomplete |
| icv1-fp32-t55395-81841389 | ic-a2-1024 vs rho-neg | rho-neg < ic-a2-1024 |
| icv1-fp32-t55395-81841389 | ic-a2-128 vs ic-a2-16 | ic-a2-128 < ic-a2-16 |
| icv1-fp32-t55395-81841389 | ic-a2-128 vs ic-a2-256 | ic-a2-256 < ic-a2-128 |
| icv1-fp32-t55395-81841389 | ic-a2-128 vs ic-a2-32 | ic-a2-128 < ic-a2-32 |
| icv1-fp32-t55395-81841389 | ic-a2-128 vs ic-a2-512 | ic-a2-512 < ic-a2-128 |
| icv1-fp32-t55395-81841389 | ic-a2-128 vs ic-a2-64 | ic-a2-128 < ic-a2-64 |
| icv1-fp32-t55395-81841389 | ic-a2-128 vs ic-a2-8 | incomplete |
| icv1-fp32-t55395-81841389 | ic-a2-128 vs rho-neg | rho-neg < ic-a2-128 |
| icv1-fp32-t55395-81841389 | ic-a2-16 vs ic-a2-256 | ic-a2-256 < ic-a2-16 |
| icv1-fp32-t55395-81841389 | ic-a2-16 vs ic-a2-32 | ic-a2-32 < ic-a2-16 |
| icv1-fp32-t55395-81841389 | ic-a2-16 vs ic-a2-512 | ic-a2-512 < ic-a2-16 |
| icv1-fp32-t55395-81841389 | ic-a2-16 vs ic-a2-64 | ic-a2-64 < ic-a2-16 |
| icv1-fp32-t55395-81841389 | ic-a2-16 vs ic-a2-8 | incomplete |
| icv1-fp32-t55395-81841389 | ic-a2-16 vs rho-neg | rho-neg < ic-a2-16 |
| icv1-fp32-t55395-81841389 | ic-a2-256 vs ic-a2-32 | ic-a2-256 < ic-a2-32 |
| icv1-fp32-t55395-81841389 | ic-a2-256 vs ic-a2-512 | ic-a2-512 < ic-a2-256 |
| icv1-fp32-t55395-81841389 | ic-a2-256 vs ic-a2-64 | ic-a2-256 < ic-a2-64 |
| icv1-fp32-t55395-81841389 | ic-a2-256 vs ic-a2-8 | incomplete |
| icv1-fp32-t55395-81841389 | ic-a2-256 vs rho-neg | rho-neg < ic-a2-256 |
| icv1-fp32-t55395-81841389 | ic-a2-32 vs ic-a2-512 | ic-a2-512 < ic-a2-32 |
| icv1-fp32-t55395-81841389 | ic-a2-32 vs ic-a2-64 | ic-a2-64 < ic-a2-32 |
| icv1-fp32-t55395-81841389 | ic-a2-32 vs ic-a2-8 | incomplete |
| icv1-fp32-t55395-81841389 | ic-a2-32 vs rho-neg | rho-neg < ic-a2-32 |
| icv1-fp32-t55395-81841389 | ic-a2-512 vs ic-a2-64 | ic-a2-512 < ic-a2-64 |
| icv1-fp32-t55395-81841389 | ic-a2-512 vs ic-a2-8 | incomplete |
| icv1-fp32-t55395-81841389 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-t55395-81841389 | ic-a2-64 vs ic-a2-8 | incomplete |
| icv1-fp32-t55395-81841389 | ic-a2-64 vs rho-neg | rho-neg < ic-a2-64 |
| icv1-fp32-t55395-81841389 | ic-a2-8 vs rho-neg | incomplete |
