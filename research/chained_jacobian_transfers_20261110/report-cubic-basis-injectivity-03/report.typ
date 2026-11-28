#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Derive cubic basis injectivity at223
#text("The encoding is injective under CommRing R, characteristic223 and z³=3. The fixed-expression theorem now needs no assumed basis injectivity. Both unchanged original Lean4.35/4.34.1 gates pass37 audited declarations/0 trusted. Twelve new named statements have complete original PCC bindings. Data97a994 retains full proof evidence; claims remain unsigned. [pcc:unverified]")

== Three Frobenius images separate the coefficients
#text("Let E=e(a)+e(b)z+e(c)z², with the canonical Fin223 embedding e. Put A=e(a), B=e(b)z, C=e(c)z². The frozen full-expression identity gives y0=A+B+C, y1=A+183B+39C, y2=A+39B+183C. The inverse combinations below explain the separation lemma. [pcc:unverified]")
#table(columns: (21%,79%), inset: 6pt,
 [Coefficient], [Recovered from the three images],
 [A], [149(y0+y1+y2)],
 [B], [149(y0+39y1+183y2)],
 [C], [149(y0+183y1+39y2)],
)
#text("These identities use 1+183+39=0, 183²=39, 39²=183, 183·39=1 and 3·149=1 in characteristic223. Formal coefficient separation applies to every stated ring; no field or domain hypothesis is supplied. [pcc:unverified]")

#image("separate-and-cancel.svg", width: 100%)
#text("Equality of encoded expressions remains equal after q-power twice. Separation gives equality of A,B,C; the relation z³=3 supplies inverse149z² for z. Cancel z and z², then use the injective canonical base embedding to recover a,b,c. The resulting fixed-expression equivalence is E²²³=E iff b=c=0. No extra encoding-injectivity hypothesis remains. [pcc:unverified]")

#pagebreak()
= Complete bindings and a structural control
#text("Six transparent typed corollaries state the canonical embedding results without named arguments inside the declaration type. The unchanged original PCC extractor binds their complete statements. The six old truncated records and correctiona1a45d remain immutable and incomplete; no verifier/keyring/policy change is made. [pcc:unverified]")
#table(columns: (43%,57%), inset: 6pt,
 [New statement group], [Exact role],
 [Six typed embedding corollaries], [Injectivity, addition, multiplication, powers, q-fixed coefficients and action constants.],
 [Three structural lemmas], [Separate three coefficient equations; cancel z and z² under z³=3.],
 [Encoding injectivity], [Derived for every stated ring and generator.],
 [Fixed expression], [Fixed iff b=c=0, without assumed encoding injectivity.],
 [Zero-generator control], [At z=0, distinct(0,1,0)/(0,0,0) collide over Fin223.],
)
#text("The zero-generator control records a concrete failure when the cubic relation is removed. All twelve complete declaration bindings are in cubic-basis-injectivity-final-03/exact-twelve-complete-statement-bindings.json. The full source preserves the exact successful02 prefix; its SHA256 is 9bd383b7e9d3ad241fa92e24d3b853cdba7011bf36d0cbe66fec89a7999e5ead. [pcc:unverified]")

== Original audits and retained scope
#text("Both original gates audit37 declarations:12 new named,23 earlier named and two generated equation theorems. The new simp use materializes encode.eq_1. Each original gate uses one worker/120s/original static proof memory defaults; only propext,Classical.choice,Quot.sound occur. No new axiom/hole/native decision or223³ enumeration is accepted. Isolated structural lemmas also passed both original versions under20s. [pcc:unverified]")
#text("Actual extension-field construction, native UInt64/powmod/compiler/counter correspondence, geometric Jacobian maps and admitted matched computational costs remain separate. Earlier02 tactic/toolchain/induction/render failures and limited bindings stay preserved. [pcc:unverified]")
#text("Sources: exact frozen01/02 and original common.rs832fd288, official actual Lean characteristic/power/cast interfaces. Primary review332d4d includes the competing hypothesis and control. KB unavailable, status/two queries/theorem wrappers127; conductor127. No novelty or absence inference. Exact-source CI retains actual head/workflow/original tools and all partial outcomes. No native population, benchmark or paid launch. [pcc:unverified]")
