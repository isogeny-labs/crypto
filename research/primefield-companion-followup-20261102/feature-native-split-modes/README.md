# Split mode evaluation

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md), [report.pdf](report.pdf) and [flow.svg](flow.svg).

Rebuild the copied sources with `python3 generate.py`, then run `python3 validate-recovery.py /absolute/path/to/fresh-output-directory`. The checker compiles standalone Rust with optimization and overflow checks; execution timeout60s. Preserve prior output directories. No timing or cloud workload. `publication.json` records hashes and byte lengths.
