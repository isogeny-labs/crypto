# Validation record

Working branch: `codex/n83-chained-s3-m4-20261017`, stacked on
`a44fc59ecf6e5746ebb7b5767fa42a49b64ed4ae` (the source of PR #2100).

| Command | Status | What it checks |
|---|---|---|
| `cargo test --release --lib chained_s3_` | PASS, 6 passed, 0 failed, 4,327 filtered out; release build finished in 4m 19s and tests in 1.37s | four-summand witness, orbit, rank and cap cases |
| `cargo test --release --lib -- --test-threads=2` with localhost access, pre-merge code commit `b14b410171` | PASS: 4,216 passed, 0 failed, 117 ignored; 654.90 s test execution | repository release library gate |
| `cargo test --release --lib chained_s3_` after main merge at `df11316e711878522b0e4da6ac1cb695743c418f` | BLOCKED: exit 101, 26 inherited compile errors; [log](final-focused.log) | current-head focused gate |
| `cargo test --release --lib -- --test-threads=2` after main merge | BLOCKED: exit 101, same 26 inherited compile errors; [log](final-release-lib.log) | current-head full gate |
| `cargo test --release --lib -- --test-threads=2` after comment correction | BLOCKED: exit 101, same 26 inherited compile errors; [log](after-comment-release-lib.log) | latest-head full gate |
| `cargo fmt --all -- --check` | PASS | Rust formatting |
| `cargo fmt --all -- --check` after main merge | FAIL on unrelated inherited files; [log](final-fmt-repo.log) | repository formatting gate |
| `rustfmt --check src/cryptanalysis/koblitz_index_calculus.rs src/cryptanalysis/binary_semaev_chain_sat.rs` after main merge | PASS | touched Rust files |
| `git diff --check` | PASS | whitespace integrity |

No degree-83 retained-base query or performance measurement is in this
validation record. The exact m=4 construction preflight counts are checked
by release library assertions, but they are not allocated-memory readings.

The first full-suite attempt in the default sandbox was terminated after
2,746 reported tests and nine failures confined to the `pollard_collab::cairn`
and `pollard_collab::net` fake-server tests. These fixtures call
`TcpListener::bind("127.0.0.1:0")`; the sandbox disallows local binds. The
partial output is preserved in [cargo-release-lib-sandbox-partial.log](cargo-release-lib-sandbox-partial.log).
The replacement run used localhost access and passed. Its complete output is
[cargo-release-lib.log](cargo-release-lib.log). A later merge of PR #2100's
Copilot-resolved head required another current-head release run. Both the
focused and exact full command stopped during compilation in ecbench and
quartic files unchanged by this branch. The specific inherited gap and exit
conditions are recorded in [the build problem](../../problems/current-main-library-build-20261017.md).
The earlier pass remains a record of the pre-merge code, not a pass on the
merged head.

The later correction to the chained-S3 doc comments did not change executable
code. Its fresh release-lib attempt stopped at the same 26 inherited errors;
the touched file still passes `rustfmt --check`.

Conductor check was attempted through the MCP tool and the CLI before edits.
Both initially failed because its local service at `127.0.0.1:8443` refused
the connection. Once the CLI service returned, the same scopes checked clear
and task `T-3` was claimed. This branch changes the core library and new
study files; the primary adapter files being handled in PR #2100 were left
untouched.
The later scoped Conductor check returned `404 not_found`; its local
coordination state could not be refreshed for the comment-only correction.
