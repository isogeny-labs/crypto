#set page(paper:"a4",margin:17mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.5em)
#text(size:22pt,weight:"bold")[Root-free genus-3 Weil evaluation]

Both explicit kernels · Correctness gate · 15 November 2026

*All 52 nonzero kernel classes now have certified principal functions and generic-chart Weil evaluators.* The implementation stays in F(127¹³), without branch splitting. 864 direct evaluations and 324 separate norm computations agree.

#image("EVALUATOR.svg",width:100%)

*Principal functions.* For P=[E−3∞], E=(u,v), lift v to w modulo u³. Solve a+bw=0 modulo u³ with deg a≤4 and deg b≤1, using the basis 1,x,x²,x³,x⁴,y,xy. Every nonzero input has nullity one. The exact norm identity a²−fb²=cu³ with c nonzero, vanishing congruence and pole order nine certify div(a+by)=3E−9∞. All 52 one-coefficient corruptions are rejected.

*Determinant reduction.* H⁰(E+2∞) has basis 1,x,(y+v)/u; H⁰(5∞) has basis 1,x,x². At X=(U,V), reduce r=(V+v)/u modulo U. The determinant ratio is its x² coefficient r₂, so η_P(X)=r₂³ Norm(a+bV). The norm is a 3×3 multiplication determinant. This specializes Tian’s Eq2.2 with the convention η_P=f₋P. No roots of U or the source branch polynomial are needed.

*Exact registered checks.* The first 16 lexicographic distinct-x triples from 16 ascending-x canonical-y base-field places supply 864 direct determinant/product comparisons across both kernels, including identities. All agree, with zero observed poles or zero values in this sample. All 54 functions propagate canonically under Frobenius. The 32 kernel-sum values lie in F127. On six supported members of the first eight public multiples, 324 norm determinants match an independent polynomial-resultant recurrence. Two multiples are outside the registered chart and are skipped. Public D₁ is rejected as 3-torsion. Norm controls include identity, constants and a zero norm.

*Domain and evidence limits.* The evaluator requires a squarefree degree-three chart, coprime supports and nonramified kernel support. Direct checks share finite-field arithmetic; they test the quotient-algebra formula against explicit points. Unnormalized Frobenius sums have not been shown invariant under kernel translation. Their base-field values do not constitute quotient theta sections. Avoiding degree 156 here does not prove that every later reconstruction step avoids it.

*Custody.* Protocol 3eb90f69aa preceded source ef2521ac49bd and checkpoint 20261115T140904Z-ef2521ac49bd. Original kernel certificate, public fixture and dependency lock are pinned; RUN.txt stores all 54 function records, 32 sums and terminal counts. Scientific execution exited successfully with empty stderr. An initial rust-objcopy warning was removed before source freeze by preserving release debug information. Native Rust; no performance ratio, cloud worker or old-record change. Existing native CI does not run this standalone gate.

*Next dependency.* Verify the translation cocycle, normalize both kernel Weil sets and construct nonzero invariant theta sections; then reconstruct quotients, establish descent and transport divisors with all costs charged. The genus 2/3/4 mathematical 10× objective remains active.

*Source.* Tian, arXiv:2007.03172, Sections 2.1 and 5; established Riemann–Roch and finite-field algebra. KB status, search gaps, derivation, exclusions and reproduction commands are retained in PROTOCOL.md and RESULTS.md.
