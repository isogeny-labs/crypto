# Native recomputation of the root-finding baseline

Status: producer-independent recomputation from committed rows, `[pcc:unverified]`.
This validator does not rerun the benchmark and does not establish an admitted
performance measurement.

| A | Default ns/point | Subtract arm ns/point | Probe ns/point | Default/probe timing ratio | Subtract/probe timing ratio | 50x threshold | Ratio gap to 50x |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 64 | 109.671 | 24.912 | 11.064 | 9.912x | 2.252x | 2.193 ns | 5.044x |
| 256 | 118.090 | 25.775 | 9.999 | 11.810x | 2.578x | 2.362 ns | 4.234x |
| 1024 | 133.630 | 45.111 | 14.387 | 9.288x | 3.136x | 2.673 ns | 5.383x |

The committed baseline A/A rows span 0.894x to 1.056x across 9 prime-field
cells. That range describes historical baseline repeatability. It does not measure
probe repeatability.

The probe rows record 172032 abscissa checks, 0 mismatches,
and 2601/2601 aggregate exact hits through the filter. These
are producer-recorded counts rather than freshly replayed per-hit witnesses.

Baseline and subtract rows contain 256 targets per cell. Probe rows contain 2048
targets per cell and time a smaller operation boundary. The displayed quotients are
descriptive normalized timing ratios across those unmatched historical workloads.
The 50x values are derived thresholds against each frozen default row, not measured
arms. Every historical prime-field baseline row records
`isolation: none`.

## Erratum

`RESULTS.md` says the A = 1024 table build equals about 1,100 default targets.
The committed 110.217 ms build divided by 195.752 microseconds per target equals
about 563 targets. The frozen results file remains unchanged. See
[`ERRATA.md`](ERRATA.md).

Recompute from the repository root with `cargo run --locked --release --manifest-path
research/rootfind_baseline_100x_20261117/native/Cargo.toml -- --check`.
