#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Exact cubic square values in 64-bit words
#text("The original UInt64 coefficient-square expression has exactly the natural output values of the retained modular model at q223,c3 and canonical inputs. Both unchanged original Lean4.35/4.34.1 gates pass 27 audited declarations/0 trusted. Ten new exact declaration bindings and complete source/attempts are retained in data 2aa7f3. All claims remain unsigned. [pcc:unverified]")

== Match the original expression tree
#text("The original Kf.mul uses three u64 coefficients, five temporaries and the following reductions when the operands coincide. The Lean model keeps each product, left-associated sum and remainder, using UInt64 operators. The variable c below is the third input coefficient; the fixed reduction constant is3. [pcc:unverified]")
$ t_0 = a a mod 223 quad t_1 = (a b+b a) mod 223 $
$ t_2 = (a c+b b+c a) mod 223 $
$ t_3 = (b c+c b) mod 223 quad t_4 = c c mod 223 $
$ w = ((t_0+3t_3) mod 223, (t_1+3t_4) mod 223, t_2) $
#text("Standard UInt64 lemmas turn word addition and multiplication into natural operations modulo2⁶⁴, and word remainder into natural remainder. Canonical Fin223 inputs embed exactly. Explicit bounds remove each possible wrap before comparing the outputs. [pcc:unverified]")

#table(columns: (58%,42%), inset: 6pt,
 [Original arithmetic expression], [Canonical upper bound],
 [One coefficient product], [49,284],
 [Sum of two products], [98,568],
 [Left-associated sum of three products], [147,852],
 [Reduced temporary], [222],
 [Three times a reduced temporary], [666],
 [Final temporary plus scaled remainder], [888],
)
#text("All bounds are below 2⁶⁴. These are exact arithmetic bounds, not measured timings or calibrated costs. Product, pair, triple, scale and final-sum lemmas explicitly eliminate word wraps. This does not enumerate223 cubed triples. [pcc:unverified]")

#pagebreak()
= Raw values and the retained obstruction
#text("The coordinate theorem first identifies decoded word coefficients with the earlier originalSquare223 natural/modular model. A separate theorem proves each raw word output is below 223 for every UInt64 input, directly from its final remainder. The exact-value theorem therefore identifies all three raw natural word values, rather than only their reductions under decoding. [pcc:unverified]")
$ ("toNat"(w_0), "toNat"(w_1), "toNat"(w_2)) = ("val"(n_0), "val"(n_1), "val"(n_2)) $
#text("The exact old cubic-norm proof is retained byte-for-byte as a prefix of the additive standalone source. Its determinant/square/unit descent and canonical base nonroot calculation already exclude natural/model squares(12,0,0) and(24,0,0). Transport gives both decoded and raw UInt64 exclusions. Allowed positive controls retain wordSquare223(2,0,0)=(4,0,0), both raw and decoded. [pcc:unverified]")

#table(columns: (38%,62%), inset: 6pt,
 [Requirement], [Evidence or remaining dependency],
 [Canonical coefficient arithmetic], [Exact UInt64 expression, no-wrap bounds and raw-value correspondence.],
 [Both original constants], [Raw/decoded12 and24 exclusions; allowed raw/decoded square4 controls.],
 [Independent original gates], [Identical final source,27 audited declarations/0 trusted in each original gate; ten new bindings.],
 [Whole Rust function/executable], [Counter state, constructor and compiler/native-binary correspondence remain separate.],
 [Geometric transfer objects], [Actual field/Jacobian and global polarized-model/program bridges remain separate.],
)
#text("Source SHA2565743a021f3767e94d5e37617958596a75ec603a09088184e3218d3e40430febe. Exact old prefix18384e633660a419; full original common.rs832fd28810909194. Bindings and whole proof logs: evidence/cubic-word-final-raw-value-proof-03. Only propext,Classical.choice,Quot.sound occur. No trusted native_decide/new axiom or signed execution standing. [pcc:unverified]")

== Preserve the unsuccessful attempt
#text("The first omega attempt leaves nested2⁶⁴/223 coordinate goals unresolved; its erroneous downstream sorryAx declarations are retained, never accepted. Explicit bounded remainder elimination fixes the proof without changing its statement/model. The separately preserved decoded-only intermediate passes both gates22/0. The final raw-output source passes27/0. [pcc:unverified]")
#text("Reproduce with the unchanged Root4.35 and Consumer4.34.1 gates, source copies inside each actual repository, one compiler worker,120s per gate and original static proof memory defaults. Sources: original Kf.mul, immutable cubic norm component and standard Lean UInt64 operator lemmas. KB queries127/locked vault imply no novelty. Next: source-pinned CI and concrete geometric/program bridges alongside runtime preparation. [pcc:unverified]")
