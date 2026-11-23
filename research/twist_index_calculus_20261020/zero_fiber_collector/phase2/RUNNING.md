# Managed panel: reconcile before continuation

The full comparison has one owner:
`twist-zero-pipeline-panel-v1.service`. Its launch is accepted with exit 0;
the observed main PID is 3372809. The source/workload freeze is
`af66996a19`, before launch. Binary copies match the registered producer and
metadata executable hashes. The service can wait at a common reservation;
an active controller alone does not establish a completed native case.

- Raw output: `../runs/phase2-panel-v1/`.
- Launch receipts and live owner output: `../results/phase2-panel-v1/`.
- Per-phase commands, queued-start/end/exit receipts: raw `metadata/`.
- Native case journals and first/A/A records: raw `cases/<id>/`.
- Terminal marker: raw `metadata/terminal-exit`.
- Final native reduction: raw `analysis/summary.json` and `cases.csv`.

Before any retry, inspect the unit, exact owner, command/exit receipts,
completed records and pending journals. The script refuses an existing run
and phase receipt. Do not start another controller or replace a timed-out
or interrupted prefix with a fresh deadline. First/A/A are the three
separately registered executions, not retries. The service has no restart
loop. A new host must reconcile this owner and its completed side effects.

Every candidate retains one worker, 8 GiB, a 1,200-second watchdog and the
1,180-second cooperative budget. Reservation wait is outside the native
operational interval; a driver's phase-start receipt is a queue entry, not
proof of the native process start. Preserve the actual native receipts and
original attempt limits on recovery.

Completion requires accounting for all 85 configurations/255 candidate
executions, 85 separate first-run audits and 170 deterministic A/A checks.
Failures and partial cases remain explicit. No complete-price or timing
claim follows merely from process exits, full rank or a lower system count.

The Phase 2 archive is a new, versioned artifact after the panel is
reconciled. The existing Phase 1 `../runs.tar.xz` and canonical store object
are immutable. Diem, the old source waiting/freeze lane and existing EKS
jobs are separate owners and have not been restarted by this panel.
