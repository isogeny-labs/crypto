# Local driver controls

REPORT.md/report.pdf state the result and exact boundary. controls.csv and summary.json retain all eight outcomes. PLAN.md was written before the single execution. check.py substitutes only the harmless local Python command prefix; it never invokes assembler main, workflow, sudo or Docker.

The archived raw-driver-receipts.tar.gz contains 26 files under raw/, listed with hashes in raw-manifest.json. Hydrate with the repository artifact tool, then verify archive member hashes before interpreting receipts. remote-readback.json records a fresh download check. Local original paths are in location.json; no claim of continued process ownership is inferred from a path.

Repeating check.py intentionally runs local software controls and creates a new private directory; do not use it to repeat an uncertain run. Current evidence is the retained single invocation with check.exit=0 and unchanged source hashes. This packet grants no assembly or scientific execution allowance.
