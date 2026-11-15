# Session log entry (kept here: task restricted writes to this directory)

## 2026-11-14T09:35:00Z — Idea 10 pilot: bit-sliced multi-target propagation

### What was done (commands, files, features)

- Wrote `PROTOCOL.md` before any measured run.
- `bs.rs` (standalone rustc): 5 arms on the m = 2 PDP leaf over F_{2^31},
  l = 5..10, 256 planted + 256 random targets per cell; `raw_part_a.jsonl`.
- `part_b.py validate`: brute force and point-level lifting checks passed
  (`validation.json`). `part_b.py run 5 8 240`: CDCL divergence statistics,
  l = 5..7 (`raw_part_b.json`). `analyze.py` -> `results.json`.
- Sources checked: KB unavailable (research-kb not installed in this
  container); background taken from the task card only.

### Decisions made (and why)

- Tested the inner m = 2 leaf, because the 2+1 scan for one m = 3 target
  already yields 2^l inner targets, the natural multi-target batch.
- No session log under `docs/session-logs/`, no Conductor task, no benchmark
  record, no pcc claim: the task card forbids writes outside this directory.

### Open threads (unfinished, pick up next time)

- Part (b) l = 8 not run (time budget); rare long-prefix E1 pairs unexplained.
- Verdict (unverified pilot): kill criterion met through K2; bit-sliced scan
  is about equal to the scalar scan at l = 10.

### Errors (what went wrong, how it was handled)

- First launch with l = 4 hung: 256 distinct planted targets do not exist at
  l = 4. Killed; l = 4 dropped and recorded as a deviation.
- Part (b) hit its 240 s budget at l = 7, target 15 of 16; reported as is.
