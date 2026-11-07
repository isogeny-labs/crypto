| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-tm68303-f902d2f0 | 31.85 | dgb | 144/144 | 31.652 | 33.141 | 46.539 | — |
| icv1-fp32-tm68303-f902d2f0 | 31.85 | dgbf | 144/144 | 31.652 | 33.141 | 46.539 | — |
| icv1-fp32-tm68303-f902d2f0 | 31.85 | rho-neg | 144/144 | 0.935 | 1.028 | 1.864 | — |
| icv1-fp32-tm68303-f902d2f0 | 31.85 | rho-neg-aa | 144/144 | 0.935 | 1.028 | 1.864 | — |
| icv1-fp32-t89141-95879fe3 | 31.88 | dgb | 144/144 | 31.735 | 33.255 | 46.932 | — |
| icv1-fp32-t89141-95879fe3 | 31.88 | dgbf | 144/144 | 31.735 | 33.255 | 46.932 | — |
| icv1-fp32-t89141-95879fe3 | 31.88 | rho-neg | 144/144 | 1.079 | 1.186 | 2.152 | — |
| icv1-fp32-t89141-95879fe3 | 31.88 | rho-neg-aa | 144/144 | 1.079 | 1.186 | 2.152 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-tm68303-f902d2f0 | dgb vs dgbf | unresolved inside the band |
| icv1-fp32-tm68303-f902d2f0 | dgb vs rho-neg | rho-neg < dgb |
| icv1-fp32-tm68303-f902d2f0 | dgb vs rho-neg-aa | rho-neg-aa < dgb |
| icv1-fp32-tm68303-f902d2f0 | dgbf vs rho-neg | rho-neg < dgbf |
| icv1-fp32-tm68303-f902d2f0 | dgbf vs rho-neg-aa | rho-neg-aa < dgbf |
| icv1-fp32-tm68303-f902d2f0 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-t89141-95879fe3 | dgb vs dgbf | unresolved inside the band |
| icv1-fp32-t89141-95879fe3 | dgb vs rho-neg | rho-neg < dgb |
| icv1-fp32-t89141-95879fe3 | dgb vs rho-neg-aa | rho-neg-aa < dgb |
| icv1-fp32-t89141-95879fe3 | dgbf vs rho-neg | rho-neg < dgbf |
| icv1-fp32-t89141-95879fe3 | dgbf vs rho-neg-aa | rho-neg-aa < dgbf |
| icv1-fp32-t89141-95879fe3 | rho-neg vs rho-neg-aa | unresolved inside the band |
