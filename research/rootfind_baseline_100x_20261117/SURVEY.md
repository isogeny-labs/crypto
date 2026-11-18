# What earlier studies already measured for root finding

Compiled 2026-11-17 from the repository's own reports (read-only survey).
All wall-time figures below were taken at L0 (shared host) unless stated,
and are `[pcc:unverified]`.

## Gap in the record

No earlier study reports a per-kernel CPU baseline of `rootfind-mitm`,
`rootfind-pairs` or `rootfind-mitm-prime-batch` across factor-base sizes:
no ns per point, no table-build time against A. Most report the counted
cost S, which batching does not change by design.

## Best numbers on record

| What | Figure | Source |
|---|---|---|
| `rootfind-mitm-prime-batch` vs `rootfind-mitm`, prime 24–28 bit, A=48 | wall ratio 0.486 [0.468, 0.504]; counted ratio 1.000 | `docs/perf/20261112-rust-deep-audit.md` |
| Prime u64 two-summand, A=120 | 2.95 µs/target at 40 bit, 8.82 µs at 61 bit | `RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md` §5 |
| `rootfind-subtract`, 31 bit, A=4096, one thread | 27.9 s over 651,264 targets | `research/ic_gpu_rootfind_ic_20261114/RESULTS.md` |
| Binary `rf-mitm`, refuted target | 3.1 µs (ℓ=8), 57.6 µs (ℓ=12), 482 µs (ℓ=14) | `research/pdp_root_finder_20261012/README.md` |
| GPU offload (A100) | relation phase 21–121x one CPU thread | `research/ic_gpu_rootfind_ic_20261114/VALIDATION_REVIEW.md` |
| Gap to matched rho, binary `rootfind-mitm` | S_IC/S_rho 2.9 (n=13) to 25,116 (n=37) | `research/pdp_root_finder_ecbench_20261013/README.md` |
| Best complete arm (`rootfind-negfold`, K13) | about 1.54x strong rho | `research/ecbench_rootfind_vs_rho_20261014/README.md` |

## Levers already tried

| Lever | Effect | Status | Source |
|---|---|---|---|
| Montgomery batch inversion, blocks of 32 | wall about 2.06x | adopted | `docs/perf/20261112-rust-deep-audit.md` |
| Batched x-only table build | setup 0.528x | adopted | `research/ecbench_rootfinding_batchtable_20261013/REPORT.md` |
| Synchronized lanes 4/8/16 | wall unresolved | no gain shown | `research/ecbench_rootfinding_lanes_20261013/REPORT.md` |
| Allocation reuse | 0.937 (0.786–1.122) | no gain | `research/ecbench_rootfinding_alloc_20261014/REPORT.md` |
| Barrett / division-free field | about 5% | rejected | deep audit R6-A |
| Flat, paged, inline pair index | flat 0.82x at K13/K19, 2.02x at K23; paged 1.05–1.55x | none integrated | `research/rootfind_pair_index_timing_20261116/RESULTS.json` |
| Single-hash probe | miss probe about 3x | adopted | `research/rootfind_faster_e2e_20261015/README.md` |
| Signed-pair shared denominator, SIMD scan, lazy x-only | never timed | unmeasured | `research/rootfind_negfold_binary_batch_20261115/` |
| Multi-hit, block-tail, first-independent | no gain | rejected | `research/rootfind_multihit_relations_20261116/` and siblings |
| CPU multithreading | relation phase 0.4–6.2x | mixed | `research/ic_gpu_rootfind_big_20261115/` |
| GPU prefetch | 1.21–1.92x slower | rejected | `research/ic_gpu_rootfind_lookup_20261114/` |

## Where earlier profiles put the time

- Prime 24 bit, before batching (Callgrind): `add_raw` 54.9%, `row_points`
  11.5%, `PairTable::get` 10.9%, `probe` 8.3%; the Euclid inversion was about
  85 of 101–107 ns per chord step (`docs/perf/20261112-rust-deep-audit.md`).
- Koblitz n=19: `add_many_clmul` 14.6%, `PairTable::get` 9.1%,
  `batch_inv_clmul` 8.8%, `inv_clmul` 8.6%.
- No profile exists after the batching change, and no roofline record exists
  for the CPU kernel.

## Stated limits

The earlier reports class root finding as constant-factor engineering: its
growth follows the pair-table and enumeration floors, so the exponent, not
the constant, sets the gap to rho (`research/pdp_root_finder_20261012`).
