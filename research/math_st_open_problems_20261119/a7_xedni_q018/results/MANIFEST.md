# Raw data kept out of Git

`results/runs.csv` (79,601,844 bytes; 300,000 runs) and `results/controls.csv`
(137 kB; 611 control cases) are over the artifact-store policy limit for data
files (64 KiB). They are not committed. Regenerate both with `./run.sh`
(seed 20261125; about 11 minutes on 4 vCPUs). A rerun of the first 2,000 runs
per sampler on 2026-11-25 reproduced every `runs.csv` column except
`seconds` (wall time). So the hash below identifies the reported run only; a
rerun matches it column by column, not byte for byte.

sha256 `8e99692a19b860ab4c2f7506da4ca2d29b78ddd2788e68e22c62f4f16726edc7`  results/runs.csv
sha256 `a4c30661e0aa4940c92813fcfe93ad2f2292968360f417344a72d351f55cb7b9`  results/controls.csv

`summary.json`, `controls_summary.json` and `fixtures.json` are committed
and were derived from these files by the same binary run.
