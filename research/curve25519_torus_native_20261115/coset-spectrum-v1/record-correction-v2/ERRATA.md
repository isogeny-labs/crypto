# Declaration-binding correction: source proof unchanged, old record retained

The producer introduced a metadata defect in PCC-20261123-8d8628: all three
lean proof objects omitted the required decl_sha256 field. The claim writer
serializes supplied fields and did not derive these hashes. The source file
and local3-theorem kernel gate passed; that did not validate the incomplete
PCC record. Current-head job137952785754 correctly fails8d8628 before a kernel
recheck. This is this change's failure, not an inherited baseline failure.
[pcc:unverified]

Correct companion PCC-20261123-cd1904 supplies all three declaration hashes
computed by the maintained lean_find_declaration/sha256_bytes functions.
DECLARATION_BINDINGS.json retains each exact normalized statement, hash and
source-file hash. Local pcc verify with the pinned Lean environment reports
pass for all three lean proofs and overall pass; unsigned attestation notes
remain. This is an UNSIGNED local verdict. Maintained standing still reports
unverified. No theorem source or original gate receipt changed.
[pcc:unverified]

The original8d8628 file and its published ledger entry remain byte-identical.
The correctly bound companion does not make the old record valid and does
not cause the active changed-claim gate to skip it. The current maintained
CLI offers claim new, verification, signing and append-only ledger operations,
but no withdrawal/supersession command. No verifier/policy/keyring exemption,
claim deletion, active-file rewrite or published re-registration was used.

Residual merge blocker: an authoritative data-preserving correction or
withdrawal path for the malformed published record is needed. See the named
[problem](../../../../problems/pcc-published-incomplete-lean-binding.md).
This is a record-management dependency, not a mathematical obstruction or
evidence against the spectral construction. The finished companion and exact
old/new evidence make the correction reviewable; broader science stays open.

CI_DIAGNOSIS.json and the complete decoded logs also distinguish two unrelated
AMM frozen-source assertion failures on the SAME1ef head. Both scripts and
both Rust input blobs equal the actual base. They stop before compilation;
their sources were not patched. Later pending/revalidation gate outcomes
were not inferred from the earlier failed job. [pcc:unverified]

The original written report does not cite a PCC ID and its mathematical scope
is unchanged. References to8d8628 in pinned version1 notes describe the initial
record and must be read with this erratum. Use cd1904 for the complete current
declaration bindings; none of these records establishes signed standing.
