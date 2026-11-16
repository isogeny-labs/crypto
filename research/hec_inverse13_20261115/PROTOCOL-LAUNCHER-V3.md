# Canonical manifest-path amendment

V2source033c9aa6c868/checkpoint215408 stopped exit101before any solver spawn:
changing cwd also made the manifest filename unresolved. Retain its stdout,
stderr and exit. No run-positive solver files exist; no scientific cell ran.

collect-v3.rs canonicalizes the manifest filename in studycwd before changing
hashcheck cwd to reporoot. Actual hashes, solverflags/cwd/caps/inputs and exact
verifier unchanged. New preflight label executes ONLY this hashcheck and returns
without spawning solver. Metadata-only preflight passed all3fingerprints before
freeze, receipt retained. It is not an inverse experiment or success claim.

New top receipts POSITIVE-COLLECT-V3.txt/stderr/EXIT-POSITIVE-V3.txt. Freeze/
checkpointgate before one positive launch. Original two producerfailures stay.
No retrywithoutamendment; target still requires verified positivecontrol.
