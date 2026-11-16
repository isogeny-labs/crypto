# Validation of the approved mixed-orbit replication

The [independent audit](evidence/audit.json) passed against the run evidence
and again against a separate copy of the retained files with the same
hash-verified S3 inputs; outputs were byte-identical. It checked the
authorization and historical budget field, exact S3 HEAD/GET metadata and
compressed bytes, all frozen Git source blobs, Linux ELF hash, live build
and search Docker limits, import counts, worker and event counters, original
public coefficient schedule, stage log hashes, cleanup and round charge. The
[negative control](evidence/negative-control.json) changed one copied event's
conflict count from 5,000 to 4,999 and the auditor rejected it.

| Check | Result | Retained evidence |
| --- | --- | --- |
| `cargo test --release --lib --locked -- --test-threads=4` | 4,085 passed; 117 ignored; zero failed; clean exit | `evidence/validation/lib-four-threads.log` |
| `cargo test --release --example koblitz_n83_mixed_orbit_worker --locked` | 6 passed; zero failed; clean exit | `evidence/validation/example.log` |
| N83 Python `unittest discover` via `uv run --with blake3` | 63 passed; zero failed | `evidence/validation/n83-python.log` |
| Boundary autolab Python `unittest discover` via `uv run --with blake3` | 46 passed; zero failed | `evidence/validation/boundary-python.log` |
| Positive, retained-copy and forged-event audits | Passed | `evidence/audit.json`, `evidence/retained-copy-audit.json`, `evidence/negative-control.json` |
| JSON parse, retained-file SHA-256 manifest and staged diff check | Passed after evidence closure | `evidence/evidence-manifest.json` |

To replay the audit, fetch both exact S3 objects named in [RESULT.md](RESULT.md)
and run `audit.py` with `--evidence`, `--small-object` and
`--envelope-object`. The auditor checks their compressed SHA-256 and BLAKE3
before accepting any stage receipt. The binary, source hashes, raw logs and
live Docker inspections are retained; the two large compressed bases remain
in S3. `negative_control.py` reproduces the event mutation in a temporary
copy. Test and stage wall times are not a complete cold index-calculus
comparison.
