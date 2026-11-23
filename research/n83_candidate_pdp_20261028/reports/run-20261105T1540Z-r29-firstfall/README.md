# N83 spike29 fixed-base first-fall run

## Result

The same spike29 factor bases were used at every multiplier degree m=2..9. All 48 frozen V2 cases completed, produced a certificate, replayed in a fresh process, and had the certificate read back from S3. Koblitz and general-binary systems each had exact first-fall degree 2 in all 16 cases. Prime systems had certified lower bound at least 5 in all 16 cases, with no exact first-fall through the searched range. No candidate distinction appears in these labels.

| Family | Cases | Exact FFD | Certified lower bound >=5 | Exact LFD | Affine Dreg |
|---|---:|---:|---:|---:|---:|
| Koblitz | 16 | 16 at degree 2 | 0 | 0 | 0 |
| General binary | 16 | 16 at degree 2 | 0 | 0 | 0 |
| Prime | 16 | 0 | 16 | 0 | 0 |

The dataset labels first fall only. It does not label exact LFD or affine degree of regularity, and therefore cannot rank bases by those targets. Independent validation was written after data collection; it checked source/certificate/replay/readback linkage and did not modify the run.

## Frozen run and provenance

- Protocol: `N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2`
- Protocol SHA-256: `50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9`
- Run prefix: `s3://setec-astronomy-artifacts/runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike29/attempt-01/`
- Summary SHA-256: `7598d41a675e4edfaf2f32b7f08456f186116560382720902d6936717a6d0f74`
- Archive: `panel-complete.tar.gz` (118893 bytes), SHA-256 `031210e42946033daa78a04066cfaa6d01c07621d0da3eabda65561ffe4615d2`
- Archive readback: verified; independent validation: pass
- Source binding audit: 48 source/replay links; common fixed-base manifests and SHA-256 manifests checked before the run.
- Execution: 1 CPU, 8 GiB, 60-second per case/replay limits, 2-hour wall deadline, no retries. 48/48 cases, certificates, replays, and S3 certificate readbacks.
- Cost ceiling: USD 2.00 compute plus USD 0.25 artifacts (USD 2.25 total). Observed Spot quote was USD 0.5321/hour; approximately 3.5 minutes of elapsed run time implies about USD 0.031 compute, an estimate rather than a billing record.

## Reproduction and saved files

The complete run archive includes attempts, certificates, S3 readback records, the summary, and its file layout. Original system inputs and factor-base source objects remain at their content-addressed Git paths and are cross-linked by `source-index.json` and the source-binding audit. The archive was downloaded from S3 and its byte length and SHA-256 matched the custody record.

`plot.csv` is the plotted count data, `plot.svg` is the graph, and `plot_source.py` regenerates the SVG using only Python's standard library. `report.pdf` is the rendered copy of this report.


The post-run analysis package is also saved at `s3://setec-astronomy-artifacts/runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike29/attempt-01/analysis/addendum-01.tar.gz` (54,138 bytes, SHA-256 `6a56c2103b5e852a6ea2fe314c46b614256b63e5878913a93b2e27957a284a87`), with a verified full-object readback.
