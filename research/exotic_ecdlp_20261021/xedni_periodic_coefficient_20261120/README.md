# First periodic coefficient beyond two: `k=14`

**Forty-eight centered CRT curves pass both auxiliary-prime relation tests for `P+14Q=O`, and none satisfies that relation over `ℚ`.** This is the first coefficient above the earlier `2,…,8` panel in the same residue class modulo `lcm(4,6)=12`. The unchanged full-law curve-selection policy supplies the candidates; the [protocol](protocol.json) and [runner](run_sage.py) were pushed before measurement in `d146f94e26a`.

| Stage on the complete 1,722-pair law | Exact count |
| --- | ---: |
| Reduced `T+14G` checks | 1,722 |
| Compatible reduced pairs | 42 |
| Compatible pairs skipped by the frozen CRT geometry rule | 8 |
| Centered CRT candidates on the remaining pairs | 816 |
| Local relation passes at 7 | 320 |
| Passes at both 7 and 11 | 48 |
| Rational relation checks | 48 |
| Rational `P+14Q=O` relations | 0 |

The [Sage result](result.json) uses exact point arithmetic on the accepted repository launcher and retains a [runtime receipt](sage-runtime-info.json). The complete [48 checked candidate rows](rational-checks.jsonl.gz) preserve source pair index, local witness indices, CRT shifts, rational curve coefficients, and relation status. The separate [exact arithmetic audit](audit.py) replays all 48 rational sums with a Python elliptic-curve law and matches every status and the raw-row digest; its [result](audit-result.json) passes. Both runs have empty stderr. The earlier full-law census independently audited the curve stream and its local order gates.

## Why the auxiliary tests repeat

On a local curve of order 4 at 7, `4Q₇=O`; on a local curve of order 6 at 11, `6Q₁₁=O`. Thus `12Q₇=12Q₁₁=O`, so replacing any coefficient `k` by `k+12` leaves both local relation tests unchanged **on the same curve and point pair**. The [Lean theorem](formal/LocalPeriodicity.lean) `two_local_periods` proves this implication with the exact annihilation hypotheses, and `coefficient_period` proves the general statement. The pinned Lean `v4.35.0-rc4` / Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9` [build](formal/build-output.txt) reports only `propext` for both theorems. The module assumes the local annihilation equations; the finite-field curve orders and points are computationally verified in the CRT packet. `lean4checker --fresh` is unavailable on this host.

The rational checks separate local feasibility from a rational lift relation in this centered representative family. The compatible coefficient is obtained by an explicit reduced-group check in this diagnostic. The 48 zero outcomes apply to `k=14` on the frozen curves; other representatives, auxiliary primes, or coefficients need their own protocol and cost accounting. A target algorithm would also need to select coefficients without treating reduced compatibility as a free oracle and use the relations to recover a public target scalar under a paired resource envelope.

Replay:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_periodic_coefficient_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_periodic_coefficient_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_periodic_coefficient_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_periodic_coefficient_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_periodic_coefficient_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_periodic_coefficient_20261120/audit-result.json
sh research/exotic_ecdlp_20261021/xedni_periodic_coefficient_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_periodic_coefficient_20261120/formal/build-output.txt
```
