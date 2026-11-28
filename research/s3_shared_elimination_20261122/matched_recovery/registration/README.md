# Unsigned matched-recovery statement registration

Statement **PCC-20261123-13014e** binds the committed matched recovery receipt
at source/evidence commit `2b1a5722dbad5ff881f6695d0cd934bc8c1fa85a`.
The receipt SHA-256 is
`13014e0abf1b1e5333e6d7902bcf819ec1ff158e82c904f0161bc8fbcf27ec81`.

The existing native register_statement.sh produced [preview.json](preview.json)
and then [append.postcall.json](append.postcall.json), using the exact preview
timestamp and claim digest. The helper checked all 1,370 pre-existing ledger
entry hashes and appended one entry at sequence 1370. Existing ledger bytes
remain a prefix; neither another branch's history nor frozen evidence was
replaced.

| Binding | SHA-256 |
| --- | --- |
| Claim canonical bytes | `5c8d5254f18e88c1181597d416ead905791887b67ed3fc6483ebaf3a428b334c` |
| Appended ledger entry | `d0fd90b87c5baf876d65421772408ee4f9bffdbe6ee1d0c20211407462713de3` |
| Prior ledger tail | `940122e0959dc32a7503866647627ed3fc3f8358f9bcfca9f38642524aa1fa5b` |

This is kind=statement and claim_type=data with an unsigned attestation.
Native jq/sha256sum metadata operations perform no scientific replay or
signing. The claim remains unverified; no verifier was run, no signature was
created and no performance or full-library success is asserted.
[pcc:unverified]
