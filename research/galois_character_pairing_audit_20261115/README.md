# Perfect pairing for actual Galois characters and continuous cocycle classes

For every finite field K of order q and finite Abelian M with a continuous Galois action, Lean proves the finite-module pairing displayed in Bruin2011 Lemma2.1 when the exponent of M divides q−1. Both perfectness isomorphisms, the exact formula on every cocycle representative, both bilinear laws, both trivial radicals and equal cardinality are proved. A characteristic-2 family refutes the statement obtained by deleting the annihilation hypothesis. Signed standing remains unverified. [pcc:unverified]

The first factor is the group of **all actual Galois-equivariant homomorphisms from M to the units of AlgebraicClosure K**. Equivariance is quantified over every element of the actual K-automorphism group with its Krull topology. A unit killed by q−1 is fixed by actual q-Frobenius and therefore descends to K. This derives unique descent for every character. Actual Frobenius density then extends invariance from Frobenius to every Galois element. Neither descent nor full equivariance is assumed as a conclusion-shaped premise. [pcc:unverified]

The second factor is **all continuous crossed cocycles modulo the complete image of all principal cocycles**. Credited earlier proofs identify this standard quotient with actual Frobenius coinvariants. Combining it with finite-field character descent and biduality constructs both multiplicative equivalences to the other factor's K-unit character group. The pairing satisfies the source formula in actual algebraic-closure units:

```text
baseMap(pair(χ,[c])) = χ(c(Frob)).
```

The value is proved to come from K. Both nondegeneracy statements follow from the constructed equivalences. This proves the qualified finite-module displayed pairing for these standard continuous-cocycle class definitions. [pcc:unverified]

For every finite K of characteristic 2, take M=Multiplicative(ZMod 2) with trivial action. Every unit of the actual algebraic closure whose square is 1 equals 1, so every actual Galois character is trivial. The proved class/coinvariant equivalence supplies an actual nonzero continuous-cocycle class. Lean therefore proves that **no bilinear homomorphism on these same domains can be right-nondegenerate**. The deleted annihilation condition is itself proved false. This is a hypothesis-deletion counterexample; the qualified source lemma survives. [pcc:unverified]

The closed theorem is `GaloisCharacterPairingAudit.galois_character_pairing_certificate` in [GaloisPairingCertificate.lean](lean/GaloisPairingCertificate.lean). See the [proof map](proof-map.json), [source plan](sources.json), [source review and reuse provenance](evidence/source-review.json), [one-page report](reports/report.pdf) and [diagram](reports/galois-pairing.svg). Exact local **PCC-20261115-4711ad** replay PASS at 2026-11-15T00:52:09Z; signed standing remains unverified. See [verification receipt](verification.json). [pcc:unverified]

Seven prior core files are copied byte-identical from the [finite continuous-cocycle packet](../procyclic_galois_cocycle_audit_20261115/README.md), PCC-20261115-ed6495, and the [finite-duality packet](../finite_tate_duality_audit_20261114/README.md), PCC-20261114-cdb4f1. The characteristic-2 unit-root argument credits [PowerTargets.lean](../char_p_pairing_domain_audit_20261114/lean/PowerTargets.lean). Originals and copies remain unchanged. The project has five new proof modules and seven reused proof modules; the gate count includes both. No novelty claim. [pcc:unverified]

Sources checked before approach: Peter Bruin, [*The Tate pairing for Abelian varieties over finite fields*](https://www.numdam.org/article/JTNB_2011__23_2_323_0.pdf), JTNB 23(2011)323–328, DOI10.5802/jtnb.764; primary649044bytes, SHA528b63797f936e094861c98a65da5eb1615dc46747960ba3444516197fbf49a7. All six article pages were previously read and all seven PDF pages inspected. Current selected printed325/PDF4 Galois-Cartier definition, Eq2.1 and Lemma2.1 were rechecked. Prior proofs and pinned official Mathlib APIs were read. KB MCP was absent; wrapper status/mechanism/negative/theorem attempts and Conductor returned127, so prior/primary/official fallback was used. Copyright primary PDFs/text/images stay local. [pcc:unverified]

Comparison with Mathlib's separate continuous homogeneous-complex H1 object remains open. Isogeny kernels, Lang, Cartier group schemes, Jacobians, divisors and the complete Tate/MOV/Frey–Rück conditions are also open. The original Frey–Rück1994, Serre, Lang and Mumford primaries are not marked read. No canonical corpus promotion or completion of all fourteen items is claimed. [pcc:unverified]

Reproduce with Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679:

```sh
cd research/galois_character_pairing_audit_20261115/lean
lake build
```

From repository root run `tools/lean/proof-gate.sh research/galois_character_pairing_audit_20261115/lean`. The whole gate passes73/0failures/0trusted native, with only propext/Classical.choice/Quot.sound. The complete1184-character written declaration has SHA117deabc6ffa81828e499acbcc42d2c0a96947d2d3e5605128590db25ba4aee7; independent native processing reports1complete/0prefix/0unresolved. That source processing is procedural, without kernel parser/hash/physical-host refinement. No fresh independent lean4checker is available. [pcc:unverified]

Restore all106 retained source/config/log/exit/protocol bytes using `python restore_attempts.py /tmp/galois-pairing-attempts`;41unique objects roundtrip through18280-byte losslessXZ. All six snapshots and four failed states are preserved, plus five gate files. Final06 passes with two unused-section-variable advisories; see the [report correction](corrections.json). The frozen PDF/preflight retain the earlier incorrect “no advisories” wording, corrected by this receipt. The one-page46919-byte PDF was fully visually inspected with all text within bounds. No holes, custom axioms, native proof evaluation, raised limits, new global flags, linter waivers, frozen edits, cryptographic Python, new native cryptographic experiment, paid compute or agent upload. [pcc:unverified]

Published in existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743) as `86c4f883202269b276b371ba6ab5edb20a4f4da1`, identical to local receipt tree `9377e952aeb48ac32ed233d6fdd03c2620282afd`; all49paths/6large-or-binary blobs checked, anonymous fetch completed and fresh PRhead/bodyv114 confirmed. The nonempty596-row ledger prefix is preserved through597entries. Last00:15UTC CI scan concerned the preceding Diem head; no current-head CI or signed promotion is asserted. The parent immutable-ledger fork remains needs-human; no rechain/rebase/force/retarget. Publication metadata is outside frozen pinsets. [pcc:unverified]
