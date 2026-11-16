# Direct oriented phase supports

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md), [report.pdf](report.pdf) and [flow.svg](flow.svg).

`python3 generate.py` reconstructs pinned predecessor copies and the separate direct adapter. `python3 validate-recovery.py /absolute/fresh/output/path` compiles standalone native Rust with optimization and overflow checks, then runs deterministic controls with a 60-second execution timeout. Preserve prior output directories. publication.json records hashes and bytes; native binaries are excluded.
