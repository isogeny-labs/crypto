# Public-coordinate filtering finds four bounded-height rational lifts

**Filtering rational `x=a/b²` candidates by the public target's `x` coordinate finds exact rational lifts for four of the 42 nonidentity `E(F₄₇)` inputs on one fixed witness curve.** Each of those four targets takes one discriminant test and reaches its lift by height 4 or 8. The other 38 targets exhaust the frozen height cap 1,024. The method uses only the public point coordinates and the fixed rational curve; it does not read a scalar index.

The [protocol](protocol.json) and [runner](run.py) were pushed before measurement in `e526e3f29f1`. For each public target `(x_t,y_t)`, the runner enumerates canonical coprime `a,b` in increasing `H=max(|a|,b)`, generates only `a≡x_t b² (mod 47)`, tests the integral elliptic-curve discriminant, and accepts the first rational root reducing to `y_t`. Every target receives a [separate row](target-rows.jsonl), including the 38 `height_cap_exhausted` outcomes.

| Result at `H≤1024` | Targets | Discriminant tests per target | Residue candidate pairs per target | `b` loop steps per target |
| --- | ---: | ---: | ---: | ---: |
| Rational lift found | 4 | 1 | 1–2 | 7–29 |
| Height cap exhausted | 38 | 26,585–26,611 | 43,737–43,742 | 524,800 |

Summed across all 42 independent searches, the run uses 1,010,754 discriminant tests, 1,662,098 residue candidate pairs, and 19,942,472 `b` loop steps. These are exact work counts, not calibrated operation or wall-time comparisons. An [independent checked-Sage audit](sage_audit.py) traverses the bounded rational candidates once, updates all active target counters, validates each found point over `Q` and its reduction over `F₄₇`, and matches every row and the [result digest](result.json). Its [result](sage-audit-result.json) is `PASS`; the accepted [Sage runtime](sage-runtime-info.json) and empty stderr files are retained.

This per-target filter saves discriminant tests relative to enumerating all rational points for one public input. Its bounded coverage is still 4/42 on the exact frozen input law, with the remaining target searches explicitly censored. The [full-height census](../xedni_target_first_height_scan_20261120/README.md) independently establishes that these are all rational points in this coordinate-height box. A further construction needs to expand reachable points on the fixed curve or select a new curve from the public target while retaining a usable shared relation system; both construction and target-dependent search work must be charged.

Replay from the repository root:

```sh
python3 research/exotic_ecdlp_20261021/xedni_target_conditioned_lift_20261120/run.py > research/exotic_ecdlp_20261021/xedni_target_conditioned_lift_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_target_conditioned_lift_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_target_conditioned_lift_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_target_conditioned_lift_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_target_conditioned_lift_20261120/sage-audit-result.json 2> research/exotic_ecdlp_20261021/xedni_target_conditioned_lift_20261120/sage-audit.stderr
```
