# PR #2365 delivery check and main rule

Checked on 2026-10-22 against exact PR head
`d44ac0e843990cc77c26f5d8714c5e1e88aa7da0` and merge commit
`02b37807a9bec066b5508be22a9f0a62bc1a2369`.

The check rollup reported 32 successes, two skips, one neutral external
review, seven queued jobs, and one cancelled macOS job. The cancelled
`icx engine (multi-arch)` macOS job had completed its build, unit,
integration, catalog, and same-family pipeline stages; its GPU FES host
verification step was cancelled, leaving later steps skipped. The seven
pending/queued jobs were `ecbench`, `CodeQL Advanced`, `Workflow files
parse`, `Rust all targets compile`, `curve-names`, `Rust lint`, and
`Workflow syntax`; their GitHub run records still showed the original
2026-10-22 05:43:42 UTC update. This does not qualify the merged PR's
current head as having a complete CI pass. No result was inferred from
queued or cancelled jobs, and no check was rerun or bypassed for a green
label. The raw check snapshot is [`runs/pr2365-checks.json`](runs/pr2365-checks.json).

The legacy branch-protection API reported no protection before this work.
Its PUT returned GitHub HTTP 500 twice, and a later GET still returned 404.
The repository ruleset API succeeded. Active [ruleset 25687816](https://github.com/aburan28/crypto/rules/25687816)
now targets `refs/heads/main` and requires pull requests, merge commits,
resolved review threads, strict current-base `gate` status, no force pushes,
and no deletion; it has no bypass actors. The `gate` job is from
`.github/workflows/validation-gate.yml`, runs for every pull request,
and is unique among the workflow job names. GitHub's effective rules
endpoint returned these rules after creation. It applies prospectively;
it cannot retroactively complete the old PR's queued jobs.
The raw returned ruleset is [`runs/main-ruleset.json`](runs/main-ruleset.json).

The exact current-head CI state should be rechecked from GitHub before
making any new qualification claim. The new research PR is subject to the
ruleset and its own current-head checks.

## Local validation

The first `cargo test --release --lib` attempt stopped during dependency
compilation because the inherited Rust temporary directory was
`/Volumes/SSD990/llm/tmp`, on a full volume. Its stdout and stderr are
retained as `runs/cargo-test-first.*`. The retry set both `TMPDIR` and
`CARGO_TARGET_DIR` under `/Volumes/T7/codex/` and completed:
`4477 passed; 0 failed; 119 ignored; 0 measured; 0 filtered out`, with
test runtime 522.37 seconds. Its full stdout and stderr are retained as
`runs/cargo-test-retry.*`. The raw Python/GP screen checks are separate
from this repository-wide Rust test.
