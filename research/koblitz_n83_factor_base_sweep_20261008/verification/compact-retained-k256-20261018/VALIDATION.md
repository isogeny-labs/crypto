# Validation for retained compact-root evidence

All checks below ran in the feature worktree after the K64/K256 experiments.
They validate the source and custody path; the long release-suite wall time
on the shared host is not an IC benchmark.

| Check | Result |
| --- | --- |
| `cargo test --release --lib --locked` with target directory on SSD2 | 4,085 passed; 117 ignored; 0 failed; 1,231.79 s test-binary time |
| `cargo test --release --example koblitz_n83_factor_base_export --locked` | 19 passed; 0 failed |
| N83 `python -m unittest discover -s research/koblitz_n83_factor_base_sweep_20261008 -p 'test_*.py' -v` via `uv run --with blake3` | 63 passed; 0 failed |
| Independent K64 and K256 `audit.py` against fresh S3 inputs, frozen source, live Docker inspection, raw worker/outer receipts and container cleanup | Both `PASS_CAPACITY_ONE_QUERY_MISS` |
| Auditor on a local copy with `worker.status` changed from `MISS` to `HIT` | Both rejected with nonzero exit and no audit output |
| `git diff --check` and JSON parse of the charge receipt | Passed |

The K64 and K256 raw experiment receipts and their SHA-linked compact copies
are in the sibling `evidence/` directories. Both exact factor-base objects
remain in the requested S3 prefix.
