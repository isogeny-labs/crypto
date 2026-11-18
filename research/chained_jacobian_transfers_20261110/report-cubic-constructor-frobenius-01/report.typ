#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Constructor arithmetic and Frobenius at223
#text("The exact constructor predicates, cubic base nonroot certificate, two basis-generator identities and stated coefficient fixed subspace are kernel checked. Both unchanged original Lean4.35/4.34.1 gates pass13 audited declarations/0 trusted, including12 new named theorems. Datacf6114 retains whole source/logs and twelve exact original PCC bindings. Claims remain unsigned. [pcc:unverified]")

== Match the constructor's arithmetic predicates
#table(columns: (42%,58%), inset: 7pt,
 [Original expression], [Exact arithmetic at q223],
 [Guard; exponent], [223 mod12=7; (223−1)/3=74],
 [First candidate2], [2⁷⁴ mod223=1: skipped],
 [Next candidate3], [3⁷⁴ mod223=183≠1: accepted],
 [Coefficients], [w=183; w² mod223=39],
 [Natural cubic size], [223³=11,089,567],
)
#text("The two-candidate certificate identifies the first accepted mathematical predicate starting at2. It does not prove Rust powmod/loop/compiler semantics. A separate theorem checks all223 base residues have no cubic root of3. It does not enumerate223³ coefficient triples or construct an extension field/irreducibility object. [pcc:unverified]")

== Explicit characteristic and basis hypotheses
#text("For a commutative ring with stated characteristic223 and z³=3, standard power grouping and the computed coefficient give the following exact identities. The generic coefficient equality is proved by characteristic-aware ring normalization. [pcc:unverified]")
$ z^223 = (z^3)^74 z = 3^74 z = 183 z $
$ (z^2)^223 = (z^223)^2 = 39 z^2 $
#text("The coefficients183 and39 are each distinct from1 and cube to1 in Fin223. This supplies the stated diagonal coefficient action shown on the next page. Actual coefficient embedding and full q-power/field correspondence remain separate. [pcc:unverified]")

#pagebreak()
= Fixed coefficient subspace and evidence
#image("coefficient-action.svg", width: 100%)
#text("Figure: T(a,b,c)=(a,183b,39c) over Fin223. The module proves T³=id and T(a,b,c)=(a,b,c) iff b=c=0. These are exact quantified statements about this declared coefficient operator; field and native function correspondence are not supplied by the diagram. [pcc:unverified]")

== Original proof custody
#text("Source SHA256e81532c9df6d5c416f26693ed7849d70e512d5225558adbab5147266f1e59738. Both unchanged original gates use one worker,120s each and original static proof memory defaults. Only propext,Classical.choice,Quot.sound occur; no new axiom/sorry/trusted native proof is accepted. Evidence: cubic-constructor-frobenius-final-04, including exact-twelve-new-statement-bindings.json. [pcc:unverified]")
#text("The first two cast-rewrite attempts7762a0/b90bcb remain complete with their sources, diagnostics and erroneous downstream sorryAx output. Neither is accepted. Final characteristic-aware normalization proves the same statement; direct compile and both complete original audits pass13/0. Existing cubic square/nonroot and odd-subgroup components are unchanged. [pcc:unverified]")

== Dependencies for geometric descent
#table(columns: (43%,57%), inset: 6pt,
 [Completed component], [Remaining bridge],
 [Constructor arithmetic and no base cubic root], [Constructed extension field/irreducibility object and Rust powmod/compiler semantics.],
 [Two generator q-power identities], [Actual coefficient embedding and full q-power action.],
 [Diagonal action period/fixed base axis], [Whole native function with word arithmetic/counters and actual Jacobian descent.],
 [Static original proof audits], [Own current source-pinned CI and separately admitted matched cost measurement.],
)
#text("Sources: exact original common.rs832fd288 constructor/powmod/Frobenius, original cubic norm/word scope, standard Lean power/characteristic/Fin ring interfaces. Review644f82; KB exact/negative/theorem wrappers127, no novelty or absence inference. Source-pinned CI preserves actual head/workflow/original tools and every partial outcome. No native population, timing result or paid build/job was launched. [pcc:unverified]")
