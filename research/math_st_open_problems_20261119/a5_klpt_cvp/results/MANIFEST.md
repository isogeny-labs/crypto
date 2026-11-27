# Raw data kept out of Git

`results/runs.csv` (220,763 bytes; 1050 runs) is over the artifact-store
policy limit for data files (64 KiB). It is not committed. Regenerate it with
`./run.sh` (seed 20261123; under one second). As with A4, every column except
`seconds` (wall time) is reproducible, so the hash identifies the reported run.

sha256 `32c754de2cf8f02dac3fb145ca0902ca44c8f76def990e139a14dad4b1fc8a38`  results/runs.csv

`paired.csv` is written by `target/release/pair ../a4_klpt/results/runs.csv
results/runs.csv`. It needs A4's regenerated `runs.csv`; check that file against
A4's `results/MANIFEST.md`.
