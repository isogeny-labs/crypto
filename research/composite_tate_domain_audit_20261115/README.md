# Known composite Tate counterexample: reconstruction check

This reconstructs the actual F5/n6 case already recorded in `pairing_composite_order_audit_20261110`, PCC-20261110-5e63e9, whose earlier gate covered50declarations and also an F3/n4 example. The duplicate was found at attempt05 after an initially too-narrow prior search. The user was informed; the original protocol and scope correction are retained. **No new source coverage, original fourteen-item gap closure or novelty is claimed.** [pcc:unverified]

Four new modules import Mathlib and no earlier custom proof modules, on the same prover/Mathlib/host. This is an auxiliary known-result cross-check, not independent implementation or physical-host validation, and does not replace the original solving-degree negative control. The [original request map](../theorem_census_20261109/REQUEST_SCOPE.md) retains its completion boundaries. [pcc:unverified]

## Actual objects and statement

Actual E/F5:y²=x³+1 is nonsingular, has actual point count6 and actual P=(0,1) of exact order3. The left domain is the actual kernel of multiplication6; the right domain is the actual curve group modulo its multiplication6 image. Both cardinalities are6. The target is actual F5 units modulo the actual sixth-power homomorphism image. The image is exactly1 and−1; the target has cardinality2 and exponent2. None of these counts is an input premise. [pcc:unverified]

Every actual function on those domains with the two multiplicative bilinearity laws kills the explicit nonzero order3 left point against every right input. The nonzero quotient class of that point also lies in every right radical. Outputs are killed by both3 and2, hence equal1. Identity and power preservation follow from the two laws; no hidden rank, primitive-root or nondegeneracy hypothesis is inserted. Lean directly negates a necessary existence consequence of the literal composite-order source condition. [pcc:unverified]

All literal arithmetic source guards hold: gcd(6,5)=1,6 divides the actual curve count and actual−1 is a nonidentity sixth root. This root has exact order2; no primitive sixth root exists in F5. A generic commutative-monoid lemma proves a nonidentity root of prime order is primitive. This is an arithmetic guard comparison; the general primitive-root/prime-order-qualified positive Tate theorem and canonical divisor/Miller identification remain open here. No error in original Frey–Rück1994 is asserted; that primary remains unread. [pcc:unverified]

The main theorem is `CompositeTateDomainAudit.known_composite_tate_control_certificate` in [CompositeTateCertificate.lean](lean/CompositeTateCertificate.lean). Its complete738-character normalized header has binding `c0b96f82c9239660a8c2dd7e8d1f5eafad909f0cf6431d8a3a1cc3f49fda1371`; independent native processing reports1complete/0prefix/0unresolved. Source processing is procedural, without kernel parser/hash/compiler/physical-host refinement. [pcc:unverified]

| Module | Checked content |
|:--|:--|
| [CurveDomain](lean/CurveDomain.lean) | Actual curve/count6/pointorder3, multiplication6 kernel/image and both actual domain cardinalities6, explicit nonzero3-witnesses. |
| [UnitsQuotient](lean/UnitsQuotient.lean) | Actual sixth-power image, quotient cardinality2/exponent2, root order2, no primitive6root, generic prime-root arithmetic. |
| [PairingObstruction](lean/PairingObstruction.lean) | Faithful bilinearity definition, derived identity/power laws, both fixed nonzero radicals and direct literal-guard counterconsequence. |
| [CompositeTateCertificate](lean/CompositeTateCertificate.lean) | Closed complete known-control certificate. |

See [proof map](proof-map.json), [source plan](sources.json), [preflight](preflight.json), [source review](evidence/source-review.json) and [one-page report](reports/report.pdf). The full PDF text and diagram were read and visually inspected; all content fits. [pcc:unverified]

## Validation and retained corrections

Whole-project repository Lean gate53theorems/0failures/0trusted native; only propext/Classical.choice/Quot.sound. Final08 and all gate logs were completely searched: no warnings/errors. Four modules/seven formal-config+14artifactpins. Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679/defaults. [pcc:unverified]

Eight snapshots retain all six failures01/03–07 and successes02/08. Failures concerned finite-instance construction, multiplication-hom argument order, explicit kernel membership, primitive-power/order APIs and nested pointwise-Hom instance elaboration. The final bilinear structure has exactly the actual function and both multiplication laws, with identity/power laws proved. A failed API probe and all three pre-edit protocols/amendments remain. Error-recovery outputs were never accepted as proofs. [pcc:unverified]

All109provenancefiles/46objects roundtrip through16304-byte lossless deduplicated XZ plus five gate files. The old known-result README was read via readonly `git show` because its directory was absent from the sparse working tree; its exact text and hash are retained. The earlier actual-count proof pattern is credited, and that original source was byte-checked against baseline0e4f27e64a13462bf32dab7a732dd1f9e54cfaf9. Frozen sources were untouched. [pcc:unverified]

No holes/custom axioms/native proof evaluation/raised limits/new global elaboration flags/linter waivers/frozen edits/cryptographic Python/new native cryptographic experiment/paid compute/agent upload. Python handles provenance, source processing, metadata and report layout only. Local verification does not establish signed standing. [pcc:unverified]

```sh
cd research/composite_tate_domain_audit_20261115/lean
lake build
cd ../../..
bash tools/lean/proof-gate.sh research/composite_tate_domain_audit_20261115/lean
python3 research/composite_tate_domain_audit_20261115/restore_attempts.py /tmp/known-composite-tate-control
```

Galbraith chapter26 extendedcorrected version2.0, primary222784bytes/16pages/SHA c06cf2df9b8003e19694a4ebfe298a1557d61169f87ec01542258438ceac015e: printed574–577/PDF2–5 freshly fully text-read/visually inspected, not all16pages. Exact natural-n/coprime/divisor/proper-domain/nontrivial-root wording distinguished from prime-order/full-primitive-root scope. Current author listed errata23–71 were read; no chapter26entry in that bounded list, no global absence claim. KB MCP absent/wrappers127, prior/primary/official fallback. Initial fallback search was too narrow; broader repository search found the known counterexample and this limitation was acknowledged. Copyrighted primary files remain local. [pcc:unverified]

Exact local **PCC-20261115-e2a305** replay PASS at 2026-11-15T03:43:24Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Published as an auxiliary known-control reconstruction in existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743): localreceipt `161fe980a487e09f39c8d482101096b44f66238f`, remoteproof `de9824b26279a9bc62c79c0b2a4d28a6ffb0d011`, shared tree `34b2597174ee1908be0c72bf6b00a29ffd1a7a34`. All41paths/seven large-or-binary blobs match; nonforce expected-head update, completed anonymous fetch and fresh exact PR head/body119 confirmed. All602ledgerrows preserve the601-row prefix;45request local links resolve. [Publication receipt](publication.json) lies outside frozen proof/artifact pins; final metadata is separate. No new source coverage/gap closure/independent validator or signed standing. [pcc:unverified]
