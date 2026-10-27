# Validation of the K4096 model and bounded search

The [auditor](audit.py) passed on original receipts and a separate retained
copy. It checks S3 HEAD/GET custody and compressed bytes, frozen Git source
blobs for both commits, the static Linux worker, panel and public fixture
hashes, Docker hard limits, first-attempt producer failure, corrected model
dimensions, bounded search counters and events, public query/group replay,
raw log hashes and container cleanup. The [negative control](negative_control.py)
increments the copied model's total clause count and recomputes the outer
worker SHA-256; the auditor rejects the forged receipt. The missing-corpus
preflight failure is retained as a separate negative control.
The auditor runs on the same host and repository as the experiment; replay
by an external independent implementation or host remains pending.

The large compressed factor-base object remains at the exact S3 URI in
[RESULT.md](RESULT.md). To reproduce the audit, GET that key and run
`audit.py --checkout CHECKOUT --evidence evidence --object OBJECT --output NEW_JSON`.
The auditor checks the fetched object's SHA-256, BLAKE3 and length before
accepting the stage records. `negative_control.py` copies the retained
receipts to a scratch directory and repeats the mutation test.

| Check | Result | Retained evidence |
| --- | --- | --- |
| `cargo test --release --lib --locked -- --test-threads=4` | 4,085 passed; 117 ignored; zero failed; clean exit | `evidence/validation/lib-four-threads.log` |
| `cargo test --release --example koblitz_n83_factor_base_export --locked` | 19 passed; zero failed; clean exit | `evidence/validation/exporter-example.log` |
| N83 Python `unittest discover` via `uv run --with blake3` | 63 passed; zero failed | `evidence/validation/n83-python.log` |
| Boundary autolab Python `unittest discover` via `uv run --with blake3` | 46 passed; zero failed | `evidence/validation/boundary-python.log` |
| Original, retained-copy and forged-model audits | Passed; false model count rejected after hash recomputation | `evidence/audit.json`, `evidence/negative-control.json` |
| JSON/JSONL parse, retained-file SHA-256 and diff checks | Passed | `evidence/evidence-manifest.json` |

The manifest covers 65 retained files and 11,406,340 bytes, excluding its
own self-reference and the separately hash-pinned S3 object. These checks do
not turn the capped search into a complete runtime comparison.
