# Main-sync validation evidence (2026-11-01)

`attempt-00-no-library-target/` preserves the invalid `--lib` command result. `attempt-00-missing-test-output/` preserves the first binary-suite failure caused by a missing persistent-output environment variable. `attempt-01-with-persistent-test-output/` is the successful complete binary-target run: 356 passed, 0 failed, 0 ignored; its 234 generated fixture files and exact Cargo log are retained. `run.json` binds the successful invocation, output directory, merge head and log hash.

`SHA256SUMS.json` lists hashes and sizes for every file in this package except the manifest itself.
