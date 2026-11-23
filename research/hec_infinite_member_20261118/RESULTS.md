# Direct decomposition of each anchor

Both controls and the combined collector solved and verified all 44 targets. Every split anchor reconstructed under group addition. The direct-only collector solved 14/44 before the 512-anchor cap. Failed cells are retained with blank timing ratios.

| Genus | Collector | Verified / attempted | Faster than deferred | Fewer anchors than deferred | Original / candidate wall | Deferred / candidate wall |
|---|---|---:|---:|---:|---:|---:|
| 2 | combined | 24/24 | 14 | 20 | 1.363–2.280× | 0.963–1.093× |
| 2 | direct only | 8/24 | 1 | 0 | 0.925–1.489× | 0.569–1.049× |
| 3 | combined | 12/12 | 0 | 10 | 1.466–2.654× | 0.776–0.907× |
| 3 | direct only | 4/12 | 0 | 0 | 0.698–0.773× | 0.369–0.404× |
| 4 | combined | 8/8 | 0 | 0 | 1.864–3.639× | 0.660–0.844× |
| 4 | direct only | 2/8 | 0 | 0 | 1.768–1.948× | 0.401–0.405× |

Each timing ratio uses median complete-solver time over twelve rotating observations on a jointly solved input. Reduced anchor counts in genus three did not repay testing every support. Genus four saved no anchor visits on this panel. Genus two's cheap quadratic split made the marginal comparison closer. These results reject unconditional direct decomposition as a cross-genus tenfold mechanism. Finite pencil relations still materially contribute to coverage within this budget.

RUN.txt records the remotely verified input commit. CONTROL.txt retains all calls, including failures; summarize.rb reproduces COMPARISON.tsv.
