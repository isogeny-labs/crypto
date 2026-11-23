# Retained Koblitz m3 artifact visitor

The test-only adapter exposes borrowed views of its existing byte owners. The
visitor allocates no metadata or buffers, serializes nothing, and performs no
publication. A callback error stops visitation immediately without consuming or
changing the retained run. A parent may retry using the same owners; it must
admit and account for its own storage operations.

The four registered artifact names retain their existing roles and encodings.
Input bytes become canonical only after the original persistence boundary;
boundary JSONL contains only acknowledged records (including a valid empty
prefix). Group and terminal result bytes become canonical only when their
respective bounded encoders report completion. Encoder completion does not
establish completion of the scientific operation.

Failed input persistence, pending native-event encoding, incomplete group/result
encoding, and an unacknowledged witness encoding are exposed under distinct
`partial/*.raw` paths with `unregistered-partial-artifact` role and `native`
encoding. Their bytes remain diagnostic evidence. Missing or empty unencoded
buffers are not invented as canonical records.

`pdp-witness.json` receives the independently verified witness identity only when
its encoding is complete and the bound publisher retained a real publication
receipt. That receipt accompanies the borrowed view. The parent must preserve
this existing publication identity rather than publishing another witness.
Independent replay still authenticates the bytes and receipt against capture.

Controls check real positive and negative native runs, byte-owner pointer
identity, parent-storage failure with repeat visitation, failed input
persistence, and withheld encoder/publication evidence. The combined verifier
fixtures write their canonical files through this visitor before independent
arithmetic and capture checks. These are software controls, not new research
dataset cells or a production storage integration.

Remaining integration: parent storage admission, outer-supervisor censoring,
central replay registration, and runtime/CLI dispatch. The adapter remains
test-only until those gates are met.
