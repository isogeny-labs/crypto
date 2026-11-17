# Full-D18 free-choice phase-first planted control

Freeze this opt-in search-order experiment before any retained run. Use the
same complete primary `a=0` dimension-18 S3 archive, public fixture, four
planted representative points, shared-root-bit encoding, chained-S3 circuit,
summand/intermediate x pins, 100,000-conflict budget and original-field
group/rank/tamper checks as the previous free-choice shared-root planted
control. All 32,643 representative selectors and all 82 Frobenius phases per
summand remain admissible. The only experimental change is search policy:
prioritize the four 7-bit phase blocks after the already-prioritized summand
x bits, and initialize those 28 saved decision polarities to false. The
expected formula is 329,726 variables, 2,807,752 total CNF clauses and
3,901 XOR rows. Do not treat a SAT answer without original-field and row
verification as a relation.

Build the example from the clean commit containing this protocol and the
solver hook; pin its full commit and binary SHA-256 in every receipt. From a
fresh S3 GET with checked provider checksum, run exactly one
`primary-full-d18-m4-l0-shared-phase-first BASE_DIR FIXTURE_DIR NEW_JSON`
arm on the same L0 Mac host. Its internal whole-process cap is 300 seconds;
the supervisor imposes an independent 310-second cap. Retain UTC interval,
stdout/stderr, exit status, phase/shape/result or cap records, process wall,
RSS, fresh S3 GET metadata and raw-file SHA-256 inventory. Independently
rehash the object and replay the four subgroup points, target sum and row
digest if SAT returns. Reject any source/input/shape/status mismatch.

Compare only the paired constructed control's observed status, conflicts,
search time and full import/process charges. If this arm is censored, retain
the bound. If it passes, it clears a free-choice constructed-control gate for
this encoding/search policy, but supplies no natural relation yield, rank
growth across targets, linear algebra, descent or matched-rho comparison.
`selected_best_total_runtime` stays `null`.
