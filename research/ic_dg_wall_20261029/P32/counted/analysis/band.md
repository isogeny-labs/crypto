| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-t16357-dd50743c | 31.40 | dg | 144/144 | 32.681 | 33.990 | 45.773 | — |
| icv1-fp32-t16357-dd50743c | 31.40 | rho-neg | 144/144 | 0.925 | 1.017 | 1.843 | — |
| icv1-fp32-t16357-dd50743c | 31.40 | rho-neg-aa | 144/144 | 0.925 | 1.017 | 1.843 | — |
| icv1-fp32-t16357-dd50743c | 31.40 | srenew | 144/144 | 32.681 | 35.280 | 58.672 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | dg | 144/144 | 32.136 | 33.628 | 47.052 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | rho-neg | 144/144 | 0.901 | 0.990 | 1.795 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | rho-neg-aa | 144/144 | 0.901 | 0.990 | 1.795 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | srenew | 144/144 | 32.136 | 35.102 | 61.795 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-t16357-dd50743c | dg vs rho-neg | rho-neg < dg |
| icv1-fp32-t16357-dd50743c | dg vs rho-neg-aa | rho-neg-aa < dg |
| icv1-fp32-t16357-dd50743c | dg vs srenew | unresolved inside the band |
| icv1-fp32-t16357-dd50743c | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-t16357-dd50743c | rho-neg vs srenew | rho-neg < srenew |
| icv1-fp32-t16357-dd50743c | rho-neg-aa vs srenew | rho-neg-aa < srenew |
| icv1-fp32-tm94829-d5c5f997 | dg vs rho-neg | rho-neg < dg |
| icv1-fp32-tm94829-d5c5f997 | dg vs rho-neg-aa | rho-neg-aa < dg |
| icv1-fp32-tm94829-d5c5f997 | dg vs srenew | unresolved inside the band |
| icv1-fp32-tm94829-d5c5f997 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-tm94829-d5c5f997 | rho-neg vs srenew | rho-neg < srenew |
| icv1-fp32-tm94829-d5c5f997 | rho-neg-aa vs srenew | rho-neg-aa < srenew |
