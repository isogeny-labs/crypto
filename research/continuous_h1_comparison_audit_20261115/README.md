# Actual continuous H1 comparison and finite Galois pairing

Lean identifies Mathlib's actual `continuousCohomology 1` with all continuous crossed cocycles modulo all principal cocycles. For every finite field and finite Abelian coefficient group with a continuous Galois action, it derives the Frobenius coinvariants. Under exponent dividing #K−1, it proves both perfectness maps and both trivial radicals for the finite-module pairing displayed in Bruin2011 Lemma2.1. The exact formula is proved on actual categorical homology-projection representatives. A characteristic-2 family refutes deleting the annihilation hypothesis on these same canonical domains. Signed standing remains unverified. [pcc:unverified]

The general comparison uses an actual `TopRep k G` and a natural joint-action continuity hypothesis. The differential formulas are proved from the actual homogeneous complex: [pcc:unverified]

```text
d0(f)(x,y) = f(y) − f(x)
d1(F)(x,y,z) = F(y,z) − F(x,z) + F(x,y).
```

The map from the actual d1 kernel to all continuous crossed cocycles is `c(g)=F(1,g)`. Its inverse is the actual continuous cochain `F(x,y)=ρ(x)c(x⁻¹y)`, constructed by continuous currying. Both inverse laws are proved. The actual d0 image maps onto the entire principal-cocycle image `g↦ρ(g)a−a`. Actual categorical kernel/cokernel universal properties and proved index isomorphisms identify the existing `continuousCohomology 1` object with that quotient. No cohomology value, comparison, duality or nondegeneracy conclusion is supplied as a premise. [pcc:unverified]

The representative construction uses the actual categorical `homologyπ`, together with the proved kernel/cycles and index isomorphisms. Its image under the comparison is proved equal to the expected crossed-cocycle quotient representative. This checks the representative formula as well as the abstract equivalence. [pcc:unverified]

For the Galois application, the code constructs the actual coefficient `TopRep` on `Additive M`. It derives joint continuity from the original continuous homomorphism into the discrete automorphism group. It translates the entire crossed/principal quotient to the previously proved multiplicative quotient of all continuous Galois cocycles by all principals, and then to the actual finite Frobenius coinvariants. K and M have an explicit common ambient universe; the closed main covers all ordinary finite fields and modules. [pcc:unverified]

Composing with the credited finite-field character duality constructs both perfectness isomorphisms for all actual Galois-equivariant homomorphisms from M to algebraic-closure units and the actual canonical continuous H1 group. For every actual homogeneous kernel representative F, the pairing value maps to `χ(F(1,Frob))` in algebraic-closure units. Multiplicative type tags express the same Abelian cohomology group in the source's notation. [pcc:unverified]

For every finite field of characteristic 2, take M=Z/2 with trivial action. Actual continuous H1 has a nonzero class, while every relevant algebraic-closure unit character is trivial. Lean proves that no bilinear homomorphism on these same canonical domains can be right-nondegenerate after deleting annihilation. The qualified source lemma survives. [pcc:unverified]

The closed main is `ContinuousH1ComparisonAudit.continuous_h1_comparison_certificate` in [ContinuousH1Certificate.lean](lean/ContinuousH1Certificate.lean). See the [proof map](proof-map.json), [source plan](sources.json), [source review](evidence/source-review.json), [one-page report](reports/report.pdf) and [diagram](reports/continuous-h1.svg). Exact local **PCC-20261115-de23d0** replay PASS at 2026-11-15T01:19:32Z; signed standing remains unverified. See [verification receipt](verification.json). [pcc:unverified]

Eleven prior core files are copied byte-identical, credited to [PCC-20261115-ed6495](../procyclic_galois_cocycle_audit_20261115/README.md), [PCC-20261115-4711ad](../galois_character_pairing_audit_20261115/README.md), [PCC-20261114-cdb4f1](../finite_tate_duality_audit_20261114/README.md) and [PowerTargets](../char_p_pairing_domain_audit_20261114/lean/PowerTargets.lean). Six new proof modules supply this comparison and its application. Originals and copies remain unchanged. Two inherited unused-section-variable advisories are retained; the prior packet's incorrect advisory-free wording is expressly corrected by its [outside-pin correction receipt](../galois_character_pairing_audit_20261115/corrections.json). [pcc:unverified]

Sources checked before approach: Bruin, [*The Tate pairing for Abelian varieties over finite fields*](https://www.numdam.org/article/JTNB_2011__23_2_323_0.pdf), JTNB23(2011)323–328, DOI10.5802/jtnb.764; all six article pages previously read/seven PDF pages inspected, selected325/PDF4 definitions, Eq2.1 and Lemma2.1 rechecked. Actual pinned Mathlib continuous-cohomology, coinduction, TopRep, categorical homology/kernel/cokernel/index and quotient APIs were read. KB MCP was absent; fresh status/two mechanism-negative queries/theorem wrapper and Conductor returned127, so prior/primary/official fallback was used. Copyright primary files stay local. No failed-search absence or novelty claim. [pcc:unverified]

The earlier separate continuous-H1 comparison gap is discharged at the stated coefficient representation and domains. Isogeny kernels, Lang, Cartier group schemes, Jacobians, divisors and complete Tate/MOV/Frey–Rück conditions remain open. Original Frey–Rück1994, Serre, Lang and Mumford primaries remain unread. There is no canonical corpus promotion or claim that all fourteen items are finished. [pcc:unverified]

Reproduce with Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679:

```sh
cd research/continuous_h1_comparison_audit_20261115/lean
lake build
```

From repository root run `tools/lean/proof-gate.sh research/continuous_h1_comparison_audit_20261115/lean`. Restore every retained source/config/log/exit/protocol byte using `python restore_attempts.py /tmp/continuous-h1-attempts`. The full1405-character written declaration has SHA65928939b66df9e96c426f625abbea3493ad127ec27c3bae6ed4cc38c7d8848d; independent native processing reports1complete/0prefix/0unresolved. That processing is procedural, without kernel parser/hash/physical-host refinement. No fresh independent lean4checker is available. No holes, custom axioms, native proof evaluation, raised limits, new global flags, linter waivers, frozen edits, cryptographic Python, new native cryptographic experiment, paid compute or agent upload. [pcc:unverified]

The whole-project gate passes100/0failures/0trusted native, including reused proofs; only propext/Classical.choice/Quot.sound occur. All189 retained files/61unique objects roundtrip through24844-byte losslessXZ, plus five gate files. Twelve snapshots retain five failed formal states and the initial setup working-directory failure. Final12 has exactly the two inherited advisories and no new-module advisories. The one-page47307-byte PDF was fully visually inspected; all text lies within bounds. [pcc:unverified]

Published in existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743) as `2e412ae5e7d1ac785d3e5707a5300a3ec7c16f6b`, identical to local receipt tree `ef54d6815682418a119ec2385a872795b3e145e2`. All56paths/6large-or-binary blob IDs match; anonymous fetch completed and fresh PRhead/bodyv115 confirmed. The nonempty597-row ledger prefix is preserved through598entries. All8executed CI checks passed at01:15UTC on preceding Galois-character headfd8228; sourceconnection/publish/sign skipped. No later canonical-H1-head CI pass or signed promotion is asserted. The parent immutable-ledger fork remains needs-human; no sidepick/rechain/rebase/force/retarget. Publication metadata is outside frozen pinsets. [pcc:unverified]
