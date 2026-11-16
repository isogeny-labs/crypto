# Run report: run-20261018T0320Z-ifma

- **Command card.** `CARD-eigen_seta_ifma-20261018-1` (sha256 6b99…4e39),
  compiled by experiment-director and run by executor-mechanical. The
  preregistration is `DISPATCH-1.txt`.
- **Commits.** eigenx main 580b633 (arm `base`) and eigenx 0a8da40 (arms
  `ifma` and `native`; `native` adds `-C target-cpu=native`). The binary
  sha256s matched the card.
- **Host.** Intel Xeon @ 2.10 GHz, 4 vCPU KVM, shared VM, AVX-512 IFMA and
  VBMI present, rustc 1.97.0.
  - Load average at start: 0.07 / 0.77 / 1.20. At end: 1.18 / 1.11 / 1.18.
  - Unpinned. Isolation level: L0. Precondition: no valgrind, dudect or
    cargo running.
- **Command.** `bench_kem <SET> <keys> <trials> 1` for L1 (3 × 5), L3
  (2 × 3) and L5 (1 × 3). The arms ran base, ifma, native within each set,
  all sequential.
- **Runs.** 9 requested, 9 completed. Every exit status was 0, every
  decapsulation agreed, and all .err files are empty.

## Measured: decapsulation, median [min–max]

| set | base | ifma | native |
|---|---|---|---|
| L1 (n=15) | 1.050 [0.987–1.245] s | 0.635 [0.598–0.752] s | 0.656 [0.581–0.836] s |
| L3 (n=6) | 4.989 [4.799–5.406] s | 1.605 [1.582–1.855] s | 1.413 [1.344–1.479] s |
| L5 (n=3) | 11.069 [11.051–11.077] s | 3.345 [3.325–3.455] s | 2.954 [2.819–2.962] s |

## Measured: other records

**F_p multiplication**, from the `field` line, base / ifma / native:

| set | base | ifma | native |
|---|---|---|---|
| L1 | 553 ns | 570 ns | 396 ns |
| L3 | 1746 ns | 624 ns | 576 ns |
| L5 | 2908 ns | 832 ns | 788 ns |

This line is a dependent chain that includes an addition per step.

**Keygen** per key:

| set | base | ifma | native |
|---|---|---|---|
| L1 | 3.4 / 7.1 / 7.9 s | 3.1 / 6.9 / 2.7 s | 4.5 / 2.7 / 2.4 s |
| L3 | 73.0 / 98.0 s | 32.7 / 24.1 s | 22.2 / 24.0 s |
| L5 | 41.1 s | 28.2 s | 14.0 s |

![decapsulation](../../figures/decaps.svg)

Raw records: `../../records/run-20261018T0320Z-ifma/`.
