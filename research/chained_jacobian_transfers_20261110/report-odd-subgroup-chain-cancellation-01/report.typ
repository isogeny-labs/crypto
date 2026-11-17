#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Cleared chains on stated odd subgroups
#text("An explicit annihilator and Bézout certificate cancel the cleared two-hop identity at the specified points. Both unchanged original Lean4.35/4.34.1 gates pass9 audited theorems/0 trusted: seven new statements plus two unchanged original statements. Dataa8584b retains the exact source, both complete audits and seven original PCC declaration bindings. Claims remain unsigned. [pcc:unverified]")

== Recovery supplies the cancellation
$ u d + v r = 1, quad [r] X = [r] Y = 0 $
$ [u][d]X = [u d + v r]X = X $
$ [d]X = [d]Y quad ==> quad X = Y $
#text("The proof concerns the two stated annihilated points. The ambient integer module may contain d-torsion. For d=4, a supplied odd exponent and the corresponding integer certificate suffice. Zero-preserving integer-linear maps transport a source annihilator to their outputs, giving a second theorem about equality of cleared maps on that source. Actual homomorphism and subgroup properties are inputs. [pcc:unverified]")

== Apply the existing two-hop relation
#text("Retain the original two cleared map identities, middle norm/pullback relation and linearity hypotheses. The old theorem gives [16]g(f(x))=[4]k2(b(a(h0(x)))). Set X=[4]g(f(x)) and Y=k2(b(a(h0(x)))); these obey [4]X=[4]Y. The new factorization assumes r annihilates g(f(x)) and Y, and uses4u+vr=1 to recover X=Y. [pcc:unverified]")
#image("cleared-cancellation.svg", width: 100%)
#text("Figure: the old cleared identity is the right-hand equality. The stated annihilators and public certificate justify the left-hand equality. The new proof uses no injectivity assumption for multiplication by4 on every ambient point. The earlier written cancellation in Hom remains a separate geometric bridge. [pcc:unverified]")

#pagebreak()
= Exact scope and original proof custody
== Public quarter certificates
#table(columns: (18%,18%,64%), inset: 7pt,
 [Annihilator r], [Inverse u], [Certificate used by the kernel],
 [109], [82], [4 × 82 − 3 × 109 = 1],
 [113], [85], [4 × 85 − 3 × 113 = 1],
 [12317], [9238], [4 × 9238 − 3 × 12317 = 1],
)
#text("Each specialization recovers [4]x only under its stated [r]x=0 hypothesis. The first two match the existing fixed rational subgroup quarters; the third applies to any explicitly12317-annihilated point. These public scalar operations need no target logarithm. They are exact algebraic certificates, not timings or calibrated costs. [pcc:unverified]")

== What the module proves
#table(columns: (45%,55%), inset: 6pt,
 [New named theorem], [Explicit scope],
 [Point recovery and cancellation], [Annihilator and integer Bézout certificate.],
 [Equality of cleared maps], [Source exponent, zero preservation and integer linearity.],
 [Two-hop outer factorization], [Original cleared identities/linearity and two output annihilators.],
 [Three public quarters], [The stated109,113,12317 annihilators.],
)
#text("Original source: ffd54023b7b5f1b26880d26c9feba6ae4c0de0cbe8d86c9f5e03fd9bb17ba002. The entire original ChainCompression file is the byte-exact prefix. Both original gates use one compiler worker,120s each and their unchanged static proof memory defaults. Audits allow only propext,Classical.choice,Quot.sound; no new axiom, sorry or trusted native proof occurs. [pcc:unverified]")
#text("Seven exact original PCC bindings: e6ad94/869df9/1e7919/725549/71025c/722066/65df52. Whole source, original commands/logs/auditors and normalized statements: evidence/odd-subgroup-chain-cancellation-final-02. Dedicated source-pinned CI reproduces both original gates. [pcc:unverified]")

== Remaining geometric and runtime inputs
#text("Actual Jacobians, geometric norm/pullback identities, subgroup cardinalities and the global Hom bridge remain separate. An ambient2-torsion point is killed by4; the new result is scoped to the explicitly annihilated outputs. Matched construction/transfer/solver/recovery costs and the full historical LP execution require their original independent admission and evidence. [pcc:unverified]")
#text("Sources: original ChainCompression, ChainTransfer.injective_with_recovery, ReverseRecovery, written forward/reverse chain derivations, and standard Lean IntModule scalar identities. Primary-reviewd13f12 retains source digests. KB status/exact/negative/theorem queries127; no novelty or absence inference. Next instantiate the scoped module result with actual geometric/program bridges and collect its own current proof CI. [pcc:unverified]")
