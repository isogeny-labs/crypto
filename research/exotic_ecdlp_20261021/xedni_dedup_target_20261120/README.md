# Duplicate-residue filtering in xedni target search

Target-independent filtering of repeated reduced residues cut exact rational
candidate evaluations from **9,881 to 985** and **3,435 to 407** for two
fresh `p=1531` public points. Their paired ordinary-host rational intervals
fell from 18.366 s to 1.967 s and from 1.078 s to 0.105 s. The three fresh
`p=2287` points reached identical first-match indices in the original and
filtered lists, exposing the cell-specific nature of the gain. All 18
baseline, filtered, and same-point rho arms returned verified scalars.

## Frozen protocol and paired results

The [protocol](protocol.json) and [runner](run_sage.py) were pushed in
`cc57ddc46b8` before execution. A new SHA-256 seed fixes three distinct
targets on each prime-order reduction, disjoint from the earlier six
fixtures. Each solver receives the same public point within its triple; arm
order rotates across the three targets. Baseline and filtered rational
search call the identical pinned solver and differ only in their prepared
candidate lists. The filtered list keeps the first height-ordered label for
each nonzero residue `(a+λb) mod q`. `λ=11` and `λ=1238` are verified source
relations available before the target input. Curve, score order, filtering,
and fixture construction are outside each online interval.

| p | Target index | Baseline candidates | Filtered candidates | Baseline rational ms | Filtered rational ms | Same-point rho ms |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1531 | 0 | 83 | 59 | 4.816 | 2.746 | 0.972 |
| 1531 | 1 | 9,881 | 985 | 18,366.465 | 1,966.887 | 0.554 |
| 1531 | 2 | 3,435 | 407 | 1,078.098 | 104.980 | 1.323 |
| 2287 | 0 | 999 | 999 | 121.612 | 121.434 | 1.805 |
| 2287 | 1 | 781 | 781 | 82.340 | 82.758 | 1.746 |
| 2287 | 2 | 1,335 | 1,335 | 195.700 | 195.624 | 1.102 |

The [arm rows](arm-rows.jsonl) retain each point, arm order, exact
nanosecond start/stop, full candidate or walk counts, status, and scalar.
Every arm is `verified` by scalar multiplication on its supplied public
point. The [fixture rows](target-fixtures.jsonl) retain the input law and
post-solve scalars; the [result](result.json) pins all source and raw-row
digests. Accepted [Sage runtime information](sage-runtime-info.json) was
saved before execution. The interval starts before the first target-dependent
rational construction or rho walk and stops after verified recovery. Failed
candidates and rho restarts are included.

The filtering cost is target-independent setup. At `p=1531`, 17,160 original
labels become 1,552 unique nonidentity labels; sorting took 14.607 ms and
filtering 0.927 ms within a 78.378 ms cell setup. At `p=2287`, 11,880 labels
become 2,250; sorting took 9.383 ms and filtering 0.722 ms within a
15.300 ms cell setup. Global rational-curve and pairing setup took
52.923 ms. The [setup rows](setup-rows.jsonl) retain full precision.
Source-`λ` discovery was measured as candidate work in earlier packets but
has no wall-time receipt in this packet, so these setup intervals are not a
complete cold total. The local host has no Tier 0/1 isolation receipt;
wall-time differences here are exploratory.

## Independent checks and interpretation

The [integer group-law audit](audit.py) rebuilds all six target points,
sorts candidates by exact rational scores, computes every reduced
`aQ̄+bR̄`, reconstructs the filtered list by first point occurrence, and
replays all six rho walks. Its [result](audit-result.json) is `PASS` for
both lists and all 18 arm rows. The earlier
[Lean scalar-control theorem](../xedni_fresh_target_pair_20261120/formal/ScalarControl.lean)
proves `aG+bR=(a+λb)G` under the explicit relation `R=λG`. The
[Lean first-hit floor](../xedni_first_hit_distribution_20261120/formal/FirstHitBound.lean)
explains why a list with one label per nonidentity residue has the minimum
possible mean sequential first-hit index. The curve-to-scalar bridge and
these exact finite lists are verified computationally.

After `λ` is known, the same candidate point can be evaluated entirely in
the finite group. The filtered rational arm quantifies the cost of retaining
exact rational point construction under that setup policy. In this panel,
same-point rho has the shorter ordinary-host interval on each target,
including the two targets with large filtering gains. A next cost case is
acquiring `λ` itself from the first rational collision, paired with rho on
`R̄` as the same public point and charging the rational source setup.

## Replay

From `/Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119`:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_dedup_target_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_dedup_target_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_dedup_target_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_dedup_target_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_dedup_target_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_dedup_target_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_dedup_target_20261120/audit.stderr
```
