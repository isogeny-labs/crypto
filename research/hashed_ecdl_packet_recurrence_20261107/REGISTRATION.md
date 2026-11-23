# PCC registration and remaining integration boundary

On 8 November 2026 the owner explicitly authorized **one official PCC ledger
append, preserving every existing row byte-for-byte**. The official CLI
created **PCC-20261108-ebd943**, kind statement. Its standing is UNVERIFIED;
no proof or verdict exists. Registration does not verify the proposal or
resolve the fork. [pcc:unverified]

Command executed once:

`python3 tools/pcc/pcc.py claim new --kind statement --statement <prepared source-review scope> --scope research/hashed_ecdl_packet_recurrence_20261107 --session hashed-ecdl-packet-recurrence-20261108`

The statement pins the source review to crypto
08fb3510592ce4849abaf660bbee88191d3d6263, and explicitly excludes a checked
theorem, complexity result, new algebraic identity or scientific control.
The actual claim JSON is ../../pcc/claims/PCC-20261108-ebd943.json.

| Identity | Value |
| --- | --- |
| Official canonical JSON digest | b098549940662ce47c93bd229f88c13981a04b612a5b86a7ba0e7f914c93edbf |
| Raw claim-file SHA256 | 955ab3a46995b3683b0490ecb34d99d386dd03374ab2d3d9bd6c5c818a035456 |
| New event hash | d7797f12a4548f8797a7606c2707ff29b463204f0a377dff39428cf7685bb88a |
| Ledger prefix | All182 old rows exact; one new event,183 rows total |
| standing | unverified, no verdicts, exit2 |
| verify | not_covered, exit2: statement has no proof kind and can never be verified |

The ledger chain/base-prefix check passes against the local draft parent
abf614dc2711f6009f04afb51c7819f6ea59de8d. Every old claim file is byte-identical.
This is NOT a prefix pass against advanced main. No old row was edited,
re-chained, deleted, reordered or selected from competing sides. No signing
key was read/generated and no verdict was signed. [pcc:unverified]

Historical hold: Nov7's draft had no new claim and remained unpublished because
registration would write the conflicted ledger. The Nov8 owner authorization
permits this single official append. It does NOT authorize resolving the
existing main/branch ledger fork, changing gates, merging while checks are
pending/failed, or promoting a statement to a proof. The Nov6 claim remains
historical and is not reused to cover this wider packet. [pcc:unverified]

PR3569 remains the sole publication destination. Copilot-first request6690196498
is still pending; no duplicate request or parallel conflict resolver. Normal
merge requires a preservation-compatible resolution plus every current-head
required gate/review and expected-head guard. The general audit stays open.
