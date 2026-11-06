| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp20-tm565-6d7d1d32 | 19.62 | bsgs-neg | 96/96 | 1.009 | 1.109 | 2.006 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | inc | 96/96 | 10.515 | 11.829 | 23.655 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | ksum4 | 96/96 | 6.660 | 7.262 | 12.675 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | ksum5 | 96/96 | 7.367 | 7.862 | 12.323 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | rho-neg | 96/96 | 1.357 | 1.474 | 2.529 | — |
| icv1-fp20-tm565-6d7d1d32 | 19.62 | rho-neg-aa | 96/96 | 1.357 | 1.474 | 2.529 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | bsgs-neg | 96/96 | 0.958 | 1.054 | 1.913 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | inc | 96/96 | 13.976 | 15.168 | 25.892 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | ksum4 | 96/96 | 6.319 | 6.954 | 12.670 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | ksum5 | 96/96 | 6.334 | 6.906 | 12.054 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | rho-neg | 96/96 | 1.013 | 1.104 | 1.919 | — |
| icv1-fp24-tm4739-bb3c4b72 | 23.51 | rho-neg-aa | 96/96 | 1.013 | 1.104 | 1.919 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | bsgs-neg | 96/96 | 1.015 | 1.116 | 2.028 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | inc | 96/96 | 20.478 | 22.204 | 37.737 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | ksum4 | 96/96 | 7.298 | 7.978 | 14.101 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | ksum5 | 96/96 | 6.785 | 7.259 | 11.526 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | rho-neg | 96/96 | 0.939 | 1.031 | 1.851 | — |
| icv1-fp28-tm1499-56171ddc | 27.57 | rho-neg-aa | 96/96 | 0.939 | 1.031 | 1.851 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | bsgs-neg | 96/96 | 0.950 | 1.045 | 1.900 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | inc | 96/96 | 32.240 | 35.309 | 62.931 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | ksum4 | 96/96 | 8.171 | 9.039 | 16.853 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | ksum5 | 96/96 | 7.040 | 7.730 | 13.941 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | rho-neg | 96/96 | 0.968 | 1.064 | 1.930 | — |
| icv1-fp32-tm36751-2eaac89e | 31.84 | rho-neg-aa | 96/96 | 0.968 | 1.064 | 1.930 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp20-tm565-6d7d1d32 | bsgs-neg vs inc | bsgs-neg < inc |
| icv1-fp20-tm565-6d7d1d32 | bsgs-neg vs ksum4 | bsgs-neg < ksum4 |
| icv1-fp20-tm565-6d7d1d32 | bsgs-neg vs ksum5 | bsgs-neg < ksum5 |
| icv1-fp20-tm565-6d7d1d32 | bsgs-neg vs rho-neg | bsgs-neg < rho-neg |
| icv1-fp20-tm565-6d7d1d32 | bsgs-neg vs rho-neg-aa | bsgs-neg < rho-neg-aa |
| icv1-fp20-tm565-6d7d1d32 | inc vs ksum4 | ksum4 < inc |
| icv1-fp20-tm565-6d7d1d32 | inc vs ksum5 | ksum5 < inc |
| icv1-fp20-tm565-6d7d1d32 | inc vs rho-neg | rho-neg < inc |
| icv1-fp20-tm565-6d7d1d32 | inc vs rho-neg-aa | rho-neg-aa < inc |
| icv1-fp20-tm565-6d7d1d32 | ksum4 vs ksum5 | unresolved inside the band |
| icv1-fp20-tm565-6d7d1d32 | ksum4 vs rho-neg | rho-neg < ksum4 |
| icv1-fp20-tm565-6d7d1d32 | ksum4 vs rho-neg-aa | rho-neg-aa < ksum4 |
| icv1-fp20-tm565-6d7d1d32 | ksum5 vs rho-neg | rho-neg < ksum5 |
| icv1-fp20-tm565-6d7d1d32 | ksum5 vs rho-neg-aa | rho-neg-aa < ksum5 |
| icv1-fp20-tm565-6d7d1d32 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | bsgs-neg vs inc | bsgs-neg < inc |
| icv1-fp24-tm4739-bb3c4b72 | bsgs-neg vs ksum4 | bsgs-neg < ksum4 |
| icv1-fp24-tm4739-bb3c4b72 | bsgs-neg vs ksum5 | bsgs-neg < ksum5 |
| icv1-fp24-tm4739-bb3c4b72 | bsgs-neg vs rho-neg | bsgs-neg < rho-neg |
| icv1-fp24-tm4739-bb3c4b72 | bsgs-neg vs rho-neg-aa | bsgs-neg < rho-neg-aa |
| icv1-fp24-tm4739-bb3c4b72 | inc vs ksum4 | ksum4 < inc |
| icv1-fp24-tm4739-bb3c4b72 | inc vs ksum5 | ksum5 < inc |
| icv1-fp24-tm4739-bb3c4b72 | inc vs rho-neg | rho-neg < inc |
| icv1-fp24-tm4739-bb3c4b72 | inc vs rho-neg-aa | rho-neg-aa < inc |
| icv1-fp24-tm4739-bb3c4b72 | ksum4 vs ksum5 | unresolved inside the band |
| icv1-fp24-tm4739-bb3c4b72 | ksum4 vs rho-neg | rho-neg < ksum4 |
| icv1-fp24-tm4739-bb3c4b72 | ksum4 vs rho-neg-aa | rho-neg-aa < ksum4 |
| icv1-fp24-tm4739-bb3c4b72 | ksum5 vs rho-neg | rho-neg < ksum5 |
| icv1-fp24-tm4739-bb3c4b72 | ksum5 vs rho-neg-aa | rho-neg-aa < ksum5 |
| icv1-fp24-tm4739-bb3c4b72 | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp28-tm1499-56171ddc | bsgs-neg vs inc | bsgs-neg < inc |
| icv1-fp28-tm1499-56171ddc | bsgs-neg vs ksum4 | bsgs-neg < ksum4 |
| icv1-fp28-tm1499-56171ddc | bsgs-neg vs ksum5 | bsgs-neg < ksum5 |
| icv1-fp28-tm1499-56171ddc | bsgs-neg vs rho-neg | rho-neg < bsgs-neg |
| icv1-fp28-tm1499-56171ddc | bsgs-neg vs rho-neg-aa | rho-neg-aa < bsgs-neg |
| icv1-fp28-tm1499-56171ddc | inc vs ksum4 | ksum4 < inc |
| icv1-fp28-tm1499-56171ddc | inc vs ksum5 | ksum5 < inc |
| icv1-fp28-tm1499-56171ddc | inc vs rho-neg | rho-neg < inc |
| icv1-fp28-tm1499-56171ddc | inc vs rho-neg-aa | rho-neg-aa < inc |
| icv1-fp28-tm1499-56171ddc | ksum4 vs ksum5 | ksum5 < ksum4 |
| icv1-fp28-tm1499-56171ddc | ksum4 vs rho-neg | rho-neg < ksum4 |
| icv1-fp28-tm1499-56171ddc | ksum4 vs rho-neg-aa | rho-neg-aa < ksum4 |
| icv1-fp28-tm1499-56171ddc | ksum5 vs rho-neg | rho-neg < ksum5 |
| icv1-fp28-tm1499-56171ddc | ksum5 vs rho-neg-aa | rho-neg-aa < ksum5 |
| icv1-fp28-tm1499-56171ddc | rho-neg vs rho-neg-aa | unresolved inside the band |
| icv1-fp32-tm36751-2eaac89e | bsgs-neg vs inc | bsgs-neg < inc |
| icv1-fp32-tm36751-2eaac89e | bsgs-neg vs ksum4 | bsgs-neg < ksum4 |
| icv1-fp32-tm36751-2eaac89e | bsgs-neg vs ksum5 | bsgs-neg < ksum5 |
| icv1-fp32-tm36751-2eaac89e | bsgs-neg vs rho-neg | bsgs-neg < rho-neg |
| icv1-fp32-tm36751-2eaac89e | bsgs-neg vs rho-neg-aa | bsgs-neg < rho-neg-aa |
| icv1-fp32-tm36751-2eaac89e | inc vs ksum4 | ksum4 < inc |
| icv1-fp32-tm36751-2eaac89e | inc vs ksum5 | ksum5 < inc |
| icv1-fp32-tm36751-2eaac89e | inc vs rho-neg | rho-neg < inc |
| icv1-fp32-tm36751-2eaac89e | inc vs rho-neg-aa | rho-neg-aa < inc |
| icv1-fp32-tm36751-2eaac89e | ksum4 vs ksum5 | ksum5 < ksum4 |
| icv1-fp32-tm36751-2eaac89e | ksum4 vs rho-neg | rho-neg < ksum4 |
| icv1-fp32-tm36751-2eaac89e | ksum4 vs rho-neg-aa | rho-neg-aa < ksum4 |
| icv1-fp32-tm36751-2eaac89e | ksum5 vs rho-neg | rho-neg < ksum5 |
| icv1-fp32-tm36751-2eaac89e | ksum5 vs rho-neg-aa | rho-neg-aa < ksum5 |
| icv1-fp32-tm36751-2eaac89e | rho-neg vs rho-neg-aa | unresolved inside the band |
