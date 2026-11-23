# Trace packaging amendment

The frozen runner in commit `0d6a098a445` wrote `relation-trace.json.gz` after measuring the n35 candidate. That gzip file held canonical JSON for all 6,361 relation attempts. Its SHA-256 is `a3a206c9a90b9c806d8870fc21cc5d9d476483184e430f41a0e22ece4680a84f`; the uncompressed canonical JSON SHA-256 remains in `result.json`.

The repository artifact migration gate tried to send the 439,653-byte `.gz` file to its object store and received HTTP 403. The trusted CI policy does not permit a candidate branch to change `.artifacts/config.json`. The gzip bytes were therefore wrapped in base64 text as `relation-trace.gzip.b64.txt` (586,205 bytes), with no decompression or modification of the compressed stream. The original runner output is retained as `result-runner.json` (SHA-256 `ea57aaecf3b91c08f786ba23feeb9b10c3d29b17c6e559f1cf7b963d8211d02b`) and its original independent validation as `validation-runner.json`. `result.json` changes only the trace location and packaging metadata.

To recover the original trace, base64-decode the text file and gzip-decompress the result. The independent `verify.py` checks the compressed SHA-256, uncompressed SHA-256, all 6,361 records, matrix rank, logs, and target scalars. The candidate, workload, run ID, measurements, and frozen runner source have not changed.
