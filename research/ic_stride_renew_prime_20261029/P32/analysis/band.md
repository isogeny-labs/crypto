| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-t10863-8c68282c | 31.20 | rho-neg | 144/144 | 0.929 | 1.021 | 1.850 | — |
| icv1-fp32-t10863-8c68282c | 31.20 | rho-neg-aa | 144/144 | 0.929 | 1.021 | 1.850 | — |
| icv1-fp32-t10863-8c68282c | 31.20 | srenew | 144/144 | 33.408 | 35.874 | 58.068 | — |
| icv1-fp32-t10863-8c68282c | 31.20 | walk | 144/144 | 33.169 | 35.578 | 57.262 | — |
| icv1-fp32-t10863-8c68282c | 31.20 | walk-aa | 144/144 | 33.169 | 35.578 | 57.262 | — |
| icv1-fp32-tm62163-8ef9adf3 | 31.93 | rho-neg | 144/144 | 0.958 | 1.053 | 1.909 | — |
| icv1-fp32-tm62163-8ef9adf3 | 31.93 | rho-neg-aa | 144/144 | 0.958 | 1.053 | 1.909 | — |
| icv1-fp32-tm62163-8ef9adf3 | 31.93 | srenew | 144/144 | 31.776 | 34.860 | 62.612 | — |
| icv1-fp32-tm62163-8ef9adf3 | 31.93 | walk | 144/144 | 31.921 | 35.026 | 62.976 | — |
| icv1-fp32-tm62163-8ef9adf3 | 31.93 | walk-aa | 144/144 | 31.921 | 35.026 | 62.976 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-t10863-8c68282c | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-t10863-8c68282c | rho-neg vs srenew | rho-neg < srenew |
| icv1-fp32-t10863-8c68282c | rho-neg vs walk | rho-neg < walk |
| icv1-fp32-t10863-8c68282c | rho-neg vs walk-aa | rho-neg < walk-aa |
| icv1-fp32-t10863-8c68282c | rho-neg-aa vs srenew | rho-neg-aa < srenew |
| icv1-fp32-t10863-8c68282c | rho-neg-aa vs walk | rho-neg-aa < walk |
| icv1-fp32-t10863-8c68282c | rho-neg-aa vs walk-aa | rho-neg-aa < walk-aa |
| icv1-fp32-t10863-8c68282c | srenew vs walk | walk < srenew |
| icv1-fp32-t10863-8c68282c | srenew vs walk-aa | walk-aa < srenew |
| icv1-fp32-t10863-8c68282c | walk vs walk-aa | unresolved inside the band |
| icv1-fp32-tm62163-8ef9adf3 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-tm62163-8ef9adf3 | rho-neg vs srenew | rho-neg < srenew |
| icv1-fp32-tm62163-8ef9adf3 | rho-neg vs walk | rho-neg < walk |
| icv1-fp32-tm62163-8ef9adf3 | rho-neg vs walk-aa | rho-neg < walk-aa |
| icv1-fp32-tm62163-8ef9adf3 | rho-neg-aa vs srenew | rho-neg-aa < srenew |
| icv1-fp32-tm62163-8ef9adf3 | rho-neg-aa vs walk | rho-neg-aa < walk |
| icv1-fp32-tm62163-8ef9adf3 | rho-neg-aa vs walk-aa | rho-neg-aa < walk-aa |
| icv1-fp32-tm62163-8ef9adf3 | srenew vs walk | srenew < walk |
| icv1-fp32-tm62163-8ef9adf3 | srenew vs walk-aa | srenew < walk-aa |
| icv1-fp32-tm62163-8ef9adf3 | walk vs walk-aa | unresolved inside the band |
