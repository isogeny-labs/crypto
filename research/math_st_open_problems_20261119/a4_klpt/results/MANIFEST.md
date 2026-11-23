# Raw data kept out of Git

`results/runs.csv` (226,502 bytes; 1050 runs) is over the artifact-store
policy limit for data files (64 KiB). It is not committed. Regenerate it with
`./run.sh` (seed 20261123); the KLPT runs take under one second. A rerun on
2026-11-23 reproduced every column exactly except `seconds` (wall time). So
the hash below identifies the reported run only; a rerun matches it column by
column, not byte for byte.

sha256 `1e17f4969f55725074395998579ce4133b089b3ca453be3e85d3ccc0d54624f1`  results/runs.csv

`summary.json` and `control1.csv` are committed and were derived from it.
