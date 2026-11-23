# Bounded-height rational point lookup covers four reduced targets

**An exact search of 1,275,855 canonical rational `x` candidates on one fixed witness curve finds four rational points, whose reductions cover four of the 42 nonidentity points of `E(F₄₇)`.** The four lifts are the two sign pairs above `x=-4` and `x=8`; the remaining 38 targets in the frozen finite input law have status `height_cap_exhausted` at `H=1024`. A public target can be looked up by its reduced coordinates in this table without using its scalar index. The table's construction work and 4/42 coverage are explicit.

For each `H`, the [frozen protocol](protocol.json) enumerates coprime integer pairs `a,b`, with `b≥1` and `max(|a|,b)≤H`, sets `x=a/b²`, and tests the integral quadratic discriminant for `y=c/b³`. Every rational point found is reduced at 47. The [runner](run.py) and protocol were pushed before the search in `512750b21d6`.

| Height cap `H` | Discriminant tests | Rational points | Distinct reduced targets | Other target status |
| ---: | ---: | ---: | ---: | --- |
| 16 | 319 | 4 | 4/42 | 38 cap exhausted |
| 64 | 5,039 | 4 | 4/42 | 38 cap exhausted |
| 256 | 79,791 | 4 | 4/42 | 38 cap exhausted |
| 1,024 | 1,275,855 | 4 | 4/42 | 38 cap exhausted |

The [four exact rows](point-rows.jsonl), [result and raw digest](result.json), and empty runner stderr are retained. An [independent Sage census](sage_audit.py) checks every canonical pair using Sage integer-square arithmetic, constructs each positive point on the rational elliptic curve, verifies its reduction on `E(F₄₇)`, and matches the full row file and all four checkpoints. Its [result](sage-audit-result.json) passes under the accepted [runtime](sage-runtime-info.json), with empty audit stderr.

This is a finite coverage and work measurement for the specified curve, height definition, and input law. It does not estimate the height distribution on other rational curves or prove a lower bound for other rational-lift algorithms. The earlier [fixed-multiple Lean theorem](../xedni_fixed_lift_map_20261120/formal/LiftIndex.lean) concerns a different lift rule; this search produces its four points directly from bounded coordinates. A next experiment should use a target-conditioned rational-point finder, record its full search cost per public point, and test whether its found points supply independent rows on a shared curve.

Replay from the repository root:

```sh
python3 research/exotic_ecdlp_20261021/xedni_target_first_height_scan_20261120/run.py > research/exotic_ecdlp_20261021/xedni_target_first_height_scan_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_target_first_height_scan_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_target_first_height_scan_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_target_first_height_scan_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_target_first_height_scan_20261120/sage-audit-result.json 2> research/exotic_ecdlp_20261021/xedni_target_first_height_scan_20261120/sage-audit.stderr
```
