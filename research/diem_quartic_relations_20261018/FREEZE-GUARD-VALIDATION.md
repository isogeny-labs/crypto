# W2a frozen-input guard validation (2026-10-25)

This is a code-integrity review of the W2a study driver. It contains no new
instance generation, cost cell, fit, performance comparison, or proof of a
general elliptic-curve transfer. The original managed generation deadline
expired with 50 of 52 durable instance records. The missing records are
`(20,0)` and `(20,2)`; no complete `instances.jsonl` or `FROZEN.sha256` was
published. The existing receipts retain their original source identity.

| Required gate | Original driver | Successor driver | Evidence |
| --- | --- | --- | --- |
| Complete, SHA-pinned 52-row list before a cell or fit | `cells` and `fit` could start without one | Both load the required files, compare the exact SHA-256 manifest, and check every ordered `(k, idx)` | Negative controls for absent files, wrong digest, short, duplicate, reversed and malformed lists |
| Selected row matches regeneration | Compared `ell` only when a row happened to exist | Requires the row and compares deterministic fields, including the trace | Source review and example test compilation; no cost cell executed |
| Existing JSONL is not silently truncated | Ignored every read error and dropped malformed lines | Only a missing optional cell/calibration file means no prior rows; empty, incomplete, invalid UTF-8 and malformed existing files stop the command | Parser negative controls |
| Concurrent local append and read | Readers did not take the writer lock | Cell readers take the same lock as appenders | Source review; no concurrent cost run executed |

The new driver file SHA-256 is
`2dd0fb7a0042dd685f477e6a297dfcba2950ae29461fb0fa67bce11d52e3fb58`.
The original driver SHA-256 was
`a9a34afae332b399525bd0cffe16980163e9b108236d79bc2f09614bd4976653`.
The driver is part of the generator's source identity. These identities differ:
the original 50 generation receipts cannot be combined with a successor run or
used to satisfy its freeze gate. A new run needs its own reviewed protocol,
source-bound receipts, complete frozen list and independent result validation.

The historical amendment's opening parenthetical says that
`instances.jsonl` was generated and checked first. The managed native state
contradicts that completion statement: only 50/52 records were durable when
the deadline expired. The sentence is retained as historical text, not treated
as evidence of a completed freeze. Likewise, its all-instance `N1` wording
does not establish checks for the two missing instances.

Verification on the isolated successor checkout: `rustfmt --check` passed;
`cargo test --offline --example diem_quartic_study` passed 12/12, including
three frozen-input guard tests. `cargo test --release --lib --offline --locked`
passed in the complete original WIP checkout: 4,402 passed, 0 failed, 118
ignored, 0 measured. Its `src/cryptanalysis/diem_quartic.rs` SHA-256 matches
the isolated successor (`2b0cb0cc2242220873f83a329a5749bbcb18a3acb003f0c39b4b11820e2e292d`).
The isolated sparse checkout could not compile that suite because it omits
tracked research fixtures. No Python suite is directly touched by these Rust
source changes.

This guard does not validate every semantic field in an existing cost record,
prove the W2a algorithm, or make the frozen run complete. Those obligations
remain open. No result graph or performance dashboard row should be promoted
from this incomplete run.
