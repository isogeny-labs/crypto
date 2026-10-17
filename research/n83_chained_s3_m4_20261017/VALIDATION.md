# Validation record

Working branch: `codex/n83-chained-s3-m4-20261017`, stacked on
`a44fc59ecf6e5746ebb7b5767fa42a49b64ed4ae` (the source of PR #2100).

| Command | Status | What it checks |
|---|---|---|
| `cargo test --release --lib chained_s3_` | PASS, 6 passed, 0 failed, 4,327 filtered out; release build finished in 4m 19s and tests in 1.37s | four-summand witness, orbit, rank and cap cases |
| `cargo test --release --lib -- --test-threads=2` with localhost access, pre-merge code commit `b14b410171` | PASS: 4,216 passed, 0 failed, 117 ignored; 654.90 s test execution | repository release library gate |
| `cargo fmt --all -- --check` | PASS | Rust formatting |
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
Copilot-resolved head requires another current-head release run; this earlier
pass remains a record of the pre-merge code.

Conductor check was attempted through the MCP tool and the CLI before edits.
Both initially failed because its local service at `127.0.0.1:8443` refused
the connection. Once the CLI service returned, the same scopes checked clear
and task `T-3` was claimed. This branch changes the core library and new
study files; the primary adapter files being handled in PR #2100 were left
untouched.
