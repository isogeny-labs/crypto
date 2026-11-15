# Priced symmetric m = 4 PKM oracle: S5 build counted, parametric S5 (2026-11-15) - DRAFT, nothing frozen, nothing run at frozen size

Lead IDG-20261115-SYM-PRICED. Follow-up to [`../pkm_symmetric_lead1_20261026/`](../pkm_symmetric_lead1_20261026/README.md)
("next measurements", item 3). Instrument: [`examples/pkm_symmetric_tower.rs`](../../examples/pkm_symmetric_tower.rs)
(new counters `cnt`, `ParamS5`, `--mode curve|calibrate|stage0|summary`; legacy arms unchanged).

**Class: component result (counted stage) and a derived priced row; no leaderboard row, no `S` claim.**
Protocol (draft, to be frozen): [`PROTOCOL.md`](PROTOCOL.md). `[pcc:unverified]` throughout.

- Question: is the 2.75e5x F4-stage gain still there once the per-target `S5` construction (grid evaluation, interpolation,
  `e`-rewrite; uncounted in the 4.010e5) is charged, and does a per-curve parametric `S5(e; x_R)` (degree <= 8 in `x_R`)
  remove it?
- Pod: `pod/build.sh` (one source archive, `--release --locked`, no Docker), `pod/run.sh` (sequential, uncontended, writes
  `$OUT` with `SHA256SUMS`).
- Reproduce on a plain Linux CPU box: `WORK=/work SRC=/work/src-<commit>.tgz bash pod/build.sh && WORK=/work OUT=/work/out bash pod/run.sh`.
- Outputs: `curve-t{3,4}.json`, `calib-t{3,4}.json`, `spec-t{3,4}.json`, `sess/rho-t{3,4}` (+ verify, band), `stage0-t{3,4}.jsonl`,
  `plain-ref-t3.jsonl`, `summary-t{3,4}.json`, `env.txt`, `ids.txt`, `SHA256SUMS`.

## Local toy check (development only, non-frozen: p = 12289, t = 3 (N = 12), seed 7; not evidence, not the frozen prime or seed)

Measured by the preparing session, `cargo test --release --example pkm_symmetric_tower -- --include-ignored` (8 pass,
including parametric == grid on 3 toy targets) and `--mode stage0 --p 12289 --t 3 --targets 3 --f4-targets 2 --seed 7`:

| quantity (toy curve, p = 12289) | value |
|:--|--:|
| grid S5 build per target | 2.479e6 mulmods (1.358e6 add/sub; 39,375 inversions = 43% of the mulmods) |
| F4 stage per target (2 targets) | 8.122e5 row ops |
| grid build / F4 | 3.05 |
| parametric precompute (9 grid builds, 495 e-monomials) | 2.207e7 mulmods = 8.9 grid builds |
| parametric per target (Horner, 495 monomials x 8) | 3,960 mulmods = 0.0049 of F4 |
| breakeven | 8.9 targets |
| coefficient check | 1,980 coefficients compared (3 targets + planted), 0 mismatches; planted vanishes in both modes |
| legacy arm regression | m = 3, t = 2..4 symmetric rows: D, widths, steps, field_ops identical to `pkm_symmetric_lead1_20261026/rows.jsonl` |

Counter wiring: stage split of the 2.479e6 build is sylvester_det 1.411e6, to_elementary 5.21e5, grid_interp 2.36e5,
s3_eval 2.03e5, s4_eval 8.1e4, subst_gw 2.6e4, vandermonde_inv 1.9e3 (these sum to the total). The lead's hand estimate
was 7e5-9e5; the executed build is about 3x higher, mostly the Fermat inversion in every determinant pivot. The toy F4
stage (8.1e5) is not the frozen 4.01e5: another prime and curve.
Wall time of one grid build: 4-5 s on this 4-core shared host (contended; evidence only as an order of magnitude).

## Runtime and memory of the frozen run (derived, not measured at frozen size)

Per grid build about 4-5 s (toy, contended), independent of p and t. Stage 0 per N: 34 builds (9 precompute, 1 planted, 24
targets) = about 2.5-3 min, plus F4: N = 12 about 8 x 10 ms, N = 16 4 x 12 s (committed README). plain-chain N = 12: 3 x 70 s.
Rho specs: seconds. Source build of `ecbench` + the example from scratch: 10-25 min on 4 vCPUs (the `crypto` crate compile
peaked at 2.8 GB RSS here). Whole `run.sh` after the build: about 15 min. Peak memory: the build 3-4 GB; the run is
single-process, plain-chain N = 12 (21,619 columns) is the peak, estimated 3-6 GB by scaling the committed m = 3 N = 18
cell (33,153 columns, > 8.5 GB) by columns squared (rough, +-3x). N = 16 symmetric (13,926 columns) about 1.5 GB.
Recommended pod: 4 vCPU, 16 GB RAM, 20 GB disk, x86_64, rustc as in the pin; one process at a time (wall times are evidence
only uncontended).
