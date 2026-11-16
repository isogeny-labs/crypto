# Actual finite-field cyclotomic degree and all-root rationality

For every finite field K and every natural r nonzero in K, Lean proves that the actual field adjoining all r-th roots of unity in AlgebraicClosure(K) has degree equal to the multiplicative order of #K modulo r. For every actual finite intermediate extension L, all r-th roots belong to L exactly when r divides #L−1, equivalently (#K)^finrank(K,L)−1. Prime r is not required. Primitive root existence, simple-extension presentation, algebraicity, finite dimension/carrier and actual Frobenius order are derived. [pcc:unverified]

Deleting the characteristic guard fails on the same actual fields: in every finite field of prime characteristic p, the actual p-th-root field is the base field and has degree1, while the modular order of #K modulo p is0 and p does not divide #K−1. For every field the first- and second-root fields have actual degree1. The correctly qualified cyclotomic statement survives. [pcc:unverified]

This advances items2/3/4. It does not identify the full elliptic E[r] field with the cyclotomic extension. General canonical Weil/Tate, geometric torsion, isogeny/Lang/Cartier/Jacobian/divisor and MOV–Frey–Rück bridges, nonprime supersingular at-most6 classification and the other [fourteen-item obligations](../theorem_census_20261109/REQUEST_SCOPE.md) remain. No canonical promotion, novelty claim or full-request completion. [pcc:unverified]

## Lean declarations

The closed main theorem is `FiniteCyclotomicDegreeAudit.finite_cyclotomic_degree_certificate` in [CyclotomicCertificate.lean](lean/CyclotomicCertificate.lean). Its entire861-character normalized header has binding `f288e1fae592ab4d95cb11362fa1784f7a077a5ba6ee54060279e215ce8c5742`; independent native processing reports1complete/0prefix/0unresolved. This source processing remains procedural, without kernel parser/hash/physical-host refinement. [pcc:unverified]

| Module | Actual mathematical content |
|:--|:--|
| [RootField](lean/RootField.lean) | All-roots adjoin in the actual algebraic closure; derive primitive root/simple presentation, algebraicity/finite dimension/carrier; r1/r2 degree1. |
| [FrobeniusDegree](lean/FrobeniusDegree.lean) | Actual nth Frobenius identity iff divisibility; actual finrank=modular order; degree-divisibility criterion. |
| [RootRationality](lean/RootRationality.lean) | Actual finite extension descent using splitting X^#L−X; root-field containment and all-root rationality iff divisibility, including actual q^finrank cardinality. |
| [GuardControls](lean/GuardControls.lean) | Actual characteristic-p roots collapse; actual degree1 versus order0 and containment with failed divisor condition. |
| [CyclotomicCertificate](lean/CyclotomicCertificate.lean) | Complete universal theorem, both small-order controls and characteristic-guard counterexample family. |

See the [proof map](proof-map.json), [source plan](sources.json), [preflight](preflight.json), [source review](evidence/source-review.json) and [one-page report](reports/report.pdf). The report was fully text-read and visually inspected; all text and diagram fit. [pcc:unverified]

## Validation and provenance

Whole-project Lean gate:23theorems/0failures/0trusted native, only propext/Classical.choice/Quot.sound. Final08 and all gate logs were searched completely: no warnings/errors. Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679. Eight formal/config pins and14artifact pins. [pcc:unverified]

Eight snapshots retain all three failed attempts01/03/04 and successes02/05/06/07/08. Failure01 was power/application parsing;03 was an unregistered Lake module target;04 was an explicit mapped-polynomial root inference issue. Error-recovery outputs are retained and never accepted as proofs. Earlier passing builds had unused-section/deprecation advisories, removed structurally/current alias before final08. The snapshot02 copy named nonexistent lakefile.lean; actual unchanged TOML was added to its retained snapshot. Setup01 succeeded before first build. [pcc:unverified]

All88provenancefiles/44uniqueobjects roundtrip through a16332-byte lossless deduplicated XZ archive, plus five gate files in a separate archive. The original pre-code protocol and every log/exit/source snapshot are retained. The archive also retains five credited original sources, checked byte-identical against local baseline ac659501ae96906e82d31f514ffe5ff9d1d5871d. No issued originals were edited. [pcc:unverified]

No holes/custom axioms/native proof evaluation/raised limits/new global elaboration flags/linter waivers/frozen edits/cryptographic Python/new native cryptographic experiment/paid compute/agent upload. Python handles only provenance, source processing, metadata and report layout. Local verification does not establish signed standing. [pcc:unverified]

Reproduce the source proof and whole-project axiom audit:

```sh
cd research/finite_cyclotomic_degree_audit_20261115/lean
lake build
cd ../../..
bash tools/lean/proof-gate.sh research/finite_cyclotomic_degree_audit_20261115/lean
python3 research/finite_cyclotomic_degree_audit_20261115/restore_attempts.py /tmp/finite-cyclotomic-attempts
```

FST2010 DOI10.1007/s00145-009-9048-z, primary723726bytes/57pages/SHA2acd937be8567abfb26bb3861e3aaf34312270c36df9a86f91cf9676c3caf785: exact printed230/PDF7 Definition2.1/Remark2.2 and printed237/PDF14 were fully text-read/visually inspected in the preceding packet, not all57pages. Prior CyclotomicDegree/GaloisGenerator/FiniteQuotients/CharTwoGaloisControl/SourceFSTBoundary arguments and pinned official primitive-root/adjoin/Frobenius/order/cardinality APIs were read, generalized and credited. KB MCP absent; fresh status/two mechanism-negative searches/theorem and Conductor wrappers127, bounded prior/primary/official fallback. No absence or novelty claim. Primary copyrighted files remain local. [pcc:unverified]

Exact local **PCC-20261115-b5d0f3** replay PASS at 2026-11-15T02:29:20Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Published in existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743): localreceipt `ca22f1fd8ae4d97d79766f0499c5c0dd6f6a96ff`, remoteproof `6c3446594f1ad34018623d7954e25fab10d340b7`, exact shared tree `aff161afae7c6d4d3989bb5307d28408812985f8`. All42paths/seven large-or-binary blobs match, nonforce expected-head update, completed anonymous fetch and fresh exact PR head/body117 confirmed. All600ledgerrows preserve the599-row prefix;43request local links resolve. [Publication receipt](publication.json) is outside the frozen proof/artifact pins; final metadata is separate. Signed standing unverified. [pcc:unverified]
