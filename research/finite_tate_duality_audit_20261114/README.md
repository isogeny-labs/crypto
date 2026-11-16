# Finite character duality behind the Tate pairing

Lean proves the finite character/coinvariant algebraic core used in
Bruin's Tate-pairing proof. For every finite field K of size q, every finite
abelian group G killed by q−1, and every automorphism σ, invariant
K*-valued characters pair perfectly with the actual quotient
`G / range(σ−1)`. Both evaluation maps are constructed group isomorphisms,
with their evaluation formulas and cardinalities proved. No primitive
root, basis, rank, supplied duality isomorphism or nondegeneracy premise
is assumed. [pcc:unverified]

This is a formalization of the **algebraic finite character/coinvariant
core**, not a full formalization of Galois cohomology or the canonical
Tate/Frey–Rück pairing. The continuous H1 identification, Lang vanishing,
isogeny divisibility, Cartier-dual identification, principal polarization
and canonical divisor comparison remain separate obligations.
[pcc:unverified]

The construction uses actual objects:

- `difference σ` is the group endomorphism x↦σ(x)/x, in multiplicative
  notation for σ−1.
- `Coinvariants σ` is the quotient by its actual image.
- `invariantCharacters σ K` is the actual subgroup of all homomorphisms
  G→K* satisfying χ(σx)=χ(x).
- `characterDescent` is the explicit lift/pullback group isomorphism from
  invariant characters to all characters of the quotient.
- `evaluate` is evaluation on that quotient; `rightEvaluationIso` is the
  reverse duality map, with the evaluation formula proved. [pcc:unverified]

The finite field's cyclic unit group supplies a primitive (q−1)th root.
The exact annihilation condition gives enough roots for the quotient's
exponent. Mathlib's existing finite abelian character/biduality theorems
are credited and reused to construct the right isomorphism; the left
isomorphism follows directly from character descent. Both radicals are
trivial and the two actual groups have equal cardinality.
[pcc:unverified]

Actual elliptic-curve controls make the hypotheses checkable:

- Over every finite field of odd characteristic, the actual rational
  two-torsion of every general model is killed by q−1. The character
  duality applies to every automorphism of that actual group.
- Over F3, the nonsingular curve y²=x³−x has the actual point (0,0) of
  exact order two. Its actual rational two-torsion has cardinality four,
  proved through the actual cubic-root equivalence. With identity action,
  both invariant-character group and coinvariant quotient have cardinality
  four, the field-unit target has cardinality two, and evaluation is
  perfect. This is a nontrivial positive control.
- Over every finite characteristic-two field, the credited actual curve
  y²+xy=x³+1 supplies an order-two point. Its actual two-torsion with
  identity action has coinvariant cardinality two but invariant-character
  cardinality one. An explicit nonidentity quotient class is killed by
  every character. Lean proves the source annihilation condition fails
  and directly negates the universal right-nondegeneracy statement after
  deleting that condition. This does not refute the qualified theorem. [pcc:unverified]

These are actual curve groups and homomorphism/quotient types, not a
substitute group table or an assumed pairing implementation.
[pcc:unverified]

The primary source is Peter Bruin, [*The Tate pairing for Abelian varieties
over finite fields*](https://www.numdam.org/article/JTNB_2011__23_2_323_0.pdf),
Journal de théorie des nombres de Bordeaux23(2011),323–328,
DOI10.5802/jtnb.764. All six article pages, their proofs and references
were read; the seven-page PDF including its cover was rendered and
visually inspected in full. Lemma2.1's finite/annihilation hypotheses,
section3's separability/Lang/Cartier bridges and Theorem4.1's divisor
comparison are distinguished. The cited Serre/Lang/Mumford/Howe works
are not marked read merely because they occur in the references.
[pcc:unverified]

The **original Frey–Rück1994 paper remains unread**. Additional exact-title
and DOI web searches found author DISCLOG links, but the author HTTP/HTTPS
routes failed with403/503, and archive CDX routes failed with503. Prior
publisher/catalog route failures remain recorded. Bruin2011 is a separate
primary source, not a replacement reading of FR1994. No global absence,
novelty, full-corpus promotion or full original fourteen-item closure is
claimed. KB and Conductor were unavailable; bounded primary/prior/official
fallback preceded selecting this approach. [pcc:unverified]

Main declaration:
`FiniteTateDualityAudit.finite_character_duality_certificate`, in
[FiniteTateDualityCertificate.lean](lean/FiniteTateDualityCertificate.lean).
The [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and
[visual report](reports/report.pdf) retain precise scope.
Exact local **PCC-20261114-cdb4f1** replay PASS at 2026-11-14T08:52:45Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/finite_tate_duality_audit_20261114/lean
python3 research/finite_tate_duality_audit_20261114/restore_attempts.py /tmp/finite-tate-duality-builds
```

Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned.
All actual failed source/configuration/log/exit states are retained.
Standard foundations/default limits only. No holes, custom axioms, native
evaluation, raised limits, new global elaboration flags, linter waivers,
new native experiment, paid compute or agent storage upload.
[pcc:unverified]
