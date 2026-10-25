# Round nine, attempt 2 — killed before any row, kept as evidence

Card `CARD-HEIC-ROUND9-2` (commit `13f21480`) was started by the
mechanical executor in the background; the executor returned before the
run completed and its background process was killed with it. The binary
had written only the table header (`R9.header-only.txt`); no receipt was
written and `run2/` was never created. Nothing here is a measurement.
The rerun (card 3) runs in the foreground.
