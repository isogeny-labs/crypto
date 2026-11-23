# V1 quota scope clarification

Frozen v1 checks one million represented entries separately for EACH rank
certificate; rank() and check::certificate() restart that accumulator. This is
not a global whole-Audit or live-memory guard. Work/coefficient-write meters
ARE shared across phases per call. The32MiB output guard is producer-side;
the in-memory reader API is not a JSON transport admission API. Read frozen
protocol/report quota wording with this clarification; do not repin old files.
[pcc:unverified]

A separate qualified Rust METADATA reader counted the existing certificate
records without executing field arithmetic or rerunning any rank/solver case.
Whole-Audit totals,including three rank certificates and image terms,are4963
for raw12 and6119 for input204; the five small cases have7,8,9,5,5 records.
All seven fixed cases fit the global reading of one million entries. These
are serialized term/origin records,not bytes,RSS or normalized field work.
The largest JSON is1111242bytes; an initial byte-bound assertion stopped
before publication and was replaced by these direct native record counts.
[pcc:unverified]

The original source/native/report packets and rank evidence remain unchanged.
New metadata-reader source3/input7 identity,pre/post records and counts are in
quota-reader-v1/. A future global guard needs a new version/qualification.
