# Height-pairing order lowers every target's rational prefix coordinate peak

**Ordering the fixed 80 rational lift candidates by a rank-two height-pairing score lowers the largest encountered coordinate size for all 42 public targets.** The median/maximum prefix peak is 148/200 bits, compared with 366/656 bits under the frozen radius order. The height order reaches full reduced-point coverage at candidate 44 rather than 69. Its first useful collision occurs at candidate 35 rather than 31, so the median one-target stop increases from 31 to 35 candidate evaluations; the summed evaluation count over 42 independent searches falls from 1,421 to 1,270.

The [protocol](protocol.json) freezes Sage's 128-bit canonical-height pairing for exact rational points `Q=(-4,19)` and `R=(52242961/137641,-1907191742616/51064811)` on `y²+94xy+188y=x³+1738x+3805`. The score for a pair `(a,b)` is `h(Q)a²+2⟨Q,R⟩ab+h(R)b²`, evaluated from the pinned decimal strings and tied by `a,b`. Computing those three pairing entries is target-independent setup. The score can be evaluated for every coefficient pair before constructing any of the 80 rational candidate points; it is a deterministic ordering heuristic, not an exact formula for each candidate's coordinate bit length. The [Sage runner](run_sage.py) recomputes and checks all three pinned entries before sorting, then constructs and reduces candidates in score order.

| Frozen measure on the same 42 public points | Radius order | Height-pairing order |
| --- | ---: | ---: |
| First useful reduction collision | 31 | 35 |
| Prefix with all 42 reduced points | 69 | 44 |
| One-target stop, min / median / max evaluations | 2 / 31 / 69 | 1 / 35 / 44 |
| Sum over independent one-target searches | 1,421 | 1,270 |
| Prefix peak coordinate bits at stop, median / max | 366 / 656 | 148 / 200 |

The height order uses fewer evaluations on 14 targets and more on 28; it lowers the coordinate peak on all 42 paired targets. Its first equal-reduction pair is `(3,2)` at index 32 and `(-4,-2)` at index 35. Their coefficient difference `(-7,-4)` yields `-7G-4R̄=0`, hence `R̄=9G` in the 43-point reduced subgroup. The pinned [Lean theorem](formal/HeightCollision.lean) proves this exact implication for arbitrary `G,R : ZMod 43`; the [build output](formal/build-output.txt) reports only standard `propext`, `Classical.choice`, and `Quot.sound` axioms under Lean `v4.35.0-rc4` and Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9`. The rational curve, pairing computation, ordered 80-point census, and scalar replays are computationally checked; their full Lean bridge remains open. `lean4checker --fresh` is unavailable on this host.

The protocol and runner were pushed before measurement in `2b8deea9e89`. The [result](result.json), [80 prefix rows](prefix-rows.jsonl), and [42 target rows](target-rows.jsonl) retain exact digests, collision, first-hit and stop counts, coordinate peaks, and scalar checks. An [independent Decimal/Fraction audit](audit.py) rebuilds the score order, every rational point and reduction, per-target stop, and scalar replay; its [result](audit-result.json) passes. The accepted [Sage runtime](sage-runtime-info.json) and empty stderr are retained.

This is a paired construction-order result for one fixed curve and coefficient box. A point evaluation is rational scalar arithmetic whose work varies with coordinate size; the count is not a calibrated operation or wall-time unit. The height-pairing computation, curve and rational generators are setup inputs here. The next experiment should time a complete one-target search with the pairing computation disclosed separately, include all rational arithmetic and reduction work, and solve the same public point with rho under a matched resource envelope.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/audit.stderr
sh research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/formal/build-output.txt 2> research/exotic_ecdlp_20261021/xedni_height_order_prefix_20261120/formal/build.stderr
```
