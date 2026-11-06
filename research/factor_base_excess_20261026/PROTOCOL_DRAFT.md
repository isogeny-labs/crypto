# Proposed native transfer test — UNRUN, not a frozen runnable protocol

Goal: determine whether product-profile screening selects a factor base
with lower complete successful decomposition cost and better independent
relation production than the matched incumbent. Finding a low structural
score alone does not meet that goal.

## Source and scope

Start from main `4355e5c648b67a679729ac7963f3c054d8362d30` plus the reviewed
native profiler in this change. Freeze the final source, executable,
field/subgroup/generator, candidate manifests and workload hashes before
launch. Do not reuse a later binary for an old byte-bound run. This file
does not authorize an S3 operation, cloud host, funding, lease or card run.

Primary exploratory curve: `icv1-f2m31-tm90707-c95f16f5`, E_0 over degree31,
modulus exponents [31,3,0], group order 2147574356, r=1439393, h=1492.
Registry representation EC1N31Ce0h3e9d0f84f702 records generator
(0xd058876,0x32d9bf79). Secondary transfer control:
`icv1-f2m19-t797-b6cf2467`, modulus [19,5,2,1,0], order523492,
r=130873,h=4, registry generator (0x5e3be,0x49b2b).
These are exact registered representations, not curves inferred from bit counts.
Reverify the identities and subgroup invariants natively before launch.

Degree31 has ord_31(2)=5, unlike degrees53/83/131; no invariant-subspace
finding here establishes high-fidelity transfer. A successful degree83
E_0 comparison remains required before any ECC2K-130 scaling claim.

## Selection and controls

Proposed cells: m=2, dimensions5/6/7; construction seeds1..16; coordinate,
geometric progression, uniformly sampled full-rank random, and correctly
XOR-sampled trace-constrained progression references. No whole-field scan
or rejection loop without a frozen finite bound. Candidate construction,
failed candidates, exact inventories and all subgroup-control searches are
charged. Trace constraints are not assumed to improve subgroup support.
The existing bounded first-hit `tracezero-gp` constructor is not the proposed
kernel sampler. The new native `tracezero-gp-kernel` implementation records
the nullspace, selected mask and nonzero scale, with at most256 coefficient
draws. Validate that implementation and freeze its exact source and
derivation before an experiment; the finite seeded panel is not claimed to
sample the entire scale kernel uniformly.

Compute exact product profiles without point-log information. Within each
cell, compare top-excess and median-excess candidates to random controls.
Tie-break by the committed candidate ID before reading probe outcomes.
Record geometric x-count, actual points, usable points and distinct signed
cofactor-projected columns separately. Fixed dimension does not imply fixed
column count. Pair only exactly matched column counts for a size-controlled
conclusion; if no matched block exists, record that gap rather than silently
resizing the base or reporting a size-confounded winner.

Selection uses a dedicated training workload. Freeze 64 additional training
and 128 fresh disjoint holdout subgroup targets per admitted candidate pair,
shared across arms, plus eight conditioned positives reported separately.
Controls and holdout never increase training rank or select a base. Evaluate
holdout rank in a separate initially empty matrix with the same column map;
it does not modify the training collector or drive reselection.
Subgroup-empty bases fail admission and remain in the construction flow.
No reseeding or replacing an unsuccessful cell after seeing outcomes.

## Solver and useful relations

First compare the native m=2 S3/F4 path. Use matched exact membership,
monomial order, variable ordering and all solver budgets. Separately record
successful observed procedural degree, bounded-refutation status, algebraic
closure and unresolved/resource-stopped calls. No value is called certified
regularity. Independently check every extracted point witness in the exact
original inventory and subgroup, not just its polynomial abscissa.

The proposed m=3/4 transfer is a distinct phase. It needs a native adapter,
exact descended systems, constraint/splitting checks and equivalent matched
controls. The m=2 fit and the sum of product dimensions must not be used as
an m=3/4 degree predictor. It is not runnable through today's m=2 CLI.

Measure incremental training rank in the same signed projected-column
space, retaining dependent rows, verification work and failed attempts.
Compare cost per useful independent relation, not simply returned roots or
coverage. End-to-end ECDLP recovery and every phase would be needed for a
whole-pipeline claim; none is included or inferred in this proposed stage.

## Accounting, thresholds and finite stops

Before launch, emit counters for construction/profile selection, inventory,
encoding, all solver work (including unsuccessful attempts), root search,
lifting, witness checks and rank; identify every still-unpriced counter on
every arm. Use the same 0/0.1/1 GAE pricing band everywhere. Missing counter
counts prevent a total-cost ranking. External same-system Magma/msolve/FGb
and relevant Singular comparisons are owed before a fastest claim.

Candidate threshold: at least20% lower complete stage cost per verified
independent relation on matched fresh holdout workloads, robust across the
pricing band and with no correctness failure. Report every cell, including
nulls, low-rank cases and worse costs. A lower degree with higher total cost
is not a win. A null result rejects this finite transfer test, not all
structured bases. Neither a sub-rho nor a degree-of-regularity conclusion is
an acceptance target.

Smaller verified improvements still count as engineering successes under
the standing success criterion; 20% prioritizes a follow-up, not whether a
real improvement counts. No rho win is required for such a stage result.
A zero-rank arm has no finite cost-per-independent-relation estimate;
retain it rather than dividing by zero or discarding the cell. Unequal or
missing completions preclude an unqualified speed comparison.

Proposed ceilings: one worker/thread, 2GiB address-space limit, 60seconds per
candidate, degree cap4, one million matrix cells, two-hour total panel cap.
No cap increase or same-case retry after a stop. Freeze actual enforceable
settings and exact argv in the executable protocol before dispatch; these
ceilings are currently a proposal, not measurements or booked resources.

Flow at this handoff: experiments attempted0, completed0; no source or
result experiment is excluded post hoc. Correctness unit tests are software
validation, not observations from this panel. Run launch remains blocked by
the S3-checkpoint/no-S3 instruction conflict. All older XP cards stay unrun.
