# Complete written binding for the actual F₅ BK certificate

`BKFullBindingAudit.full_bk_certificate` is a versioned wrapper for the
unchanged actual-object proof of the Balasubramanian–Koblitz criterion on
E/F₅: y²=x³+1, r=3. Its written type explicitly quantifies the algebraically
closed extension L and its field, algebra and decidable-equality structures.
It preserves every clause of the original certificate. This adds a complete
receipt; it makes no new general mathematical claim. [pcc:unverified]

The theorem proves nonsingularity, #E(F₅)=6 and the prime/coprime/divisor and
r∤q−1 source conditions. For every k it proves that all geometric3-torsion
points are fixed by Frobeniusᵏ iff3∣5ᵏ−1, supplies an actual order3 nonrational
witness with Frobenius(P)=−P≠P, proves the corresponding actual point-image
criterion for every compatible chosen finite extension of cardinality5ᵏ,
and constructs an actual GaloisField5k embedding/cardinality for every
positive k. No rank, determinant, Hasse or pairing conclusion is supplied as
a premise. [pcc:unverified]

[ActualPointMaps.lean](lean/ActualPointMaps.lean) defines the rational,
chosen-extension and GaloisField maps using actual Mathlib affine Point.map.
They are explicit functions, not assumed predicates. The
[main theorem](lean/FullBKCertificate.lean) applies the frozen
`BKActualCertificateAudit.bk_actual_case_audit_certificate` clause by clause.
Recursive custom-source/config pins appear in [sources.json](sources.json);
see the [proof map](proof-map.json) and [source review](evidence/source-review.json).
[pcc:unverified]

The [independent written-header audit](../statement_binding_audit_20261114/README.md)
found that old issued **PCC-20261110-99983d** stores only176characters of its
913-character written header, ending inside a named argument. Its original
source, claim, ledger row and whole37-declaration proof receipt remain
unchanged. That observation did not refute the mathematical theorem.
This wrapper writes all field assumptions and avoids embedded delimiter
syntax in its main type. Its complete934-character written binding is:
[pcc:unverified]

```
255334b4de88ddc92803431efa042b40aa02db63affd414666c7a20c01cb02ed
```

The frozen extractor and the independently executed official Lean parser
agree on this entire written header:1complete/0prefix/0unresolved. Native
source/hash parsing is separate unsigned procedural evidence, not a theorem
proved by the wrapper. The canonical shared verifier remains unchanged;
global semantic telescope/dependency binding remains an open contract.
The [named problem](../../problems/lean-statement-binding-completeness.md)
distinguishes the local repair from that larger work. [pcc:unverified]

The new project whole proof gate passes1theorem/0failures/0trusted native.
Its transitive axioms are propext, Classical.choice and Quot.sound. Build01
fails with a duplicate local instance and conjunction elaboration mismatch;
build02 retains the mismatch; build03 constructs the unchanged conjunction
clause by clause and passes. All three source/config states, full logs,
protocol/native inputs and outputs remain losslessly preserved:44files,
25objects,8824-byte XZ archive; five-file final gate separately retained.
No failed build is counted as a proof. [pcc:unverified]

```sh
cd research/bk_full_binding_audit_20261114/lean
lake update
lake build
cd ../../..
tools/lean/proof-gate.sh research/bk_full_binding_audit_20261114/lean
python3 research/bk_full_binding_audit_20261114/export_binding_input.py --out /tmp/bk-binding-input
bash research/statement_binding_audit_20261114/run_audit.sh /tmp/bk-binding-input/manifest.json
python3 research/bk_full_binding_audit_20261114/restore_attempts.py /tmp/bk-binding-attempts
```

The exporter only copies source/header metadata from the frozen extractor;
the independent comparison executes in native Lean. The output directory
must be new. Resolve the pinned Lake dependencies before running the audit.
Exact local **PCC-20261114-8c5c1e** replay PASS at 2026-11-14T15:14:07Z; signed standing remains unverified. See [verification receipt](verification.json).
The [visual report](reports/report.pdf) was fully inspected, one page.
[pcc:unverified]

Sources checked: original BK1998 Theorem1/conditions/proof pp141–143,
earlier full141–145 read receipt credited; frozen actual proof/dependencies;
official Mathlib affine Point.map/base-change/tower interface. KB MCP absent;
fresh status/two mechanism-negative queries/exact theorem query all127,
with primary/repository/official fallback. Lean4.35.0/Mathlib
56a005855ea17e0b0958f73ec801ae140b0a1679, standard foundations/default settings.
No holes/custom axioms/native proof evaluation/global flags/raised limits/
waivers/new cryptographic experiment/paid compute/agent upload. No novelty
claim. General r≥5 BK, canonical Weil necessity, complete all-k census and
the other original fourteen-item gaps remain separate. [pcc:unverified]
