# Frozen measurements and PR integration

All v1/v2/v3 measured commits remain ancestors with exact binary/source/input
pins, and every mechanical relay was checked before moving the measurement
checkout's HEAD. Frozen source/binary records are not relabeled as integration
measurements. The upstream PR advanced to8f2a08c8718025c1df7f1034224afd24dfb6a75c
while the cloud continuation was running.

Merge0dbd1fdf937ed5d06c3687ad4b719979340a196f preserves that history and
its immutable attempted-run records. It retains the native runner's
`n83.chain-phase/v1`/`n83.chain-query/v1` instrumentation, instead of enabling
two incompatible phase schemas simultaneously. It also preserves upstream's
RelationQueryStarted progress event, independent query replay assertions and
adapter trace validation. The older prefixed phase audit requires its own
frozen upstream source; it is historical tooling, not the new execution path.

The measured native core was fully tested with explicit exit0. A separate
fresh release validation of the merged source is recorded in validation/
when complete. No measured worker binary is replaced by that build. No
Python research harness/auditor/test is run in this continuation; upstream
Python provenance and files are preserved without being extended or rerun.
