# Raw evidence retrieval

Use `ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/root-library-v4-results-20261109` from repository root. This hydrates the native archive and partial named-row CSV; it does not rerun tests.

Archive SHA256: 47118d6241964a6da200130ffb2b97a532da30df1147522e6873113f5f04b631; 321660 bytes;29 native/caller files. Its S3 URI is s3://setec-astronomy-artifacts/store/sha256/47/47118d6241964a6da200130ffb2b97a532da30df1147522e6873113f5f04b631. Full independent GET and SHA verified after publication. Compare raw-hashes.json after extraction. Terminal-output-manifest and log-prefix checks are independently verified by this review. Missing final suite summary and post-custody remain missing.

The CSV contains only complete named output rows seen before interruption; the unfinished line remains in result.json. No replay/compilation command is authorized by this retrieval note.
