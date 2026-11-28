# H4 n31 subgroup screen

The frozen first-eligible rule selected **instance 1**: its curve order is `2,147,394,118 = 2 × 31 × 34,635,389`, giving a prime subgroup of order 34,635,389 and cofactor 62. Instances 0 and 2 did not meet the precommitted `2^25` prime-subgroup threshold. The exact field, curve coefficients, and eleven-vector x-subspace basis for every index are in [`result.json`](result.json).

| Instance | Exact order factorization | Largest prime divisor | Gate |
| ---: | --- | ---: | --- |
| 0 | `2 × 3 × 191² × 9,811` | 9,811 | rejected |
| 1 | `2 × 31 × 34,635,389` | 34,635,389 | selected |
| 2 | `2 × 5 × 13 × 16,518,599` | 16,518,599 | rejected |

[`protocol.json`](protocol.json) and [`run.py`](run.py) were committed in `ea347d697af` before execution. The checked Sage runtime is [`sage-runtime-info.json`](sage-runtime-info.json). [`audit.py`](audit.py) separately reconstructs all three curves and subspaces, rechecks their exact orders and factorizations, the Hasse bounds, and the first-eligible selection; [`audit.json`](audit.json) records the pass.

The selected prime subgroup is smaller than the n29 candidate's 268,447,807-order subgroup. A subsequent pair-index test can measure the effect of the larger n31 field and eleven-dimensional x-subspace, with subgroup order and cofactor stated as changed variables. The screen itself contains no pair-index or target-lookup measurement.
