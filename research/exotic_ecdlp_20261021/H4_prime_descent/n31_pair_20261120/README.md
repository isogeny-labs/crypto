# H4 n31 selected-subgroup pair index

The preselected `n=31`, instance-1 curve yielded **2,090 distinct nonidentity points** after cofactor clearing into its 34,635,389-order prime subgroup. Its first-pair index processed **4,368,100 ordered pairs** and retained **2,116,611 distinct sums**. All **32 frozen SHA256-derived subgroup targets** produced group-verified three-sum witnesses; their median first-hit position was **8.5 third-point attempts**. An independent checked-Sage process rebuilt the curve, base, pair-sum set, and all 32 target outcomes.

| Stage or count | n31 selected instance |
| --- | ---: |
| Curve order | `2,147,394,118 = 62 × 34,635,389` |
| Raw lifted points / direct prime-subgroup points | 2,093 / 46 |
| Cleared nonidentity points / identity images / duplicate images | 2,090 / 1 / 2 |
| Ordered pair attempts / distinct pair sums | 4,368,100 / 2,116,611 |
| Instance / base construction | 0.091 / 3.381 s |
| First-pair index construction | 39.705 s |
| Verified / exhausted target lookups | 32 / 0 |
| Median first-hit position / lookup interval | 8.5 points / 0.106 ms |
| Process elapsed / peak RSS | 43.190 s / 1,453,228,032 bytes |

The exact source and selected-screen hashes, first-pair rule, 32 target labels, and 300-second/8-GiB observed limits were frozen in [`protocol.json`](protocol.json) and committed with [`run.py`](run.py) as `38c380df56d` before execution. [`result.json`](result.json) retains the field, curve, subspace, base digest, stage times, target points, witness indices, and statuses. [`sage-runtime-info.json`](sage-runtime-info.json) identifies the checked build. [`audit.py`](audit.py) independently reconstructed all group objects and used a point-only pair-sum set to verify the 32 results; [`audit.json`](audit.json) passed.

| Context for the earlier n29 instance-1 diagnostic | n29 | n31 |
| --- | ---: | ---: |
| Field size | `2^29` | `2^31` |
| Prime subgroup order | 268,447,807 | 34,635,389 |
| Cofactor | 2 | 62 |
| Cleared base points | 1,072 | 2,090 |
| Ordered pair attempts | 1,149,184 | 4,368,100 |
| First-pair index time on ordinary local host | 10.500 s | 39.705 s |
| Frozen 32-target verified lookups | 15 | 32 |

The n31 index attempted 3.80 times as many ordered pairs and took 3.78 times as long in these local diagnostic runs. The different prime-subgroup orders, cofactors, curves, and bases all affect lookup yield; the 32/32 versus 15/32 outcome cannot be assigned to field degree alone. These are component measurements. A complete n31 candidate would still need ordinary relation collection, full-rank factor-log recovery, fresh-target descent, and a paired same-point rho solve before an online IC comparison.

Reproduce the audit from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/n31_pair_20261120/audit.py
```
