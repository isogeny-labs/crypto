Original proposal files and MANIFEST.json are copied unchanged. Absolute paths in that manifest identify original sources; map their basenames to this directory. Historical proposal binaries are kept in the private software-evidence archive recorded in s3.receipt.json, not in Git.

Compile with `rustc +1.98 --edition=2021 --test harness.rs -o /tmp/boolean-batch-tests`, then `timeout 60 /tmp/boolean-batch-tests --test-threads=1`. This is standalone research source, outside the library and CLI.
