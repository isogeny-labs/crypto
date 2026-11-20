# H4 n=31 direct-integer pair index

An injective integer-key pair index retained the exact 2,116,611 sums of the n=31 first-pair index and reproduced all 32 frozen first-hit witnesses and attempt counts in both repetitions. It reduced completed-index peak process RSS by 58.3% and 64.4% against matched point-key controls on one ordinary local macOS host. Index construction took 6.4% and 10.7% longer, respectively. The improvement is to the reusable index's memory footprint; it changes neither the curve, base, pair policy, nor target law.

| Arm in frozen order | Index build s | Peak process RSS bytes | Distinct sums | Exact target outcomes |
| --- | ---: | ---: | ---: | ---: |
| Point-key 1 | 34.184 | 1,379,844,096 | 2,116,611 | 32/32 |
| Integer-key 1 | 36.370 | 574,980,096 | 2,116,611 | 32/32 |
| Integer-key 2 | 37.295 | 534,380,544 | 2,116,611 | 32/32 |
| Point-key 2 | 33.677 | 1,502,085,120 | 2,116,611 | 32/32 |

The key is `int(x.to_integer()) | (int(y.to_integer()) << 31)` for a nonidentity point, with `1 << 62` reserved for identity. The payload is `i*2090+j` for the first ordered pair at each sum. The checked [Lean theorem](formal/PackedPointKey.lean) proves the 62-bit coordinate packing is injective under the exact coordinate bounds and cannot collide with the sentinel; [the proof map](formal/PROOF_MAP.md) states the Sage encoding bridge. The runner checks all base-point coordinates against the slower polynomial-bit encoding. The earlier [polynomial-conversion screen](../n31_packed_index_20261120/README.md) remains recorded with both 180-second censored arms.

[protocol.json](protocol.json) and [run.py](run.py) were committed before execution in `20173937df6`. Four separate checked-Sage processes ran in the frozen `point, integer, integer, point` order; [sage-runtime-info.json](sage-runtime-info.json) identifies the build. [audit.py](audit.py) independently checks source hashes, all four complete index counts, and every recorded target outcome against the earlier audited point-index diagnostic; [audit.json](audit.json) passed. The timings are local component measurements. A larger-base experiment with the same memory cap is the next way to test whether the saved memory extends the practical index size, with its setup and conversion costs charged.
