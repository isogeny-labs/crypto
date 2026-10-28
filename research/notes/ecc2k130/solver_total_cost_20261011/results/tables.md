<!-- t1_headline -->
| Engine (W1a, 12 inputs) | complete | natural relations / natural attempts | planted verified | solver stage ms/input (CPU) | total T_core ms/input (CPU) | solver share f_s | Amdahl bound g |
|:--|--:|--:|--:|--:|--:|--:|--:|
| FES | 12/12 | 0/8 | 4/4 | 0.38 | 79.21 | 0.5% | 1.00x |
| CryptoMiniSat (native XOR) | 12/12 | 0/8 | 4/4 | 8.31 | 82.15 | 9.9% | 1.11x |
| Z3 (And/Xor) | 12/12 | 0/8 | 4/4 | 76.04 | 154.85 | 50.5% | 2.02x |
| F4-style prototype | 12/12 | 0/8 | 4/4 | 138.18 | 211.40 | 65.7% | 2.91x |
| F5B prototype | 12/12 | 0/8 | 4/4 | 91.22 | 167.48 | 55.1% | 2.22x |

<!-- t2_stages -->
| Engine (mean CPU ms per input) | fb setup | input gen | eq build | engine load | solver setup | solve | model extr. | lifting | verif. | rank upd. | T_core (A) |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| FES | 53.140 | 23.686 | 7.371 | 0.000 | 0.000 | 0.304 | 0.074 | 10.443 | 5.076 | 0.011 | 79.21 |
| CryptoMiniSat (native XOR) | 51.804 | 21.874 | 7.249 | 1.728 | 0.677 | 7.468 | 0.160 | 10.336 | 5.843 | 0.012 | 82.15 |
| Z3 (And/Xor) | 52.039 | 21.638 | 6.793 | 23.795 | 66.282 | 9.343 | 0.430 | 10.900 | 5.440 | 0.012 | 154.85 |
| F4-style prototype | 49.585 | 22.153 | 7.050 | 0.000 | 0.000 | 137.528 | 0.661 | 10.634 | 5.378 | 0.013 | 211.40 |
| F5B prototype | 52.447 | 22.985 | 6.875 | 3.369 | 0.283 | 90.815 | 0.128 | 11.057 | 5.977 | 0.013 | 167.48 |

<!-- t3_accountings -->
| Engine (W1a) | A: core, cold ms/input | f_s (A) | B: setup amortised ms/input | f_s (B) | C: fully loaded ms/input | f_s (C) | A in wall ms/input | f_s (A, wall) |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| FES | 79.21 | 0.5% | 32.85 | 1.1% | 102.90 | 0.4% | 80.16 | 0.5% |
| CryptoMiniSat (native XOR) | 82.15 | 9.9% | 39.89 | 20.9% | 105.70 | 7.7% | 83.11 | 10.0% |
| Z3 (And/Xor) | 154.85 | 50.5% | 108.77 | 69.9% | 200.30 | 38.7% | 159.04 | 50.4% |
| F4-style prototype | 211.40 | 65.7% | 170.08 | 81.5% | 233.55 | 59.5% | 213.92 | 65.5% |
| F5B prototype | 167.48 | 55.1% | 123.77 | 73.7% | 193.83 | 47.7% | 168.89 | 55.6% |

<!-- t4_ordering -->
| W1a ranking comparison | value |
|:--|--:|
| order by solver stage (T_solver, CPU) | FES < CryptoMiniSat (native XOR) < Z3 (And/Xor) < F5B prototype < F4-style prototype |
| order by total cost (T_core, A, CPU) | FES < CryptoMiniSat (native XOR) < Z3 (And/Xor) < F5B prototype < F4-style prototype |
| Kendall tau between the two orderings | 1.00 |
| order by T_core under B (setup amortised) | FES < CryptoMiniSat (native XOR) < Z3 (And/Xor) < F5B prototype < F4-style prototype |
| order by T_core under C (fully loaded) | FES < CryptoMiniSat (native XOR) < F5B prototype < Z3 (And/Xor) < F4-style prototype |
| Kendall tau, solver stage vs T_core under B / under C | 1.00 / 0.80 |
| max/min over engines, solver stage | 365.4x |
| max/min over engines, total T_core (A) | 2.7x |
| max/min over engines, m83-definition solver stage (incl. library load) | 567.7x |
| post hoc steady state: max/min over engines, T_ss | 19.9x |
| post hoc steady state: Kendall tau, solver stage vs T_ss | 1.00 |

<!-- t5_cost_per_relation -->
| Engine (W1a) | new independent natural relations / attempts | CPU ms per natural attempt (T_core) | CPU ms per verified planted relation | 95% upper bound on yield/attempt | CPU s per relation, measured 95% lower bound | CPU s per relation, derived lower bound (uniform targets) |
|:--|--:|--:|--:|--:|--:|--:|
| FES | 0/8 | 58.53 | 107.09 | 0.312 | 0.187 | 1.01e+21 |
| CryptoMiniSat (native XOR) | 0/8 | 68.65 | 109.15 | 0.312 | 0.220 | 1.19e+21 |
| Z3 (And/Xor) | 0/8 | 138.29 | 185.20 | 0.312 | 0.443 | 2.39e+21 |
| F4-style prototype | 0/8 | 195.13 | 243.93 | 0.312 | 0.625 | 3.37e+21 |
| F5B prototype | 0/8 | 151.30 | 195.29 | 0.312 | 0.484 | 2.61e+21 |

<!-- t6_w1b -->
| s | Engine | complete / timeout / setup_failed (cells, median over reps) | cells whose status changed between repetitions | solver ms/attempt (CPU) | T_core ms/attempt (CPU) | f_s | natural relations / completed natural |
|:--|--:|--:|--:|--:|--:|--:|--:|
| s2 | FES | 2/0/2 | - | 0.21 | 35.17 | 0.6% | 0/2 |
| s2 | CryptoMiniSat (native XOR) | 2/0/2 | - | 10.16 | 45.26 | 24.1% | 0/2 |
| s2 | Z3 (And/Xor) | 2/0/2 | - | 69.49 | 106.01 | 68.0% | 0/2 |
| s2 | F4-style prototype | 2/0/2 | - | 131.68 | 172.57 | 79.3% | 0/2 |
| s2 | F5B prototype | 2/0/2 | - | 80.89 | 124.00 | 65.6% | 0/2 |
| s3 | FES | 2/0/2 | - | 6.53 | 78.24 | 8.3% | 0/2 |
| s3 | CryptoMiniSat (native XOR) | 2/0/2 | - | 12.15 | 84.06 | 14.7% | 0/2 |
| s3 | Z3 (And/Xor) | 2/0/2 | - | 443.41 | 532.06 | 84.3% | 0/2 |
| s3 | F4-style prototype | 0/2/2 | - | 4934.21 | 5002.51 | 98.6% | 0/0 |
| s3 | F5B prototype | 0/2/2 | s3-260939-i2 | 4917.70 | 4993.55 | 98.5% | 0/0 |
| s4 | FES | 3/0/1 | - | 234.93 | 507.80 | 46.3% | 0/2 |
| s4 | CryptoMiniSat (native XOR) | 3/0/1 | - | 152.95 | 436.02 | 35.1% | 0/2 |
| s4 | Z3 (And/Xor) | 3/0/1 | s4-260939-i0 | 4493.91 | 4747.15 | 94.2% | 0/2 |
| s4 | F4-style prototype | 0/3/1 | - | 4936.27 | 5175.44 | 95.5% | 0/0 |
| s4 | F5B prototype | 0/3/1 | - | 5252.37 | 5470.51 | 96.0% | 0/0 |

<!-- t11_posthoc -->
| Engine (W1a, 8 natural attempts; post hoc, not preregistered) | T_ss CPU ms per attempt | of which solver stage ms | solver share f_s | Amdahl bound g |
|:--|--:|--:|--:|--:|
| FES | 7.31 | 0.22 | 2.9% | 1.03x |
| CryptoMiniSat (native XOR) | 17.72 | 10.26 | 59.8% | 2.49x |
| Z3 (And/Xor) | 81.86 | 74.26 | 91.4% | 11.67x |
| F4-style prototype | 145.83 | 138.73 | 95.1% | 20.53x |
| F5B prototype | 96.98 | 89.94 | 92.7% | 13.78x |

<!-- t2b_w1b_stages -->
| s | Engine (mean CPU ms per attempt) | fb setup | input gen | eq build | engine load | solver setup | solve | model extr. | lifting | verif. | rank upd. | T_core (A) |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| s3 | FES | 37.14 | 22.26 | 34.57 | 0.00 | 0.00 | 6.52 | 0.01 | 0.00 | 0.00 | 0.00 | 78.2 |
| s3 | CryptoMiniSat (native XOR) | 37.27 | 21.54 | 33.43 | 1.72 | 2.60 | 9.56 | 0.01 | 0.00 | 0.00 | 0.00 | 84.1 |
| s3 | Z3 (And/Xor) | 44.93 | 28.15 | 34.49 | 27.80 | 311.75 | 130.26 | 0.01 | 0.00 | 0.00 | 0.00 | 532.1 |
| s3 | F4-style prototype | 37.14 | 20.35 | 32.68 | 0.00 | 0.00 | 4934.21 | 0.00 | 0.00 | 0.00 | 0.00 | 5002.5 |
| s3 | F5B prototype | 37.37 | 21.70 | 34.27 | 7.04 | 1.27 | 4916.14 | 0.00 | 0.00 | 0.00 | 0.00 | 4993.6 |
| s4 | FES | 80.02 | 23.22 | 155.20 | 0.00 | 0.00 | 228.44 | 6.67 | 42.41 | 4.07 | 0.01 | 507.8 |
| s4 | CryptoMiniSat (native XOR) | 76.38 | 22.97 | 149.10 | 1.66 | 10.85 | 127.17 | 14.01 | 44.32 | 4.47 | 0.01 | 436.0 |
| s4 | Z3 (And/Xor) | 78.62 | 21.30 | 149.56 | 24.86 | 1305.62 | 3132.67 | 18.93 | 40.90 | 4.16 | 0.01 | 4747.2 |
| s4 | F4-style prototype | 84.55 | 23.38 | 149.96 | 0.00 | 0.00 | 4936.27 | 0.00 | 0.00 | 0.00 | 0.00 | 5175.4 |
| s4 | F5B prototype | 76.36 | 21.08 | 145.25 | 4.74 | 6.59 | 5245.77 | 0.00 | 0.00 | 0.00 | 0.00 | 5470.5 |

<!-- t7_m83_bridge -->
| Engine | m83 published median solver-stage wall s (recomputed from archive) | this host, same definition (incl. library import), wall s | this host, library load excluded, wall s | this host / m83 |
|:--|--:|--:|--:|--:|
| FES | 0.000335 | 0.000236 | 0.000236 | 0.70x |
| CryptoMiniSat (native XOR) | 0.028128 | 0.011433 | 0.009825 | 0.41x |
| Z3 (And/Xor) | 0.212127 | 0.103238 | 0.078695 | 0.49x |
| F4-style prototype | 0.171046 | 0.136095 | 0.136095 | 0.80x |
| F5B prototype | 0.149750 | 0.089875 | 0.086955 | 0.60x |

<!-- t8_reps -->
| Engine (W1a, 12 inputs summed) | T_core CPU s, min over reps | median | max | T_solver CPU s, min | max | max 1-min load average at a child start |
|:--|--:|--:|--:|--:|--:|--:|
| FES | 0.852 | 0.951 | 0.961 | 0.0039 | 0.0046 | 2.32 |
| CryptoMiniSat (native XOR) | 0.958 | 0.986 | 1.084 | 0.0979 | 0.1070 | 2.32 |
| Z3 (And/Xor) | 1.718 | 1.858 | 1.858 | 0.8672 | 0.9562 | 2.32 |
| F4-style prototype | 2.487 | 2.537 | 2.549 | 1.6337 | 1.6686 | 2.32 |
| F5B prototype | 1.887 | 2.010 | 2.130 | 1.0386 | 1.2093 | 2.32 |

<!-- t9_w2_targets -->
| Engine | target | outcome | T_core CPU s | solver stage CPU s | f_s | solve wall s | rejected tuples | new independent natural relation | peak RSS MB |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| CryptoMiniSat (native XOR) | 0 | verified | 12.14 | 12.11 | 99.7% | 12.28 | 0 | 1 | 56 |
| CryptoMiniSat (native XOR) | 1 | verified | 34.14 | 34.10 | 99.9% | 34.51 | 0 | 1 | 113 |
| CryptoMiniSat (native XOR) | 2 | verified | 32.76 | 32.72 | 99.9% | 33.17 | 0 | 1 | 109 |
| CryptoMiniSat (native XOR) | 3 | verified | 49.80 | 49.76 | 99.9% | 50.35 | 0 | 1 | 95 |
| Z3 (And/Xor) | 0 | timeout | 118.01 | 117.98 | 100.0% | 118.91 | None | 0 | 121 |
| Z3 (And/Xor) | 1 | verified | 42.47 | 42.43 | 99.9% | 42.24 | 0 | 1 | 101 |
| Z3 (And/Xor) | 2 | verified | 46.34 | 46.31 | 99.9% | 45.72 | 0 | 1 | 115 |
| Z3 (And/Xor) | 3 | timeout | 118.54 | 118.52 | 100.0% | 118.89 | None | 0 | 131 |

<!-- t10_w2_total -->
| Engine (W2, 4 natural targets) | verified | timeouts | unsat | new independent relations | T_core CPU s (sum) | f_s | T_core CPU s per new independent relation | T_core CPU s per attempt |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| CryptoMiniSat (native XOR) | 4/4 | 0 | 0 | 4 (rank 4) | 128.8 | 99.89% | 32.2 | 32.2 |
| Z3 (And/Xor) | 2/4 | 2 | 0 | 2 (rank 2) | 325.4 | 99.97% | 162.7 | 81.3 |
