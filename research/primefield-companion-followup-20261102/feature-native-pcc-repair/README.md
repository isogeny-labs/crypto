# PCC producer-input correction

At source commit 22142669387255487947f04de9e6cd8573067a06, ten companion claims contain thirty flat execution proof pins. Every referenced file still matches its pin (audit.json). This establishes file custody only. The existing verifier rejects these entries because execution evidence requires a receipt, verifier pin, image identity and raw-result journal binding. A native test log does not provide that proof.

The CLI now rejects execution:PATH and incomplete structured execution inputs before creating a claim file or appending the ledger. Complete structured inputs remain subject to the existing verifier and trust policy. Producer observations can use attestation:PATH, which remains unsigned/not_covered without a trusted signature. No old claim, ledger entry, run output, verifier policy or keyring is changed. The historical verify-all failure remains visible; this change prevents recurrence rather than certifying the old records.

Sources checked: docs/SABOTAGE_RESISTANCE.md proof-kind table; tools/pcc/pcc.py verify_execution, verify_attestation and claim creation; PR3602 PCC run42326534442. No literature claim or new cryptographic experiment is involved.

Validation command: python3 tools/pcc/test_pcc.py. The new subprocess test checks seven malformed inputs, byte-for-byte ledger preservation, absence of claim-file writes, and a complete untrusted structured input that remains not_covered. Existing attestation coverage checks unsigned status and rejects altered artifact bytes.

Initial test run: 13 methods, one failure. The synthetic receipt was plain text and the verifier correctly rejected non-JSON before reaching the untrusted-verifier check. Changed the test fixture to valid synthetic JSON; final output is tests.log. No scientific output was regenerated.

Research continuation: root_resultant currently constructs a source product tree, evaluates it over a destination remainder tree, materializes all leaf values and folds their product. Projection cannot move through the polynomial additions. A useful next construction is a recursive product-of-evaluations accumulator that retains full-field remainders but removes the leaf-value vector, followed by one projection of the combined result. Its time and memory costs need explicit comparison; projecting each leaf separately would add exponentiations.
