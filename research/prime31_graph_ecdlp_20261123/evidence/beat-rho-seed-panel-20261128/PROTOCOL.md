# M31 same-target rho seed panel — frozen before execution

Date: 2026-11-28. Status: diagnostic only. Public B677 curve, generator and target are those in `../beat-rho-diagnostic-20261128/README.md`; the solver receives only the target coordinates. This is a follow-up to the seven-round, one-seed panel there. No new IC or rho speedup claim follows from a single target.

## Source and arms

- IC: unchanged `native/mitm_four_fast.rs` from the task base, SHA-256 `098c6d3c67440677f0b53cf81ede2ddd6850c4ec600345635f526f6de4519c60`; invoke `40 12345678 4` in a fresh process for every pair.
- Rho: copy the published `../beat-rho-diagnostic-20261128/rho_neg_dp.rs` (SHA-256 `2b0e87dfee5e3257ff9fb5b3d05dd8d4d73bf797b7981826477a0b80763bc587`) and change only its initial RNG seed from a fixed literal to a required CLI `u64`. Keep its 64 jumps, negation folding, short-cycle handling, 7-bit distinguished points, 2,048-walk limit and 2,000,000-step walk cap. Charge its setup, failed walks, collisions and verification. This remains a v5-inspired control, not the repository's exact `RhoV3::v5(true)`.
- Arithmetic and B677 fixture are the unchanged source files used by the existing diagnostic; record build hashes and Rust version with results. Build both arms in the same isolated manifest. Do not alter the full package's source-hash gates.

## Seed and run order

The eight independently derived rho seeds are the first eight bytes (big endian) of SHA-256 of `m31-rho-seed-panel-20261128:<i>` for `i=0..7`:

`0x0ec0550c9344eb9e`, `0xf10c1a4a1cd16e4e`, `0xb63d134c29e5a4e3`, `0xc3d3b32963d19adf`, `0xaf062fa466f7d0d4`, `0x8f46b508f3a94547`, `0xf18d577c0116286f`, `0xddfb8a4a903f3935`.

Run eight fresh-process pairs, alternating IC/rho then rho/IC. One additional fresh rho process repeats seed 0 as an A/A trajectory check. Preserve every exit status and raw stdout/stderr. No retuning or replacement seeds after seeing outcomes.

## Checks and interpretation

Require `[d]G=Q` by an independent affine replay and equality of both returned scalars for every completed pair. Require the repeated rho seed to reproduce walk, step, add, cycle and distinguished-point counts; wall and CPU may vary. Record external process wall, child CPU and maximum RSS, internal wall, IC probes/rows/queries, rho walks/steps/additions, exact source hashes and host description. Report paired wall and CPU ratios descriptively, including failures and censoring. The host is shared and this fixed-target panel cannot establish a general speedup or scaling law.
