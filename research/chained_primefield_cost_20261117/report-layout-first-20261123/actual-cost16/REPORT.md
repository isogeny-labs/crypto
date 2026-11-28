# Prime-field work in guarded Richelot inverses

Historical cost16 unit: 17 November 2026 · publication and custody update: 23 November 2026

The historical first cost16B record compares the guarded cubic inverse with the original norm-and-Euclid inverse on 17 forward and 17 dual Richelot map calls. All 34 coarse operation vectors are candidate-lower; separating inversion inputs leaves 2 candidate-lower and 32 incomparable pairs. This is a fixed same-engine component count, not a measured runtime advantage. Current unsigned record: PCC-20261123-2eba0e. [pcc:unverified]

![Recorded classifications for 34 paired map calls, source e7efdbf1, 17 November 2026. These are complete fixed populations; no statistical interval was recorded. [pcc:unverified]](fixed34-map-cost-classifications.svg)

Exact totals below cover only work inside Field.inv. M, A, S and R count explicit BigUint multiplication, addition, subtraction and remainder outside modpow. I counts actual modpow calls atomically; R64 counts the CubicField.c u64 remainder. Internal Montgomery limbs, allocations and CPU prices are not expanded. [pcc:unverified]

| Arm | M | A | S | R | I | R64 |
| --- | --- | --- | --- | --- | --- | --- |
| Original | 43,340 | 64,662 | 46,463 | 122,332 | 10,114 | 1,054 |
| Guarded | 29,988 | 48,990 | 33,848 | 88,038 | 4,844 | 1,054 |

The aggregate I coordinate assigns one common atom to different inversion inputs. Expanded coordinates retain every normalized input and do not assign fitted prices. Across 64 cubic primitive pairs the coarse model gives 64 lower; the expanded model gives 1 lower and 63 incomparable. All 8 general inverse pairs are equal in both models. [pcc:unverified]

The simultaneous all-backend budgets cover the instrumented FpPoly/CubicField sites, but omit direct Fp2/u64 work elsewhere in the maps. Neither budget is a complete map-discovery, destination-solver or end-to-end ECDLP cost. The native test harness elapsed time is not a benchmark. [pcc:unverified]

[Immutable native-emitted aggregates, full-input histograms and exact totals](https://github.com/isogeny-labs/crypto/blob/7f9d552c804a0f58e403b1671d07901a735b4499/research/chained_primefield_cost_20261117/first-native-B-success-04/ACTUAL_FIRST16B_SUCCESS_AND_NATIVE_AGGREGATES.json)

## Frozen checks and current evidence

The recorded B caller exited zero and all 15 phase expectations match, including the expected Clippy-negative exit. Strict compilation, Clippy, format, 12 hand-calculated calibration cases at all 17 sites, one corrupt-counter/restored-positive control, and all 106 prior scientific rows are retained in the checkpoint records. One new native test passed; 15 inherited tests were compiled and filtered. There are 64 cubic and 8 general primitive pairs, 2 zero, 8 sign, 3 constructor controls, and 17 forward plus 17 dual map calls. [pcc:unverified]

The execution record pins 85 original sources, 28 copied sources, 3 prebuilt dependency rlibs and 3 compiler tools, with distinct build/pre/post manifests and qualifier d9db82db. The official num-bigint 0.4.8 archive matches Cargo.lock and the three retained primary source files. Those historical input and guard records survive. No native compilation or arithmetic was repeated during this publication. [pcc:unverified]

The full 161,784-byte stdout has historical SHA-256 7d528378104dc4f2646f3f6d92bf9b2426c6b38b0af67a72bd86d09607a6d666. The 66-member, 1,773,712-byte whole archive has SHA-256 eefd5d1a321425be038136d45196decc3c9a51d3b1c66e5e5bc392edb47bedef. It contained source, three rlibs, both ELFs and complete recorded streams when sealed on 17 November. The full stdout was deliberately kept outside Git under the unchanged artifact policy. [pcc:unverified]

On 23 November the recorded tmpfs packet and raw-output paths are absent after restart. Current public custody consists of exact source, guards, aggregate native lines, hashes and inventory; the complete raw stdout and private archive are not presently available at the inspected paths. Historical seal assertions remain unchanged and are not assertions of current availability. Recovery or approved whole-packet storage remains open. [pcc:unverified]

SourceA 8f3d314c failed the exact-publication guard before copy, compilation or native arithmetic because ignored Cargo.lock was omitted. B e7efdbf1 explicitly published the same lock and changed metadata capture/helper selection only. The missing original A Git stderr is preserved as a separate gap; a later diagnostic never replaces it. [pcc:unverified]

| Requirement | Publication status |
| --- | --- |
| Exact frozen source and native aggregate record | Retained and hash-matched |
| Complete original raw stdout/private packet | Missing at inspected restart paths |
| Full Field/Picard formal bridge, independent signing | Open |
| Admitted runtime and complete solver comparison | Open; no new measurement |

[Prospective protocol dd6223fa (before implementation and calls)](https://github.com/isogeny-labs/crypto/blob/dd6223faae3ff638eb2582e8f40ceca41c5a142c/research/chained_primefield_cost_20261117/PROTOCOL_16.md)

[Resume custody record and continuing queue: PR #4990 / issue #3904](https://github.com/isogeny-labs/crypto/pull/4990)
