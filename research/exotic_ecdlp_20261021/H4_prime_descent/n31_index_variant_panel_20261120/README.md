# H4 n=31 paired index-variant target panel

On ten fresh, precommitted public points in the same 34,635,389-order subgroup, both complete frozen IC candidates and installed Sage 20-partition rho recovered and independently replayed every scalar. The arm order rotated among point-key IC, direct-integer-key IC, and rho. All ten pre/post 512-addition calibration ratios passed the frozen 1.25 gate. The median paired point-key/packed-key IC target-online ratio was **1.0186** on this ordinary local host; the observed paired ratios ranged from **0.9898 to 1.0542**. A post-hoc paired-target bootstrap gives a 95% median-ratio interval of **[1.0024, 1.0533]** under target resampling only. That interval excludes host, run-order, and Tier 0/1 isolation uncertainty, so the small difference remains exploratory.

| Target | Frozen arm order | Point-key IC ms | Integer-key IC ms | Same-point rho ms | Calibration ratio |
| ---: | --- | ---: | ---: | ---: | ---: |
| 0 | point, integer, rho | 7.146 | 6.784 | 385.352 | 1.116 |
| 1 | integer, rho, point | 7.795 | 7.562 | 436.659 | 1.175 |
| 2 | rho, point, integer | 7.429 | 7.397 | 614.967 | 1.094 |
| 3 | point, integer, rho | 7.259 | 7.255 | 530.320 | 1.022 |
| 4 | integer, rho, point | 7.779 | 7.459 | 603.029 | 1.021 |
| 5 | rho, point, integer | 6.952 | 7.023 | 241.328 | 1.010 |
| 6 | point, integer, rho | 7.028 | 6.998 | 192.037 | 1.039 |
| 7 | integer, rho, point | 7.080 | 6.717 | 234.533 | 1.047 |
| 8 | rho, point, integer | 7.305 | 6.929 | 273.413 | 1.004 |
| 9 | point, integer, rho | 7.089 | 7.044 | 246.964 | 1.045 |

Both indexes were built and all 1,045 representative factor logs replayed before any target clock; this joint setup took 74.783 s and is recorded once. Its co-resident 1,255,800,832-byte process peak is not a variant memory comparison. The separately measured complete candidates retain their own empty-cache cold costs and peak RSS. Each one-target row includes all five exclusive IC target-online phases, every direct or shifted scan, target scalar replay, and same-point rho scalar replay. The median rho/point-key and rho/integer-key ratios were 45.68 and 48.13 on this host.

The [protocol](protocol.json), [runner](run.py), and exact [fixture points](fixtures.json) were committed before measurement in `f9bb53a3946`. The fixtures exclude all 2,737 prior relation scalars, the original full-pipeline target, and the earlier five-target panel. The [checked Sage runtime](sage-runtime-info.json) and [result](result.json) retain the environment, shared setup, all arm outcomes, timing phases, calibration, and peak memory. [verify.py](verify.py) independently rebuilds the curve and base, uses a point-only pair-sum set to audit first-hit and exhausted scans, checks signed log equations and every original-point scalar, and validates exact source and fixture hashes. Its [validation receipt](validation.json) passed, and all twenty [IC v2 one-target rows](runs.jsonl) passed the measurement contract in [contract-validation.txt](contract-validation.txt). [analyze.py](analyze.py) reproduces the labelled post-hoc paired bootstrap in [analysis.json](analysis.json).

The next CPU comparison is the same frozen candidate and target design on a Tier 1 instance or isolated Tier 0 host, with alternating order and the tier's topology, cgroup, frequency, and noise receipts. The local panel establishes correct paired behavior and a small observed online difference; the component and complete-candidate packets establish the larger memory reduction separately.
