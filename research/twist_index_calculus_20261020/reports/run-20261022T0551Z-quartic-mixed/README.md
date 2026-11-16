# Quartic twist two-line relation-yield run

UTC start 2026-10-22 05:51:35.462; completion 05:52:50.982. Shared Intel Xeon 6975P-C x86-64 host, 32 logical CPUs, `rustc 1.99.1`, `RAYON_NUM_THREADS=16`, L0. The [registered protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md) compares the line `x∈Fp` with `x∈Fp ∪ t²Fp` on seven curves and seeds `0,1,2`, using the same one-target residual stream in each pair. Producer `quartic_folds.rs` SHA-256 `fb772ea12828b7f1f150a44416836edf93f371433bd9bb201cee20e8c0894c2a`; driver `quartic_twist_mixed_lines.rs` SHA-256 `60e0511efe0f79a61f01287901528ca89a797c4bf531e0b32ba4ffbc0fc91d17`.

```sh
RAYON_NUM_THREADS=16 TMPDIR=/home/ec2-user/.cache/gls-report-tmp timeout 1800s target/release/examples/quartic_twist_mixed_lines > research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson 2> research/twist_index_calculus_20261020/quartic_twist_mixed_v1.log
```

All 42 registered runs completed: 126 flat/cubic/twist24 IC solves, zero failures, 69,045 processed residuals, 7,275 decompositions, 5,298 selected relations. The [native audit](../../quartic_twist_mixed_native_audit_v1.json) independently recomputed every query's hit count with a second signed-pair algorithm, reconstructed every selected relation and row, verified all scalars and rank spans, and checked paired target-stream prefixes and line-0 hit inclusion. Raw [NDJSON](../../quartic_twist_mixed_v1.ndjson) SHA-256 `d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c`; [stderr](../../quartic_twist_mixed_v1.log). The [native summary](../../quartic_twist_mixed_summary_v1.json) retains all 21 pairs.

![Paired relation yield and separate cost units](quartic_mixed_lines_v1.svg)

The union raises same-target hit yield in all seven curves (median paired ratios 2.04–17.55), while increasing matrix columns and row work. Counted field multiplications improve across all three seeds at `p=229,241`, regress at `p=37,73,157`, and straddle parity at `p=97,337`. These are count projections, not isolated whole-method speedups. The [results report](../../QUARTIC_TWIST24_REPORT.md) and [PDF](README.pdf) give the cost boundary.
