# Round nine, attempt 3 — output complete, wrapper timed out

Card `CARD-HEIC-ROUND9-3` (commit `0c5c5ba2`) stopped at command 2 with a
timeout at 540 s (`run3/receipt.json`). The binary itself completed: its
output, copied here, has all 18 rows and the verdict lines, and was last
written just before the timeout. The isolation wrapper's record was not
written, so the run's isolation conditions are unrecorded.

The counts are deterministic in the seed: rows 1-7 are identical to
attempt 1's partial output. This attempt is not the registered result;
attempt 4 reruns it with a 1800 s limit for a clean receipt.
