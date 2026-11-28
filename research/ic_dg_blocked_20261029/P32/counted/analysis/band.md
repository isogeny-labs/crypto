| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-tm43175-08b04f34 | 31.82 | dg | 144/144 | 31.823 | 33.312 | 46.714 | — |
| icv1-fp32-tm43175-08b04f34 | 31.82 | dgb | 144/144 | 31.823 | 33.312 | 46.714 | — |
| icv1-fp32-tm43175-08b04f34 | 31.82 | rho-neg | 144/144 | 0.912 | 1.003 | 1.818 | — |
| icv1-fp32-tm43175-08b04f34 | 31.82 | rho-neg-aa | 144/144 | 0.912 | 1.003 | 1.818 | — |
| icv1-fp32-tm81759-3ad5e8c5 | 31.84 | dg | 144/144 | 31.517 | 32.988 | 46.222 | — |
| icv1-fp32-tm81759-3ad5e8c5 | 31.84 | dgb | 144/144 | 31.517 | 32.988 | 46.222 | — |
| icv1-fp32-tm81759-3ad5e8c5 | 31.84 | rho-neg | 144/144 | 1.003 | 1.103 | 2.000 | — |
| icv1-fp32-tm81759-3ad5e8c5 | 31.84 | rho-neg-aa | 144/144 | 1.003 | 1.103 | 2.000 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-tm43175-08b04f34 | dg vs dgb | unresolved inside the band |
| icv1-fp32-tm43175-08b04f34 | dg vs rho-neg | rho-neg < dg |
| icv1-fp32-tm43175-08b04f34 | dg vs rho-neg-aa | rho-neg-aa < dg |
| icv1-fp32-tm43175-08b04f34 | dgb vs rho-neg | rho-neg < dgb |
| icv1-fp32-tm43175-08b04f34 | dgb vs rho-neg-aa | rho-neg-aa < dgb |
| icv1-fp32-tm43175-08b04f34 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-tm81759-3ad5e8c5 | dg vs dgb | unresolved inside the band |
| icv1-fp32-tm81759-3ad5e8c5 | dg vs rho-neg | rho-neg < dg |
| icv1-fp32-tm81759-3ad5e8c5 | dg vs rho-neg-aa | rho-neg-aa < dg |
| icv1-fp32-tm81759-3ad5e8c5 | dgb vs rho-neg | rho-neg < dgb |
| icv1-fp32-tm81759-3ad5e8c5 | dgb vs rho-neg-aa | rho-neg-aa < dgb |
| icv1-fp32-tm81759-3ad5e8c5 | rho-neg vs rho-neg-aa | unresolved inside the band |
