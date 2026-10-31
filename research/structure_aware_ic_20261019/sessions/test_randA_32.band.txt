| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-tm78429-4fdf9409 | 31.40 | ic-a2-512 | 8/8 | 30.198 | 35.244 | 80.652 | — |
| icv1-fp32-tm78429-4fdf9409 | 31.40 | rho-neg | 8/8 | 1.394 | 1.533 | 2.781 | — |
| icv1-fp32-tm78429-4fdf9409 | 31.40 | rho-plain | 8/8 | 0.948 | 0.948 | 0.948 | — |
| icv1-fp32-tm22941-db72b9ad | 31.46 | ic-a2-512 | 8/8 | 35.915 | 42.127 | 98.029 | — |
| icv1-fp32-tm22941-db72b9ad | 31.46 | rho-neg | 8/8 | 1.074 | 1.180 | 2.140 | — |
| icv1-fp32-tm22941-db72b9ad | 31.46 | rho-plain | 8/8 | 1.723 | 1.723 | 1.723 | — |
| icv1-fp32-tm94979-7274dc6c | 31.47 | ic-a2-512 | 8/8 | 33.473 | 39.198 | 90.725 | — |
| icv1-fp32-tm94979-7274dc6c | 31.47 | rho-neg | 8/8 | 0.877 | 0.964 | 1.748 | — |
| icv1-fp32-tm94979-7274dc6c | 31.47 | rho-plain | 8/8 | 1.360 | 1.360 | 1.360 | — |
| icv1-fp32-tm2129-e61a7cac | 31.78 | ic-a2-512 | 8/8 | 33.456 | 39.276 | 91.655 | — |
| icv1-fp32-tm2129-e61a7cac | 31.78 | rho-neg | 8/8 | 1.057 | 1.162 | 2.108 | — |
| icv1-fp32-tm2129-e61a7cac | 31.78 | rho-plain | 8/8 | 1.425 | 1.425 | 1.425 | — |
| icv1-fp32-tm13719-d64de000 | 31.78 | ic-a2-512 | 8/8 | 33.924 | 39.840 | 93.084 | — |
| icv1-fp32-tm13719-d64de000 | 31.78 | rho-neg | 8/8 | 1.116 | 1.226 | 2.224 | — |
| icv1-fp32-tm13719-d64de000 | 31.78 | rho-plain | 8/8 | 1.233 | 1.233 | 1.233 | — |
| icv1-fp32-t9261-25219a93 | 31.95 | ic-a2-512 | 8/8 | 35.704 | 42.025 | 98.908 | — |
| icv1-fp32-t9261-25219a93 | 31.95 | rho-neg | 8/8 | 0.916 | 1.007 | 1.826 | — |
| icv1-fp32-t9261-25219a93 | 31.95 | rho-plain | 8/8 | 1.520 | 1.520 | 1.520 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-tm78429-4fdf9409 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-tm78429-4fdf9409 | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-tm78429-4fdf9409 | rho-neg vs rho-plain | rho-plain < rho-neg |
| icv1-fp32-tm22941-db72b9ad | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-tm22941-db72b9ad | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-tm22941-db72b9ad | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-tm94979-7274dc6c | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-tm94979-7274dc6c | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-tm94979-7274dc6c | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-tm2129-e61a7cac | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-tm2129-e61a7cac | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-tm2129-e61a7cac | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-tm13719-d64de000 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-tm13719-d64de000 | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-tm13719-d64de000 | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-t9261-25219a93 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-t9261-25219a93 | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-t9261-25219a93 | rho-neg vs rho-plain | unresolved inside the band |
