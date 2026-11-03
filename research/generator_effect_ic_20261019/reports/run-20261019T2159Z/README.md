# Run report: E5a, P-192 and P-224 x-statistics, 2026-10-19T21:59Z

**Observations only.** The interpretation is in [`../../E5.md`](../../E5.md).
No ECDLP run.

| item | value |
|:--|:--|
| command | `cargo run --release --example generator_effect_e5a -- research/generator_effect_ic_20261019/e5/e5a.json` |
| protocol | [`e5/PROTOCOL.md`](../../e5/PROTOCOL.md), frozen at commit `ff44dd462` before any run |
| code | `examples/generator_effect_e5a.rs` as committed in the PR that adds this report; the run used the working tree on `ff44dd462` with that file, byte-identical to the commit |
| host | Apple arm64, Darwin 25.6.0, 14 logical CPUs; isolation **L0** (no pinning, other builds and sessions running) |
| curves | `CurveParams::p192()`, `CurveParams::p224()` (FIPS 186-4 §D.2.1, §D.2.2), cofactor 1 |
| sets | `gmul` = `x([i]G)`; `rscalar` = `x([s]G)`, `s` uniform in `[1, n−1]`; `rx` = uniform `x` on the curve; `2^24` each; `B = 2^20` is the prefix |
| seeds | `StdRng::seed_from_u64(20261019·2^20 + curve·2^12 + set·2^8 + chunk)` (the protocol's bitwise OR; the fields do not overlap), 256 chunks of `2^16` |
| requested / completed | 6 streams of `2^24` / 6 of 6; no failures |
| wall time (informational, L0) | P-192: gmul 3.3 s, rscalar 59.9 s, rx 190.0 s; P-224: gmul 3.6 s, rscalar 58.6 s, rx 119.7 s |
| earlier run | 20261019T2147Z, same code before `cargo fmt` and a clippy rename: identical JSON apart from wall times. Its process exit was 1 because `/usr/bin/time -l` could not read a sysctl under the sandbox after the program had finished |
| raw output | [`e5/e5a.json`](../../e5/e5a.json), [`e5/e5a.log`](../../e5/e5a.log) |

## Measured values

- Checks: `G` on curve and `[n]G = O` on both curves; 202 BigUint
  cross-checks per curve, 0 mismatches; checkpoints `i ∈ {1, 2, 3, 1000, 2^16,
  2^16+1, 2^20, 2^24}` all seen.
- Family of 20 tests (two-sample chi-square on leading zeros and Hamming
  weight, gmul against each control; order-statistic test of gmul's minimum),
  Holm at α = 0.05: **0 rejections**; p-values from 0.225 to 0.992.
- Smallest abscissa among `gmul` (`2^24`): `2^167.13` at `i = 15 193 886`
  (P-192), `2^200.73` at `i = 4 133 403` (P-224); against the expectation
  `p/(B+1)`, log2 ratio −0.87 and +0.73.
- Mean Hamming weight (`2^24`): gmul 96.0002 / rscalar 96.0012 / rx 95.9987
  (P-192); 111.9963 / 111.9994 / 111.9992 (P-224).
- Positive controls (first valid abscissa planted in `rx`): detected in 4 of 4
  cells (p = 0, 0, 2.3e-61, 3.7e-60 against 0.0025).
- Calibration against analytic laws (24 tests, outside the family): smallest
  p 0.022.

![E5 figure](../../e5/e5.svg)

*Figure (panels A–C are this run; D–F are the E5b model, see E5.md). Run
20261019T2159Z, seed 20261019, `2^24` points per set and curve. Source:
`e5/plot_e5.py`.*
