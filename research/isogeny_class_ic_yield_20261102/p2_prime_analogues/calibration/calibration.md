# Calibration runs (builder, not data of the protocol)

One curve (the first of class A of each size), 1 target, 1 warm-up round + 2 measured rounds, arms main and pc, host load average 5-7 on 4 cores, isolation L1 (--allow-busy). Specs in this directory were written by an earlier build of `p2tool spec` (m=3 attempt counts are 3x too small there: the yield law was refitted afterwards); per-run process times below are for those attempt counts. Used only to size the full run.

| class | m | arm | \|F\| | yield (relations/attempt) | attempts/run | process wall s/run | oracle_setup wall s |
|--|--|--|--|--|--|--|--|
| c24A | 2 | m2-main | 370 | 0.0199 | 4980 | 0.03 | 0.02 |
| c24A | 2 | m2-pc | 405 | 0.0243 | 4980 | 0.04 | 0.04 |
| c24A | 3 | m3-main | 70 | 0.0299 | 418 | 0.01 | 0.00 |
| c24A | 3 | m3-pc | 74 | 0.0359 | 418 | 0.01 | 0.00 |
| c28A | 2 | m2-main | 1291 | 0.019 | 4993 | 0.35 | 0.34 |
| c28A | 2 | m2-pc | 1414 | 0.0235 | 4993 | 0.50 | 0.51 |
| c28A | 3 | m3-main | 161 | 0.0327 | 964 | 0.05 | 0.00 |
| c28A | 3 | m3-pc | 171 | 0.041 | 964 | 0.05 | 0.00 |
| c32A | 2 | m2-main | 1448 | 0.00098 | 98426 | 0.61 | 0.50 |
| c32A | 2 | m2-pc | 1586 | 0.00119 | 98426 | 0.80 | 0.71 |
| c32A | 3 | m3-main | 470 | 0.0367 | 994 | 0.19 | 0.03 |
| c32A | 3 | m3-pc | 499 | 0.0453 | 994 | 0.18 | 0.04 |
| c36A | 2 | m2-main | 1448 | 6.98e-05 | 1569123 | 1.90 | 0.67 |
| c36A | 2 | m2-pc | 1586 | 8.35e-05 | 1569123 | 1.83 | 0.89 |
| c36A | 3 | m3-main | 1181 | 0.038 | 999 | 0.89 | 0.35 |
| c36A | 3 | m3-pc | 1255 | 0.043 | 999 | 0.99 | 0.41 |
| c40A | 2 | m2-main | 1448 | 4.34e-06 | 25809707 | 22.26 | 0.63 |
| c40A | 2 | m2-pc | 1586 | 5.06e-06 | 25809707 | 23.12 | 0.78 |
| c40A | 3 | m3-main | 1448 | 0.0037 | 8913 | 7.03 | 0.70 |
| c40A | 3 | m3-pc | 1539 | 0.0046 | 8913 | 8.02 | 0.93 |
