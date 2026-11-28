# Broader repository library gate

Source baseline: original PR #5087 source head
`76998f1c0b15255bb26ea60427e35c52b968776e`, plus the tracking-only ACTIVE change
and the isolated standalone crate. The new crate is its own workspace and does
not modify the root library. All discovered local path dependencies were
materialized before these attempts; no missing-directory failure was invented.

Both commands used two Cargo build jobs, incremental compilation disabled and
the same retained target cache. The command was `cargo test --release --lib
--locked`, under a `timeout` wrapper. Logs are retained verbatim.

| Attempt | Outer compilation cap | Observed exit | Observation |
|---|---:|---:|---|
| 01 | 45 seconds | 124 | Dependency compilation; tests did not run |
| 02 | 180 seconds | 124 | Continued compilation from the same cache; tests did not run |

No root-library pass, compiler defect, out-of-memory event, or test failure is
inferred from these timeouts. A completed broader repository gate remains open.
These are development build attempts, not scientific solver timings.

Runtime target directory:
`/workspace/scratch/04738cbc9e23/qualification/root-target`.
The source cache and these logs were not deleted after the caps.

[pcc:unverified]
