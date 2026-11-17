# Selector-positive capped-outcome derivation control

Before the degree-83 run, `derive-capped-outcome.jq` was checked against an eight-phase synthetic receipt and 100 progress samples. The fixture has one `orbit_choices_pinned` record with four phase-zero, seven-wire choices; root `Fixpoint`; SAT entry; exit 124 without OOM; exact 4 GiB and zero swap; and monotone decisions, conflicts, restarts, completed XOR passes, nanoseconds, row scans, repivots and row operations. The accepted [outcome](deriver-fixtures/outcome.json) is `UNKNOWN_wall_cap` with 99 completed XOR passes and 990 row scans at the last sample. This is a deriver fixture, not an N83 solver run.

The negative fixture changes only sequence 99's completed XOR row scans from 990 to 0. The deriver rejects it with exit 5; [raw stderr](deriver-fixtures/nonmonotone-stderr.log) and [exit receipt](deriver-fixtures/nonmonotone-exit.txt) are retained. Thus the new work counters cannot decrease silently in an accepted capped receipt.

From this study directory, reproduce the positive case with:

```sh
jq -n --slurpfile state deriver-fixtures/state-object.json \
  --slurpfile host deriver-fixtures/host-object.json \
  --slurpfile phases deriver-fixtures/phases.jsonl \
  --slurpfile progress deriver-fixtures/progress.jsonl \
  --arg start '2026-10-18T00:00:00Z' \
  --arg end '2026-10-18T00:30:00Z' --arg peak '2661863424' \
  -f derive-capped-outcome.jq
```

Substituting `progress-nonmonotone.jsonl` for `progress.jsonl` must exit 5. The actual run still needs its own source, input, container, phase, progress, hash and checked-model receipts.
