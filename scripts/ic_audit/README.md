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

An explicitly pinned `zero_replay_probe` covers a historical negative session
whose failed outcomes the native registry already checks statically. It binds
the exact session ID, complete input file map, record count and zero verified
records. A positive native replay request must report zero eligible candidates,
no replays, all sixteen required static checks passing, and exit0 on the current
binary. Every partition must carry that probe; ordinary empty receipts still
fail. Both serial and partitioned output label this case **static audit only**,
without claiming a solved scalar or reproduced replay. Changing the inputs,
adding an eligible candidate or omitting the explicit pin fails closed.

A plan may pin `expected_drift` as the exact nonempty array of
`{seq, record_id, detail}` for each known-drift session. Both serial and
partitioned checkers reject an extra or healed failure, a different failed
record, or a changed native error detail. `require_pinned_drift: true` requires
these pins on every drift entry. Historical unpinned plans retain their old
behavior; the completed roster's correction must be based on actual native
receipts and preserve the prior evidence. An identical entry may pin only[].

For an interrupted local verification, `REPLAY_RECEIPT_CACHE` may name a
directory containing indexed **full native receipts**. Reuse requires exactly
the same executable SHA-256, every current input SHA-256, the complete native
candidate count, and every candidate identity. Reuse is labeled `cached-full`;
neither a partial receipt nor a union summary can substitute. `REPLAY_START_INDEX`
changes only execution order and still visits the entire roster once.

`REPLAY_PARTITION_CACHE` may name an earlier output directory for that same
partition. Completed explicit native receipts can resume only with the same
roster, partition, candidate vector, selected identities, native probe count,
input and executable hashes, and successful native status binding. Reuse is
labeled `cached-explicit`, preserving the native receipts and a hash-bound copy
of the prior partition binding. An interrupted or missing binding is rerun;
an invalid completed binding is rejected. A partial cache never substitutes
for the other partitions or for full coverage.

For interruption-prone local audits, `REPLAY_CHUNK_SIZE=1` checkpoints each
selected record as a separate, unchanged native audit. Larger positive sizes
checkpoint that many selected records; the default0 preserves a single native
receipt per partition. `REPLAY_CHUNK_CACHE` may point to an earlier, distinct
output directory, including one interrupted before its final partition binding.
Only completed native statuses0/1 with the exact executable, input map, native
candidate probe and selected identities are reused. Failed statuses are rerun;
an invalid completed receipt is rejected. All original failed attempts remain in
their earlier directories.

A completed `chunked` partition binds an ordered manifest of native receipt
filenames, SHA-256 digests and actual exit statuses. The final checker validates
every native receipt, its session/file map, and the exact union against the
partition's selected identities before checking global coverage. It never
manufactures a combined `ecbench.audit/v1` receipt. Missing, duplicate, borrowed,
changed or unsuccessful chunks cannot complete the union. Chunking changes no
record, native algorithm, candidate selection or inner resource budget.

The workflow also divides the roster into eight disjoint session groups using
`REPLAY_GROUP_INDEX/REPLAY_GROUP_COUNT`, with at most four matrix jobs active.
Each group retains the four record partitions and the original per-job timeout.
Artifacts retain their partition directory and unique global session indices;
the final checker still requires all four partitions of every session. A
missing group cannot pass, and group splitting changes no replay selection.

Receipt bindings retain the full source plan SHA-256. Coverage binds the ordered
`dir/replay` roster separately, allowing evidence-backed expectation corrections
after an audit without claiming that its original plan already predicted them.

Run `test_replay_checks.sh` and `test_replay_partitions.sh` for metadata admission
and tamper controls. Their stubs never solve, benchmark, or supply research
evidence. The original `replay_plan.sh` remains the serial full-replay entry point.
