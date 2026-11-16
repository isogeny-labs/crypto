# Native restart software controls

Read REPORT.md or report.pdf. flow.svg is editable; controls.csv gives all 64 outcomes, summary.json aggregates them, and stream-manifest.json binds all 192 raw files. The test uses the fixed controls in PLAN.md and leaves every result, including cap exhaustion, in the receipt directory.

Hydrate this packet with the repository artifact tool (`python3 tools/artifacts/artifacts.py pull` with this directory as its argument). native-streams.tar contains only the flat names listed in stream-manifest.json. Restore it to a fresh directory and verify each member hash before replay. The archive preserves exact native bytes; original local streams are ignored by Git, not discarded.

Run check.sh from the repository root only when intentionally repeating these software controls under an applicable allowance; it creates a new receipt directory and never overwrites the retained check. No automatic retry is configured. Planning limits for the scientific continuation remain 25,200 active-worker seconds and deadline 2026-11-03 06:00 UTC, subject to the recorded execution gates.
