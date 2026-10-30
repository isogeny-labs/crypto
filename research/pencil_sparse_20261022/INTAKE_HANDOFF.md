# Pencil continuation: non-solving handoff and synthetic intake gate

This handoff fixes the evidence boundary for the 717 open workloads without
launching a collector, exporting a matrix, computing a nullspace, replaying a
target, or recovering a scalar. The executable in this directory validates
only the fabricated `SYNTH-PENCIL-*` fixture. It has no production mode and
rejects every real cohort identifier.

An independently authorized executor may use the frozen run contract below.
Returned real evidence remains quarantined until a separate N4 admission
review implements the full source-scalar and price recomputation. Passing the
synthetic gate is evidence about the intake controls, not evidence that any of
the 717 workloads ran.

## Requirement and evidence map

| Requirement | N2 evidence | Status |
| --- | --- | --- |
| Versioned structural contract | [`intake/contract-v1.json`](intake/contract-v1.json) | implemented for fabricated public records |
| Accept valid primary/replay records | [`intake/fixtures/valid/`](intake/fixtures/valid/) | tested on three synthetic workloads |
| Fail closed on coordinates, workload/plan/source/archive/algorithm/binary identities, seed, exact file sets, paths, lengths and hashes, checksum index, pinned setup, process-receipt bijection, guard outcomes, known answers, replay, rho, and failure-price errors | [`intake/fixtures/negative-cases.json`](intake/fixtures/negative-cases.json), [`tests/test_intake_gate.py`](tests/test_intake_gate.py) | tested across 49 negative cases |
| Always emit a standardized receipt | `pencil-intake-validation/v1` on standard output; optional safe relative JSON path | tested for acceptance, rejection, and bad CLI use |
| No computation-launch surface | standard-library import allowlist and AST denial of process/eval/exec APIs | tested |
| Fix all real workload identities and seeds | N1 artifact identified below by exact digest | implemented and checksum-bound; no execution may start with any other inventory |
| Run and independently replay 717 targets | N3 | not executed by N2 |

## Frozen real-cohort input

The executor must receive these immutable inputs:

| Item | Fixed value |
| --- | --- |
| Source archive | `research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst` |
| Archive SHA-256 | `550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339` |
| Unique parent workloads | 1,260 |
| Open-workload rows | 717: 177 at 28 bits and 180 each at 32, 36, and 40 bits |
| Canonical open inventory | [`../four_followups_review_20261022/open-pencil-inventory/README.md`](../four_followups_review_20261022/open-pencil-inventory/README.md); 717 rows, 434,290 bytes |
| Canonical open inventory SHA-256 | `efd504b64437309408c3f199fcf3ed85e94322ef01d7d1545d87f6bf1409ae3e` |
| Protocol file SHA-256 | `4ff8aa2c3f9cdfb030e97bfdeadb4766df4ae1c4d9680e6c554deaac8c8c951b` |
| Solver source revision | `cf346160f1ef472c2182663c891611a73a92c953` |
| Algorithm-set digest | `c1992c97aedb317f0410d3ab1920e1841aa1532b9158eb6c05e69869fb44d34f` |
| Parameter digest | `f72a31e992843cdb831f181833cd21e3b0e40a5ee8872568134908462ba17eab` |

The N1 inventory is the authority for every `(bits,p,target_index)`, workload
ID, workload hash, source record, planted public scalar, algorithm seed and
target seed.
Its rows are sorted by `(bits,p,target_index,workload_id)` and hashed as exact
JSONL bytes. Seeds must be read from those rows; they may not be regenerated,
renumbered, or inferred. The only previously attempted cap-only identity is
`(32,186379,0)`, workload `Wa0fc1e72d7a0`, and it stays in the 717 rows.

The canonical N1 artifact is stored through the repository artifact mechanism,
outside this N2 directory. Its generator proves the exact 1,260/543/717 set
subtraction, agreement of `algorithm_seed`, `planted`, and `target_seed` across
all three archived arms, and the complete workload-ID/status partition against
the pinned pencil coverage file (SHA-256
`738083bd44d9a1e5133701b37985fd7c9f5e293065ddcbc3876a0082948169af`).
The earlier `16d33685…` inventory digest omitted these required top-level
fields and is explicitly superseded. A prior digest, similarly named file or
reconstructed list is not a substitute.

The algorithm-set digest is SHA-256 over this canonical JSON object:

```json
{"Cargo.lock":"343447433b3e61475691e768786938ce24773ac427905aef575a55de10206221","Cargo.toml":"8c99a66eb0385a8554b23da8a02eb1f6733233643b20df97548f2158ccd586e7","src/bin/pencil_sparse.rs":"1fe3ac972667b6183c784a6bfd21409ac3c9396e9f1a8553312f6c7a46c022a6"}
```

The parameter digest is SHA-256 over this canonical JSON object:

```json
{"matrix_coefficient_cap":25000000,"memory_limit_bytes":8589934592,"node_order":"identity","solver_mode":"peel","wall_limit_seconds_per_target":3600}
```

Any source, binary, protocol, inventory, seed, or parameter drift creates a
different experiment and must be rejected before the first target.

## Executor output contract

Primary and replay are separate passes over the exact same 717 inventory rows.
Each pass returns exactly one retained terminal target record per workload,
including caps and failures. Primary and replay process receipt sets must be
disjoint. A retry is retained as process history, but only the preregistered
terminal selection occupies the workload's single primary or replay slot.

The executor returns one relative-path bundle with this layout:

```text
bundle/
  manifest.json
  setup.json
  primary.jsonl
  replay.jsonl
  process-receipts.jsonl
  files.sha256
```

Absolute paths, `..` components, symbolic links, duplicate JSON keys,
duplicate workload rows, missing files and unlisted files or directories fail
admission. `manifest.json` must name exactly the other five files. It binds the
schema versions, source revision, archive, inventory, protocol, algorithm,
parameter, setup and binary SHA-256 values and the exact digest and byte length
of every payload file. `files.sha256` must byte-for-byte enumerate the setup,
primary, replay and process-receipt payload digests; its own digest and length
are bound by the manifest. The contract pins the exact `setup.json` SHA-256, so
rehashing a substituted setup into the manifest and checksum index still
fails.

The synthetic `pencil-intake-process-receipt/v1` row contains exactly its
process ID, workload ID, pass role, binary digest, source revision, exit status,
wall nanoseconds, peak memory, peak stored coefficients and three guard
outcome booleans. Target records must use pairwise-distinct process IDs, and
there must be exactly one receipt for every target record with no extra
receipt. Its identity and measurements must equal the associated target
record. The gate recomputes wall-limit and memory-limit exceedance and whether
the coefficient cap was reached from the contract guards. A solved row must
have exit zero and no guard event; a capped row must have a guard event and a
nonzero exit; a failed row must have a nonzero exit and no guard event.

Each `pencil-open-target/v1` row retains:

- the inventory identity fields and seed without modification;
- pass role and process-receipt ID;
- all source, archive, protocol, algorithm, parameter, setup and binary
  digests;
- terminal status `solved`, `failed`, or `capped` and a nonempty reason for a
  non-solve;
- verified relation counts, phase counts, field/native operation counts,
  coefficient visits, row updates, peak stored coefficients, peak memory and
  wall diagnostics;
- candidate scalar and the contract-declared price cells only for a verified
  solve;
- rho scalar, verification and the contract-declared price cells for every
  row.

For `failed` and `capped` rows the candidate scalar, every candidate price and
every candidate/rho ratio are JSON `null`; completed work counters remain.
Missing or unverified rho makes the row inadmissible rather than silently
changing the reference. Reaching a wall, memory or coefficient guard is an
observed terminal record, not a measured operation cost and not evidence of a
solve.

Replay comparison removes only pass role, process-receipt identity, host and
wall-clock diagnostics. Every other field must match exactly. A missing replay,
a workload-set difference, or a non-wall mismatch fails the bundle. The two
passes may not share a process receipt.

Attempt-complete means 717 admissible primary rows and 717 admissible replay
rows, including honest caps and failures. Solve-complete additionally requires
717 source-verified candidate scalars and 717 verified matched rho scalars.
Only solve-complete evidence can extend the 543 existing candidate solves into
a 1,260-target aggregate cost ratio.

## Synthetic gate

From the repository root, the only supported invocation is:

```text
python3 research/pencil_sparse_20261022/intake_gate.py \
  --bundle research/pencil_sparse_20261022/intake/fixtures/valid \
  --receipt synthetic-intake-receipt.json
```

Both paths must be relative to the current directory and remain within it.
The receipt is also printed as one JSON line. Accepted input has
`status:"synthetic_fixture_accepted"`; rejected input exits nonzero with
`status:"rejected"`. Every error is an ordered object with stable `code`,
`path`, `workload_id`, `field`, and `detail` members, using JSON `null` when a
member does not apply. `accepted` is retained as a compatibility boolean. The
fixture has one solved, one failed and one capped public row, with a
fresh-process replay of each. Each row binds its exact coordinates, workload
digest, solver mode, source revision, archive, plan, binary and resource
guards. Candidate and rho scalars are checked against the contract's fabricated
known answer. The solved fixture stays below all guards, the failed fixture has
a non-guard failure, and the capped fixture reaches its declared coefficient
cap. It is deliberately unrelated to the real cohort and its numeric values
support no performance or cryptanalytic claim. The synthetic gate does not
perform the real N4 source-scalar recomputation or price recomputation.

Run the gate tests with:

```text
python3 -m unittest discover -s research/pencil_sparse_20261022/tests -v
```

The test suite mutates a temporary copy of the fabricated bundle. It does not
read the frozen source archive or invoke a collector, solver, matrix exporter,
nullspace implementation, replay runner, or recovery executable.
