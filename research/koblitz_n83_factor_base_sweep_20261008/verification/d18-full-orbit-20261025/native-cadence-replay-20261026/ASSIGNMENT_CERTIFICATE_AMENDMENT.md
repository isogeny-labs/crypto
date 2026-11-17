# Full-D18 retained assignment amendment

**Freeze status:** source and verification amendment before any new
cadence-32 worker launch. The [original bounded protocol](PROTOCOL.md),
curve, S3 generation, constructed/public targets, ordered first-pair policy,
XOR cadence 32, 300/2,400-second internal caps, 310/2,410-second outer caps,
host launch gate, and one-positive-then-at-most-one-natural sequence remain
the experiment definition. The earlier [capacity preflight](PREFLIGHT.md)
ended before worker launch and is preserved unchanged.
The implementation is in the [native worker](../../../primary_adapter.rs)
and [separate verifier](../../../../../examples/n83_d18_native_relation_replay.rs);
the original-assignment check is in the
[chained-S3 circuit library](../../../../../src/cryptanalysis/binary_semaev_chain_sat.rs).

## Hypothesis and verification obligation

A natural SAT result cannot be counted as an independently checked relation
unless its full assignment is retained and replayed apart from the worker.
The existing worker checks its own original CNF/XOR rows before reporting
SAT; this amendment makes that check reproducible from a sidecar and a
separate native process. It is an evidence improvement, not a predicted
runtime improvement. The sidecar write and checksum are charged in worker
process wall and usage if SAT. Record the separate verifier's wall, CPU and
memory as audit cost; keep that cost visible and separate from the cold
one-target IC pipeline ledger. No sidecar is expected for Unknown or Unsat.

For every cadence-32 SAT result, the worker writes the sidecar before the terminal
result: output `/path/result.json` produces `/path/result.model.bin`. Bit `i`
of byte `i/8` encodes variable `i+1`, low bit
first; unused high bits must be zero. The terminal result binds the sidecar
through `model_certificate` schema `n83.full-d18-m4-model-certificate/v1`,
exact bit and byte counts, SHA-256, encoding and path. A cap or missing
terminal result never promotes the sidecar to a relation observation.

The separate Rust verifier requires the sidecar at the path derived from the
terminal result path, rehashes it, checks length and padding, and reconstructs
the complete four-summand chained-S3 circuit and full 32,643-representative
shared-root domain from the checked S3 object. It reinstalls the first-pair
order constraint and, for the positive arm, every planted x-coordinate pin.
It compares variable, CNF-clause and XOR-row counts with the worker's frozen
shape. It then checks the full assignment against the reconstructed original
CNF and XOR rows, evaluates the field equations, and checks that the decoded
four summand x-coordinates match the independently checked signed-Frobenius
group row. The generic multi-limb group replay and changed-target/row-digest
controls in the original verifier remain required. This is a separate
process replay using the same circuit library; it is not a second,
independently implemented circuit construction.

## Controls, outcomes and accounting

- Reject a missing sidecar, wrong bound path, changed SHA-256, wrong bit/byte
  count, or nonzero padding bits before formula replay.
- Reject reconstructed formula shape mismatches, any failed original CNF/XOR
  or field check, and any assignment whose decoded x-coordinates differ from
  the checked group row.
- Retain the worker's internal SAT check and all prior input, target,
  phase, group, rank-one and changed-target controls.
- Report `PASS_NATIVE_GROUP_ROW_AND_ASSIGNMENT_REPLAY` only if both layers
  pass. Retain `PENDING_INDEPENDENT_NATURAL_REPLAY` in a natural worker
  receipt until this separate verifier passes.
- Preserve Unknown, cap, producer failure and rejected certificates as
  distinct outcomes. A relation still does not establish natural yield,
  graph closure, useful rank, full-width logs, unseen-target descent or a
  complete cold one-target cost. `selected_best_total_runtime` remains null.

The focused verifier tests must reject changed certificate path/hash and
padding; the touched worker and verifier example tests, full release library
suite, N83 and boundary compatibility suites, rustfmt, Clippy and diff check
are required before pushing. Actual full-D18 assignment replay remains
unverified until a resource-qualified SAT run yields a retained sidecar.

## Prelaunch implementation validation, 2026-10-26

The touched release examples passed 20 worker and 4 verifier tests. The
full release library suite passed 4,168 tests, with 117 ignored and zero
failed, under four test threads with loopback access. Its [compressed
log](evidence/full-lib-assignment-4threads.log.gz) is 67,131 bytes, SHA-256
`649792df764d8b26dc89907db46a0e7b6002738855d4a8fd92fb5d41d17fa3d6`;
the uncompressed log SHA-256 is
`ab602867d3e56f217ec2ade90815cc04536cbe92cc0e29f7c37799915dc730d0`.
The historical N83 and boundary compatibility suites passed 63 and 52 tests
(70 and 9 subtests), respectively. Targeted rustfmt, diff check and focused
Clippy passed; Clippy emitted the existing repository unknown-lint warning.
The new tests check bit order, zero padding, wrong certificate path and hash,
plus the original group/row negative controls. They do not exercise a
retained full-D18 SAT assignment: that verification status remains
`IMPLEMENTED_UNVERIFIED_ON_FULL_D18` until the frozen worker and separate
replay complete on a qualified host.
