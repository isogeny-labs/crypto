| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-f2m19-t797-b6cf2467 | 17.00 | f4-blind | 8/8 | 4.927 | 120892.292 | 1208878.577 | — |
| icv1-f2m19-t797-b6cf2467 | 17.00 | f4-tracekernel | 8/8 | 4.895 | 163298.771 | 1632943.647 | — |
| icv1-f2m19-t797-b6cf2467 | 17.00 | mitm-blind | 8/8 | 195.753 | 196.431 | 202.529 | — |
| icv1-f2m19-t797-b6cf2467 | 17.00 | mitm-tracekernel | 8/8 | 160.816 | 161.523 | 167.892 | — |
| icv1-f2m19-t797-b6cf2467 | 17.00 | rho-neg | 8/8 | 1.721 | 1.851 | 3.013 | — |
| icv1-f2m23-t5197-69e76b73 | 21.00 | f4-blind | 8/8 | 5.382 | 30340.728 | 303358.841 | — |
| icv1-f2m23-t5197-69e76b73 | 21.00 | f4-tracekernel | 8/8 | 3.806 | 26711.915 | 267084.890 | — |
| icv1-f2m23-t5197-69e76b73 | 21.00 | mitm-blind | 8/8 | 53.084 | 53.996 | 62.202 | — |
| icv1-f2m23-t5197-69e76b73 | 21.00 | mitm-tracekernel | 8/8 | 49.641 | 50.280 | 56.030 | — |
| icv1-f2m23-t5197-69e76b73 | 21.00 | rho-neg | 8/8 | 1.175 | 1.281 | 2.233 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-f2m19-t797-b6cf2467 | f4-blind vs f4-tracekernel | unresolved inside the band |
| icv1-f2m19-t797-b6cf2467 | f4-blind vs mitm-blind | unresolved inside the band |
| icv1-f2m19-t797-b6cf2467 | f4-blind vs mitm-tracekernel | unresolved inside the band |
| icv1-f2m19-t797-b6cf2467 | f4-blind vs rho-neg | rho-neg < f4-blind |
| icv1-f2m19-t797-b6cf2467 | f4-tracekernel vs mitm-blind | unresolved inside the band |
| icv1-f2m19-t797-b6cf2467 | f4-tracekernel vs mitm-tracekernel | unresolved inside the band |
| icv1-f2m19-t797-b6cf2467 | f4-tracekernel vs rho-neg | rho-neg < f4-tracekernel |
| icv1-f2m19-t797-b6cf2467 | mitm-blind vs mitm-tracekernel | mitm-tracekernel < mitm-blind |
| icv1-f2m19-t797-b6cf2467 | mitm-blind vs rho-neg | rho-neg < mitm-blind |
| icv1-f2m19-t797-b6cf2467 | mitm-tracekernel vs rho-neg | rho-neg < mitm-tracekernel |
| icv1-f2m23-t5197-69e76b73 | f4-blind vs f4-tracekernel | f4-tracekernel < f4-blind |
| icv1-f2m23-t5197-69e76b73 | f4-blind vs mitm-blind | unresolved inside the band |
| icv1-f2m23-t5197-69e76b73 | f4-blind vs mitm-tracekernel | unresolved inside the band |
| icv1-f2m23-t5197-69e76b73 | f4-blind vs rho-neg | rho-neg < f4-blind |
| icv1-f2m23-t5197-69e76b73 | f4-tracekernel vs mitm-blind | unresolved inside the band |
| icv1-f2m23-t5197-69e76b73 | f4-tracekernel vs mitm-tracekernel | unresolved inside the band |
| icv1-f2m23-t5197-69e76b73 | f4-tracekernel vs rho-neg | rho-neg < f4-tracekernel |
| icv1-f2m23-t5197-69e76b73 | mitm-blind vs mitm-tracekernel | mitm-tracekernel < mitm-blind |
| icv1-f2m23-t5197-69e76b73 | mitm-blind vs rho-neg | rho-neg < mitm-blind |
| icv1-f2m23-t5197-69e76b73 | mitm-tracekernel vs rho-neg | rho-neg < mitm-tracekernel |
