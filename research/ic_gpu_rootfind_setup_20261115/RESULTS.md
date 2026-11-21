REV: 0c8c431185
bench-pairs: bench-pairs: n=1024 keys=1049600 per-cell 0.0004 s batched-runs 0.0001 s (4.0x) mismatches 0 | bench-pairs: n=2048 keys=4196352 per-cell 0.0012 s batched-runs 0.0002 s (5.1x) mismatches 0 | bench-pairs: n=4096 keys=16781312 per-cell 0.0047 s batched-runs 0.0008 s (5.7x) mismatches 0
Record status: rf3-cpu-echelon-F2048 {'verified': 12}; rf3-cpu-echelon-F4096 {'verified': 12}; rf3-gpup-echelon-F2048 {'verified': 12}; rf3-gpup-echelon-F4096 {'verified': 12}; rf3-gpup-echelon-aa-F4096 {'verified': 12}; rho-neg {'verified': 12}; rho-neg-aa {'verified': 12}
Identity checks against rf3-cpu-echelon: 36; mismatches: 0
GPU A/A per-pair solve ratio: 0.868–1.131 (n=12)

Pair build stages (daemon RF_SERVE_TIMING, median s): 
- n_fb=32 (n=8): keys 0.0003, dedupe 0.0000, compact 0.0000, probe 0.0000, copy 0.0000
- n_fb=2048 (n=24): keys 0.0013, dedupe 0.0020, compact 0.0020, probe 0.0016, copy 0.0155
- n_fb=4096 (n=48): keys 0.0019, dedupe 0.0069, compact 0.0051, probe 0.0029, copy 0.0473
- session create, n_probe=4194196: median 0.0341 s (0.0223–0.0456, n=12)
- session create, n_probe=4194286: median 0.0375 s (0.0212–0.0523, n=12)
- session create, n_probe=16775705: median 0.1050 s (0.0874–0.1396, n=24)
- session create, n_probe=16776850: median 0.1052 s (0.0854–0.1244, n=24)

| curve | log2 r | \|F_x\| | arm | S / rho (w=0 / 0.1 / 1) | solve s | relations s | set-up s | LA s | rho solve s |
|---|--:|--:|---|--:|--:|--:|--:|--:|--:|
| icv1-fp38-tm68843-fe40d997 | 37.4 | 2048 | cpu | 390 / 392 / 400 | 4.251 | 3.842 | 0.396 | 0.019 | 0.0325 |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 2048 | gpup | 390 / 392 / 400 | 0.115 | 0.051 | 0.040 | 0.023 | 0.0325 |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 4096 | cpu | 220 / 233 / 292 | 5.488 | 3.026 | 2.378 | 0.090 | 0.0325 |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 4096 | gpup | 220 / 233 / 292 | 0.255 | 0.035 | 0.116 | 0.096 | 0.0325 |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 4096 | gpup-aa | 220 / 233 / 292 | 0.248 | 0.034 | 0.112 | 0.095 | 0.0325 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 2048 | cpu | 685 / 685 / 689 | 15.659 | 15.241 | 0.392 | 0.024 | 0.0690 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 2048 | gpup | 685 / 685 / 689 | 0.220 | 0.152 | 0.041 | 0.022 | 0.0690 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 4096 | cpu | 352 / 357 / 382 | 15.512 | 12.458 | 2.971 | 0.097 | 0.0690 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 4096 | gpup | 352 / 357 / 382 | 0.292 | 0.075 | 0.109 | 0.104 | 0.0690 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 4096 | gpup-aa | 352 / 357 / 382 | 0.284 | 0.073 | 0.109 | 0.099 | 0.0690 |
