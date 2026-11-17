# Unknown-log bounded-coefficient recovery pilot, revision 1

Status: implementation and development controls; full-width experiment not launched.
This is a separately scoped extension. The original strict two-sum phase, its
14,720 polynomial systems and zero random recoveries remain unchanged.

## Original requirement and next step

The original deliverable remains ten generators, ten paired scalars each, on all
46 vertices of the selected rational degree-2 component. This pilot uses exactly
the frozen public source instance C0/G0/K0 and its original 12 unknown-log base
points. It establishes a prerequisite if successful; one pilot never discharges
the 4,600-instance requirement. Neither historical rho/BSGS answers, original
sampling coefficients nor scalar witnesses are solver inputs.

Before expanding across the graph, obtain a full-rank relation matrix, derive all
12 factor-base logs, decompose this unchanged target and independently replay
every equation. A failed pilot is retained. Isogenous copies of a transported
base do not create independent relation-yield samples.

## Algorithm and hypothesis

Write a point as `sum(c_i B_i)` with 12 coefficients in `[-12,12]`. Store every
six-point left representation; scan the six-point right representations of
`R - sum(c_i B_i)`. Equal full point encodings give a decomposition. Duplicate
left representations remain linked; a uniform choice among them and a uniform
circular start in the right enumeration avoid discarding available coefficient
rows. Infinity has a separate encoding. Equality includes the sign of y.

For setup, sample a fresh nonzero t using Linux `getrandom` and unbiased rejection
sampling, set R=tP, search for coefficients and verify the full group equation.
Insert only verified equations into a 12-column row space modulo the original
prime subgroup order r. Stop setup when rank is 12 or after 48 attempts. At full
rank independently check every derived base log against P. Search Q with the
same table, derive k by the coefficient/log dot product and check kP=Q. The
independent verifier redoes group arithmetic with affine coordinates and
extended-Euclidean inversion, and does fresh batch elimination of all rows.

The coefficient domain has `25^12` vectors; this is **not** a measured unique
support size or a proved success probability. The testable hypothesis is that
these original base points yield useful independent rows in this larger domain.
No uniform-image, Poisson-success, rank or runtime estimate is assumed as fact.

This is the relation-generation/linear-algebra architecture of index calculus
with a **generic meet-in-the-middle point-decomposition fallback**. It is not a
new algebraic PDP solver, a Gröbner search, a new sub-rho algorithm, or a new
regularity measurement. Its full cost may exceed rho. A generic decomposition
recovery must never be relabeled as an algebraic/PDP speed improvement. The
existing regularity certificates describe the old polynomial systems only.

## Frozen workload and resources

- Parameters and P/Q/base: `public_source_case.json`; source projection hashes
  are in `input_provenance.json`. Input and config objects reject extra fields,
  including a target scalar. The source group order is 18014494963342201.
- Radius 12, six coefficients per side, 244,140,625 entries per side. Hash slots
  and chains require exactly 9,566,497,092 payload bytes by the checked layout;
  this excludes allocator, batch, parser and journal overhead. These are static
  byte calculations, not executed large allocations or peak-RSS measurements.
- Application table cap 12 GiB, intended CPU Spot worker `c8a.2xlarge`, ordinary
  search classification, a 3,600-second original deadline including construction,
  and no automatic retries. The worker request must leave room for all overhead.
- Exact source/config/input hashes are in `SOURCES.sha256`; bind a tested commit,
  compiler options, executable hash and actual OCI digest before any EKS launch.
  A caller SHA is not an image build. Do not submit the default ecbench fixture.
- A verified S3 resume checkpoint is mandatory before launch. The current
  GitHub ecbench-only workflow cannot execute this standalone native binary;
  a configured custom-command Iso Lab route and commit-built image are required.
  No fresh or non-resumable job may be called a resumed job. This implementation
  does not itself serialize the hash table or claim CRIU qualification.

## Accounting, preservation and verification

Preserve every setup probe (including failed and dependent probes), coefficients,
random words and rejected draws, phase records, partial-table progress, failure
messages, native answers, and independent replay. The raw journals contain
scalar witnesses/randomness and remain private encrypted artifacts. The public
summary excludes them. Never publish the process-memory checkpoint to Git.

Counters are actual standard-residue field calls, separate from the old native
Montgomery backend. Multiplication and square counts are disjoint. Inversion
overlaps its executed multiply/square components; adding them double-counts.
Group addition includes attempted identity and exceptional additions; a doubling
executed inside an addition is separately identified. Hash probes are separate.
Scalar-field elimination is separate from Fp arithmetic. Parameter primality
tests are validation overhead, not field-kernel counts. Batch-normalization work
is charged to its construction or query phase. A batch computed after an early
match is still charged in full; right_examined counts equality tests only.

Record setup generation, table construction, relation search, relation checks,
linear algebra, base-log checks, target decomposition, target recovery/check,
wall/CPU intervals and cumulative process peak RSS. Keep total epoch time and
unattributed orchestration overhead; do not silently normalize either away.
These are non-isolated search diagnostics. No online/cold speedup, end-to-end S,
fastest-implementation claim or benchmark admission is established here. Any
later performance comparison needs same-point single-target rho, equal resources,
separate reusable setup, L2/A/A admission and all existing validation gates.

Success for this pilot requires full rank, all 12 log equations, unchanged target
recovery and a passing independent replay bound to exact input/config/source.
Deadline, insufficient rank, absent target decomposition, OOM and verification
failure are distinct failures; retain all records and do not silently retry.
After success, freeze the ten-generator extension before executing the full
matched workload. After inadequate rank, inspect coverage and representation
diversity before choosing a new radius or ordering; retain this protocol intact.

## Prior work checked before implementation

- The local lead atlas was read, then `tools/lead_atlas.py --brief` and
  `--match "bounded coefficient meet in the middle"` were run. Relevant existing
  leads are `LEAD-ICP-generic-table-decomposition`,
  `LEAD-ICP-full-width-arity-grid`, `LEAD-ICP-algebraic-membership-factor-base`,
  `LEAD-ICP-isogeny-transported-relation-yield` and
  `LEAD-ICP-representation-subset-sum`. This advances a correctness/measurement
  prerequisite; it does not reopen their excluded sub-rho claims.
- [Petit, Kosters and Messeng, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  sections 2–4, explicitly separates relation collection and linear algebra,
  rational factor-base membership, and the unestablished cost of polynomial
  solving. Its parameter regimes do not establish an advantage for this curve.
- Semaev's [2004/031](https://eprint.iacr.org/2004/031) was located; direct PDF
  access returned HTTP 403. The 2017/609 PDF was also inaccessible. No unread
  algorithm from either paper is claimed implemented or newly validated.
- The original fixed-base support audit, source IC solver and
  `problems/photo-volcano-two-sum-ic-recovery.md` were read. The new search uses
  no private factor-base sampling logs and imports no generic DLP answers.

## Development checks versus execution

`test.sh NEW_DIRECTORY` builds all three native programs with warnings as errors,
checks Jacobian/batched arithmetic against separate affine arithmetic, checks
wrapped enumeration and retained duplicate representations, exercises full rank
and all nonzero targets of a small known-answer control, validates arithmetic on
the actual public field/points, runs standalone replay and rejects corrupted
target/rank/bounds/identity/incomplete journals, witness-bearing inputs and an
insufficient explicit memory budget. These are local development tests permitted
by the Iso Lab skill. They are not the original full-width experiment.

Conductor was attempted before editing and was unavailable (`command not found`);
the referenced `.conductor/WORKFLOW.md` was not present at the inspected main.
No claim of a Conductor lease, full-library Rust test pass, or new EKS execution
is made. The original IC recovery requirement stays open until real evidence
meets it.
