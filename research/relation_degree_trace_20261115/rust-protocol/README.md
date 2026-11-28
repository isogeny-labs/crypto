# Compact protocol analysis adapter

This standalone Rust package calculates exploratory case statistics from a
typed transcript. It launches no workers, CAS processes or cloud jobs and
generates no elliptic-curve fixtures. The process/backend adapter, mature-CAS
replay, checkpoint and isolation/budget admission remain separate obligations.
Read the frozen `../COMPACT_ANALYSIS_CONTRACT.md` and
`../COMPACT_SPEED_PROTOCOL.md` before use.

Build with `cargo build --release --locked --manifest-path
research/relation_degree_trace_20261115/rust-protocol/Cargo.toml`. For recorded
research, copy the binary to an immutable path and capture its pre-call hash
before using it. The CLI takes no arguments and reads at most 16 MiB of JSON
from stdin. Its input schema is `compact-speed-transcript/v1`; see public
`Case`/`Sample` types in src/lib.rs for exact names and types. Unknown fields
are errors. All durations are positive integer nanoseconds.

Case identities bind each sample to the same original input and immutable
three-arm worker, and bind native/validator basis keys to a separate reference
key. A matching digest is only an equality check on producer data: execution
and mathematical provenance are not established by these keys. Input fields
`worker_wall_ns`, `validation_wall_ns` and `setup_wall_ns` must contain the
paid scopes defined in the contract; internal worker clocks cannot substitute
for parent launch-through-consumption intervals.

The report retains native-only and full-cost paired ratios, ordinary 95%
percentile intervals, the adjusted 1/720 lower endpoint, A/A diagnostics and a
conditional `numerical_rule_passes` flag. It always reports
`execution_admission_verified=false` and `family_confirmed=false`. These
false fields are part of the output's scope, not an independent verdict.
Fresh seeds are rejected until a family-admission layer is implemented.

Exit 0 means a complete exploratory case was calculated, including a complete
case that loses the numerical criterion. Exit 2 means input/accounting/order,
completeness or A/A rejection; a JSON failure report is still emitted. Missing
or failed samples are never replaced, and failed cases have no main numeric
comparison. Preserve the raw transcript and execution receipts outside this
calculator. The caller must create output files exclusively; the CLI writes
stdout and does not manage evidence directories.

tests/support/ contains hand-authored synthetic numbers/dummy digest keys,
never observations. The tests check structural/accounting/statistical behavior
only. Their elapsed times and synthetic numerical flags are not performance
results. No benchmark or fresh-family confirmation has been run by this package.
