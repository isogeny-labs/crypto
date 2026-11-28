# Five distinct n29 target pairs

The frozen n29 candidate recovered **five distinct previously unseen subgroup target scalars**; installed Sage 20-partition rho independently recovered each same-point scalar. The median paired target-online ratio was **44.67** (`rho / IC`) on one ordinary local macOS host. Every pre/post 512-addition calibration ratio passed the frozen 1.25 gate. The result is a local exploratory panel without a dedicated Tier 0 or Tier 1 allocation receipt.

| Target | Order | IC online (ms) | Rho online (ms) | Rho/IC | Calibration ratio | Shift scans | Result |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 0 | IC, rho | 6.031 | 2,398.745 | 397.76 | 1.142 | 1 | both verified |
| 1 | rho, IC | 16.209 | 408.966 | 25.23 | 1.118 | 2 | both verified |
| 2 | IC, rho | 11.788 | 526.554 | 44.67 | 1.016 | 1 | both verified |
| 3 | rho, IC | 8.712 | 366.929 | 42.12 | 1.046 | 1 | both verified |
| 4 | IC, rho | 5.007 | 958.401 | 191.40 | 1.030 | 1 | both verified |

The exact public points, SHA256 scalar law, workload IDs, and exclusions from all 2,575 prior relation-query scalars and the earlier full-pipeline target are in [`fixtures.json`](fixtures.json). The candidate is `IC1N29Ch4randfb1072PDP3mitmRCsampleLAgaussTDdescentISO0hb60653be7f42`, on curve `EC1N29Ch4randh1775caf67a64`, subgroup order 268,447,807. The candidate, prior result, fixture generator, fixture file, and panel runner hashes were frozen in [`protocol.json`](protocol.json) and committed as `11653a60289` before timing.

The panel rebuilt the base and first-pair index and replayed all 536 factor-base logs before any target clock. That shared restoration took **10.946 s**; it is distinct from the **29.537 s** complete cold preparation measured in the original candidate packet. Each IC interval starts with the public target point and ready precomputation, includes all failed shift attempts, group checks, scalar descent, and original-point replay. Each rho interval covers the target-dependent Sage walk and scalar replay. The fixture scalar is outside both intervals. Rho used a fresh frozen Sage seed for each point and no cross-target table.

[`result.json`](result.json) retains all five arm orders, five exclusive IC phases, scalar certificates, target-shift traces, calibration intervals, caps, and peak RSS. [`verify.py`](verify.py) independently rebuilt the 573,927-point pair-sum set, checked the fixture selection law and all five workload IDs, replayed each shift and first-hit or exhaustion result, checked the signed log equation, and replayed both scalar certificates. Its [`validation.json`](validation.json) passed; the five [`runs.jsonl`](runs.jsonl) rows pass the [IC v2 contract](contract-validation.txt). The checked Sage startup receipt is [`sage-runtime-info.json`](sage-runtime-info.json).

The exact resampling interval for the median of these five observed paired ratios is [25.23, 397.76], reflecting substantial rho variation and small panel size. This interval describes the five local pairs; it is not an isolated-host performance claim. The candidate's algebraic target-shift proof and the concrete matrix audit are in the prior full-pipeline packet. The next measurement is an isolated Tier 1 panel with the same candidate, fresh targets, alternating order, and pre/post calibration gate.

Reproduce the audit from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/n29_local_panel_20261120/verify.py
```
