# Leaf-sum coordinates complete the N31 F4 solve with less matrix work

The invertible `s=u+v` leaf coordinate map reduced msolve's work on the frozen ordinary N31 residual `R=(551855874,2085658254)`. The candidate completed between two fresh same-binary controls and its two roots mapped back to the exact control roots. Checked Sage independently lifted both ordered root pairs to the frozen 124-point factor base and verified their sum to the residual and relation query.

| One residual, msolve 0.11.0, one thread, 256-pair batches | Control before | Leaf-sum | Control after |
| --- | ---: | ---: | ---: |
| Status | Completed | Completed | Completed |
| Rows reduced | 35,540 | **29,490** | 35,540 |
| Pairs reduced | 17,735 | **14,730** | 17,735 |
| Zero reductions | 8,189 | 7,109 | 8,189 |
| Largest matrix | 29,163 × 32,752 | **28,859 × 32,184** | 29,163 × 32,752 |
| Process wall, local | 92.416 s | **67.279 s** | 92.512 s |
| Logged CPU, local | 85.96 s | **63.55 s** | 86.00 s |
| Sampled process-tree peak RSS | 661,028,864 B | 524,812,288 B | 591,855,616 B |

The exact reduction is **6,050 rows (17.0%)** and **3,005 pairs (16.9%)** relative to either control. The mean of the two control wall intervals divided by the candidate interval is **1.374**. This is a complete, point-verified PDP solver component on one frozen residual. The local wall ratio is exploratory under the repository CPU isolation contract; a controlled performance claim requires repeated paired runs on an isolated or frozen cloud envelope, including equation construction and target-dependent setup.

The [protocol](PROTOCOL.md) fixes the original and transformed inputs, same msolve and FLINT hashes, run order, caps, and correctness checks. Exact postmeasurement copies of both inputs are in [source_inputs](source_inputs), with their hashes checked against the frozen paths in [SOURCE_MIRROR.json](SOURCE_MIRROR.json). The [runner](run.py), [run receipt](run_receipt.json), and raw logs retain all three arms, including the earlier control's immutable receipt. The [validator](validate.py) and [validation receipt](validation.json) bind the identical original-root set, the archived independent equation-map checks, and direct curve-point replay. The checked [Sage runtime receipt](sage_runtime_info.json) was saved before the run. The older msolve 0.9.5 leaf-sum run reached its wall cap under heavy host contention; this current 0.11.0 run gives a completed same-binary comparison.

The next test should repeat alternating control and leaf-sum solves on a Tier 1 or Tier 0 host, charge the `s=u+v` conversion and equation build, and include failed ordinary residual attempts. The static rank-preserving column projection from the adjacent G2 packets remains a separate solver integration question.
