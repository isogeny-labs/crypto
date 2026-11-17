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

Corrected source `35345db424ccf3217cb8d6a6f346aeee4e0f10a2` passed 12 release
standalone tests, zero failed, one explicitly ignored native-fixture test;
strict release all-target Clippy and the CLI build passed. Invoking that
ignored test separately passed against all six retained native streams.
Five complete ranks remain 0, 1, 1, 2 and 92; the capped prefix has no final
rank. Four rehashed arithmetic corruptions reject, and all six CLI reports
match the library reports. Input fixture hashes match the earlier archive.

The independent parent read the corrected runtime and tests before publication
and found no actionable issue in this narrow change. The 2,129,329-byte archive
in `input-digest-validation-archive.json` passed conditional PUT, ETag-bound
HEAD/fresh GET, byte/length/hash comparisons, contained-file checks and restored
source-bundle verification at 2026-10-29T03:10:27Z. It includes the original
failed tests and the explicit disqualification of a late source-hash capture;
that capture is not used to identify the failed runtime.

Root runtime is byte-identical to c14d4552e513f6e28bda0e0e045f5e64beb492db;
its inherited 5183/0/123 gate is not represented as a new root test run.
Current-head hosted CI remains to be checked after publication. This is
software correctness work, not a new corpus cell, experiment, IAM change,
budget increase or deadline extension.
