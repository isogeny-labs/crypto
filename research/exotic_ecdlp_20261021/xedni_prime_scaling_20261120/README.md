# One rank-two rational lift lattice has sharply varying costs across seven primes

**The same rational curve and two rational points give complete reduced-point coverage at seven selected prime-order reductions, while the first relation and exact rational coordinate costs vary substantially.** The first collision appears after 27–729 height-ordered rational candidates; full nonidentity coverage takes 56–1,864 candidates. At `p=857`, subgroup order 809, the coverage prefix reaches an 8,862-bit rational coordinate. At `p=1307`, subgroup order 1,237, the first collision occurs only at candidate 729.

The [protocol](protocol.json) freezes `E/Q: y²+94xy+188y=x³+1738x+3805`, rational points `Q=(-4,19)` and `R=(52242961/137641,-1907191742616/51064811)`, and the selected `(p,#E(Fp))` panel. Each reduced group has the frozen prime order and the reduction of `Q` generates it. For group order `q`, the coefficient cap is `B=⌊√q⌋+2`. Every nonzero pair in `[-B,B]²` is scored by the same pinned 128-bit height pairing and sorted by score, then `a,b`. The runner constructs exact `aQ+bR` points, reduces them at each prime, verifies their finite-group sums, and records the first reduction relation and first prefix covering every nonidentity point. A zero reduction would count as a relation; all seven first events here are collisions between distinct rational points with the same finite reduction.

| `p` | Prime group order `q` | First relation index | Coordinate peak there | Full coverage index | Coordinate peak there |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 47 | 43 | 27 | 62 bits | 56 | 146 bits |
| 83 | 101 | 39 | 98 bits | 158 | 443 bits |
| 137 | 127 | 85 | 200 bits | 150 | 366 bits |
| 283 | 283 | 37 | 95 bits | 922 | 5,816 bits |
| 499 | 457 | 317 | 767 bits | 528 | 1,426 bits |
| 857 | 809 | 151 | 358 bits | 1,864 | 8,862 bits |
| 1307 | 1237 | 729 | 1,734 bits | 1,552 | 4,585 bits |

The [Sage runner](run_sage.py) and protocol were pushed before the full rational-coordinate census in `60b593e78ab`. It checks good reduction, exact group order, subgroup generation and every rational-to-finite point map. The [seven rows](prime-rows.jsonl) retain each coefficient/candidate cap, relation witness and derived `R/G` coordinate, distinct reduced points, status, stop indices, and coordinate peaks. All seven cells have `full_coverage` status. Rational points are cached across the structural panel; 2,110 distinct rational points were constructed, with maximum coordinate size 8,862 bits. Each row's candidate index remains the work count for a separate run of that prime's frozen enumeration. The accepted [Sage runtime](sage-runtime-info.json), [result](result.json), and empty stderr are retained.

An [independent Fraction/finite-field audit](audit.py) reconstructs all 2,110 rational points with a separate group law, tests every candidate's reduction and finite-group sum, and exactly matches every prime row and raw digest. Its [result](audit-result.json) passes. The prior pinned [Lean box theorem](../xedni_two_generator_lift_20261120/formal/BoxCoverage.lean) and [collision theorem](../xedni_two_generator_relations_20261120/formal/CollisionDirection.lean) give structural finite-group statements under their exact hypotheses. This packet adds an empirical cross-prime census rather than a new mathematical theorem; the selected rational curve, reduction maps, and complete panel remain computationally checked.

The prime panel was chosen after exploratory screening for good prime-order reductions and a feasibility check, then frozen before the full exact rational run. It is a selected sample from one curve, not an estimate of the distribution across random elliptic curves or large cryptographic groups. The wide spread in first-relation and coverage work is an observed feature of this height order and these reduced generator coordinates. A next experiment should freeze a larger panel without looking at collision outcomes, compare height order with a predeclared radius order, and charge rational arithmetic plus generator discovery on one public target before evaluating any performance claim.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_prime_scaling_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_prime_scaling_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_prime_scaling_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_prime_scaling_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_prime_scaling_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_prime_scaling_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_prime_scaling_20261120/audit.stderr
```
