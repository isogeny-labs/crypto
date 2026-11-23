# Cost and audit contract

`ic.aps_v3` keeps v1/v2 identities and old serialized records unchanged.
Its native diagnostic totals remain explicitly incomplete. Numerical replay
certification counts coefficient multiplications and term steps; it does not
turn missing allocation, map, field or trace-tracking work into zero-priced
operations. No complete S/GAE ranking is admitted.

The optional inclusive path reuses the repository's native Callgrind parser:

1. The ordinary ecbench child emits the existing `ecbench_before_solve` and
   `ecbench_solve` markers. Every intervening part contributes Ir, including
   nested dumps. v3 also emits the nested online markers.
2. A separate sealed profiled record must have the same binary, method,
   workload, seed, status, recovered scalar, counters and complete detail as
   the unprofiled record. Clocks and placement are excluded from that match.
3. `aps_scalar_check` seal-checks and loads the record before its frame,
   then uses the export reader's separate ordinary-residue curve/scalar
   arithmetic inside the frame. Its binary hash and result are pinned.
4. `aps.inclusive-ir/v1` binds record identities, protocol/source/binary
   digests, every contiguous numeric profile part, and scalar-check receipt.
   The reader hashes the exact bytes it parses, checks terminal markers and
   rejects mixed prefixes, gaps, contradictory totals and integer overflow.
   Each frame is capped at 4096 parts and 64 MiB cumulatively.
5. Frozen protocol extension `aps_export.require_inclusive_ir` controls
   whether every represented execution requires this evidence. Missing or
   resumed profiles stay missing; rows remain represented and `check` refuses
   to qualify an export with required profiles absent.

Counters above 2^53 retain exact decimal-string representation. Method and
scalar parts cannot reuse a path. `total_ir = method_ir +
independent_scalar_check_ir` uses checked integer addition. `method_online_ir`
is optional and never replaces the complete method count. A failed execution
without a recovered scalar has no scalar multiplication to check; it still
requires a complete method frame for a qualified cost.

All reconstruction receipts retain `execution_attested=false`,
`wall_qualified=false` and `gae_calibrated=false`. A matching transcript and
unkeyed file digests do not prove execution, raw-profile authenticity, thread
coverage or pre-data filing [pcc:unverified]. Actual profiler/image configuration must record
combined threads and be reviewed at run admission. Cooperative deadlines can
make profiled and unprofiled trajectories differ; that is a retained missing
cost, never permission to alter a transcript or rerun for a favorable seed.

See [PROTOCOL.md](PROTOCOL.md) for excluded work, held-out comparisons and
unmet gates. Synthetic profile tests establish parser contracts only; no
Callgrind process, benchmark or EKS session was launched by those tests.
