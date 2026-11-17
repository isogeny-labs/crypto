# Separate immutable R32 artifact

Raw archive SHA25648a75846af1715fe17efdaf3b2d7da01bc9c686b38791966db3f08cccbba5fe4,
114748bytes/49 regular files, is in setec-astronomy-artifacts/store and indexed
by .artifacts.json. It includes all five invocation directories(two pretrial,
one rejected native,two accepted native), the Lean compile/audit, native input
templates and complete root software-check receipts. A partial running log is
explicitly named as a snapshot; the final log and exit124 are retained too.

Fresh GetObject, every expected member hash and deterministic GNU tar repack
pass. See artifact-verification.json. This is byte integrity, not independent
mathematical replay. R31's separate archive is unchanged.

Source-revision reconciliation is explicit: sealing observed HEAD05d183445
while the metadata-only commit was finishing. The original seal metadata and
source-revision.txt are preserved. Every one of the28 source pins has since
been checked against the complete committed tree
d689353f6a6ae20549ec4c1ee8a66b74574eb2f5. The receipt is
source-revision-reconciliation.json. No source bytes, native result, archive
bytes or original seal receipt were rewritten to hide that ordering issue.
The export records both the observed seal HEAD and full containing revision.

Restore via the repository artifact CLI and extract inside this directory.
Use fresh attempt names and preserve the durable gate/caps for any authorized
replay. Completed native/Lean runs should not be repeated after migration
merely because an observer was interrupted.
