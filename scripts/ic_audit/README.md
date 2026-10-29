# Complete native leaderboard replay

`check_plan_coverage.sh` finds every complete session cited by the board and
lists the exact frozen core files and stderr attachments to hydrate. The
two-pass transport in `pull_replay_inputs.sh` retains the existing artifact
authentication and hash checks.

The distributed workflow builds one native `ecbench`, then invokes
`replay_partition.sh BINARY OUTPUT PARTITION COUNT` for every partition. It
supports the plan's full-replay entries only; it never reduces the sample.
The script proposes eligible sequence numbers using the registry predicate in
`src/cryptanalysis/ecbench/audit.rs`. A native single-record probe supplies the
authoritative candidate count. Every proposed sequence is then explicitly
validated and replayed by the native auditor. Equal cardinality plus native
membership checks establish full coverage; a changed predicate fails closed.

`check_replay_partitions.sh BINARY ROOT COUNT` requires all partitions under
`ROOT/0`, `ROOT/1`, etc. It checks the same binary, session and complete file
hash map, and requires each candidate's `(seq, record_id)` exactly once.
Native integrity checks must all pass. Only then does it compare identical or
drift with the plan; healed drift is a failure too. Its `ic-replay-union/v1`
output is a derived coverage summary, never a native audit receipt. The
original native receipts remain separate artifacts.

For an interrupted local verification, `REPLAY_RECEIPT_CACHE` may name a
directory containing indexed **full native receipts**. Reuse requires exactly
the same executable SHA-256, every current input SHA-256, the complete native
candidate count, and every candidate identity. Reuse is labeled `cached-full`;
neither a partial receipt nor a union summary can substitute. `REPLAY_START_INDEX`
changes only execution order and still visits the entire roster once.

Receipt bindings retain the full source plan SHA-256. Coverage binds the ordered
`dir/replay` roster separately, allowing evidence-backed expectation corrections
after an audit without claiming that its original plan already predicted them.

Run `test_replay_checks.sh` and `test_replay_partitions.sh` for metadata admission
and tamper controls. Their stubs never solve, benchmark, or supply research
evidence. The original `replay_plan.sh` remains the serial full-replay entry point.
