| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-t49501-bb10a7f0 | 31.44 | ic-a2-512 | 8/8 | 29.369 | 34.262 | 78.299 | — |
| icv1-fp32-t49501-bb10a7f0 | 31.44 | rho-neg | 8/8 | 1.060 | 1.165 | 2.112 | — |
| icv1-fp32-t49501-bb10a7f0 | 31.44 | rho-plain | 8/8 | 1.622 | 1.622 | 1.622 | — |
| icv1-fp32-tm93251-f4d28523 | 31.46 | ic-a2-512 | 8/8 | 30.795 | 35.980 | 82.641 | — |
| icv1-fp32-tm93251-f4d28523 | 31.46 | rho-neg | 8/8 | 0.904 | 0.993 | 1.800 | — |
| icv1-fp32-tm93251-f4d28523 | 31.46 | rho-plain | 8/8 | 1.290 | 1.290 | 1.290 | — |
| icv1-fp32-tm99915-f45b8f41 | 31.51 | ic-a2-512 | 8/8 | 33.901 | 39.727 | 92.160 | — |
| icv1-fp32-tm99915-f45b8f41 | 31.51 | rho-neg | 8/8 | 0.852 | 0.937 | 1.697 | — |
| icv1-fp32-tm99915-f45b8f41 | 31.51 | rho-plain | 8/8 | 1.219 | 1.219 | 1.219 | — |
| icv1-fp32-t81129-cd3541f0 | 31.56 | ic-a2-512 | 8/8 | 35.003 | 41.065 | 95.629 | — |
| icv1-fp32-t81129-cd3541f0 | 31.56 | rho-neg | 8/8 | 1.157 | 1.272 | 2.307 | — |
| icv1-fp32-t81129-cd3541f0 | 31.56 | rho-plain | 8/8 | 1.693 | 1.693 | 1.693 | — |
| icv1-fp32-tm36569-7e61bac9 | 31.75 | ic-a2-512 | 8/8 | 32.045 | 37.574 | 87.342 | — |
| icv1-fp32-tm36569-7e61bac9 | 31.75 | rho-neg | 8/8 | 0.963 | 1.059 | 1.920 | — |
| icv1-fp32-tm36569-7e61bac9 | 31.75 | rho-plain | 8/8 | 1.473 | 1.473 | 1.473 | — |
| icv1-fp32-t87547-b5676b0b | 31.80 | ic-a2-512 | 8/8 | 37.517 | 44.156 | 103.901 | — |
| icv1-fp32-t87547-b5676b0b | 31.80 | rho-neg | 8/8 | 0.935 | 1.028 | 1.864 | — |
| icv1-fp32-t87547-b5676b0b | 31.80 | rho-plain | 8/8 | 1.415 | 1.415 | 1.415 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-t49501-bb10a7f0 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-t49501-bb10a7f0 | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-t49501-bb10a7f0 | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-tm93251-f4d28523 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-tm93251-f4d28523 | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-tm93251-f4d28523 | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-tm99915-f45b8f41 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-tm99915-f45b8f41 | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-tm99915-f45b8f41 | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-t81129-cd3541f0 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-t81129-cd3541f0 | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-t81129-cd3541f0 | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-tm36569-7e61bac9 | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-tm36569-7e61bac9 | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-tm36569-7e61bac9 | rho-neg vs rho-plain | unresolved inside the band |
| icv1-fp32-t87547-b5676b0b | ic-a2-512 vs rho-neg | rho-neg < ic-a2-512 |
| icv1-fp32-t87547-b5676b0b | ic-a2-512 vs rho-plain | rho-plain < ic-a2-512 |
| icv1-fp32-t87547-b5676b0b | rho-neg vs rho-plain | unresolved inside the band |
