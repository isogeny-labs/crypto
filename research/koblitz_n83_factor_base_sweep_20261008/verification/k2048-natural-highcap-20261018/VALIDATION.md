# Validation of the retained K2048 search

The independent [audit](evidence/run-01/audit.json) passed against the
original experiment directory and again against the committed evidence copy
using the frozen exact S3 object. It verified source snapshots, the static
binary digest, S3 bytes and public fixtures, live Docker limits, worker and
outer receipts, original-curve public-query replay, and container cleanup.
The [negative control](evidence/negative-control.json) forged matching worker
and outer success statuses and recomputed the worker hash; the auditor
rejected the altered receipt.

| Check | Result | Retained evidence |
| --- | --- | --- |
| `cargo test --release --lib --locked -- --test-threads=4` | 4,085 passed, 117 ignored, zero failed; clean process exit | `evidence/validation/lib-four-threads.log` |
| `cargo test --release --example koblitz_n83_factor_base_export --locked` | 19 passed, zero failed; clean process exit | `evidence/validation/example.log` |
| N83 Python `unittest discover` via `uv run --with blake3` | 63 passed, zero failed | `evidence/validation/n83-python.log` |
| Boundary autolab Python `unittest discover` via `uv run --with blake3` | 46 passed, zero failed | `evidence/validation/boundary-python.log` |
| `git diff --check`, JSON parse and retained-file SHA-256 manifest | Passed after evidence closure | `evidence/evidence-manifest.json` |

The first default-parallelism release suite is also retained, rather than
discarded. It reported 4,081 passes, four `jv_cover` failures and 117
ignored tests. Its failure traces show timed-out F4 work and skipped samples
under concurrent load. A serial rerun of the entire `jv_cover::tests::`
group passed 14 tests with one ignored in 24.02 seconds. The subsequent full
four-thread rerun passed all 4,085 active library tests in 525.77 seconds.
The broad-run log and serial replay are
`evidence/validation/lib.log` and `evidence/validation/jv-cover-serial.log`.
These validation times are not index-calculus measurements.
