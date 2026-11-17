# Supplemental U cadence-1 run after an input-path launch failure

This amendment was written after scheduled rows 1–7 completed and before
any supplemental run. Scheduled row 8 exited 1 immediately with
`No such file or directory (os error 2)` because the launch command used
`research/koblitz_n83_factor_base_sweep_20261027/pilot-01/probe-corpus.json`.
The frozen corpus is in the `20261008` directory. Row 8 produced no worker
receipt, model, or solver search and remains an input-path failure in the
ordered record. Its exact attempted path and status are retained in
`evidence/launch-error-08.json`.

A ninth, supplemental row uses U, cadence 1, the unchanged locked source and
binary, the exact U archive and frozen public corpus with the corrected path.
It has the original 5,000-conflict and 240-second caps. It is not a replacement
for row 8 in the frozen schedule. Report it separately alongside the seven
scheduled completed cells; distinguish the preregistered panel from the
post-failure repeat when summarizing paired work or wall times.
