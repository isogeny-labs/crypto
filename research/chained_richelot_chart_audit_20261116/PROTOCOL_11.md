# Independent implementation audit of the frozen chart10 transcript

Prospective correctness unit11. This protocol is written before auditor code,
compilation or native calls. It checks the entire existing chart10 transcript;
the chart10 producer is not rerun. No result is recorded yet. [pcc:unverified]

## Sources and fixed inputs

- Exact producer library `06fe86f1edd56d21a732dbdb8a0def01311cf3f50936cd3039929d4aa3039489`, public source commit `3eaf2d5733a98c2cb51e8103fdb6be5e0c8084a7` and outcome commit `d870495bd76c9f6c05554e0fa40b4437acb5f322`.
- Full original stdout, 287462 bytes, retained in chart10's whole first-unit archive under `all-four-native-units.entire-stdout.txt`. The separate local original is `/workspace/scratch/ct621-first-frozen-chart10-unit-01/all-four-native-units.entire-stdout.txt`. The exact SHA is frozen in `INPUT_CUSTODY_11.json` before code.
- The twelve literal sextics are copied byte for byte into `fixtures-11.csv` from chart10's frozen CSV, SHA `8809526e6a4f7dee113835e3cd34844355743474569436cdd808254938269096`.
- Primary sources already read: chart10 library, CT49/CT50 and the full native `nonsplit_map.rs` retained in its `primary-inputs/`. CT49 supplies exact models and CT50 the dual-orientation/forced-return boundary; curve equation checks alone cannot select the dual sign. Reuse the established reciprocal model change, with no novelty claim. [pcc:unverified]
- Sources checked (KB): no KB tools are loaded in this runtime; repository fallback previously exits127 for unavailable backend. The locked autoresearcher vault is not opened. The primary files above and repository `ecdlp-surfaces`/higher-genus and `report-evidence` guidance are the fallback. No claim of literature absence follows. [pcc:unverified]

## Question and independent implementation

Does a separate standard-library Rust implementation reconstruct every frozen
trace value and its exact coverage? Use a separate standalone crate and no
imports from the producer. Planned arithmetic is canonical `u128` modulo101,
signed extended Euclid for inverse, explicit repeated multiplication for powers,
iterated polynomial multiplication by `(1+aU)` for the reciprocal transform,
and square tables for rational-point sets. The producer instead uses its own
`u64` field type, Fermat inverse, binomial expansion, Horner evaluation and
polynomial Euclid. Check each source and target's squarefreeness through the
nonzero determinant of the 11-by-11 Sylvester matrix, rather than the producer's
polynomial GCD. These are finite computations; no generic resultant, field,
compiler or curve theorem is asserted. [pcc:unverified]

Recompute the least admissible a and least positive b, b²=H(a)≠0, all seven
monic coefficients, all102 normalized projective arguments `(u,1)` and `(1,0)`,
complete finite and rational infinity point lists, forward and inverse charts,
and both b-sign images. Include `(a,±b)`, output infinities ±1, source infinities
and output u=0. Validate all100 base inverses and all fixture/sign summaries.
Enforce exact fixture order and exact ordered coverage of every trace type;
duplicates or omitted records must fail even when total counts are unchanged.
Reject unknown trace tags and noncanonical values. Require exactly one four-test
passing harness summary and every declared test start; elapsed harness time is
metadata, excluded from mathematical checks and from every speed claim.

The audit executable has `audit INPUT` (0 accept, 1 semantic rejection, 2 I/O or
usage failure) and `make-control KIND INPUT OUTPUT` (0 constructs an entire
modified transcript, 2 generation failure). Generator failures, build failures
and timeouts do not count as semantic rejection.

## Frozen acceptance and controls

The entire original must exit0. Each following full transcript is generated
independently from the original by Rust, saved in full, and must audit with
exit1 for its actual semantic defect:

1. `forward-coordinate`: change one canonical finite forward-image coordinate.
2. `drop-projective`: remove one projective trace row without removing a test-start prefix.
3. `duplicate-forward`: replace a forward row with another, retaining the count.
4. `sign-relation`: replace one opposite-sign image with the ordinary image where they differ; both points are valid.
5. `summary-coefficient`: alter one canonical target-coefficient summary entry.
6. `lost-infinity`: replace a forward row mapping to infinity by a finite-image row, retaining the count.

A seventh `timing-only` full transcript changes only the elapsed harness time
and must still exit0. This demonstrates the stated metadata exclusion, not a
performance/admission waiver. Acceptance requires all six semantic exit1s and
both positive exit0s, unchanged fixed inputs/source/qualifier guards, strict
no-deps all-target Clippy and format checks. Preserve every failed attempt or
unexpected control outcome without revising the acceptance criteria.

## Execution and custody

Ordinary bounded local correctness development only: Rust1.99.1, release,
locked/offline, no dependencies, one compiler worker/codegen unit and120s cap
per build/native call. Publish exact source in a normal immutable Git tag before
compilation or native calls. Build first without execution, then copy the native
auditor to a separate chmod555 qualifier and hash it before calls. Retain full
commands/exits/stdout/stderr, source/fixture/transcript/compiler/qualifier hashes,
all eight complete input transcripts and before/after guards as one whole
archive, with every member's full bytes/SHA/mode checked.

No benchmark or producer rerun, new Lean declaration, original EKS LP workload,
paid compute/image/build, runtime admission or signed PCC result is authorized
by this unit. The paid STOP and locked vault persist. Publish results in the
main-based successor PR after retaining the PR4455 ledger fork; do not push its
closed lane. Exact canonical ledger hydration/prefix, original store/CI and
merge requirements remain in force. [pcc:unverified]

## Subsequent technical step

After an accepted trace audit, carry the checked reciprocal charts through the
existing balanced divisor representation and Richelot factors, including the
exceptional/infinity fibres and dual orientation. This finite point audit does
not itself establish a Jacobian homomorphism or DLP-preserving chain. Preserve
CT50's forced rational-return result and the original full-population tasks.
