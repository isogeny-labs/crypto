# Claim pin correction without rewriting history

PCC-20261115-dfed92 pinned the initial LEADS.md draft before the final prose
lint pass. That draft is preserved byte-for-byte in lead-draft-at-claim.txt
(SHA-256 4dc0635e8bc5b780855efdb938b54c7e4e62e08184c0ad0e15f1b72cd7f27e96).
The live note subsequently received explicit unverified tags, so the old
claim's live-path attestation pin is stale. It must not be used as a current
proof. Neither the old claim nor its ledger row is edited or promoted.

The replacement producer statement pins the frozen QPROJECTION_PROTOCOL.md
instead. Both records remain unverified; no verifier signature is supplied.
This corrects a producer metadata error and preserves the audit trail.

## Integration correction

The main CI gate later rejected the stale live-path pin, as expected from its
hash check. Integration restores LEADS.md to the exact originally pinned
bytes without changing any claim or ledger row. The current evolving note
is retained separately as LEAD_STATUS.md. The earlier stale-pin account above
is historical; the original subject now matches its pin again. Its standing
is still unverified because no independent signature has been supplied.
