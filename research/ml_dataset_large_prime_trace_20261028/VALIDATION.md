# Completed native filter software gates

Runtime `52ded7eb1e42ab97c2d23cb6a4354e985ddd8540` passed the full release
library suite: **5,191 passed, 0 failed, 123 ignored**, exit 0. The existing
ignored tests remain ignored; no test was disabled by this increment.
The fixed executable SHA256 is
`3cd0b3daf64282c1e0ae8a6179b723df7fb298082a4bdfd78fcd7830c0b81f7e`.
Its build preceded the runtime commit. Unchanged hashes of all four compiled
native source files bind it to that commit; this is source equivalence, not a
claim that compilation started from a clean committed tree.

The focused gate passed 13 tests (eight new, five legacy), with zero failures
or ignored tests. Strict Rust1.98 release Clippy with `--lib --tests -- -D
warnings` and scoped formatting passed. The tests include all729 selected
two-row F3 systems,200 bounded F5/F7 systems with a separate dense reference,
every event/sink boundary, byte/work/input/state limits, refused fills,
tampering, and near-word-limit cancellation. These are bounded correctness
fixtures. Diagnostic test duration is not a performance comparison.

The native source/binary archive and completed-gate archive are immutable:

- `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/large-prime-trace/63ee9e07ccdb181a7cd5caa30ac24c2178cb59fb68762ec0ee9adf41e7f0a994/source.tar.gz` —36,707,474bytes, source, executable, focused/lint logs and historical full-pending receipt.
- `s3://setec-astronomy-artifacts/ml-dataset/20261028/software/large-prime-trace-final/def2fa6b73384b0ff2bc979a987186422296398bd072648c5dc335e1bdbabd3b/validation.tar.gz` —90,337bytes, final full log, source/executable hashes and final validation JSON.

Both conditional uploads and fresh ETag-bound downloads passed exact-byte,
archive-hash and contained-hash checks. Final verification completed at
2026-10-28T16:28:44Z. Git receipts identify these archives; the earlier pending
record was preserved instead of rewritten.

Merge `b49839f8a3591dd460fa3e0a05677fc286ca0ce4` brings the existing binary
producer delivery documentation into this branch. Its runtime/build/test diff
against52ded7 is empty. The subsequent publication changes only evidence/docs.
The PR is dependent on binary producer2782; hosted checks and main landing are
separate from these local results.

The separate checker validates supplied sparse row identities, legal recorded
transitions, source combinations and cap/acknowledgment semantics. A completed
batch requires an acknowledged Finish. Censored prefixes cannot establish
UNSAT, scalar recovery or complete dependency discovery. Legacy filter code
remains unchanged. Traced singleton ordering is deterministic and can differ
from the legacy queue; compatibility tests check core, result and report.

Dataset v2 integration, signed-Frobenius capture and rank-reducer capture are
separate unfinished increments. Complete execution and total-operation-cost
flags remain false. No scientific dataset, new cloud job, IAM change, budget
increase, deadline extension or first/last-fall-degree result follows here.
The original USD200 total ceiling and existing owners remain unchanged.
