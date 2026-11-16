# Fresh native source-to-decoder recovery

The native pipeline now takes a public Q, generates the complete width-q shared
source window, evaluates the compiled signed-character program modulo r, and
checks [k]P=Q before returning k. Decoder stores its curve, width, Source and Program
privately; PreparedDecoder borrows immutable state and retains prepared source
strides. The generic constructor uses the same r for the source and coefficient
arithmetic. It validates packet representation; it does not certify the polynomial
identity. The final point check applies even to a incorrect but well-formed coefficient packet.

The repaired_479 constructor embeds the previously checked477-term packet for
E/F509: y²=x³+2x+10, r=479, P=(3,68), q=16. Its TSV is projected exactly from the
hash-bound repaired program.json. No new coefficient synthesis is performed.
An arbitrary curve still needs a suitable coefficient program and the inherited
prime-field, prime-order, word-size and source normalization assumptions.

## Data flow and verification

The online recover call accepts only Option<[u64;2]> for Q. Neither expected scalar,
saved signs nor lookup-by-target coefficient state is supplied. Source windows
are generated natively; the Program queries every prefix offset once. Any source
or sign error propagates. The resulting candidate is checked with a separate
ordinary affine addition/doubling implementation and binary known-scalar
multiplication. It does not reuse the source's projective geometry or seed state,
and does not search for a scalar. Only successful equality returns Recovery.
The public receipt contains logical source methods and verifier group-call counts.

## Exact checks

One rustc build and one selected invocation pass three methods. All eight public
points for k=0,1,2,25,173,292,440,478 recover exactly and pass the mandatory point
check. Only their Q coordinates are passed to recover. The four k values
25,173,292,440 exercise the character-fiber repair's two collision pairs.
Every target generates16 fresh signs. Saved source signs are not loaded.
Targets are a projection of the original r479 candidate records; the whole-file
hash and selected record hashes are retained in target-provenance.json.

Removing the final405*chi(X+15) term and recovering the k=173 point yields an
incorrect candidate, which the API rejects with Verification. Invalid public point,
width65, noncanonical coefficient479 and offset16 with width16 are rejected.
The independent multiplier reproduces all eight saved public points plus zero,
base, doubled base, negation and [479]P=O. Twenty-five earlier tests are filtered.
The inherited unused diagnostic-fields warning is preserved in build.log.

recoveries.json is an exact projection of successful test-log records. It reports
128 sign queries,9 direct phases,109 seed recurrences,10 anchor hits and25/36
verification addition/doubling calls across the eight successful queries.
Addition calls include identity handling; these are not counts of nontrivial
field operations. The negative control and setup are not included in those totals.
No elapsed-time claim is derived from the test harness duration.

## Cost boundary and next construction

Setup includes loading/compiling an existing packet, source calibration and stride
preparation. Producing a coefficient packet is a separate construction cost.
Query cost includes joint geometry, initial phases, seed recurrences (with their
inversions), value division, inner recurrence, polynomial/resultant work, raw
quotients, calibration, sign-mask evaluation and candidate point verification.
Only selected logical calls are currently counted. Allocation/retained memory,
field operations and integer work need complete accounting before comparison.

The original4563-case, three-curve/three-arm validation plan is unchanged and was
not run by these integration checks. Next add stage-specific native arithmetic
accounting and a replayable runner with separate setup/query/verifier receipts,
then reconcile EKS admission for the original full cohort. Preserve all original
curves/arms rather than substituting this eight-target fixture set.

## Prior sources

This binding reuses the checked shared-source and sign-mask components and the
preserved decoder contract, plus ordinary affine group formulas. See
../feature-native-shared-seeds/DERIVATION.md and
../feature-native-signmask/DERIVATION.md. Ten source/data locks bind the exact
components and repaired JSON. No new identity or coefficient-construction claim
is introduced by joining these components.
