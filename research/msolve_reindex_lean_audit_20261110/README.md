# Lean audit of the observed coefficient-index relocation

The new Lean module defines the copied coefficient table by three
operational updates: old basis indices[0,2,0] are copied into matrix
indices[2,3,4]. These are the values logged by both source variants in
the separately frozen native experiment. The basis has three live
entries. Native trace authenticity is not proved by Lean.
[pcc:unverified]

For every coefficient type and every basis content, Lean proves that
reading the new matrix index returns the coefficient at the old basis
index. Reading the rewritten index through the live basis fails at
index3 for every possible basis. A concrete natural-valued basis also
shows that rewritten index2 can remain in range while returning a
different coefficient. The universal basis-reindex invariant is directly
negated. [pcc:unverified]

The live prefix is distinct from allocated C capacity. The theorem does
not assert that index3 exceeds the allocated pointer array, and does
not prove C initialization, pointer safety, a parser/trace refinement or
whole msolve conformance. The observed native fault and passing patched
fixture have separate unsigned evidence. [pcc:unverified]

| Declaration in MsolveReindexAudit | Exact obligation |
|:--|:--|
| source_and_destination_bounds | Old indices are below3, copied slots lie in[2,5) |
| copied_lookup_correct | Operational copy then matrix lookup returns the original coefficient |
| observed_live_index_failures | New3/4 absent from live basis; present in copied matrix |
| observed_lookup_disagrees | Every basis disagrees at new3 |
| in_range_lookup_can_change_coefficients | New2 is in range but can return another coefficient |
| universal_basis_reindex_invariant_false | Direct negation of the universal wrong-table invariant |
| signed_shift_boundary / unsigned_bit_fits | Exact integer representability only, no Csemantics claim |
| reindex_audit_certificate | Combined copy, mismatch, negation and representability theorem |

The [native experiment](../msolve_crash_diagnostic_20261110/coefficient-remap-v3/README.md)
retains all four calls. Common-logged baseline ordinary aborts134 after
looking up3 in a live basis of3; the corrected matrix lookup ordinary
passes the exact known-answer basis. Both saturation calls still time
out at5s. The source variants differ in one lookup expression after
identical logging and build flags. No signed execution, generic solver
repair or novelty claim. [pcc:unverified]

Pinned0.6.5's16bit reducer and current upstream32bit reducer already
use the matrix table. This is a concrete backport/source audit.
Relevant code paths, current-source hash, primary URLs and read scopes
are pinned in evidence/source-review.json and the native provenance.
KB unavailable, wrapper127; bounded source/GCC/Mathlib fallback.
[pcc:unverified]

Lean4.35.0 / Mathlib56a00585 are pinned. Full gate14declarations,
0failures/0trusted-native; only propext, Classical.choice and Quot.sound.
No holes, custom axioms, native_decide, waived linters or increased
elaboration limits. All attempt/gate bytes are archived losslessly.
Sources.json freezes four formal/config files and exact native artifacts
before the Lean packet is issued. Local replay and independent signed
standing are separate. No paid compute. [pcc:unverified]

```sh
cd research/msolve_reindex_lean_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/msolve_reindex_lean_audit_20261110/lean
```

See sources.json, verification.json and the inspected
[report](reports/proof-20261110/report.pdf), with editable proof.dot.
[pcc:unverified]

Source78222dee precedes PCC-20261110-9f17a1; actual local replay PASS
at17:51:02Z. All four formal/config pins and exact native artifact pins
match. Two attempt files and five gate files have byte-equal archive
roundtrips. Final two-page50611byte joint report inspected on both
pages. Signed standing remains unverified. [pcc:unverified]
