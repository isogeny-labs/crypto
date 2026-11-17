# CRIU/EBS qualification: arity-criu-qual-20261108

Run `arity-criu-qual-20261108`, request `e45cf2fd…` (`request-qual-20261108.json`,
identical to `request-qual.json` except run name and deadline), image
`isolab/tasks@sha256:fd4a039b…`, controller package `07fba4dd…`.
Instrument: prime-tower control `ctl-m3-n15-kummer` (`pkm_tower_pilot`
`5b6d8c9c…`), reference rows `/opt/study/resume/prime/cells/ctl-m3-n15-kummer.jsonl`
(`dc26d5e5…`).

## Pass criteria (from README) and observations

| Criterion | Observation | Source |
|---|---|---|
| At least one worker loss | `i-037cca11b696d6f76` terminated by this session at 05:46Z after generation 1 committed; `restores: 1`, `terminated_instances: [i-037cca11b696d6f76]` | `induced-loss-*.json`, `result/state.json`, `result/command-result.json` |
| Restore from a committed generation | fenced 05:47Z, g1 selected 05:48Z, restored on `i-08f6f1883de6f039a` at 11:00Z (pod waited 5 h 12 min for cpu-spot capacity) | `watch-*.log` |
| Rows equal the recorded rows except timing | job compare: `identical_except_timing True`; independent re-check (recursive removal of `ms`, `wall_s`, `wall_seconds`): 3 of 3 rows equal; differing keys only `ms`, `wall_s`, `summary` (its nested timing) | `result/computation/out/qualification-compare.txt` |
| `command_completed`, exit 0 | phase `command_completed` 11:19Z, `exit_code 0`; one `start` line in `job.log` (05:41:21Z), so the cell continued rather than restarted | `result/command-result.json`, `result/computation/out/job.log` |
| Output integrity | `computation/SHA256SUMS` (12) and `computation/out/SHA256SUMS` (9) all OK on the downloaded copy | — |

Outcome: **qualification passed** under its stated criteria; this is a
statement about the recovery path, not about any study result.

## Caveats

- `timing_valid: false`, isolation `L0`: wall-clock fields of a restored run
  are not measurements. Studies whose claims use time caps must treat
  restored cells' timings as censored or re-run them unrestored.
- 4 generations committed; the latest two (g3, g4) are retained after
  pruning; checkpoint archives (1.1–1.2 GB each) stay in S3 and are not copied here.
- Capacity, not recovery, dominated wall time: after the loss Karpenter gave
  the freed cpu-spot capacity to an r8i.4xlarge for other jobs, so the c8a
  restore waited until 10:59Z. The controller was re-submitted at 07:48Z and
  10:06Z (same request bytes) because of the 45-minute build cap.
