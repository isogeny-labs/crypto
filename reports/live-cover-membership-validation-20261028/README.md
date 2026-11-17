# Complete live catalog membership controls

Main's constructor controls retained dated counters of 15,791 models and
142 binary EC1 bindings. Current inputs contain 20,592 models and 148
bindings. Replace those stale counters with independent exact set joins
between the live registry and cover graph. Require nonempty, unique model
and model/EC1 sets; all existing per-model constructor and typed arithmetic
checks still run. Missing, extra or duplicate graph entries now fail.

All 13 native constructor controls pass with Rust 1.98.0. They exercise every
current model and every binary EC1 binding. Actual standalone catalog export
matches every canonical non-provenance value. Rebuilding and independently
checking the canonical catalog verifies all 20,592 entries, without changing
the catalog or graph bytes. No experiment or timing comparison is run.

The wrapper's full-source digest includes its tests. The mutable infrastructure
documentation now records the actual replayed producer digest
`1db54f0953ef0c75a15948df432e6a238436f0c801900050f512ed8042887693`.
The first independent check caught that stale documentation; after its
source-bound update, generation, byte-currentness and independent verification
all pass. All 17 scientific assessment sections remain identical; only source
artifact provenance changes. The existing 13-page PDF remains byte-exact.

This establishes current software membership and conditional typed arithmetic
coverage. Exact subgroup-order proofs and per-cover end-to-end transfer
obligations remain open. Historical dated reports and inputs are preserved.
