REV: 8c5ae6a512
Daemon: daemon start to socket: 0.242 s
cgroup: cat: /sys/fs/cgroup/cpu.max: No such file or directory | 1360000 | 100000
Record status: rf3-cpu-echelon-F2048 {'verified': 12}; rf3-cpu-echelon-F4096 {'verified': 12}; rf3-cpu-gauss-F2048 {'verified': 12}; rf3-cpu-gauss-F4096 {'verified': 12}; rf3-gpup-echelon-F2048 {'verified': 12}; rf3-gpup-echelon-F4096 {'verified': 12}; rf3-gpup-echelon-aa-F4096 {'verified': 12}; rf3-gpup-gauss-F2048 {'verified': 12}; rf3-gpup-gauss-F4096 {'verified': 12}; rho-neg {'verified': 12}; rho-neg-aa {'verified': 12}
Identity checks against rf3-cpu-gauss: 84; mismatches: 0
A/A (gpup-echelon-aa / gpup-echelon solve, per pair): 0.98–1.07 (n=12)

| curve | log2 r | \|F_x\| | arm | S / rho | solve s | relations s | set-up s | LA s | LA row_ops | solve echelon/gauss | LA row_ops echelon/gauss |
|---|--:|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|
| icv1-fp38-tm68843-fe40d997 | 37.4 | 2048 | cpu-echelon | 340 | 5.386 | 4.895 | 0.458 | 0.023 | 6,461,066 | 0.95 | 0.142 |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 2048 | cpu-gauss | 340 | 5.657 | 4.976 | 0.473 | 0.207 | 45,484,662 | nan | nan |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 2048 | gpup-echelon | 340 | 0.118 | 0.055 | 0.038 | 0.023 | 6,461,066 | 0.41 | 0.142 |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 2048 | gpup-gauss | 340 | 0.287 | 0.052 | 0.042 | 0.192 | 45,484,662 | nan | nan |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 4096 | cpu-echelon | 194 | 6.403 | 3.667 | 2.589 | 0.125 | 48,315,550 | 0.79 | 0.134 |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 4096 | cpu-gauss | 194 | 8.088 | 4.065 | 2.609 | 1.429 | 361,156,545 | nan | nan |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 4096 | gpup-echelon | 194 | 0.280 | 0.037 | 0.122 | 0.117 | 48,315,550 | 0.18 | 0.134 |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 4096 | gpup-echelon-aa | 194 | 0.284 | 0.037 | 0.130 | 0.117 | 48,315,550 | nan | nan |
| icv1-fp38-tm68843-fe40d997 | 37.4 | 4096 | gpup-gauss | 194 | 1.551 | 0.040 | 0.123 | 1.382 | 361,156,545 | nan | nan |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 2048 | cpu-echelon | 737 | 19.715 | 19.214 | 0.468 | 0.024 | 5,897,608 | 0.98 | 0.140 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 2048 | cpu-gauss | 737 | 20.168 | 19.459 | 0.466 | 0.216 | 42,239,714 | nan | nan |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 2048 | gpup-echelon | 737 | 0.223 | 0.162 | 0.032 | 0.024 | 5,897,608 | 0.58 | 0.140 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 2048 | gpup-gauss | 737 | 0.387 | 0.153 | 0.038 | 0.189 | 42,239,714 | nan | nan |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 4096 | cpu-echelon | 383 | 17.780 | 15.106 | 2.557 | 0.127 | 42,778,046 | 0.89 | 0.129 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 4096 | cpu-gauss | 383 | 20.021 | 15.993 | 2.569 | 1.386 | 331,130,846 | nan | nan |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 4096 | gpup-echelon | 383 | 0.305 | 0.076 | 0.121 | 0.104 | 42,778,046 | 0.20 | 0.129 |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 4096 | gpup-echelon-aa | 383 | 0.307 | 0.077 | 0.123 | 0.105 | 42,778,046 | nan | nan |
| icv1-fp40-tm106485-8d2c6f92 | 39.3 | 4096 | gpup-gauss | 383 | 1.541 | 0.081 | 0.119 | 1.335 | 331,130,846 | nan | nan |
