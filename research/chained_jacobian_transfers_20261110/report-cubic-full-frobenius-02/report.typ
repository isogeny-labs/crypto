#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Full cubic Frobenius transport at223
#text("Eleven new named statements prove the coefficient embedding and full expression q-power identity. Both unchanged original Lean4.35/4.34.1 gates pass24 audited declarations/0 trusted, including12 old named statements and one old definition equation. DataPCC-20261116-d5a397 retains the whole source and all outcomes; claims remain unsigned. [pcc:unverified]")

== Canonical coefficients in the stated ring
#text("Assume a commutative ring R with characteristic223. The canonical map e(a)=(a.val:R), for a in Fin223, is injective and preserves addition, multiplication and every natural power. A kernel certificate over223 base residues gives e(a)²²³=e(a). [pcc:unverified]")
#text("For z³=3, the frozen01 generator identities give z²²³=183z and (z²)²²³=39z². Characteristic-aware normalization proves (u+v)²²³=u²²³+v²²³. Their combination transports the complete expression. [pcc:unverified]")

#image("full-expression.svg", width: 100%)
#text("The square commutes under exactly CommRing R, IsCharP R223 and z³=3. Define T(a,b,c)=(a,183b,39c). The earlier coefficient identity T³=id yields ((E²²³)²²³)²²³=E for every coefficient triple. No cubic population is enumerated. [pcc:unverified]")

== Fixed expression and the explicit hypothesis
#text("If E_z is injective, E_z(a,b,c)²²³=E_z(a,b,c) iff b=c=0. The forward proof turns equality of encoded expressions into T(a,b,c)=(a,b,c), then applies the frozen coefficient fixed-axis theorem. This module keeps encoding injectivity explicit; deriving it is the next additive proof obligation. [pcc:unverified]")

#pagebreak()
= Exact proof and failure custody
#table(columns: (42%,58%), inset: 6pt,
 [New statement group], [Scope checked],
 [Characteristic additivity], [Every u,v in the stated ring; original kernel proof.],
 [Base coefficients], [223 residues; canonical embedding injective and preserves +,×,powers.],
 [Full expression], [E_z(x)²²³=E_z(Tx), all coefficient triples.],
 [Three power steps], [Repeated q-power is identity on encoded expressions.],
 [Fixed expression], [Fixed iff b=c=0, with encode injectivity explicit.],
)
#text("Eleven original statement records are retained in full-cubic-frobenius-final-08/exact-eleven-new-statement-bindings.json. Five records bind whole statements; six truncate at named arguments and remain incomplete. A later additive03 supplies explicit typed corollaries. The full kernel audits check all actual declarations. The complete module retains frozen01 as an exact byte prefix. Final SHA2565198ffb8ef0945dd4754255ef2f37d420ca9ad7f4064d03f799c8ca593011edf. [pcc:unverified]")

== Failed attempts remain separate
#text("The first complete attempt reached120s; default/restricted/done additivity diagnostics reached20s. A direct interactive ring diagnostic emitted No goals and downstream sorryAx and was rejected. Structured try-ring then done compiled. The first dual attempt passed Root24/0 but failed Consumer because Fin.val_pow is absent in4.34. All complete sources/logs remain, including the rejected downstream output. [pcc:unverified]")
#text("Induction through shared semiring power laws replaced only the unavailable-interface proof. Final unchanged gates both pass24/0, at one worker/120s each/original static proof memory defaults. Only propext,Classical.choice,Quot.sound occur. No new axiom, hole, native decision or failed module is accepted. [pcc:unverified]")

== Next structural bridge
#text("Use the three encoded Frobenius equations to separate the coefficients, then cancel z using z³=3 and the invertibility of3 in characteristic223. That is a later additive task; the current fixed-point statement retains its exact hypothesis. Field construction, native UInt64/powmod/compiler/counters, actual Jacobian descent and admitted matched costs remain separate. [pcc:unverified]")
#text("Sources: frozen01, whole original common.rs832fd288, actual Lean power/characteristic/Fin and interactive ring interfaces; primary reviewb717b7 and complete later failure records. KB unavailable, wrappers127; no novelty/absence inference. Source-pinned CI captures actual head/workflow/original tools and every partial result. No native population, benchmark or paid launch. [pcc:unverified]")
