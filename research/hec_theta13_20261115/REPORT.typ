#set page(paper:"a4",margin:17mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.5em)
#text(size:22pt,weight:"bold")[Normalized genus-3 theta sections]

Both explicit 3-kernels · Constructive correctness gate · 15 November 2026

*Both kernels now have symmetric normalized Weil sets and nonzero invariant theta sections.* All arithmetic stays in F(127¹³). Compatible square/cube conditions determine the normalization without root extraction. Every registered theta evaluation lies in F127 and is nonzero.

#image("THETA.svg",width:100%)

*Translation cocycle.* Use the sign convention fₚ=η₋ₚ from the certified root-free evaluator. The Weil divisors make C(P,Q)=fₚ(X)fQ(X+P)/f(P+Q)(X) a nonzero constant. Values at four regular public multiples agree in all 4,374 comparisons beyond the first point. All 1,458 within-kernel commutators are trivial, and all 39,366 associativity triples pass. This does not evaluate nontrivial pairings between the two complementary kernels.

*Normalization.* Put sₚ=f₋ₚ(X)/fₚ(−X), A₃=1/(C(P,P)C(P,−P)) and A₂=sₚ/C(P,−P). Then αₚ=A₃/A₂. Every scalar satisfies α³=A₃ and α²=A₂; all four points give the same normalization. Across both kernels, 5,832 pair checks give αₚαQ C(P,Q)=α(P+Q). All 216 symmetry checks and 54 Frobenius-scalar checks pass. The identity scalar is one.

*Nonzero sections.* With Fₚ=αₚfₚ and Θ=ΣFₚ, reindexing the kernel proves Fₚ(X)Θ(X+P)=Θ(X). All 216 registered action checks pass. This is invariance under the theta-group action, rather than ordinary translation invariance of Θ. The eight recorded values at n=2,3,4,5 are (2,15,37,82) for kernel zero and (20,18,4,29) for kernel one. All lie in F127; nonzero values certify neither section vanishes identically. Symmetry at −X also passes. These scalars are not performance measures or kernel-quality scores.

*Controls and exclusions.* The frozen ascending search rejects n=1 outside the generic evaluator chart and admits n=2 through 5 for both kernels, their negatives and all translates. Cap 32 is not reached. Multiplying one nonidentity normalization scalar by two causes 152 normality failures and 208 section-action failures. Original source/kernel/curve inputs remain unchanged. The checks share finite-field and Cantor arithmetic; they are not independent external validation.

*Custody.* Protocol 2d193a800a preceded source 932ead8b52db and checkpoint 20261115T142338Z-932ead8b52db. Locked offline Rust build is clean. The original execution exits successfully with empty stderr, no cap or rerun. Raw alpha vectors, theta values, attempts, hashes and counts are retained. Existing native CI does not automatically execute this standalone gate. No timing claim, cloud worker, closed-record or default change.

*Next dependency.* Quotient functions and equations, same-field/Serre testing, and divisor transport still need construction and complete cost accounting. These theta sections do not alone identify a quotient Jacobian over F127. The full mathematical 10× genus 2/3/4 objective remains active.

*Sources.* Tian, arXiv:2007.03172, Section 2.2 and Section 4; prior pinned root-free and torsion reports. Established theta-group normalization and descent construction. KB status, source gaps, formulas and reproduction commands are in PROTOCOL.md and RESULTS.md.
