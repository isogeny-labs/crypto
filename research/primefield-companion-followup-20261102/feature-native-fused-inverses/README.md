# Fused coefficient denominator inversions

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md), [report.pdf](report.pdf), [flow.svg](flow.svg).

`python3 generate.py` reconstructs pinned predecessor copies and adapter/evaluator variants. `python3 validate-recovery.py /absolute/fresh/output/path` compiles optimized standalone Rust with overflow checks and runs deterministic controls with execution timeout60s. Preserve historical output directories. publication.json records hashes/bytes; native binaries are excluded.
