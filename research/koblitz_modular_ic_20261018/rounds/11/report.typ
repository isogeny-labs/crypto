#set page(paper: "a4", margin: (x: 19mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.40em)
#set heading(numbering: none)
#show link: set text(fill: rgb("245c91"))
#heading(level: 1)[#text("Round11: closing the diagonal multiplicity gap")]

#text("2026-10-19 UTC. Parent33db29ba4bcf5f72f5fe8d908b0fb80c0f44d7bc. The local characteristic2 equations account for all860 noncuspidal diagonal multiplicity. Projective saturation advances on a controlled coupled circuit. Seven whole noncuspidal divisors and the proper quotient embeddings remain incomplete. No IC cost is measured or assessed.")

#image("routes.svg", width: 100%)

#heading(level: 2)[#text("Diagonal:400 labels,860 multiplicity")]

#text("The round10 matching labels are retained unchanged. Direct local series, computed without fitting the global degree, give:")

#table(columns: 4, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Contact")],
[#text("Geometric labels")],
[#text("Closed degree10 points")],
[#text("Degree contribution")],
),
[#text("1")],
[#text("260")],
[#text("26")],
[#text("260")],
[#text("2")],
[#text("100")],
[#text("10")],
[#text("200")],
[#text("6")],
[#text("20")],
[#text("2")],
[#text("120")],
[#text("14")],
[#text("20")],
[#text("2")],
[#text("280")],
[#text("Total")],
[#text("400")],
[#text("40")],
[#text("860")],
)

#text("The former lower sum400 missed460 multiplicity. The new sum860 equals the independently retained diagonal degree1280 minus cuspidal degree420. All contacts are Frobenius invariant. The raw closed divisor gives each orbit of the original labels and its exact multiplicity. This completes the noncuspidal diagonal under the written moduli/deformation bridge in PROOF_​MAP.md. The bridge and native field semantics are not fully Lean formalized. Explicit49-torsion point coordinates remain uncomputed.")

#text("Use E_​s:y²+sxy+y=x³ over F1024[[s]]. Enumerate all1089 special-fiber points,121 points of11-torsion and60 nonzero x-coordinates. The actual I,J automorphisms identify a torsion basis with the split quaternion module. Each simple11-division root is lifted through s^31. For the five x-values in a cyclic kernel,let c(s) be their sum. Velu's normalized quotient has a1=s,a2=0,a3=1 and")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("a4=s+s²*c, a6=1+s*c+s³+s⁴*c.", block: true)]

#text("Every chosen norm11 quaternion is bound to this actual quotient map, up to a recorded target automorphism,on1078 nonkernel points. The12936 point checks exceed a written global degree bound400 for the doubled map differences. All24 unit bindings are additionally checked on1089 points each. Coordinates of all twelve kernels are exported.")

#text("For each of the400 matches,solve the Weierstrass a2,a3,a4,a6 isomorphism equations with the recorded special-fiber unit. The remaining a1 residual is s*(u+1). Its first nonzero order is1,2,6 or14;none reaches the precision limit32. A separately invoked parser rechecks1920 division-root and51200 transformation coefficients from exported parameters,without solving again. It shares native primitives and is not an external audit.")

#text("The return endomorphisms have norm121 and traces3,4,10,11,17,18,with counts100,100,20,80,80,20 respectively. Their characteristic identities, 49-line preservation and maximal scalar congruence depths at2 are checked. No contact length is inferred from trace or depth alone.")

#heading(level: 2)[#text("Projective saturation:five quadrics")]

#text("For xy=z,xu=1,yv=1,the exact projective closure in coordinates [x:z:y:u:v:h] has homogeneous ideal")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("xy-zh, xu-h², yv-h², zu-yh, zv-xh.", block: true)]

#text("A native elimination followed by graded reverse lex computation checks all10 S-pairs. Every leading monomial is h-free. Explicit h-power membership certificates identify this ideal with the h-saturation of the first three equations. All six dehomogenized chart ideals are exported. The closure has10 F2 boundary points and excludes5 spurious points of the naive homogenization. Its h-chart recovers the original two-dimensional torus graph;the ideal is prime by injective localization.")

#text("This settles this coupled graph closure and its closed projective embedding. It does not compute the full quotient ideal's saturation, recover that quotient's global boundary,or establish its proper polarized projective model. The four round10 closed affine open embeddings remain the effective quotient models available.")

#heading(level: 2)[#text("Seven residual divisors:more exact structure")]

#text("The rational two-torsion point T=(0,1) on X0(49) satisfies x(P+T)=1/x(P). Consequently")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("div(x0*xinf+1)=(phi0-phiinf)^*(T)+(phi0+phiinf)^*(T)\n              -2phi0^*(O)-2phiinf^*(O).", block: true)]

#text("The two positive fibers have degrees1280 and1360. This is an exact Cartier decomposition,not an evaluated finite divisor. Neither fiber has been enumerated.")

#text("The ambient zero correspondence x0+xinf+1=0 has a connected genus3 normalization. With u=x+1,y=x*z+1,v=(x+1)*w+1,on x!=0,1:")

#block(inset: 5pt)[#set text(size: 8pt)
#raw("z²+z=x+1+1/x,\nw²+w=x+1/(x+1).", block: true)]

#text("The third quadratic subcover has right side1+1/(x*(x+1)). The three subcovers each have two simple poles and genus1;Riemann-Hurwitz gives genus3 for the connected biquadratic cover. This is an ambient correspondence. A cover from C or a new carrier transfer has not been constructed from it.")

#text("For y0+yinf=0,the two elliptic equations factor into the diagonal and v=y,y=x²+x*u+u²+x+u. The pulled-back residual curve has degree2680; its support is not evaluated. The completed diagonal also occurs here, with the usual polar corrections. All three new identities receive 212363 finite-field controls across F4 through F256,plus universal Lean identities. Point controls do not replace the written Cartier arguments.")

#text("The noncuspidal x0+xinf zero degree2100 now splits into the completed diagonal860 and an unresolved antidiagonal1240. All seven whole residual supports remain incomplete. No new relation row or evaluated column is promoted. Ranks remain12/1308 and nonzero-image11 on1165/1160 columns.")

#heading(level: 2)[#text("Failures and checks")]

#text("Retained failures include a Rust expression delimiter,a wrong source working directory,an unnecessary Lean tactic after field_​simp,and two unsuccessful lex-order h-regular certificates. Grevlex resolves the certificate issue. A write-once save correctly rejected replacing the preliminary local export;the original survives as an attempt artifact. A full temporary filesystem interrupted tool startup;this task's older temporary artifacts were preserved on the workspace disk before resuming.")

#text("Twenty-two new named Lean proofs cover the universal Weierstrass transformation,Hamilton norm,chart recovery,saturation identities,AS relations and degree bookkeeping. The pinned full proof gate checks168 project theorems with zero native trusts. Two source-bound identities receive64 issuer and128 additional checks each. Native and root tests, new/prior byte replay,site/workflow checks and archive validation are recorded beside the immutable raw archive. Standard Lean foundations are retained;global moduli geometry remains written.")

#heading(level: 2)[#text("Next discriminating work")]

#text("Construct the antidiagonal degree1240 support and the two translated T fibers,with local multiplicities and projected column images. For the quotients,apply controlled saturation to an actual coefficient circuit boundary and recover its retained Kummer charts;the torus prototype does not discharge that obligation. Complete each of the seven finite divisors before making a whole-row relation or rank claim.")

#text("Sources: ")#link("ledger.json")[#text("raw ledger")]#text(",")#link("PROTOCOL.md")[#text("protocol")]#text(",")#link("PROOF_MAP.md")[#text("proof map")]#text(", ")#link("REPLAY.sh")[#text("replay")]#text(",")#link("../10/REPORT.md")[#text("previous round")]#text(". Primary geometric sources and exact proof scopes are given in the proof map.")

#text("The first formatter invocation also supplied paths relative to the wrong working directory;its four file-not-found errors are retained in format-01.log. Formatting with explicit source paths then passed.")

#heading(level: 1)[#text("Formal proof map and written arguments")]

#heading(level: 2)[#text("Formal and native layers")]

#text("KoblitzRound11.lean supplies22 named results. The full pinned gate audits 168 project theorems. Only propext,Classical.choice and Quot.sound are admitted;there is no custom axiom,sorry or native decision trust.")

#table(columns: 2, inset: 5pt, stroke: 0.4pt + rgb("cbd5e1"),
table.header(
[#text("Lean results")],
[#text("Scope")],
),
[#text("saturation_​other_​inverse,saturation_​cubic,saturation_​cubic_​redundant")],
[#text("Universal homogeneous membership identities")],
[#text("projective_​chart_​transition,affine_​two_​inverse_​recovery")],
[#text("Chart ratio and inverse substitutions")],
[#text("hamilton_​norm_​product")],
[#text("Universal quaternion norm multiplication")],
[#text("cube_​correction_​residual,normalization_​tangent_​determinant")],
[#text("Cube Hensel derivative and characteristic2 normalization tangent matrix")],
[#text("weierstrass_​transformation")],
[#text("Full universal integer coordinate-change residual")],
[#text("velu_​w_​residual,velu_​special_​coefficients")],
[#text("Exact specialized quotient coefficient algebra")],
[#text("local_​contact_​degree,recovered_​multiplicity,closed_​contact_​degree,diagonal_​antidiagonal_​residual")],
[#text("Integer degree bookkeeping;not standalone moduli theorems")],
[#text("translated_​product_​identity,translated_​divisor_​cancellation")],
[#text("Algebra and formal divisor cancellation")],
[#text("equal_​y_​factorization,AS_​recovery_​residual,AS_​sum,reciprocal_​pole_​identity,AS_​genus_​arithmetic")],
[#text("Ambient correspondence identities and conditional genus arithmetic")],
)

#text("The other-inverse saturation and equal-y factorization identities have source-hash-bound IDC certificates with64 issuer and128 extra checks. Native finite-field semantics,division-polynomial recursion,Buchberger implementation,formal smoothness and the identification with modular local intersection lengths are not asserted as Lean theorems.")

#heading(level: 2)[#text("Why the local series calculate the diagonal")]

#text("Write E0:y²+y=x³ over algebraically closed characteristic2. A general Weierstrass deformation has five coefficients. Coordinate changes fixing O have parameters u,r,v,t. At E0,the tangent map on the four normalized coefficients a2,a3,a4,a6,in the order r,u,v,t,is the identity. Equivalently, in order u,r,v,t it is the permutation matrix recorded in Lean. Hensel normalization gives the slice E_​s:y²+sxy+y=x³. Thus s is a parameter on the deformation space with marked special fiber. The discriminant 1+s³ is a unit. This formal-slice argument,including effectiveness of Weierstrass deformations,is written,not a full Lean scheme construction.")

#text("Prime-to2 torsion is finite etale. Its points and cyclic subgroups lift uniquely over a strictly Henselian nilpotent thickening. Hence every11 kernel and cyclic49 subgroup in the retained label has a unique lift. An isomorphism whose reduction carries one49 subgroup to the other also carries their unique lifts. The target stabilizer is precisely {+1,-1} at all400 labels. The central minus sign extends over every deformation and acts trivially on its base. There is therefore no additional quotient ramification between s and the local parameter of the coarse X0(49) curve at these targets. The full11 source level similarly removes all noncentral automorphisms. This is the level-rigidity bridge needed to interpret the explicit isomorphism obstruction as the local order of phi0-phiinf at O.")

#text("The actual11 basis P,I(P) is found over F1024 and checked against the retained J matrix. Its symplectic determinant can differ from the original choice by a scalar. Rescaling the first basis vector corrects it while preserving lines0 and infinity and commuting with normalized Frobenius. Thus it identifies the determinant components for these two maps,as in round10. No conclusion for the other ten maps is inferred without their pairing adapter. The actual49 Tate module realizes the maximal Hurwitz order tensor Z/49 as M2(Z/49);any such split representation is conjugate. Consequently the retained56 abstract lines label the actual cyclic49 subgroups. This existence argument remains distinct from explicit49 point coordinates,which have not been computed.")

#text("The special-fiber normalized Velu targets are y²+y=x³+1. The shift (x,y)->(x,y+omega),omega²+omega=1,identifies them with E0. The recorded post-Velu unit binds each norm11 quaternion to the actual quotient map. For all1078 nonkernel rational points,the doubled map equals the quaternion-coordinate sum of actual unit maps. The latter sum has at most12 terms. The degree of their difference is bounded by (2*sqrt(11)+12)²<400. More than400 distinct zeros force equality of endomorphisms globally. Cancellation of [2] is in the endomorphism domain,not on a possibly nonreduced2-torsion scheme. Unit bindings have an analogous degree36 bound and1089 point checks. Actual quaternion endomorphisms and maximal-order identification use the round10 bridge and ")#link("https://abel.math.harvard.edu/~elkies/M223.24/index.html")[#text("Elkies's characteristic2 account")]#text(".")

#text("For each lifted kernel,c(s) is the sum of five lifted x-coordinates. The general characteristic-independent Velu formulas specialize to the recorded a4,a6. See ")#link("https://ddd.uab.cat/pub/pubmat/pubmat_a2007vEXTRA/pubmat_a2007vextrap147.pdf")[#text("Miret–Moreno–Rio,section1")]#text(". The odd11-division polynomial has60 simple roots at s=0;coefficient Hensel recursion lifts each uniquely. Both roots and substitutions are exported and rechecked through order31.")

#text("A target isomorphism is x=u²*X+r,y=u³*Y+u²*v*X+t. Its a2 equation sets r=s*v+v²;its a3 equation sets u³=1+s*r. After fixing the special fiber unit,the a4 and a6 equations have identity Jacobian in v,t,so there is a unique solution through every finite order. The remaining a1 equation is s*(u+1)=0. The universal transformation identity is Lean checked;the parser verifies all other residual coefficients vanish. The first nonzero coefficient of this final obstruction is the exact contact order. The only observed orders are1,2,6,14,strictly below32. The local equation gives a length in k[[s]],not a fitted trace-based formula. Summing the400 positive lengths gives860. Along with the round10 ordinary exclusion and cuspidal degree420,this exhausts degree1280 and establishes completeness of the diagonal under these bridges.")

#text("A tempting imported deformation formula was not used: ")#link("https://arxiv.org/pdf/1902.10789")[#text("Li,Theorem1.1")]#text(" explicitly assumes an odd residue field. The direct characteristic2 series avoid that unresolved applicability issue.")

#heading(level: 2)[#text("Saturation and projective chart proof")]

#text("Let I=(xy-zh,xu-h²,yv-h²) in F2[x,z,y,u,v,h]. The final ideal J also contains zu-yh and zv-xh. The two h-membership identities are Lean checked. Native Buchberger checks all10 S-pairs in grevlex with h smallest. Every leading monomial omits h. Therefore normal forms show multiplication by h is injective modulo J. Since I subset J and every new generator enters I after multiplying by h,J=I:h^infinity. Localization at h yields the Laurent torus graph,so the injective localization shows J is prime. Its homogeneous quotient has dimension3 and Proj dimension2. The six emitted affine chart ideals and ordinary ratio transitions construct the closed projective subscheme in P5. ")#link("https://stacks.math.columbia.edu/tag/084M")[#text("Stacks,closed relative Proj")]#text(" gives the scheme framework. All monomials,S-pairs and h-membership reductions are in the raw certificate. The complete four large quotient ideals are not substituted for this specified circuit.")

#heading(level: 2)[#text("Cartier identities and genus3 correspondence")]

#text("On E:y²+xy=x³+x²+1,the origin is O and T=(0,1) has order2. The group law gives x(P+T)=1/x(P). Substitute this into the standard x-difference Cartier identity on E times E and add div(x(P)). The vertical terms at P=T cancel,giving the translated T-fiber formula in REPORT.md. Finite controls supplement this global divisor identity. The fibers remain unevaluated.")

#text("On y=v,the difference of the two curve equations factors into (u-x)*(y-u²-u*x-x²-u-x),as proved universally in Lean. The two components are the diagonal and the displayed residual. Their pullback degrees sum3960,so the residual has degree2680 before common polar cancellation. A finite support has not been inferred from this equation alone.")

#text("For x+u=1,the substitution in REPORT.md gives three independent Artin-Schreier quadratic characters. Their reduced right sides have simple poles at {0,infinity},{1,infinity},{0,1}. A rational AS coboundary has even pole order at its poles,so no nonzero character is trivial. The cover is connected of degree4. At each of the three branch points, inertia has order2,there are two points above it,and each different exponent is2. Riemann-Hurwitz gives2g-2=-8+3*4=4,hence g=3. The AS recovery algebra is Lean checked;normalization and ramification arguments remain written. This ambient genus3 curve is not an evaluated cover or a new principal relation on C.")
