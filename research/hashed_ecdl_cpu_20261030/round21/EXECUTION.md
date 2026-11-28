# Round21 execution, 2026-11-01

## What was done

T-1252 executed the frozen comparison once after explicit authorization for normal
Spot autoscaling from zero. Job 3375af37-f645-44d8-b25d-0c63e1ae2bc5
completed successfully at 17:45:56 UTC. All 180 executions completed, including
18 warmups and 162 measured executions. The frozen native analyzer verified
all 18 cells and byte-identical full certificates. The native exporter rechecked
the manifests, row coverage and analyzer identity and wrote results/export.json.
The downloaded worker manifest verified before export and again afterward.

## Decisions made

Historical README and qualification receipts describe preparation and remain
unchanged. execution-admission.json and execution-outcome.json supersede their
operational status. No frozen inputs, pool limits or scheduling rules changed.
No retry or deadline extension occurred. Raw outputs are published before the
separate reports/round21 report.

## Open threads

The complete latency rows are shared-host L0 diagnostics. Candidate selection
must retain the original baseline, all fixtures, complete cost and full replay.
The compact pivot directory has mixed complete-time medians across fixtures;
do not label it a uniform measured gain. Further profiling should focus on
relation collection and reduction, the largest remaining measured components.
An isolated acceptance run and independent review remain outstanding.

## Errors

One status observation timed out; polling the same Job UID then established
success. No submission was repeated. The container logged a harmless missing
username for UID 1000 and exited zero.
