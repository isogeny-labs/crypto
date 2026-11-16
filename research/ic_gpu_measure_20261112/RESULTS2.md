## Round 2 (A100 80GB PCIe; main c5678a98ae vs this branch); cells: median (ratio to base)
GPU: NVIDIA A100 80GB PCIe, 8.0, 595.91.07

### FES identity

```
selftest M: MATCH: 2 expected
selftest B6: MATCH: 2 expected
selftest G: MATCH: 2 expected
selftest B4: MATCH: 2 expected
selftest B5: MATCH: 2 expected
selftest B7: MATCH: 2 expected
selftest B8: 
fes16 cpu c09f56e859833bad n=8160
fes16 M c09f56e859833bad n=8160 SAME
fes16 B6 c09f56e859833bad n=8160 SAME
fes16 G c09f56e859833bad n=8160 SAME
fes16 B4 c09f56e859833bad n=8160 SAME
fes16 B5 c09f56e859833bad n=8160 SAME
fes16 B7 c09f56e859833bad n=8160 SAME
fes16 B8 e3b0c44298fc1c14 n=0 DIFF
fes24m12 cpu 223ccfe3421d57a9 n=4175
fes24m12 M 223ccfe3421d57a9 n=4175 SAME
fes24m12 B6 223ccfe3421d57a9 n=4175 SAME
fes24m12 G 223ccfe3421d57a9 n=4175 SAME
fes24m12 B4 223ccfe3421d57a9 n=4175 SAME
fes24m12 B5 223ccfe3421d57a9 n=4175 SAME
fes24m12 B7 223ccfe3421d57a9 n=4175 SAME
fes24m12 B8 e3b0c44298fc1c14 n=0 DIFF
fes28m16 cpu 43d8ba17d57b544d n=4047
fes28m16 M 43d8ba17d57b544d n=4047 SAME
fes28m16 B6 43d8ba17d57b544d n=4047 SAME
fes28m16 G 43d8ba17d57b544d n=4047 SAME
fes28m16 B4 43d8ba17d57b544d n=4047 SAME
fes28m16 B5 43d8ba17d57b544d n=4047 SAME
fes28m16 B7 43d8ba17d57b544d n=4047 SAME
fes28m16 B8 e3b0c44298fc1c14 n=0 DIFF
fes26 cpu e3b0c44298fc1c14 n=0
fes26 M e3b0c44298fc1c14 n=0 SAME
fes26 B6 e3b0c44298fc1c14 n=0 SAME
fes26 G e3b0c44298fc1c14 n=0 SAME
fes26 B4 e3b0c44298fc1c14 n=0 SAME
fes26 B5 e3b0c44298fc1c14 n=0 SAME
fes26 B7 e3b0c44298fc1c14 n=0 SAME
fes26 B8 e3b0c44298fc1c14 n=0 SAME
fes30 cpu e3b0c44298fc1c14 n=0
fes30 M e3b0c44298fc1c14 n=0 SAME
fes30 B6 e3b0c44298fc1c14 n=0 SAME
fes30 G e3b0c44298fc1c14 n=0 SAME
fes30 B4 e3b0c44298fc1c14 n=0 SAME
fes30 B5 e3b0c44298fc1c14 n=0 SAME
fes30 B7 e3b0c44298fc1c14 n=0 SAME
fes30 B8 e3b0c44298fc1c14 n=0 SAME
fes32 M af0aa2ac02f86f88
fes32 B6 af0aa2ac02f86f88
fes32 G af0aa2ac02f86f88
fes32 B4 af0aa2ac02f86f88
fes32 B5 af0aa2ac02f86f88
fes32 B7 af0aa2ac02f86f88
fes32 B8 e3b0c44298fc1c14
```

### FES kernel time, M = main Gray walk; G = this tree Gray (A/A); B<K> = block walk

| metric | M | B4 | B5 | B6 | B7 | B8 | A/A G/M |
|---|---:|---:|---:|---:|---:|---:|---:|
| fes24m12 (ms) | 0.1507 | 0.0975 (0.647) | 0.0924 (0.613) | 0.091 (0.604) | 0.0981 (0.651) | — | 0.9967 |
| fes26 (ms) | 0.3921 | 0.1799 (0.459) | 0.1677 (0.428) | 0.1635 (0.417) | 0.1799 (0.459) | — | 1.0031 |
| fes28m16 (ms) | 1.328 | 0.4624 (0.348) | 0.4149 (0.313) | 0.3976 (0.299) | 0.4492 (0.338) | — | 0.9990 |
| fes30 (ms) | 5.043 | 1.552 (0.308) | 1.377 (0.273) | 1.301 (0.258) | 1.469 (0.291) | — | 0.9999 |
| fes32 (ms) | 19.85 | 5.853 (0.295) | 5.142 (0.259) | 4.834 (0.244) | 5.828 (0.294) | — | 0.9999 |

macaulay selftests: mac-selftest-D1024 ok, mac-selftest-D512 ok, mac-selftest-M ok, mac-selftest-small-1024 ok, mac-selftest-small-512 ok

### macaulay ms per batch, M = main (shared u16, 1024 thr); D = MAC_UPDATE_2D

| metric | M | D1024 | D512 | A/A MA/M |
|---|---:|---:|---:|---:|
| batch 64 (ms) | 2.55 | 2.3 (0.902) | 2.84 (1.114) | 1.0000 |
| batch 128 (ms) | 5.09 | 4.56 (0.896) | 5.64 (1.108) | 1.0000 |
| batch 256 (ms) | 7.73 | 6.93 (0.897) | 8.55 (1.106) | 1.0000 |
| batch 512 (ms) | 13 | 11.66 (0.897) | 14.38 (1.106) | 0.9992 |
| batch 1024 (ms) | 25.89 | 23.18 (0.895) | 28.61 (1.105) | 1.0000 |
| batch 2048 (ms) | 49.39 | 44.36 (0.898) | 54.71 (1.108) | 1.0000 |
| batch 4096 (ms) | 98.72 | 88.59 (0.897) | 109.3 (1.107) | 1.0000 |
| batch 8192 (ms) | 197.1 | 176.9 (0.897) | 218.1 (1.107) | 1.0002 |

rootfind selftests passed (of 2): rf-selftest-R128_0 2, rf-selftest-R128_0b 2, rf-selftest-R128_2 2, rf-selftest-R256_0 2, rf-selftest-R64_0 2

### rootfind |F|=65536, 65536 targets, launch knobs RF_BLOCK_MINBLOCKS (base R128_0 = main default)

| metric | R128_0 | R64_0 | R256_0 | R128_2 | A/A R128_0b/R128_0 |
|---|---:|---:|---:|---:|---:|
| sorted/C16 (ms) | 403.6 | 403.1 (0.999) | 454.3 (1.126) | 439.3 (1.089) | 0.9998 |
| sorted/C32 (ms) | 627.8 | 626.7 (0.998) | 648.6 (1.033) | 627 (0.999) | 0.9998 |
| sorted/C64 (ms) | 621.8 | 621.4 (0.999) | 633.9 (1.019) | 621.6 (1.000) | 0.9997 |
| hash/C16 (ms) | 577 | 573.4 (0.994) | 718 (1.244) | 690.8 (1.197) | 1.0000 |
| hash/C32 (ms) | 846.5 | 841.5 (0.994) | 1263 (1.492) | 1232 (1.455) | 0.9998 |
| hash/C64 (ms) | 1219 | 1213 (0.995) | 1236 (1.014) | 1219 (1.000) | 0.9999 |

### rootfind |F|=1024, 262144 targets, launch knobs RF_BLOCK_MINBLOCKS (base R128_0 = main default)

| metric | R128_0 | R64_0 | R256_0 | R128_2 | A/A R128_0b/R128_0 |
|---|---:|---:|---:|---:|---:|
| sorted/C16 (ms) | 21.85 | 21.81 (0.998) | 24.2 (1.108) | 23.4 (1.071) | 1.0000 |
| sorted/C32 (ms) | 31.2 | 31.06 (0.996) | 32.27 (1.034) | 31.11 (0.997) | 0.9996 |
| sorted/C64 (ms) | 30.81 | 30.9 (1.003) | 31.29 (1.015) | 30.82 (1.000) | 0.9997 |
| hash/C16 (ms) | 19.98 | 24.72 (1.237) | 20.22 (1.012) | 19.69 (0.986) | 0.9999 |
| hash/C32 (ms) | 25.7 | 26.61 (1.036) | 25.66 (0.999) | 25.61 (0.997) | 0.9996 |
| hash/C64 (ms) | 24.39 | 24.75 (1.015) | 24.17 (0.991) | 24.38 (1.000) | 1.0000 |

ncu: not available: ==PROF== Disconnected from process 6232
