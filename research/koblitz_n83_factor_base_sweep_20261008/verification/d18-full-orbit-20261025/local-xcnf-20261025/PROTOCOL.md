# Full-D18 external XOR solver comparison

Freeze source, S3 input and CryptoMiniSat before timing. Use the primary
`a=0` Koblitz GF(2^83) curve, the complete direct-subgroup D18 base
(32,643 signed-Frobenius columns, 5,418,738 point rows), the public
`(a=0,0)` fixture, `m=4`, identity mask zero, shared-root-bit orbit domain,
all selectors and phases admissible, and the phase-zero-first policy. The
public target is `(1834913121178197242856238,8797964190732846213981377)`;
the natural worker receives only this point. The bounded native run of this
formula reached its 1,200-second whole-process cap before the solver returned.

From one clean source commit, build one release exporter and freshly GET the
exact S3 manifest and compressed object. Check provider SHA-256 checksums,
lengths, producer-manifest binding and full signed-orbit importer replay.
Use `primary-full-d18-m4-l0-xcnf positive|natural BASE FIXTURE NEW_JSON` to
export two exact CryptoMiniSat-style extended DIMACS files with a 300-second
internal and 310-second supervisor cap per export. The positive arm pins the
known four-point witness's x blocks; the natural arm has no witness pins.
Record each original CNF clause count, XOR-row count, XCNF byte count and
SHA-256. The exporter runs no SAT search.

Use one pinned CryptoMiniSat 5.14.7 binary, SHA-256
`a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`,
one thread, `--maxtime 1200 --maxconfl 100000000 --verb 1 --verbstat 1
--printsol 1`, and a 1,210-second independent process watchdog for each
solver arm. Run the positive arm first. Its SAT assignment must satisfy every
original CNF clause and XOR row, and independently replay the original-field
four-point sum, signed-orbit coefficient row, rank one and altered-target
rejection before the natural arm. Save exact argv, UTC times, source,
exporter/solver hashes, XCNF hashes, stdout/stderr, exit codes, final native
status and maximum RSS. The host is a shared Mac without hard CPU or memory
isolation; charged walls are not a matched isolated throughput comparison.

For a natural SAT assignment, independently check every original CNF clause
and XOR row, field Semaev equations, factor-base orbit membership, four-point
group sum, signed-orbit row, rank, and tamper rejection. Keep a SAT status
pending until this replay passes. A solver time/conflict cap, process
watchdog, incomplete assignment, missing terminal status or group rejection
is censored or invalid, never a measured zero-yield rate. A single public
query cannot select a factor base: natural relation yield over disjoint
targets, useful rank, factor-base logs, unseen-target descent, complete cold
one-target accounting and matched same-target rho remain required.
`selected_best_total_runtime` stays null.
