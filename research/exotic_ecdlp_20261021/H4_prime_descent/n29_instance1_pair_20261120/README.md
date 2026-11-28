# H4 n29 instance-one pair-index result

On the frozen `n=29`, instance-1 curve, cofactor clearing produced **1,072 distinct nonidentity points** in the prime-order subgroup of size 268,447,807. The first-pair index reduced 1,149,184 ordered pairs to 573,927 distinct sums. On 32 fresh SHA256-derived public subgroup targets, the complete base scan found and group-verified **15** three-sum witnesses; **17** targets exhausted the scan. An independent Sage process rebuilt the base and pair-sum set and reproduced all 32 statuses and witnesses.

| Stage or count | Result |
| --- | ---: |
| Curve order | `536,895,614 = 2 × 268,447,807` |
| Raw lifted points | 1,073 |
| Direct prime-subgroup points | 540 |
| Cleared nonidentity base points | 1,072 |
| Ordered pair attempts / distinct pair sums | 1,149,184 / 573,927 |
| Instance reconstruction / base construction | 0.091 / 1.810 s |
| Pair-index construction | 10.500 s |
| Fresh targets with verified witness / exhausted | 15 / 17 |
| Median full target-lookup interval | 10.781 ms |
| Peak RSS | 595,722,240 bytes |

The target fraction is **15/32 on this frozen deterministic sample**, a diagnostic of this base and lookup algorithm. The 32 points were constructed from their scalars outside lookup timing; the lookup code received only public points and did not use scalar-coordinate tables. The precommitted scan checks every cleared base point before marking a target exhausted. Index build and base construction are separate from target lookup; this packet does not solve factor-base logarithms or a complete target DLP. The host is an ordinary local macOS process and its timings are algorithmic diagnostics.

[`protocol.json`](protocol.json) and [`run.py`](run.py) were committed in `60895d8c086` before measurement. [`result.json`](result.json) retains all 32 target identities, statuses, attempts, witness indices, and times. [`sage-runtime-info.json`](sage-runtime-info.json) records the checked build. [`audit.py`](audit.py) independently reconstructs the curve, factorization, x-subspace, cofactor-cleared base, pair-sum set, and every target outcome; [`audit.json`](audit.json) records its pass. Reproduce the audit from the repository root with:

```sh
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/n29_instance1_pair_20261120/audit.py
```

The next experiment is a larger, separately frozen target sample to determine whether the 15/32 yield persists. A complete candidate would then need relation collection, base-log solving, one-target descent, and a paired rho control under the IC measurement contract.
