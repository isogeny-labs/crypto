# Materializer journal: bounded storage fault qualification

Run: pencil-materializer-qualification-20261109. Task: T-1957. Publication: 6d380c379d63c49f6e1366c09701242560f4dc68. Candidate: 6bfc278cdfb8cd5f2adad13f5e920a2baee131be9f7dedc4b472329f2f0da4f7. Full source, reader and plan bindings are in the adjacent qualification packet and raw result.

## Recorded observations

All15 requested deterministic cases recorded passing assertions; process exit0. The two normal/repeat controls yielded storage_complete. Six injected write/flush/fsync cases retained storage_failed; five interrupted/checkpoint cases retained storage_incomplete. Two malformed receipts were rejected. These failure and incomplete labels remain in the raw records and CSV. A passing assertion denotes expected storage behavior, not a completed materializer.

The producer recorded elapsed0.11730553704546764seconds inside the harness. The enclosing receipt charges60active worker-seconds conservatively. No per-case durations, CPU time, RSS or sampling uncertainty were recorded; none is inferred. One invocation, no retries. Internal work bound50seconds; outer timeout55seconds plus5seconds kill grace, address-space512MiB, per-file limit1MiB. Source qualification ran locally, without invoking materializer top-level or compiler.

## Method and raw readback

The harness executes only AST-extracted record_cleanup and checkpoint functions after verifying candidate SHA and AST equality to the earlier independently reviewed functions. The independently reviewed receipt reader consumes each retained receipt. Cases cover append failures, file/directory fsync failures, incomplete tails, checkpoint replacement/directory-fsync faults, interruption boundaries, normal repeat, and hash/count corruption. The frozen PLAN.md lists the exact command and cases.

This archive readback inspected all15 raw receipts, recomputed every available prefix SHA and newline count, checked both intended malformed inputs, and found stdout JSON equal to result.json. No fixture was rerun. The archivist authored the harness; this is a raw consistency readback, not independent scientific verification. The separate source reviewer and reader review remain identifiable in their original packets.

## Scope and remaining dependency

The fixtures inject software exceptions and short writes. They do not establish storage persistence after power loss or an actual process crash. The checkpoint directory-fsync failure exposes a readable replacement receipt while leaving crash persistence unestablished. Missing journal pointers acknowledge no prefix and do not prove no child existed. Cleanup semantics, native process ownership and terminal custody remain false throughout the reader output.

The next admitted materializer attempt must separately establish complete input closure and native terminal custody. No scientific cohort, cost ratio or materialization performance was measured here. Exact source/plan review and current campaign publication remain separate from this observed storage behavior.

## Archive and reproduction

raw-fixtures.tar.gz retains35 raw files from the fixtures and control directory. raw-manifest.json contains each uncompressed file hash and byte count. result.json, publication.json and exit.json are exact copies; raw-readback.json records the consistency checks. metrics.csv preserves null or unavailable fields as empty cells. records.svg plots only recorded acknowledged record counts; malformed receipt counts are explicitly excluded. render_report.py renders these archived values without invoking fixtures. Run using the existing /home/ec2-user/.venvs/report/bin/python interpreter.

Sources: materializer-qualification-20261109/PLAN.md and qualify.py; materializer-source-successor-review-20261107/review.json; materializer-journal-reader-review-20261107/review.json. KB unavailable in this runtime; exact primary repository sources were used. No novelty, speed or population claim is made.
