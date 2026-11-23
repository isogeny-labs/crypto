| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-tm11535-8e4578de | 31.20 | dg | 144/144 | 32.978 | 34.180 | 44.997 | — |
| icv1-fp32-tm11535-8e4578de | 31.20 | dg-small | 144/144 | 27.349 | 28.989 | 43.751 | — |
| icv1-fp32-tm11535-8e4578de | 31.20 | rho-neg | 144/144 | 0.970 | 1.066 | 1.931 | — |
| icv1-fp32-tm11535-8e4578de | 31.20 | rho-neg-aa | 144/144 | 0.970 | 1.066 | 1.931 | — |
| icv1-fp32-tm11535-8e4578de | 31.20 | srenew | 144/144 | 32.978 | 35.362 | 56.815 | — |
| icv1-fp32-tm11535-8e4578de | 31.20 | srenew-small | 144/144 | 27.349 | 30.615 | 60.005 | — |
| icv1-fp32-tm11535-8e4578de | 31.20 | walk | 144/144 | 32.787 | 35.124 | 56.155 | — |
| icv1-fp32-tm11535-8e4578de | 31.20 | walk-aa | 144/144 | 32.787 | 35.124 | 56.155 | — |
| icv1-fp32-t46781-77aa234d | 31.84 | dg | 144/144 | 31.902 | 33.415 | 47.033 | — |
| icv1-fp32-t46781-77aa234d | 31.84 | dg-small | 144/144 | 29.860 | 31.970 | 50.961 | — |
| icv1-fp32-t46781-77aa234d | 31.84 | rho-neg | 144/144 | 0.967 | 1.063 | 1.927 | — |
| icv1-fp32-t46781-77aa234d | 31.84 | rho-neg-aa | 144/144 | 0.967 | 1.063 | 1.927 | — |
| icv1-fp32-t46781-77aa234d | 31.84 | srenew | 144/144 | 31.902 | 34.912 | 62.000 | — |
| icv1-fp32-t46781-77aa234d | 31.84 | srenew-small | 144/144 | 29.860 | 34.068 | 71.939 | — |
| icv1-fp32-t46781-77aa234d | 31.84 | walk | 144/144 | 31.876 | 34.873 | 61.849 | — |
| icv1-fp32-t46781-77aa234d | 31.84 | walk-aa | 144/144 | 31.876 | 34.873 | 61.849 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-tm11535-8e4578de | dg vs dg-small | dg-small < dg |
| icv1-fp32-tm11535-8e4578de | dg vs rho-neg | rho-neg < dg |
| icv1-fp32-tm11535-8e4578de | dg vs rho-neg-aa | rho-neg-aa < dg |
| icv1-fp32-tm11535-8e4578de | dg vs srenew | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | dg vs srenew-small | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | dg vs walk | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | dg vs walk-aa | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | dg-small vs rho-neg | rho-neg < dg-small |
| icv1-fp32-tm11535-8e4578de | dg-small vs rho-neg-aa | rho-neg-aa < dg-small |
| icv1-fp32-tm11535-8e4578de | dg-small vs srenew | dg-small < srenew |
| icv1-fp32-tm11535-8e4578de | dg-small vs srenew-small | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | dg-small vs walk | dg-small < walk |
| icv1-fp32-tm11535-8e4578de | dg-small vs walk-aa | dg-small < walk-aa |
| icv1-fp32-tm11535-8e4578de | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | rho-neg vs srenew | rho-neg < srenew |
| icv1-fp32-tm11535-8e4578de | rho-neg vs srenew-small | rho-neg < srenew-small |
| icv1-fp32-tm11535-8e4578de | rho-neg vs walk | rho-neg < walk |
| icv1-fp32-tm11535-8e4578de | rho-neg vs walk-aa | rho-neg < walk-aa |
| icv1-fp32-tm11535-8e4578de | rho-neg-aa vs srenew | rho-neg-aa < srenew |
| icv1-fp32-tm11535-8e4578de | rho-neg-aa vs srenew-small | rho-neg-aa < srenew-small |
| icv1-fp32-tm11535-8e4578de | rho-neg-aa vs walk | rho-neg-aa < walk |
| icv1-fp32-tm11535-8e4578de | rho-neg-aa vs walk-aa | rho-neg-aa < walk-aa |
| icv1-fp32-tm11535-8e4578de | srenew vs srenew-small | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | srenew vs walk | walk < srenew |
| icv1-fp32-tm11535-8e4578de | srenew vs walk-aa | walk-aa < srenew |
| icv1-fp32-tm11535-8e4578de | srenew-small vs walk | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | srenew-small vs walk-aa | unresolved inside the band |
| icv1-fp32-tm11535-8e4578de | walk vs walk-aa | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | dg vs dg-small | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | dg vs rho-neg | rho-neg < dg |
| icv1-fp32-t46781-77aa234d | dg vs rho-neg-aa | rho-neg-aa < dg |
| icv1-fp32-t46781-77aa234d | dg vs srenew | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | dg vs srenew-small | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | dg vs walk | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | dg vs walk-aa | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | dg-small vs rho-neg | rho-neg < dg-small |
| icv1-fp32-t46781-77aa234d | dg-small vs rho-neg-aa | rho-neg-aa < dg-small |
| icv1-fp32-t46781-77aa234d | dg-small vs srenew | dg-small < srenew |
| icv1-fp32-t46781-77aa234d | dg-small vs srenew-small | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | dg-small vs walk | dg-small < walk |
| icv1-fp32-t46781-77aa234d | dg-small vs walk-aa | dg-small < walk-aa |
| icv1-fp32-t46781-77aa234d | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | rho-neg vs srenew | rho-neg < srenew |
| icv1-fp32-t46781-77aa234d | rho-neg vs srenew-small | rho-neg < srenew-small |
| icv1-fp32-t46781-77aa234d | rho-neg vs walk | rho-neg < walk |
| icv1-fp32-t46781-77aa234d | rho-neg vs walk-aa | rho-neg < walk-aa |
| icv1-fp32-t46781-77aa234d | rho-neg-aa vs srenew | rho-neg-aa < srenew |
| icv1-fp32-t46781-77aa234d | rho-neg-aa vs srenew-small | rho-neg-aa < srenew-small |
| icv1-fp32-t46781-77aa234d | rho-neg-aa vs walk | rho-neg-aa < walk |
| icv1-fp32-t46781-77aa234d | rho-neg-aa vs walk-aa | rho-neg-aa < walk-aa |
| icv1-fp32-t46781-77aa234d | srenew vs srenew-small | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | srenew vs walk | walk < srenew |
| icv1-fp32-t46781-77aa234d | srenew vs walk-aa | walk-aa < srenew |
| icv1-fp32-t46781-77aa234d | srenew-small vs walk | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | srenew-small vs walk-aa | unresolved inside the band |
| icv1-fp32-t46781-77aa234d | walk vs walk-aa | unresolved inside the band |
