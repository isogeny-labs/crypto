# Combined v5 failure receipts

The materializer failed while opening the first declared overlay. All eight declared overlay paths were absent at readback; their exact v3 copies matched every declared size/hash. Native/outer/caller receipts show one materialize command, exit1, reaped, no compiler/tests. Cleanup completed and output seal passed.

raw-receipts.tar.gz preserves exact native/caller receipts; raw-manifest.json binds original paths and bytes. Terminalmanifest entries and completiondigest were independently rehashed. This reporter did not change stagedsource or cache; root cache reclamation is a separate task.
