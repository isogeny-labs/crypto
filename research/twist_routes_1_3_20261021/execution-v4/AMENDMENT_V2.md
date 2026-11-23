# Pre-data execution and solver amendment

No scientific data had been collected when the incoming PR branch was merged
as `fe47b91029597a6b5553adb802c7fc0db75c83fa`. Preparation commit
`26efb57a3` and its original protocol/880-cell plan remain preserved.

The incoming AGENTS.md defaults authorized x86 experiments to EKS and requires
a Setec resume checkpoint before launch. EKS is now the default execution
route; a local pilot requires an explicit user backend choice. No absence of
reply counts as that choice. Local builds, unit tests, literature review and
transport preparation continue independently. This changes no mathematical
workload, cell count, acceptance gate or stop condition.

The incoming shared F4 solver adds observation/bit-operation instrumentation
with dependencies outside this standalone experiment crate. This experiment
retains the exact already-tested baseline solver: the byte-identical source
from `26efb57a3:src/cryptanalysis/f4_fp.rs` (also the bytes at `d475ddb25`) is
included as `runner/src/f4_baseline_20261026.rs`. Its associated hash helper is
also copied byte-for-byte as `runner/src/fx_hash_baseline_20261026.rs` because
the incoming helper imports a new packed-mask module. No source edits are made
to either copy. Their hashes are recorded with the source/binary freeze. All arms use
this same baseline. The comparison is about factor-base construction and
redundant equations under this solver, not the latest instrumented engine.

The full-campaign criteria remain open. No research run, cost observation,
relation-yield finding, external-backend substitution or speedup is implied
by successful development tests. A custom command/image route is required
for EKS; the deployed ecbench integration fixture is not a replacement.
