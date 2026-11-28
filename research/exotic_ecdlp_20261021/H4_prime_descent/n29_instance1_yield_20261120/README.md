# H4 n29 three-sum yield replication and symmetry bound

The frozen 1,072-point subgroup base returned **138 group-verified three-sum witnesses among 256 new public targets** (53.906%); the other **118** targets exhausted every base point in the first-pair lookup. An independent checked-Sage process rebuilt the base and pair-sum set and replayed all 256 target scalars, witness equations, and exhaustion statuses. The earlier disjoint 32-target diagnostic returned 15/32 (46.875%).

| Measured item | Result |
| --- | ---: |
| Curve order | `536,895,614 = 2 × 268,447,807` |
| Cleared prime-subgroup base / distinct pair sums | 1,072 / 573,927 |
| New target count | 256 distinct scalars, zero overlap with preceding 32 |
| Verified / exhausted lookups | 138 / 118 |
| Index construction | 10.328 s |
| Sum of all 256 lookup intervals | 1.611 s |
| Median lookup interval | 5.128 ms |
| Process elapsed / peak RSS | 14.310 s / 588,087,296 bytes |

The new protocol and runner were committed in `b1076ab3ad7` before measurement. [`protocol.json`](protocol.json) pins the preceding result hash and a new SHA256 label domain. [`result.json`](result.json) retains every target point, fixture scalar, lookup status, attempt count, witness index, and interval. Fixture scalar construction stayed outside the target lookup clock. The runner checked the preceding base digest, generator, and pair-sum count before any target scan. [`audit.py`](audit.py) independently checked all 256 records, with result in [`audit.json`](audit.json). The accepted Sage build is recorded in [`sage-runtime-info.json`](sage-runtime-info.json).

## Exact combinatorial bound and model

For a finite base `A` of size `B` in a commutative group, every ordered triple `(a,b,c)` has the same sum as its permutations. There are exactly `C(B+2,3)` three-element multisets with repetition, so the number of distinct three-sums obeys

`|A+A+A| ≤ C(B+2,3)`.

For `B=1,072`, the right side is **205,895,824**, at most **76.699%** of the 268,447,807-element subgroup. This is a bound on all subgroup elements including identity; excluding identity changes a fraction's denominator and possible numerator by at most one. The calculation uses the audited base size as an external finite-instance premise. The theorem itself is general and does not assume random sums, a particular curve, or distinct summands.

If the sum of each three-element multiset behaved as an independent uniform subgroup draw, its predicted coverage would be `1-(1-1/r)^C(B+2,3) = 53.559%`. The observed 53.906% is close to that **occupancy model**; this agreement does not prove independence of the sums or exact full-subgroup coverage. Counting ordered pair-sum/base combinations as independent draws would predict 89.917%, which exceeds the rigorous 76.699% ceiling and double-counts permutations. The exact 256-point sample fraction is the measured statistic. A 95% Clopper–Pearson interval of [47.590%, 60.131%] applies only under an independent uniform-target sampling model; these committed SHA256 labels are a deterministic pseudorandom construction.

The formal proof is [`formal/TripleSumBound.lean`](formal/TripleSumBound.lean). `card_orderedThreeSums_le` proves the full structural and cardinality implication; `card_orderedThreeSums_le_1072` instantiates its numerical bound. The [proof map](formal/PROOF_MAP.md) states exact hypotheses, dependency revision, and finite-instance bridge. The current measurement is a component diagnostic on an ordinary local host. A complete candidate would next collect ordinary relations, solve and replay the base logs, recover a fresh target, and pair it with same-point rho under the IC measurement contract.

Reproduce the group audit from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/n29_instance1_yield_20261120/audit.py
```
