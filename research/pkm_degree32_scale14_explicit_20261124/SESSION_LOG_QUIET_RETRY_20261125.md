# Quiet-load retry session log, 2026-11-25

All times below are America/Los_Angeles. This log records the qualification
decision; the JSONL store is the raw machine-readable record.

- 14:39: the pre-existing `p18_full.sh` benchmark was still active. The
  source-pinned `ic` executable at
  `/Volumes/SSD990-2/.codex/tmp/pkm-a550-target/release/ic` matched SHA-256
  `c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`.
- 14:42: `p18_full.sh` finished, while separate `msolve` and `ecbench`
  compilation jobs occupied the host. The 14-bit timed sequence remained idle.
- After explicit authorization to stop competing jobs, SIGTERM was sent to
  process groups 45291 (`run2.py`/`msolve`), 3709 (`ecbench` build), and 82145
  (`cargo test`). A process check found these groups exited. Further jobs
  began in groups 29148 (`ecbench` build), 75127 (`p61_parity_probe`), 34107
  (`ecbench` build), and 34692 (Picard solver); SIGTERM was sent to each.
- 14:52: the fresh qualifying store was initialized from the 96 frozen
  source-pinned selection and size-only records. The source store and its
  aborted timed attempt were left intact. The new store's provenance row
  identifies the source SHA-256.
- 14:52: the guarded curve-6 runner appended `load-check:6:1` and exited
  before any timed invocation. The raw receipt lists a concurrent
  `p61_parity_probe` and transient coordination processes. Additional
  research jobs started afterward, so no subsequent timed call was attempted.

The new store is
`source_pinned_explicit14_qualifying_20261125.jsonl`, SHA-256
`ebb2550492f5c2dab621d0d4dff6dfed3a3057b4fd5c51cba2084f5e5512ee33`.
The three selected curves remain 6, 13, and 36; no target or base was changed.
The full-rank panel was not started while the explicit panel was unqualified.
