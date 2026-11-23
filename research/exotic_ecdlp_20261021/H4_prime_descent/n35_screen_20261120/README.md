# H4 n=35 prime-subgroup screen

The frozen five-instance screen found **no curve meeting its (2^{30}) prime-subgroup threshold**. All five exact group orders, factorizations, curve coefficients, and twelve-vector x-subspace bases were independently reconstructed and audited. Instance 1 has subgroup order 520,604,191 with cofactor 66, but is below the stated threshold 1,073,741,824; it remains an ineligible row for this screen.

| Instance | Exact curve order | Largest prime divisor | Cofactor | Meets (2^{30}) gate |
| ---: | ---: | ---: | ---: | --- |
| 0 | 34,359,908,066 | 504,299 | 68,134 | no |
| 1 | 34,359,876,606 | 520,604,191 | 66 | no |
| 2 | 34,359,634,844 | 11,437,961 | 3,004 | no |
| 3 | 34,359,445,596 | 292,381 | 117,516 | no |
| 4 | 34,359,601,902 | 3,760,079 | 9,138 | no |

The [protocol](protocol.json) and [runner](run.py) were committed before execution in `cf0042b7a59`. [result.json](result.json) preserves every row and the explicit null selection. The checked [Sage runtime](sage-runtime-info.json) and independent [audit](audit.json) cover the exact field, curve, factorization, Hasse bound, and subspace selection for each index.

For the separate question of whether the memory-efficient pair index can scale to a base of roughly 4,000 points, a new protocol may use a lower (2^{28}) subgroup gate. That would select instance 1 from these frozen records, with a 15.0-times larger prime subgroup than the selected n31 curve. It would be a new eligibility criterion and a new pair-index experiment, not a reinterpretation of this screen's null result.
