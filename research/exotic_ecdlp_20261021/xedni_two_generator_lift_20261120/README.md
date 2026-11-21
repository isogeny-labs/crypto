# A rank-two rational lift box covers the 43-point reduced group

**On the fixed rational witness curve, the 81 combinations `aQ+bR` with `-4≤a,b≤4` reduce onto all 43 points of `E(F₄₇)`.** Here `Q=(-4,19)` and `R=(52242961/137641, -1907191742616/51064811)` are exact rational points. Sage's Simon two-descent reports rank bounds `(2,2)` with these known points; the reduction of `R` is `(27,14)=9G`, where `G` is the reduction of `Q`. A pinned Lean theorem independently proves that the centered 9-by-9 coefficient box covers every scalar in `ZMod 43` under the exact relation `R̄=9G`.

The [box protocol](protocol.json) and [Sage runner](run_sage.py) were pushed before measurement in `2f8a35237fe`; commit `3be54453fef` corrected Sage integer serialization before the successful recorded run. The [81 rows](box-rows.jsonl) retain coefficient pairs, reduced points, exact rational-coordinate hashes and bit lengths. Among the 80 nonidentity combinations, every one of the 42 nonidentity reduced targets appears. For each target the [paired rows](comparison-rows.jsonl) select the smallest coordinate-size representative in the box.

The first comparison used the previously frozen positive `kQ` enumeration. A separate [centered control protocol](centered-control-protocol.json) and [runner](run_centered_sage.py) were pushed before that control in `bd7916fc53c`. It compares the natural 42 nonidentity single-generator entries `kQ` for `-21≤k≤21` with the 81-entry two-generator box. Coordinate height is the maximum bit length among the numerators and denominators of rational `x` and `y`.

| Complete lookup table | Entries | Median minimum height across 42 targets | Maximum minimum height |
| --- | ---: | ---: | ---: |
| Centered one-generator `kQ` | 42 | 206 bits | 729 bits |
| Two-generator `aQ+bR` | 81 | 56 bits | 200 bits |

The two-generator representative has smaller height on 31 of 42 targets. The initial positive-index `kQ` table has median/max 765/2,904 bits and is retained in the [box result](result.json) as a diagnostic, not the preferred control. The [centered result](centered-result.json) and [42 paired rows](centered-comparison-rows.jsonl) fix the fairer control and raw digest. These are exact coordinate sizes and table counts; no calibrated operation or wall-time comparison was made.

An [independent Fraction group-law audit](audit.py) reconstructs every one of the 81 rational box points and all 42 centered control points, checks their coordinate hashes, bit lengths and reductions, and matches both row digests. Its [result](audit-result.json) passes. The accepted [Sage runtime](sage-runtime-info.json), successful runner stderr, and centered-control stderr are retained; the main stderr contains only a Sage two-descent deprecation warning.

## Proof map and boundary

The pinned [Lean module](formal/BoxCoverage.lean) defines `centered : Fin 9 → ZMod 43` and proves `box_covers : ∀ t : ZMod 43, ∃ a b : Fin 9, t = centered a + 9 * centered b`. The actual [build output](formal/build-output.txt) reports only standard `propext`, `Classical.choice`, and `Quot.sound` axioms under Lean `v4.35.0-rc4` and Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9`; it contains no unproved placeholders. The existence of the rational points, good reduction, and `R̄=9G` are checked by exact Sage and Fraction arithmetic rather than encoded as Lean elliptic-curve objects. That curve-to-field bridge remains the formalization limit. `lean4checker --fresh` is unavailable on this host.

The construction is a complete precomputed lift lookup for this fixed 43-point reduced group. Its 81 rational combinations and their exact arithmetic are target-independent setup, while lookup is target-dependent; a single-target comparison must charge or separately disclose that setup according to the declared metric. The result shows shorter representatives for one rank-two curve. It does not establish a general height law, a relation matrix of sufficient rank, or an end-to-end IC speedup. The next structural test is to form rational relations among these 81 lifts on this same curve, reduce them to 42 target columns, and measure independent row rank and construction cost.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/run_centered_sage.py > research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/centered-result.json 2> research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/centered.stderr
python3 research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/audit.stderr
sh research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/formal/build-output.txt 2> research/exotic_ecdlp_20261021/xedni_two_generator_lift_20261120/formal/build.stderr
```
