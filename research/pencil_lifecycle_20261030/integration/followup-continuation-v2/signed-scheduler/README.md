# Signed scheduler checker

`src/lib.rs` verifies saved streams after strict graph replay and group-label validation. REPORT.md and report.pdf state the exact scope. flow.svg is editable; streams.csv and summary.json provide structured results.

From the repository root, run `bash research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-scheduler/check.sh`. Restore archived logs and prior fixtures with `python3 tools/artifacts/artifacts.py pull` on this packet and the adjacent signed-replay packet first. The check does not invoke the collector. The seed must come from separately trusted custody for an external application; these tests pin saved local headers.

Prior reports and scientific records are unchanged. Planning limits remain 25,200 active-worker seconds with deadline 2026-11-03 06:00 UTC, conditional on the recorded execution gates.
