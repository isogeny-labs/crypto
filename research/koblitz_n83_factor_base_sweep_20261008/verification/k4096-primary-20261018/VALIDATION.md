# Validation of the primary K4096 S3 object

The [independent audit](evidence/audit.json) passed against the original
construction directory and again against a copy of the retained metadata
rehydrated with a fresh S3 GET. The two audit outputs are byte-identical. It
checked the frozen source and supervisor Git blobs, binary and input hashes,
live Docker limits and cleanup, construction and native-stage receipts, S3
HEAD/GET identity, and every one of the 679,936 downloaded point rows. The
[negative control](evidence/negative-control.json) altered the full replay
point count, recomputed the outer receipt hash, and was rejected by the audit.

| Check | Result | Retained evidence |
| --- | --- | --- |
| `cargo test --release --lib --locked -- --test-threads=4` | 4,085 passed; 117 ignored; zero failed; clean exit | `evidence/validation/lib-four-threads.log` |
| `cargo test --release --example koblitz_n83_factor_base_export --locked` | 19 passed; zero failed; clean exit | `evidence/validation/example.log` |
| N83 Python `unittest discover` via `uv run --with blake3` | 63 passed; zero failed | `evidence/validation/n83-python.log` |
| Boundary autolab Python `unittest discover` via `uv run --with blake3` | 46 passed; zero failed | `evidence/validation/boundary-python.log` |
| Positive audit, retained-copy audit and forged-receipt rejection | Passed | `evidence/audit.json`, `evidence/retained-copy-audit.json`, `evidence/negative-control.json` |
| JSON parse, retained-file SHA-256 manifest and staged diff check | Passed after evidence closure | `evidence/evidence-manifest.json` |

For replay from the committed evidence, copy its `evidence/` tree to a fresh
directory, then download the exact S3 URI in [RESULT.md](RESULT.md) both to
`fresh-get.jsonl.gz` and to the `construction-01/object/objects/` path named
in the retained manifest. Require SHA-256
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`
and BLAKE3
`a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779`
on each copy before running `audit.py`. The native exporter used for this
audit has SHA-256
`5881dc22b3367b6076ef5c3d452eb270bfa3c4013ec63782a0276b4670cdc094`.
The object is excluded from Git and retained under its content-addressed S3
key. `negative_control.py` reproduces the forged-receipt rejection without
changing the retained source evidence. Validation timings do not measure a
complete index-calculus attack.
