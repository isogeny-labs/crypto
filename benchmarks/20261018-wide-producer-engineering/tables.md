
**Scan, n = 73, K=40, 40000 states, ns per probe**

| arm | median ns/probe | min-max | ratio to first arm | repeats |
|:--|--:|:--|--:|--:|
| control | 416.97 | 414.27-424.67 | 1.000 | 5 |
| lazy_canon | 209.88 | 202.99-223.13 | 0.503 | 5 |
| blocked_bloom | 432.96 | 417.69-483.87 | 1.038 | 5 |
| prefetch | 401.73 | 374.95-456.49 | 0.963 | 5 |
| all_three | 216.99 | 204.47-244.98 | 0.520 | 5 |

**Scan, n = 73, K=300, 20000 states, ns per probe**

| arm | median ns/probe | min-max | ratio to first arm | repeats |
|:--|--:|:--|--:|--:|
| control | 614.45 | 559.29-684.57 | 1.000 | 5 |
| lazy_canon | 366.98 | 349.31-416.87 | 0.597 | 5 |
| blocked_bloom | 575.97 | 525.78-598.69 | 0.937 | 5 |
| prefetch | 434.75 | 386.82-485.77 | 0.708 | 5 |
| all_three | 225.57 | 216.69-250.48 | 0.367 | 5 |

**Scan, n = 73, K=600, 20000 states, ns per probe**

| arm | median ns/probe | min-max | ratio to first arm | repeats |
|:--|--:|:--|--:|--:|
| control | 621.60 | 583.11-653.49 | 1.000 | 5 |
| lazy_canon | 382.68 | 372.01-433.08 | 0.616 | 5 |
| prefetch | 441.59 | 422.23-459.51 | 0.710 | 5 |
| all_three | 234.97 | 223.43-302.98 | 0.378 | 5 |

**Scan, n = 73, K=300 (second run: pair arms), ns per probe**

| arm | median ns/probe | min-max | ratio to first arm | repeats |
|:--|--:|:--|--:|--:|
| control | 573.60 | 558.18-659.95 | 1.000 | 5 |
| all_three | 226.75 | 207.68-309.63 | 0.395 | 5 |
| prefetch_blocked | 431.58 | 407.52-550.71 | 0.752 | 5 |
| lazy_prefetch | 232.76 | 225.93-272.25 | 0.406 | 5 |

**Index build, n = 73, K=300, seconds**

| arm | median s | min-max | ratio to first arm | repeats |
|:--|--:|:--|--:|--:|
| serial | 14.60 | 14.39-15.81 | 1.000 | 3 |
| batch_inv_1thread | 8.51 | 8.21-12.45 | 0.583 | 3 |
| threads_only | 5.83 | 5.79-6.40 | 0.399 | 3 |
| threads_batch_inv | 5.56 | 4.85-5.86 | 0.381 | 3 |

**Index build, n = 73, K=600, seconds**

| arm | median s | min-max | ratio to first arm | repeats |
|:--|--:|:--|--:|--:|
| serial | 66.38 | 60.61-72.14 | 1.000 | 2 |
| threads_batch_inv | 29.13 | 28.59-29.66 | 0.439 | 2 |
