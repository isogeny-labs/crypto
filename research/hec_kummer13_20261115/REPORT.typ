#set page(paper:"a4",margin:17mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.5em)
#text(size:22pt,weight:"bold")[Base-field quotient coordinates]

Both genus-3 kernels · Constructive correctness gate · 15 November 2026

*Each quotient now has eight explicit descended coordinate functions spanning its complete level-two section space.* Construction uses only F(127¹³), with coordinates defined over F127. Source 2-torsion and branch roots are not split. Both rank-eight results and the registered action checks pass.

#image("COORDINATES.svg",width:100%)

*Character construction.* With the pinned normalized functions Fₚ, form Φₐ=Σχₐ(P)Fₚ using all 27 characters of the existing kernel coordinates and ζ=19 in F127. The theta-group law gives Fₚ(X)Φₐ(X+P)=χₐ(−P)Φₐ(X). Opposite products divided by Θ², where Θ=Φ₀, are therefore invariant under kernel translations and negation. There are 14 opposite-character pairs. All 54 character sections across the two kernels are nonzero on at least one registered point.

*Why rank eight is sufficient.* The products descend from sections of L² to sections of 2M on A=J/V, since L=O(3W)=ψ*M. Riemann–Roch gives h⁰(A,2M)=8. Eight independent evaluation columns therefore establish a complete section basis, using the global descent argument and finite rank together. No projective-normality theorem is assumed. The quotient need not yet be identified as an indecomposable Jacobian; its coordinate image still needs equations and geometry checks.

*Coefficient descent.* Frobenius permutes characters by inverse pullback of its kernel action. For the 13 field-basis weights, sum each weighted function over that permutation and coefficient Frobenius. This yields 182 trace candidates per kernel, including zeros. At base points the result agrees with a field trace. At non-base-field translates, the explicit function permutation is used: tracing a computed value alone would be invalid.

*Registered evidence.* The ascending public-multiple search rejects n=1 and 8 for the original chart, admitting 16 points by n=18. The first 12 select the basis; n=15–18 are held out from selection. Both ranks are eight and choose D(0,0), D(1,0), …, D(1,6). All 23,328 character eigenactions, 12,096 opposite-product actions, 5,824 trace comparisons, 6,912 selected-coordinate translations and 256 coordinate negation checks pass. A broken character causes 832 failures; a duplicated basis descriptor lowers rank to seven. All translated coordinates are fixed by Frobenius. At n=2 the vectors are [13,113,9,83,81,43,38,75] and [13,20,28,10,12,107,29,90].

*Custody and limits.* Protocol 61923beee0 preceded source 26a74be6430e and checkpoint 20261115T143749Z-26a74be6430e. Clean locked offline Rust build; terminal run exits successfully with empty stderr and no cap or rerun. Full 16×182 tables, coordinate vectors, exclusions, descriptors and hashes are retained. Checks share the prior arithmetic and are not independent external validation. No timing result, cloud worker, historical-record or default change. Existing native CI does not execute this standalone gate.

*Next dependency.* Compute relations and equations of both coordinate images, establish quotient geometry and same-field/Serre conditions, and construct divisor transport. Complete-cost comparisons must charge setup and solve work. The full mathematical 10× genus 2/3/4 objective remains active.

*Sources.* Tian, arXiv:2007.03172, Sections 3–4; pinned theta and root-free reports. Established character projections, field trace and line-bundle descent. KB/source gaps, derivation and reproduction commands are in PROTOCOL.md and RESULTS.md.
