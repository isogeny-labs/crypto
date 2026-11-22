# F4 over F_p on a real index-calculus consumer (jv_cover), 2026-11-20

**Standing.** `[pcc:unverified]`: one shared 4-vCPU cloud VM, isolation L0, no PMU, no pcc claim filed.
**Scope.** Wall-clock and CPU time of `jv_cover --exp dlp` (Joux–Vitse cover index calculus over
E(F_{p^6}), p = 53), whose relation search is 19,020 Nagao F4 calls. Counted operations (S) do not
change, so no IC row or gap to rho moves. Follow-up to #4473; PR #4851.

## Result

With `--features jemalloc`, `jv_cover --exp dlp` runs **1.256× faster in wall time (18.15 → 14.60 s)
and 1.251× less CPU (68.0 → 54.7 s)** than main, in all 8 pre-registered rounds, with identical work
in all 32 runs (`reports/run-20261120T2241Z/`, protocol `CONFIRM_PROTOCOL.md` frozen before data)
[pcc:unverified]. The library change alone (move instead of copy) is not measurable here (1.033, 5/8).

## What was measured, in order

| step | result | record |
|:--|:--|:--|
| #4473's f4-parallel default vs `F4_FP_PAR_OLD=1` on jv_cover | 19.33 vs 21.32 s median, 6 rounds | `20261120T2059Z-jv-cover-f4-dispatch-ab` |
| knob bisection of the small-system cost (`f4_fp_bench`) | no knob explains it; sub-ms noise | `20261120T2057Z-f4-small-step-bisect` |
| serialize nested F4 calls' inner parallelism (`F4_FP_INNER`) | null (±2 % in ±7 % spread); reverted | `20261120T2149Z-jv-cover-f4-inner-policy-ab` |
| Faugère–Lachartre column threshold | counted −0.5 % at best; wall in noise | `20261120T2209Z-jv-cover-fl-threshold-screen` |
| callgrind of 104 Nagao tests | F4 77.6 % of a test; glibc allocator 30.4 % of F4 | `data/callgrind/` |
| jemalloc preloaded vs glibc | 18.18 → 15.40 s, 6/6 rounds | `20261120T2212Z-jv-cover-allocator-ab` |
| pre-registered confirmation | 1.256 wall, 1.251 CPU, 8/8 | `20261120T2241Z-jv-cover-f4-allocation-confirm` |

Each run reproduced identical residuals, relations, counted oracle operations and a correct logarithm
[pcc:unverified].

## Changes

- `src/cryptanalysis/f4_fp.rs`: the final interreduction moves the live basis instead of deep-copying
  it; `F4_FP_FL_MIN_COLS` overrides the FL column threshold for measurement (default 256 unchanged).
- `Cargo.toml`: feature `jemalloc` (optional `tikv-jemallocator` 0.6.1), off by default;
  `examples/jv_cover.rs` sets it as the global allocator when the feature is on.
- `examples/f4_fp_bench.rs`: an `xl` tier (quad_n10, quad_n11, cubic_n6 from `gb_cas_export`).
- `examples/f4_ab_confirm.rs`: `F4_AB_NEW`, `F4_AB_OLD`, `F4_AB_TIER`, `F4_AB_THREADS`, `F4_AB_KOBLITZ`.

## Why jemalloc is opt-in

Making it the default for every binary would change every wall-clock series in the lab, rho
references included, and add a C dependency to every build. That is the owner's decision. Runs of
`jv_cover` that care about wall time can build with `--features jemalloc` today.

## Root cause and next step

`f4_fp::Poly` stores each term's exponent as its own heap `Vec<u32>`, so F4 makes about 22,700 heap
allocations per call on these systems; converting packed rows back to polynomials and the final
interreduction dominate them. Storing exponents packed (the engine already packs monomials into
`u128` internally when n ≤ 15) would remove most of these allocations for every consumer, under any
allocator. That is a representation change across the module's public API and is left as the next
step.

## Layout

`CONFIRM_PROTOCOL.md`; `data/` raw runs of each record, callgrind summaries, binary hashes;
`scripts/` the run scripts and the plotting script (layout of recorded values only);
`reports/run-20261120T2241Z/` the report, two graphs and the PDF.
