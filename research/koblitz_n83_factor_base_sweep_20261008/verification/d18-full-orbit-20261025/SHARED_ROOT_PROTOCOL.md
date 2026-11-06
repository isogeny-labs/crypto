# Full-D18 shared-root encoding comparison

The source-pinned default full-D18 worker remains per-representative. This
opt-in study applies the exact shared-root-bit encoding from PR #2490 to the
same 32,643-column, 5,418,738-point S3 archive and primary Koblitz `a=0`
GF(2^83) public fixture. The committed `orbit-classes.csv` has 66 common
zero x-coordinate bits across its 32,643 representatives; the independent
CSV count predicts that one unit per common bit and summand replaces
8,617,488 representative implications at `m=4`. The predicted domain count
is 2,621,168 clauses, with 329,726 variables and 3,901 XOR rows unchanged.
These are representation counts, not a solver speedup estimate.

Freeze the source before any retained run. First execute
`primary-full-d18-m4-shared-model BASE_DIR FIXTURE_DIR NEW_OUTPUT_JSON` with
the same complete hash-checked base and public fixture as `MODEL_PROTOCOL.md`.
Require exact preflight and actual model counts; record whole-process time,
import/model intervals, architecture, memory and executable SHA-256. No SAT
search runs in this arm. Compare against the previously retained default
model only as a model-size/build diagnostic; the host conditions differ.

After that, execute one
`primary-full-d18-m4-l0-shared-positive BASE_DIR FIXTURE_DIR NEW_OUTPUT_JSON`
on the same local host, with the 300-second internal whole-process cap and
310-second independent outer cap. This is a constructed planted relation
using the same source-bound full import, pinned summand and intermediate x
blocks, SAT-model check, group lift, signed-orbit row, tamper rejection and
rank-one check as `L0_POSITIVE_PROTOCOL.md`. Retain phase, shape, result or
cap, process/host, S3 GET and raw-file hash receipts. A cap or failure is
censored; no natural query follows from it. Record any `PASS_CONSTRUCTED_CONTROL`
only after the original-field row checks and an independent receipt audit.

The shared-root option is a solver representation for the same factor base;
it is not a new base. Neither a model-size reduction nor a planted control
selects a factor base or determines complete cold one-target IC runtime.
Keep `selected_best_total_runtime=null` pending checked natural useful-rank,
linear algebra, unseen-target descent and same-target rho gates.
