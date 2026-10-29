# Censored-prefix input digest correction

PR #2800 review thread `PRRT_kwDOSMdswM6xQlnD` identified that the
independent checker waited for the first arithmetic event before validating
the Begin input digest. On source `1005b2aa24ee3330043c731a08571788c4120fbb`,
new regression tests reproduced successful acceptance of an incorrect digest
both at a zero-input Begin and immediately after the last Input. These are
incorrect integrity acceptances even though the returned report correctly
withholds final rank, whole execution, encoding, UNSAT and fall-degree claims.

The correction validates the commitment in `Replay::begin` when the declared
input count is zero, and in `Replay::input` when the final declared input is
accepted. There is no delayed arithmetic-event check. Each complete input set
is hashed once. A prefix with fewer than the declared number of inputs remains
incomplete: its full input commitment cannot yet be verified. The checker does
not invent missing inputs or strengthen its mathematical claims.

Two regression tests cover 36 combinations across zero, one and two inputs,
correct and incorrect declared digests, every available input prefix, and
three endings: clean EOF, an acknowledged Stopped event, or an unacknowledged
partial JSON tail. All nine incorrect complete-input commitments must reject;
the other 27 cases retain the existing censored, rank-free report semantics.
The original failed test logs and executable are retained separately. Native
interoperability uses the six previously captured producer streams from source
`29973950752cd0e2389a07608eee63781841afc2`; no producer execution is repeated.

The existing design, wire clarifications and native schema were read first.
Official RustCrypto SHA-2 documentation and serde_json's `to_vec` documentation
confirm the existing operation is a digest over the serialized input tuple;
this change moves its validation to the first event boundary where the complete
tuple is known. It changes neither the serialization contract nor SHA-256.

- https://docs.rs/sha2/0.10.9/sha2/
- https://docs.rs/serde_json/1.0.154/serde_json/fn.to_vec.html

Validation and publication evidence will be recorded after the corrected
standalone tests, strict lint and retained-fixture replay complete. Root crate
runtime is unchanged; its prior source-bound library gate is reused only after
a byte-equivalence check. This is software correctness work, not a new corpus
cell, experiment, IAM change, budget increase or deadline extension.
