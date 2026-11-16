# Native N83 diagnostic runner

`freeze-free-selector` selects the separate original fixed-coordinate /
free-selector workload. The post-solve export worker now exports actual
model selector bits; native audit independently replays each index/phase
and the original group equation. It never adds selector assumptions.
`check-sat-trace FILE` validates a single solve's native boundary trace or
valid killed prefix; only solve_return supplies terminal counters. Docker
sets N83_SAT_PHASE_TRACE=1. Initial propagation and CDCL entry are separate,
with existing cumulative engine timers and conflict snapshots. Historical
workers without this trace use their own frozen runner.

Native card reports preserve original receipt bytes after checking identity;
they never reserialize floating-point seconds for relay comparison. The
exact problematic float spelling has a native regression test. The original
failed check and both corrected assessments remain preserved in the study.

Rust replacement for the bounded retained-base phase observation path. It
uses no Python. Build/test with `cargo test --offline --manifest-path
 tools/n83-sweep-runner/Cargo.toml` (remove the line-wrap space when entering).
`freeze` creates a write-once, pre-pinned spec; `run SPEC NEW_DIR` executes it;
`audit DIR` independently checks custody, resources, phase order, original-
group public queries and the supported zero-relation terminal. Columns0
selects the pinned N83 fixed-witness encoding/group-sum control. Columns2048
and4096 select retained m5 natural-query diagnostics. Resource and algorithm
constants are frozen in source and the study protocol.

The runner rejects unsupported positive natural relations/scalars rather
than treating them as verified. Full native relation/rank/scalar replay is
future work. Censored observations retain unknown counters, including empty
traces. An independent in-container timeout limits computation after a
controller failure. Failure logs and available final state are retained.

`card-prompt`, `card-run`, `card-report`, `card-check` implement a strict native
subset of the existing canonical JSON frozen-command card/receipt schemas.
The original `tools/task_card.py` is unchanged and is never executed here.
Supported: SHA256 binds, exact HEAD, fresh scoped outputs, shell-free argv,
per-command process-group deadlines, write-once receipts, output hashes and
20-line tails. Refused: runbooks, EXP-* approvals, retries, non-root cwd and
environment overrides other than the two fixed trace/thread flags. Exit0
means completed,2 refused,3 stopped. A launch/bind/I/O error after execution
starts is preserved as a failed command (null native exit if no child exit
exists); receipt-storage failure itself remains an explicit fatal I/O error.
Card SHA256 records the originally loaded bytes; edits stop the card. Relay
verification checks exact receipt rendering plus current stream/output hashes.
This is an explicit native execution-path substitution, not a claim of full
feature parity with the general Python card implementation.

The ignored study exec workspace permits sequential cards to retain raw
outputs while the source checkout stays clean. Compact evidence is added
explicitly after audit; source snapshots and raw binary copies are not
silently staged into Git. See the study native-phase-20261019 protocol.

The v2 selector-witness control additionally replays each declared domain
index and Frobenius phase with independent polynomial-word arithmetic. A
wall-censored control now emits a typed UNKNOWN_CONTROL_WALL_CAP audit and
still fails its positive gate. The original free-selector control is retained
as censored; the fully specified witness is a distinct workload.
