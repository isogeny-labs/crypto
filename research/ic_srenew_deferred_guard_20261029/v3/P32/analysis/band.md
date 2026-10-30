| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-tm103307-fa421912 | 31.88 | dg | 144/144 | 31.635 | 33.145 | 46.732 | — |
| icv1-fp32-tm103307-fa421912 | 31.88 | dg-small | 144/144 | 29.831 | 31.950 | 51.025 | — |
| icv1-fp32-tm103307-fa421912 | 31.88 | rho-neg | 144/144 | 1.010 | 1.111 | 2.014 | — |
| icv1-fp32-tm103307-fa421912 | 31.88 | rho-neg-aa | 144/144 | 1.010 | 1.111 | 2.014 | — |
| icv1-fp32-tm103307-fa421912 | 31.88 | srenew | 144/144 | 31.635 | 34.638 | 61.668 | — |
| icv1-fp32-tm103307-fa421912 | 31.88 | srenew-small | 144/144 | 29.831 | 34.057 | 72.098 | — |
| icv1-fp32-tm103307-fa421912 | 31.88 | walk | 144/144 | 32.047 | 35.125 | 62.835 | — |
| icv1-fp32-tm103307-fa421912 | 31.88 | walk-aa | 144/144 | 32.047 | 35.125 | 62.835 | — |
| icv1-fp32-tm24839-2a7bda13 | 31.97 | dg | 144/144 | 31.516 | 33.062 | 46.977 | — |
| icv1-fp32-tm24839-2a7bda13 | 31.97 | dg-small | 144/144 | 30.193 | 32.373 | 52.001 | — |
| icv1-fp32-tm24839-2a7bda13 | 31.97 | rho-neg | 144/144 | 0.980 | 1.077 | 1.954 | — |
| icv1-fp32-tm24839-2a7bda13 | 31.97 | rho-neg-aa | 144/144 | 0.980 | 1.077 | 1.954 | — |
| icv1-fp32-tm24839-2a7bda13 | 31.97 | srenew | 144/144 | 31.516 | 34.593 | 62.283 | — |
| icv1-fp32-tm24839-2a7bda13 | 31.97 | srenew-small | 144/144 | 30.193 | 34.543 | 73.693 | — |
| icv1-fp32-tm24839-2a7bda13 | 31.97 | walk | 144/144 | 31.770 | 34.890 | 62.979 | — |
| icv1-fp32-tm24839-2a7bda13 | 31.97 | walk-aa | 144/144 | 31.770 | 34.890 | 62.979 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-tm103307-fa421912 | dg vs dg-small | unresolved inside the band |
| icv1-fp32-tm103307-fa421912 | dg vs rho-neg | rho-neg < dg |
| icv1-fp32-tm103307-fa421912 | dg vs rho-neg-aa | rho-neg-aa < dg |
| icv1-fp32-tm103307-fa421912 | dg vs srenew | unresolved inside the band |
| icv1-fp32-tm103307-fa421912 | dg vs srenew-small | unresolved inside the band |
| icv1-fp32-tm103307-fa421912 | dg vs walk | dg < walk |
| icv1-fp32-tm103307-fa421912 | dg vs walk-aa | dg < walk-aa |
| icv1-fp32-tm103307-fa421912 | dg-small vs rho-neg | rho-neg < dg-small |
| icv1-fp32-tm103307-fa421912 | dg-small vs rho-neg-aa | rho-neg-aa < dg-small |
| icv1-fp32-tm103307-fa421912 | dg-small vs srenew | dg-small < srenew |
| icv1-fp32-tm103307-fa421912 | dg-small vs srenew-small | unresolved inside the band |
| icv1-fp32-tm103307-fa421912 | dg-small vs walk | dg-small < walk |
| icv1-fp32-tm103307-fa421912 | dg-small vs walk-aa | dg-small < walk-aa |
| icv1-fp32-tm103307-fa421912 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-tm103307-fa421912 | rho-neg vs srenew | rho-neg < srenew |
| icv1-fp32-tm103307-fa421912 | rho-neg vs srenew-small | rho-neg < srenew-small |
| icv1-fp32-tm103307-fa421912 | rho-neg vs walk | rho-neg < walk |
| icv1-fp32-tm103307-fa421912 | rho-neg vs walk-aa | rho-neg < walk-aa |
| icv1-fp32-tm103307-fa421912 | rho-neg-aa vs srenew | rho-neg-aa < srenew |
| icv1-fp32-tm103307-fa421912 | rho-neg-aa vs srenew-small | rho-neg-aa < srenew-small |
| icv1-fp32-tm103307-fa421912 | rho-neg-aa vs walk | rho-neg-aa < walk |
| icv1-fp32-tm103307-fa421912 | rho-neg-aa vs walk-aa | rho-neg-aa < walk-aa |
| icv1-fp32-tm103307-fa421912 | srenew vs srenew-small | unresolved inside the band |
| icv1-fp32-tm103307-fa421912 | srenew vs walk | srenew < walk |
| icv1-fp32-tm103307-fa421912 | srenew vs walk-aa | srenew < walk-aa |
| icv1-fp32-tm103307-fa421912 | srenew-small vs walk | unresolved inside the band |
| icv1-fp32-tm103307-fa421912 | srenew-small vs walk-aa | unresolved inside the band |
| icv1-fp32-tm103307-fa421912 | walk vs walk-aa | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | dg vs dg-small | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | dg vs rho-neg | rho-neg < dg |
| icv1-fp32-tm24839-2a7bda13 | dg vs rho-neg-aa | rho-neg-aa < dg |
| icv1-fp32-tm24839-2a7bda13 | dg vs srenew | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | dg vs srenew-small | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | dg vs walk | dg < walk |
| icv1-fp32-tm24839-2a7bda13 | dg vs walk-aa | dg < walk-aa |
| icv1-fp32-tm24839-2a7bda13 | dg-small vs rho-neg | rho-neg < dg-small |
| icv1-fp32-tm24839-2a7bda13 | dg-small vs rho-neg-aa | rho-neg-aa < dg-small |
| icv1-fp32-tm24839-2a7bda13 | dg-small vs srenew | dg-small < srenew |
| icv1-fp32-tm24839-2a7bda13 | dg-small vs srenew-small | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | dg-small vs walk | dg-small < walk |
| icv1-fp32-tm24839-2a7bda13 | dg-small vs walk-aa | dg-small < walk-aa |
| icv1-fp32-tm24839-2a7bda13 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | rho-neg vs srenew | rho-neg < srenew |
| icv1-fp32-tm24839-2a7bda13 | rho-neg vs srenew-small | rho-neg < srenew-small |
| icv1-fp32-tm24839-2a7bda13 | rho-neg vs walk | rho-neg < walk |
| icv1-fp32-tm24839-2a7bda13 | rho-neg vs walk-aa | rho-neg < walk-aa |
| icv1-fp32-tm24839-2a7bda13 | rho-neg-aa vs srenew | rho-neg-aa < srenew |
| icv1-fp32-tm24839-2a7bda13 | rho-neg-aa vs srenew-small | rho-neg-aa < srenew-small |
| icv1-fp32-tm24839-2a7bda13 | rho-neg-aa vs walk | rho-neg-aa < walk |
| icv1-fp32-tm24839-2a7bda13 | rho-neg-aa vs walk-aa | rho-neg-aa < walk-aa |
| icv1-fp32-tm24839-2a7bda13 | srenew vs srenew-small | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | srenew vs walk | srenew < walk |
| icv1-fp32-tm24839-2a7bda13 | srenew vs walk-aa | srenew < walk-aa |
| icv1-fp32-tm24839-2a7bda13 | srenew-small vs walk | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | srenew-small vs walk-aa | unresolved inside the band |
| icv1-fp32-tm24839-2a7bda13 | walk vs walk-aa | unresolved inside the band |
