# Source session

T-2157, fence 1, 2026-11-11. Preserved the unexecuted v4 source and v3 terminal
failure. Replaced the broad `Path.exists()` absence classification with an
`lstat()` check that catches only `FileNotFoundError` during materialization,
and assigned fresh v5 workspace/unit identities. A local `py_compile` syntax
check passed and its generated cache was removed. No Rust/Cargo build, test,
resolver, or payload was run.
