# Required software validation, not execution authorization

This is a proposed finite software acceptance plan. No fixture has been generated
or audited by this design task. Freeze the exact protocol/spec/config hashes and
expected semantic coverage before any execution; retain every failed/capped cell.
Do not launch cloud work or consume another retry/budget authorization.

## Independent module tests

Use hand-derived sparse equations over small prime r, with an exhaustive small
matrix oracle or separately enumerated linear combinations where feasible. Test
normalization/cancellation, first duplicate representative, singleton peeling,
core order and static degrees, smooth rows, preferred existing pivots, tie breaks,
accepted/rejected fill, source-weight limits, immutable pivot snapshots and row
versions, full cancellation into complete rows, and an acknowledged Finish.
Include two different legal singleton choices: the wire checks a valid singleton,
not a falsely specified queue order.

Exercise malformed and rehashed mutations of every identity: raw input/source
hash, source order/id, normalized coefficient, RHS, source combination, pivot
reference, inverse/scale, fill acceptance, report count, reservation, event
sequence, final rows and false completion flags. Native arithmetic tests should
include raw u64::MAX coefficients/RHS and high IDs with an odd modulus near2^63
and manually known unit-pivot cancellation; the standalone native checker may
check odd-modulus unit operations, while the dataset rank path additionally
requires prime r. Explicitly test r=3 rather than calling Field::prime blindly.

Check InputLimit before hashing; exact State/Event/Work priority; ByteLimit with
one computed-but-unacknowledged event; all zero remainders; Finish refusal after
complete rows exist; and a reported SinkFailure prefix. A sink prefix can be
recognized by the low-level verifier but must be rejected as a successfully
sealed dataset-v2 batch. Frozen production v2 never accepts reduced test policies.

For cumulative tests use at least two cumulative source sets and a third terminal
attempt. Exhaust each events/bytes/work allowance exactly; reject full-cap reset,
subtracting only acknowledged work, dropping the refused-event bytes, accepting
a computed event as acknowledged, and appending queries/batches after censorship.
Remember Begin serializes its own limits: precise byte-boundary fixtures must
encode the changed policy and account for decimal-length changes, not subtract
bytes from an event produced with a different numeric cap.

Rank tests independently enumerate small solution sets and assert determined
columns, including a target determined while some base columns are free, full
rank, consistent rank deficiency, inconsistent augmented row, zero rows and zero
completed rows. Reject treating arbitrary free-variable-zero values as logs.
Require kG=Q and every supplied full-rank base log equation independently.

Hash-format tests distinguish sorted whole-object binding from typed native input
and native event-byte serialization. Reorder nested JSON object keys without
changing canonical object hashes; change array order and require rejection.
Add unknown/duplicate fields, float/negative/string integer aliases, noncanonical
point decimals, input/state size overflow and32MiB reader boundaries. Preserve
existing root sorted-hash behavior if moving its helper into common.

## Sealed producer/consumer acceptance

Use a separately built, source-bound producer; the retained current candidate is:
source58bd5ab64424f7710540e0aea502225de6e6c438, CLI SHA256
968a316daab4084ef61fd46c779832b9ebd9e482e58492674bc82174c4b1cfb7 at
/home/ec2-user/ml-dataset-resume-20261028/generic-filter-trace-validation/crypto-source-bound.
Check that source/binary still match and coordinate with its owner before use.
A design-time preflight is not fixture execution or an arithmetic pass.

Proposed first frozen panel (32 small software cells):

- prime n13, m2 and3, both twists;
- binary n12 and13, m2 and3, both twists;
- Koblitz n12 and13, m2, both twists;
- each with absent factor_base_policy and explicit {kind:legacy};
- factor_base_size2, work_limit20000, degree_bound3; producer unit-test seed
  tuple curve_seed7,generator_index1,key_index2,seed42, solver double-large-prime.

Retain actual route classification. Prime/binary generic calls should carry v2;
some Koblitz calls legitimately retain signed-Frobenius v1. Do not force generic
mode or relabel those to manufacture coverage. A curve-generation failure is a
retained error, not a replaced draw. A later additive supplement needs its own
frozen software spec before execution.

Required semantic acceptance observations, not assumed outcomes: at least one
completed v2 scalar recovery, a completed batch with a source combination using
multiple input IDs, more than one cumulative batch, both twists, odd/even binary
fields, at least one genuine Koblitz v1 preservation cell, and one zero/no-query
or no-recovery result. All generated rows, including errors/unsupported/censored,
are retained and audited according to their real status. Existing saved v1,
policy and root fixtures must replay without changing their evidence or features.
Larger requested field-size boundaries use existing public arithmetic vectors
or already saved valid cells; this panel does not prove large-size corpus coverage.

Production-policy censoring needs a real sealed v2 fixture. A bounded additive
candidate panel, if needed and separately frozen, is prime/binary n13, both
twists, m15, factor_base_size128 or256, work_limit100000, same fixed seed tuple
(eight cells). State conservative per-cell software limits before launching;
suggest no more than30 seconds,1024MiB memory and128MiB capture per cell, then
confirm these fit existing supervisor/capture admission. This is a maximum of
40 proposed local cells, not an open search. If no candidate seals with an actual
native-policy censor, report that coverage gap. Outer timeout/payload failure is
not a sealed native censor, and reduced internal test caps must not be relabeled
as accepted production v2. Do not keep scanning seeds after seeing results.

The mathematical acceptance path should run entirely in the independent consumer
without the producer verifier to establish independent evidence. Run the same
retained cells with the optional producer replay in a separate gate, and report
that result independently. Do not regenerate cells between those checks.

## Sealed tamper and feature tests

Copy retained fixtures into distinct directories and rehash all modified artifact/
manifest references so tests reach semantic checks. Originals remain unchanged.
Reject missing/extra batch payload, wrong role/encoding/path/index, transcript/
payload disagreement, false native-certificate claim without v2, changed full
config/seed/public/base/domain binding, wrong query/source prefix, source order,
reset cumulative allowance, wrong unacknowledged accounting, truncated query
schedule, missing Finish, rank/recovery on a censored batch, and post-censor work.
Test a changed valid public target with recomputed generic binding, not only an
obviously malformed point. This must fail on source/group/recovery arithmetic.

For an equivalent public/base/config pair, compare legacy/generic-v2 feature
values byte-for-byte under canonical consumer serialization. Inject sentinel
seeds, source coefficients, private logs, pivots, RREF, counters, stop reasons,
construction metadata and outcome text into allowed restricted provenance; none
may enter features. Root/policy schemas, static split assignments and unsupported
null-feature behavior must retain their current tests. Do not claim matching the
producer's different training-export view.

## Evidence and completion

Retain frozen input files, source commits, compiler/dependency versions, clean
build identities, binary hashes, preflights, stdout/stderr/exit codes, all sealed
and incomplete cells, independent consumer reports, optional producer-replay
reports, export comparisons, and all tamper receipts. Separate hand-derived,
native producer, sealed adapter and restored-artifact evidence. No hand-authored
trace establishes native execution or S3 durability.

After implementation approval, coordinate native test/lint/build ownership once;
no full producer suite duplication. Publication must wait for the actual required
terminal results. The parent can persist the source/fixture bundle with the
existing software S3 checkpoint route and a fresh read/hash verification. A
controller archive roundtrip does not establish EKS worker publication rights.
