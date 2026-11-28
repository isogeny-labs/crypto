# Shared coefficient traversal across phase branches

See [DERIVATION.md](DERIVATION.md), [RESULTS.md](RESULTS.md), [report.pdf](report.pdf) and [flow.svg](flow.svg).

`python3 generate.py` reconstructs pinned predecessor copies and the shared adapter. `python3 validate-recovery.py /absolute/fresh/output/path` compiles standalone native arithmetic with optimization/overflow checks and runs deterministic checks with execution timeout60s. Preserve existing evidence. publication.json records hashes/bytes; native binaries are excluded.
