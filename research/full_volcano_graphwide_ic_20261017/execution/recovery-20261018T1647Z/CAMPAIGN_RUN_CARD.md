# Bounded continuation of the existing frozen survey

Continue the authorized existing survey, not a new mathematical experiment.
No paid cloud resources, new child budgets or original-full-matrix launch.

## Fixed inputs and guards

- Root: `/workspace/volcano_ic_repaired_survey_20261017`.
- Solver SHA-256: `35d89073f4d666b094c213a1d31b0c819b63600123e067c111af3c947d7609a9`.
- Driver SHA-256: `b8b489c7225c64ed492ffce39286c995f5ca820960956cf706c0d3110f5bdd2d`.
- Queue SHA-256: `45e277bede6017c30a49405096afefab3d34a18553ecd6b1a79238f2b9559728`.
- Queue: 24,823 vertices; two public targets, one round, bases 16/64,
  4,096 source trials, 3-second child watchdog. No changes.
- Campaign-spool cap: 6,442,450,944 bytes, including existing root contents.
- Original outer deadline: the native worker's `started_ms` in
  `before-worker.json` plus 259,200 seconds, approximately
  2026-10-20 20:13:42 UTC. Resumption uses only remaining seconds and never
  resets that deadline. Stop when expired or a guard/audit fails.

## Inspect and recover

Use process readback and native `driver status`, not a historical PID or a stale
SQLite `running` row, to identify live work. The native driver holds
`writer.lock`. Do not start a second writer. If live, no builds, tests, audits
or compression may overlap its timed work. Ordinary process/ledger metadata
reads are allowed.

For a checkpoint, create an owned empty STOP marker and let the live driver
finish its vertex before proceeding. Preserve its actual exit reason and all
previous launch/process/log receipts. If it is absent with stale running work,
preserve the original ledger and receipts, then use the unchanged driver's
zero-job stop with that owned STOP marker:

```
ROOT/volcano-ic-driver run ROOT 0 0 6442450944
```

This retains and marks the old attempt interrupted. Do not edit sealed sessions
or manufacture a timeout. A resumed attempt gets a new numbered directory.

## Verify and deliver meaningful progress

When at least 1,024 additional vertices have audited since the last delivered
checkpoint, or the queue stops/completes, produce a stopped snapshot with the
existing native helpers:

```
/workspace/scratch/volcano-ic-checkpoint ROOT ROOT/volcano-ic-driver FRESH_CHECKPOINT
/workspace/scratch/volcano-ic-summarize-v2 FRESH_SUMMARY FRESH_CHECKPOINT/campaign
```

The inventory must pass before using the data. Preserve every failure and
interrupted attempt. Native summaries alone aggregate measurements. Python
scripts only copy/format/present them. Reports use
`render_survey_checkpoint.py`; inspect every PDF page. Export the viewer with
the existing native graph exporter, Node/Chromium checks and visual inspection.
Use fresh paths and retain exact binaries/queue/source/receipts. Package with
`package_survey_checkpoint.py` and a current delivery note; extract and check
every manifest entry before uploading the new immutable archive/report/CSV/
viewer to Drive folder `109DpcQpX7ATz6GeI8iSWAExGMWBoR-H_`.
Read back remote names/byte counts/parent and retain local SHA-256 values.
Do not claim remote content hashing, outer repacking or independent solver
replay unless actually performed. No S3 upload without a ready runtime identity.

Finished evidence lands in the existing research/benchmarks paths and PR #2106
(or a follow-up if merged), following AGENTS.md. Leave the frozen mathematical
protocol/source untouched. Existing unrelated CI defects are not patched around.
After successful verification/delivery, remove only reproducible staging and
extraction copies as needed; keep final archives, authoritative B raw data,
live campaign and all failures/receipts. Preserve enough free space for the
remaining spool allowance plus operational headroom.

## Resume last

After heavy work finishes, verify the three frozen hashes. Require a positive
remaining deadline and no live writer. Remove only the owned empty STOP marker.
Launch the unchanged driver detached, with fresh epoch/log/receipt paths:

```
ROOT/volcano-ic-driver run ROOT 24823 REMAINING_SECONDS 6442450944
```

Use `nohup` and a new session, closed stdin and a retained log. Record the actual
UTC time/PID, original deadline, remaining guard, free space and hashes.
Read back PPID 1, own session, non-zombie state, native progress and the retry
outcome before stating that it is running. Host survival is not guaranteed.

Continue bounded follow-ups while this queue remains unfinished and the guards
permit work. Stop on deadline, storage/audit error, completion or user stop.
Notify meaningful verified deliveries, recoveries or new blockers, not repeated
unchanged PR status. Queue completion still does not complete original A/C/D/E,
the full requested method matrix, independent arithmetic or large-k IC.

## Larger-checkpoint storage constraint

The current helper copies raw trees into a stopped snapshot and a packaging
stage. Do not blindly create three full copies as the queue grows. Check
space first. If copies would exceed the volume, first add and validate a
metadata-only packaging mode that links immutable snapshot files into its
stage, while keeping mutable live ledger/worker files independent. Keep the
authoritative live campaign stopped until archive verification finishes. Closed
reproducible snapshot/stage caches can be removed before fresh extraction only
while that authoritative campaign and final archive remain intact. Never drop
raw accepted/interrupted data or weaken inventory/hash checks to save space.
This file-copy engineering step changes no mathematical protocol or claim.
