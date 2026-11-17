# Validation receipts

The committed release-library log records `cargo test --release --lib --locked -- --test-threads=1`: 4,397 passed, 118 ignored, zero failures. The exporter-example log records `cargo test --release --example koblitz_n83_factor_base_export --locked`: 24 passed. The orbit-example log records `cargo test --release --example koblitz_orbit_dlp_fast --locked`: 22 passed. These logs are gzip-compressed with zero modification time.

The three Python logs record `python3 -m unittest discover -s research/koblitz_n83_factor_base_sweep_20261008 -p 'test_*.py'` (69 passed), `python3 -m unittest discover -s research/sat_factor_base_review_20260908/autolab -p test_boundary_autolab.py` (52 passed), and `python3 -m unittest discover -s research/n83_xor_decided_cache_20261019 -p 'test_*.py'` (2 passed).

`current-source-custody-repeat.log` and `attempt-archive-repeat.log` record idempotent second S3 HEAD/download/hash checks using the pinned scripts and original inputs. Python compilation and `git diff --check` also passed. Separate local negative controls rejected a modified pilot scalar sidecar and a provider snapshot falsely changed from `failure` to `success` before any S3 operation.
