## Cards S/F/M/R (base e9d5dbdccd = A, A2 second base build; cand = B)

GPU: NVIDIA A100 80GB PCIe, 8.0, 595.91.07, 1410 MHz, 1512 MHz

### Worker kernels (launch to sync, ms)

| metric | A median | B median | B/A |
|---|---:|---:|---:|
| spmv (ms) | 8.175 | 0.5398 | 0.06603 |
| fes26 (ms) | 6.92e+04 | 7.277 | 0.0001052 |
| fes26lb10 (ms) | 6.446 | 7.279 | 1.129 |

### A/A control (base vs second base build)

| metric | A median | A2 median | A2/A |
|---|---:|---:|---:|
| spmv (ms) | 8.151 | 8.144 | 0.9991 |
| fes26 (ms) | 6.919e+04 | 6.919e+04 | 1 |
| fes26lb10 (ms) | 6.384 | 6.373 | 0.9984 |

### macaulay throughput (kernel ms, lower is better)

| metric | A median | B median | B/A | A/A (A2/A) |
|---|---:|---:|---:|---:|
| batch 64 (ms) | 12.94 | 9.28 | 0.7172 | 0.9954 |
| batch 128 (ms) | 22.12 | 17.33 | 0.7835 | 0.9991 |
| batch 256 (ms) | 35.63 | 32.06 | 0.8998 | 1.0000 |
| batch 512 (ms) | 57.95 | 46.32 | 0.7993 | 0.9990 |
| batch 1024 (ms) | 111.8 | 92.05 | 0.823 | 0.9996 |
| batch 2048 (ms) | 212.6 | 182.9 | 0.8604 | 0.9997 |
| batch 4096 (ms) | 418.1 | 361 | 0.8634 | 0.9997 |
| batch 8192 (ms) | 831.7 | 718.2 | 0.8636 | 0.9997 |

### Worker output digests (sorted solution lists, sha256)

```
spmv cpu  d9aafdd30b461b3d8e521cf0fc60e5513981bb93758685121a423e8b21a34117  -
spmv base d9aafdd30b461b3d8e521cf0fc60e5513981bb93758685121a423e8b21a34117  -
spmv cand d9aafdd30b461b3d8e521cf0fc60e5513981bb93758685121a423e8b21a34117  -
fes26 cpu  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  - n=0
fes26 base e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  - n=0 head=SOLUTIONS 0
fes26 cand e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  - n=0 head=SOLUTIONS 0
fes16 cpu  ea5ddca55c9a98d028318e039d289d5352ffd5f980fac439f4ddcee278f77e33  - n=8256
fes16 base 953f107928c5ac265a3d650647cb7a03415d0099269ed2aa270eb5ccd3aadc0c  - n=64 head=SOLUTIONS 64
fes16 cand ea5ddca55c9a98d028318e039d289d5352ffd5f980fac439f4ddcee278f77e33  - n=8256 head=SOLUTIONS 8256
```

## Rootfind sweep variants (B = base, R0 = main, R0b = second R0 build, R1/R2 fused)

Selftests: p256-B PASSED, p256-R0 PASSED, p256-R1 PASSED, p256-R2 PASSED, selftest-B PASSED, selftest-R0 PASSED, selftest-R1 PASSED, selftest-R2 PASSED, selftest4096-B PASSED, selftest4096-R0 PASSED, selftest4096-R1 PASSED, selftest4096-R2 PASSED

### rootfind |F|=65536, 65536 targets: R0 vs B (ms)

| metric | B median | R0 median | R0/B |
|---|---:|---:|---:|
| sorted/C16 (ms) | 403.9 | 401.9 | 0.9951 |
| sorted/C32 (ms) | 625.8 | 625.7 | 0.9997 |
| sorted/C64 (ms) | 620.2 | 620.2 | 1 |
| hash/C16 (ms) | 577 | 576.7 | 0.9995 |
| hash/C32 (ms) | 846.2 | 846.2 | 1 |
| hash/C64 (ms) | 1219 | 1219 | 1 |

### rootfind |F|=65536, 65536 targets: R1 vs B (ms)

| metric | B median | R1 median | R1/B |
|---|---:|---:|---:|
| sorted/C16 (ms) | 403.9 | 409.3 | 1.013 |
| sorted/C32 (ms) | 625.8 | 491.5 | 0.7854 |
| sorted/C64 (ms) | 620.2 | 670 | 1.08 |
| hash/C16 (ms) | 577 | 538.1 | 0.9326 |
| hash/C32 (ms) | 846.2 | 732.2 | 0.8653 |
| hash/C64 (ms) | 1219 | 913.9 | 0.7499 |

### rootfind |F|=65536, 65536 targets: R2 vs B (ms)

| metric | B median | R2 median | R2/B |
|---|---:|---:|---:|
| sorted/C16 (ms) | 403.9 | 428.4 | 1.061 |
| sorted/C32 (ms) | 625.8 | 615 | 0.9826 |
| sorted/C64 (ms) | 620.2 | 594.5 | 0.9587 |
| hash/C16 (ms) | 577 | 728.9 | 1.263 |
| hash/C32 (ms) | 846.2 | 1264 | 1.494 |
| hash/C64 (ms) | 1219 | 1259 | 1.033 |

### rootfind |F|=65536, 65536 targets: A/A R0b vs R0 (ms)

| metric | R0 median | R0b median | R0b/R0 |
|---|---:|---:|---:|
| sorted/C16 (ms) | 401.9 | 404.8 | 1.007 |
| sorted/C32 (ms) | 625.7 | 627.2 | 1.002 |
| sorted/C64 (ms) | 620.2 | 621 | 1.001 |
| hash/C16 (ms) | 576.7 | 577.7 | 1.002 |
| hash/C32 (ms) | 846.2 | 847.4 | 1.001 |
| hash/C64 (ms) | 1219 | 1220 | 1.001 |

### rootfind |F|=1024, 262144 targets: R0 vs B (ms)

| metric | B median | R0 median | R0/B |
|---|---:|---:|---:|
| sorted/C16 (ms) | 21.77 | 21.76 | 1 |
| sorted/C32 (ms) | 31.1 | 31.1 | 1 |
| sorted/C64 (ms) | 30.73 | 30.72 | 0.9996 |
| hash/C16 (ms) | 23.53 | 19.91 | 0.8461 |
| hash/C32 (ms) | 25.59 | 25.61 | 1.001 |
| hash/C64 (ms) | 24.33 | 24.36 | 1.001 |

### rootfind |F|=1024, 262144 targets: R1 vs B (ms)

| metric | B median | R1 median | R1/B |
|---|---:|---:|---:|
| sorted/C16 (ms) | 21.77 | 25.35 | 1.165 |
| sorted/C32 (ms) | 31.1 | 28.28 | 0.9094 |
| sorted/C64 (ms) | 30.73 | 33.49 | 1.09 |
| hash/C16 (ms) | 23.53 | 23.96 | 1.018 |
| hash/C32 (ms) | 25.59 | 34.58 | 1.351 |
| hash/C64 (ms) | 24.33 | 68.75 | 2.826 |

### rootfind |F|=1024, 262144 targets: R2 vs B (ms)

| metric | B median | R2 median | R2/B |
|---|---:|---:|---:|
| sorted/C16 (ms) | 21.77 | 22.55 | 1.036 |
| sorted/C32 (ms) | 31.1 | 30.34 | 0.9756 |
| sorted/C64 (ms) | 30.73 | 28.86 | 0.9391 |
| hash/C16 (ms) | 23.53 | 21.56 | 0.9165 |
| hash/C32 (ms) | 25.59 | 28.41 | 1.11 |
| hash/C64 (ms) | 24.33 | 26.56 | 1.092 |

### rootfind |F|=1024, 262144 targets: A/A R0b vs R0 (ms)

| metric | R0 median | R0b median | R0b/R0 |
|---|---:|---:|---:|
| sorted/C16 (ms) | 21.76 | 21.82 | 1.003 |
| sorted/C32 (ms) | 31.1 | 31.14 | 1.001 |
| sorted/C64 (ms) | 30.72 | 30.75 | 1.001 |
| hash/C16 (ms) | 19.91 | 19.95 | 1.002 |
| hash/C32 (ms) | 25.61 | 25.65 | 1.002 |
| hash/C64 (ms) | 24.36 | 24.38 | 1.001 |

## Macaulay shared-resident u16 (G = main binary, G2 = new binary global path, S* = shared path)

Selftests: selftest-G ok, selftest-G2 ok, selftest-S1024 ok, selftest-S256 ok, selftest-S512 ok, selftest-small-MAC_SMEM=0 ok, selftest-small-MAC_SMEM_THREADS=1024 ok, selftest-small-MAC_SMEM_THREADS=512 ok

### macaulay S256 vs G (ms per batch)

| metric | G median | S256 median | S256/G | A/A (G2/G) |
|---|---:|---:|---:|---:|
| batch 64 (ms) | 9.3 | 5.47 | 0.5882 | 1.0000 |
| batch 128 (ms) | 17.34 | 10.86 | 0.6263 | 1.0000 |
| batch 256 (ms) | 32.05 | 16.39 | 0.5114 | 1.0012 |
| batch 512 (ms) | 46.33 | 27.44 | 0.5923 | 0.9996 |
| batch 1024 (ms) | 92 | 54.71 | 0.5947 | 0.9992 |
| batch 2048 (ms) | 182.9 | 104.7 | 0.5721 | 1.0003 |
| batch 4096 (ms) | 361 | 209.2 | 0.5796 | 1.0000 |
| batch 8192 (ms) | 718.3 | 417.4 | 0.5811 | 1.0002 |

### macaulay S512 vs G (ms per batch)

| metric | G median | S512 median | S512/G | A/A (G2/G) |
|---|---:|---:|---:|---:|
| batch 64 (ms) | 9.3 | 3.3 | 0.3548 | 1.0000 |
| batch 128 (ms) | 17.34 | 6.57 | 0.3789 | 1.0000 |
| batch 256 (ms) | 32.05 | 9.95 | 0.3105 | 1.0012 |
| batch 512 (ms) | 46.33 | 16.7 | 0.3605 | 0.9996 |
| batch 1024 (ms) | 92 | 33.23 | 0.3612 | 0.9992 |
| batch 2048 (ms) | 182.9 | 63.6 | 0.3477 | 1.0003 |
| batch 4096 (ms) | 361 | 127.2 | 0.3523 | 1.0000 |
| batch 8192 (ms) | 718.3 | 253.4 | 0.3528 | 1.0002 |

### macaulay S1024 vs G (ms per batch)

| metric | G median | S1024 median | S1024/G | A/A (G2/G) |
|---|---:|---:|---:|---:|
| batch 64 (ms) | 9.3 | 2.55 | 0.2742 | 1.0000 |
| batch 128 (ms) | 17.34 | 5.09 | 0.2935 | 1.0000 |
| batch 256 (ms) | 32.05 | 7.73 | 0.2412 | 1.0012 |
| batch 512 (ms) | 46.33 | 12.95 | 0.2795 | 0.9996 |
| batch 1024 (ms) | 92 | 25.8 | 0.2804 | 0.9992 |
| batch 2048 (ms) | 182.9 | 49.34 | 0.2697 | 1.0003 |
| batch 4096 (ms) | 361 | 98.61 | 0.2732 | 1.0000 |
| batch 8192 (ms) | 718.3 | 196.5 | 0.2736 | 1.0002 |

## FES O(1) Gray walk (M = main fes_cuda, S = new tree with -DFES_WALK_SYM=1, G = Gray walk)

```
selftest M: MATCH: 2 expected
selftest S: MATCH: 2 expected
selftest G: MATCH: 2 expected
fes16 cpu ea5ddca55c9a98d0 n=8256
fes16 M ea5ddca55c9a98d0 n=8256
fes16 S ea5ddca55c9a98d0 n=8256
fes16 G ea5ddca55c9a98d0 n=8256
fes24m12 cpu a8913f861b16d98d n=4163
fes24m12 M a8913f861b16d98d n=4163
fes24m12 S a8913f861b16d98d n=4163
fes24m12 G a8913f861b16d98d n=4163
fes26 cpu e3b0c44298fc1c14 n=0
fes26 M e3b0c44298fc1c14 n=0
fes26 S e3b0c44298fc1c14 n=0
fes26 G e3b0c44298fc1c14 n=0
```

### FES kernel, Gray walk G vs main M (ms)

| metric | M median | G median | G/M | A/A (S/M) |
|---|---:|---:|---:|---:|
| fes26 (ms) | 7.271 | 0.3916 | 0.05386 | 1.0011 |
| fes26lb10 (ms) | 7.279 | 0.3935 | 0.05406 | 1.0004 |
| fes30 (ms) | 132.8 | 5.144 | 0.03873 | 1.0061 |
| fes24m12 (ms) | 1.683 | 0.1492 | 0.08864 | 0.9994 |

## Final check of the committed tree c80afaa98e32190bbcc2e98f2442e6678e82cec7

```
spmv cpu  d9aafdd30b461b3d
spmv cuda d9aafdd30b461b3d
fes16 cpu  ea5ddca55c9a98d0
fes16 cuda ea5ddca55c9a98d0
fes24m12 cpu  a8913f861b16d98d
fes24m12 cuda a8913f861b16d98d
fes26 cpu  e3b0c44298fc1c14
fes26 cuda e3b0c44298fc1c14
fes30 cpu  be86adc0664dfe00
fes30 cuda be86adc0664dfe00
```
```
fes-test.log: rc=0
mac-test.log: rc=0
semaev-test.log: all checks passed (0 failures)
fes-selftest.log: MATCH: 2 expected
mac-selftest.log: selftest ok (0 of 64 instances differ)
semaev-selftest.log: selftest ok (8 targets, 4 yes, 4 no, 0 failures)
semaev-build.log: rc=0
spmv-build.log: rc=0
```
macaulay batch 8192 at the default path: [195.89, 195.8, 196.0] ms
