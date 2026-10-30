| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp20-tm565-6d7d1d32 | 19.62 | inc | 96/96 | 12.853 | 14.826 | 32.577 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | inc-aa | 96/96 | 12.853 | 14.826 | 32.577 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | rho-neg | 96/96 | 1.357 | 1.474 | 2.529 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | rho-neg-aa | 96/96 | 1.357 | 1.474 | 2.529 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | sel | 96/96 | 10.515 | 11.829 | 23.655 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | size | 96/96 | 10.515 | 11.829 | 23.655 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | stride | 96/96 | 10.402 | 11.693 | 23.306 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | inc | 96/96 | 13.976 | 15.168 | 25.892 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | inc-aa | 96/96 | 13.976 | 15.168 | 25.892 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | rho-neg | 96/96 | 1.013 | 1.104 | 1.919 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | rho-neg-aa | 96/96 | 1.013 | 1.104 | 1.919 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | sel | 96/96 | 12.605 | 14.260 | 29.149 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | size | 96/96 | 12.605 | 14.260 | 29.149 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | stride | 96/96 | 12.543 | 14.183 | 28.942 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | inc | 96/96 | 20.478 | 22.204 | 37.737 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | inc-aa | 96/96 | 20.478 | 22.204 | 37.737 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | rho-neg | 96/96 | 0.939 | 1.031 | 1.851 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | rho-neg-aa | 96/96 | 0.939 | 1.031 | 1.851 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | sel | 96/96 | 19.017 | 21.618 | 45.036 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | size | 96/96 | 19.017 | 21.618 | 45.036 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | stride | 96/96 | 18.987 | 21.583 | 44.947 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | inc | 96/96 | 32.240 | 35.309 | 62.931 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | inc-aa | 96/96 | 32.240 | 35.309 | 62.931 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | rho-neg | 96/96 | 0.968 | 1.064 | 1.930 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | rho-neg-aa | 96/96 | 0.968 | 1.064 | 1.930 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | sel | 96/96 | 32.240 | 35.309 | 62.931 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | size | 96/96 | 32.240 | 35.309 | 62.931 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | stride | 96/96 | 31.880 | 34.877 | 61.848 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp20-tm565-6d7d1d32 | inc vs inc-aa | unresolved inside the band |
| icv1-fp20-tm565-6d7d1d32 | inc vs rho-neg | rho-neg < inc |
| icv1-fp20-tm565-6d7d1d32 | inc vs rho-neg-aa | rho-neg-aa < inc |
| icv1-fp20-tm565-6d7d1d32 | inc vs sel | sel < inc |
| icv1-fp20-tm565-6d7d1d32 | inc vs size | size < inc |
| icv1-fp20-tm565-6d7d1d32 | inc vs stride | stride < inc |
| icv1-fp20-tm565-6d7d1d32 | inc-aa vs rho-neg | rho-neg < inc-aa |
| icv1-fp20-tm565-6d7d1d32 | inc-aa vs rho-neg-aa | rho-neg-aa < inc-aa |
| icv1-fp20-tm565-6d7d1d32 | inc-aa vs sel | sel < inc-aa |
| icv1-fp20-tm565-6d7d1d32 | inc-aa vs size | size < inc-aa |
| icv1-fp20-tm565-6d7d1d32 | inc-aa vs stride | stride < inc-aa |
| icv1-fp20-tm565-6d7d1d32 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp20-tm565-6d7d1d32 | rho-neg vs sel | rho-neg < sel |
| icv1-fp20-tm565-6d7d1d32 | rho-neg vs size | rho-neg < size |
| icv1-fp20-tm565-6d7d1d32 | rho-neg vs stride | rho-neg < stride |
| icv1-fp20-tm565-6d7d1d32 | rho-neg-aa vs sel | rho-neg-aa < sel |
| icv1-fp20-tm565-6d7d1d32 | rho-neg-aa vs size | rho-neg-aa < size |
| icv1-fp20-tm565-6d7d1d32 | rho-neg-aa vs stride | rho-neg-aa < stride |
| icv1-fp20-tm565-6d7d1d32 | sel vs size | unresolved inside the band |
| icv1-fp20-tm565-6d7d1d32 | sel vs stride | stride < sel |
| icv1-fp20-tm565-6d7d1d32 | size vs stride | stride < size |
| icv1-fp24-tm4739-bb3c4b72 | inc vs inc-aa | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | inc vs rho-neg | rho-neg < inc |
| icv1-fp24-tm4739-bb3c4b72 | inc vs rho-neg-aa | rho-neg-aa < inc |
| icv1-fp24-tm4739-bb3c4b72 | inc vs sel | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | inc vs size | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | inc vs stride | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | inc-aa vs rho-neg | rho-neg < inc-aa |
| icv1-fp24-tm4739-bb3c4b72 | inc-aa vs rho-neg-aa | rho-neg-aa < inc-aa |
| icv1-fp24-tm4739-bb3c4b72 | inc-aa vs sel | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | inc-aa vs size | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | inc-aa vs stride | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | rho-neg vs sel | rho-neg < sel |
| icv1-fp24-tm4739-bb3c4b72 | rho-neg vs size | rho-neg < size |
| icv1-fp24-tm4739-bb3c4b72 | rho-neg vs stride | rho-neg < stride |
| icv1-fp24-tm4739-bb3c4b72 | rho-neg-aa vs sel | rho-neg-aa < sel |
| icv1-fp24-tm4739-bb3c4b72 | rho-neg-aa vs size | rho-neg-aa < size |
| icv1-fp24-tm4739-bb3c4b72 | rho-neg-aa vs stride | rho-neg-aa < stride |
| icv1-fp24-tm4739-bb3c4b72 | sel vs size | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | sel vs stride | stride < sel |
| icv1-fp24-tm4739-bb3c4b72 | size vs stride | stride < size |
| icv1-fp28-tm1499-56171ddc | inc vs inc-aa | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | inc vs rho-neg | rho-neg < inc |
| icv1-fp28-tm1499-56171ddc | inc vs rho-neg-aa | rho-neg-aa < inc |
| icv1-fp28-tm1499-56171ddc | inc vs sel | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | inc vs size | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | inc vs stride | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | inc-aa vs rho-neg | rho-neg < inc-aa |
| icv1-fp28-tm1499-56171ddc | inc-aa vs rho-neg-aa | rho-neg-aa < inc-aa |
| icv1-fp28-tm1499-56171ddc | inc-aa vs sel | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | inc-aa vs size | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | inc-aa vs stride | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | rho-neg vs sel | rho-neg < sel |
| icv1-fp28-tm1499-56171ddc | rho-neg vs size | rho-neg < size |
| icv1-fp28-tm1499-56171ddc | rho-neg vs stride | rho-neg < stride |
| icv1-fp28-tm1499-56171ddc | rho-neg-aa vs sel | rho-neg-aa < sel |
| icv1-fp28-tm1499-56171ddc | rho-neg-aa vs size | rho-neg-aa < size |
| icv1-fp28-tm1499-56171ddc | rho-neg-aa vs stride | rho-neg-aa < stride |
| icv1-fp28-tm1499-56171ddc | sel vs size | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | sel vs stride | stride < sel |
| icv1-fp28-tm1499-56171ddc | size vs stride | stride < size |
| icv1-fp32-tm36751-2eaac89e | inc vs inc-aa | unresolved inside the band |
| icv1-fp32-tm36751-2eaac89e | inc vs rho-neg | rho-neg < inc |
| icv1-fp32-tm36751-2eaac89e | inc vs rho-neg-aa | rho-neg-aa < inc |
| icv1-fp32-tm36751-2eaac89e | inc vs sel | unresolved inside the band |
| icv1-fp32-tm36751-2eaac89e | inc vs size | unresolved inside the band |
| icv1-fp32-tm36751-2eaac89e | inc vs stride | stride < inc |
| icv1-fp32-tm36751-2eaac89e | inc-aa vs rho-neg | rho-neg < inc-aa |
| icv1-fp32-tm36751-2eaac89e | inc-aa vs rho-neg-aa | rho-neg-aa < inc-aa |
| icv1-fp32-tm36751-2eaac89e | inc-aa vs sel | unresolved inside the band |
| icv1-fp32-tm36751-2eaac89e | inc-aa vs size | unresolved inside the band |
| icv1-fp32-tm36751-2eaac89e | inc-aa vs stride | stride < inc-aa |
| icv1-fp32-tm36751-2eaac89e | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-tm36751-2eaac89e | rho-neg vs sel | rho-neg < sel |
| icv1-fp32-tm36751-2eaac89e | rho-neg vs size | rho-neg < size |
| icv1-fp32-tm36751-2eaac89e | rho-neg vs stride | rho-neg < stride |
| icv1-fp32-tm36751-2eaac89e | rho-neg-aa vs sel | rho-neg-aa < sel |
| icv1-fp32-tm36751-2eaac89e | rho-neg-aa vs size | rho-neg-aa < size |
| icv1-fp32-tm36751-2eaac89e | rho-neg-aa vs stride | rho-neg-aa < stride |
| icv1-fp32-tm36751-2eaac89e | sel vs size | unresolved inside the band |
| icv1-fp32-tm36751-2eaac89e | sel vs stride | stride < sel |
| icv1-fp32-tm36751-2eaac89e | size vs stride | stride < size |
