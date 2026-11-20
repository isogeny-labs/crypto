# Compact full-cost analysis development

The Rust rust-protocol/ package implements the calculation layer needed by
COMPACT_SPEED_PROTOCOL.md. It validates a typed exploratory transcript and
calculates joint paired-bootstrap intervals for both controls and both cost
scopes. Worker launch-through-consumption, per-call validation and unamortized
shared reference setup are charged separately, then summed for the full-cost
comparison. Input, worker and reference-basis equality keys reject accidental
mixing; unknown fields and malformed/zero/overflowing costs are errors.

This is a calculator, not the process/backend adapter. It starts no workers,
CAS tools or cloud jobs and creates no elliptic-curve fixtures. Its numerical
flag grants no execution admission or family standing: those report fields
are always false. It cannot authenticate producer timestamps/digests, certify
basis arithmetic or detect costs omitted before its input was constructed.
Independent replay and the frozen execution gates remain required.

## Qualification

At source f018e71f6, copied/read-only release qualifiers completed ten library
and three CLI tests using synthetic costs and dummy digest keys only. They
cover paid setup suppression of an apparent component win, strict two-control
thresholds, paired-ratio estimation, shared resampling, quantile indices,
A/A rejection, incomplete/mismatched records, input/worker identity mistakes,
unsupported/fresh roster identifiers and CLI output scope. All three executed
binary hashes match their pre-call records. Strict release all-target clippy
passed. PCC-20261116-e82274 is an unsigned, unverified producer record
[pcc:unverified].

The initial a8db8cde3 library qualifier also passed ten synthetic checks
(PCC-20261116-8aa211, unverified), before strict clippy flagged its indexed
sample buffer. The revised row layout preserves the estimator/index draws and
was requalified. Initial CLI/analyzer copies were prepared but never called;
they have no post-call outcome and are not counted as passing
[pcc:unverified]. Both snapshots/failed static log remain in the evidence.

analysis-development/ retains distinct build/pre/post manifests, raw logs and
an archive envelope for the four executed copies across both sources. No
synthetic numerical value or unit elapsed time is a cryptographic measurement.
The full root library was not rebuilt for this independent package; its
previous qualification scope remains in SELECTED_ROOT_REPORT.md.

## Remaining execution work

No scientific timing or fresh fixture generation occurred. The environment
still has no checkpoint secret/outbound identity binding and its offered tools
cannot write that setting. The next runtime step needs the existing checkpoint
transport, supported backend/image/isolation/budget admission, and a qualified
process/capture/mature-CAS validation adapter that produces these transcripts.
The calculator alone does not complete that adapter.

The 15x full-cost criterion and conditional fresh-family rule remain unchanged;
no measured runtime gain is claimed. Percentile-bootstrap coverage is not an
exact guarantee at these sample sizes. Sources, deterministic estimator/RNG/
quantile choices and synthetic-only qualification are frozen before observations
in COMPACT_ANALYSIS_CONTRACT.md (PCC-20261116-c25572, unverified).
