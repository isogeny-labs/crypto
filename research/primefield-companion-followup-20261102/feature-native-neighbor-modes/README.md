# Joint neighboring-mode coefficients

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md), [report.pdf](report.pdf), [flow.svg](flow.svg).

`python3 generate.py` reconstructs pinned predecessor copies/adapters. `python3 validate-recovery.py /absolute/fresh/output/path` compiles standalone Rust with optimization/overflow checks, then runs deterministic controls with execution timeout60s. Preserve prior evidence. publication.json records hashes/bytes; native binaries are excluded.
