| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-t16357-dd50743c | 31.40 | dg | 144/144 | 32.681 | 33.990 | 45.773 | — |
| icv1-fp32-t16357-dd50743c | 31.40 | dg-small | 144/144 | 28.673 | 30.517 | 47.115 | — |
| icv1-fp32-t16357-dd50743c | 31.40 | srenew | 144/144 | 32.681 | 35.280 | 58.672 | — |
| icv1-fp32-t16357-dd50743c | 31.40 | srenew-aa | 144/144 | 32.681 | 35.280 | 58.672 | — |
| icv1-fp32-t16357-dd50743c | 31.40 | walk | 144/144 | 32.471 | 35.020 | 57.960 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | dg | 144/144 | 32.136 | 33.628 | 47.052 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | dg-small | 144/144 | 29.700 | 31.771 | 50.407 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | srenew | 144/144 | 32.136 | 35.102 | 61.795 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | srenew-aa | 144/144 | 32.136 | 35.102 | 61.795 | — |
| icv1-fp32-tm94829-d5c5f997 | 31.77 | walk | 144/144 | 32.275 | 35.261 | 62.137 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-t16357-dd50743c | dg vs dg-small | unresolved inside the band |
| icv1-fp32-t16357-dd50743c | dg vs srenew | unresolved inside the band |
| icv1-fp32-t16357-dd50743c | dg vs srenew-aa | unresolved inside the band |
| icv1-fp32-t16357-dd50743c | dg vs walk | unresolved inside the band |
| icv1-fp32-t16357-dd50743c | dg-small vs srenew | dg-small < srenew |
| icv1-fp32-t16357-dd50743c | dg-small vs srenew-aa | dg-small < srenew-aa |
| icv1-fp32-t16357-dd50743c | dg-small vs walk | dg-small < walk |
| icv1-fp32-t16357-dd50743c | srenew vs srenew-aa | unresolved inside the band |
| icv1-fp32-t16357-dd50743c | srenew vs walk | walk < srenew |
| icv1-fp32-t16357-dd50743c | srenew-aa vs walk | walk < srenew-aa |
| icv1-fp32-tm94829-d5c5f997 | dg vs dg-small | unresolved inside the band |
| icv1-fp32-tm94829-d5c5f997 | dg vs srenew | unresolved inside the band |
| icv1-fp32-tm94829-d5c5f997 | dg vs srenew-aa | unresolved inside the band |
| icv1-fp32-tm94829-d5c5f997 | dg vs walk | dg < walk |
| icv1-fp32-tm94829-d5c5f997 | dg-small vs srenew | dg-small < srenew |
| icv1-fp32-tm94829-d5c5f997 | dg-small vs srenew-aa | dg-small < srenew-aa |
| icv1-fp32-tm94829-d5c5f997 | dg-small vs walk | dg-small < walk |
| icv1-fp32-tm94829-d5c5f997 | srenew vs srenew-aa | unresolved inside the band |
| icv1-fp32-tm94829-d5c5f997 | srenew vs walk | srenew < walk |
| icv1-fp32-tm94829-d5c5f997 | srenew-aa vs walk | srenew-aa < walk |
