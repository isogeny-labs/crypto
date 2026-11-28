#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:0.55em)
#text(size:18pt,weight:"bold")[Interpolation states and their symbolic union]
#v(5pt)
15 November 2026 · source b94180f · local formal component

Two endpoints: exactly bᵗ genuine bounded kernels, and an exact symbolic
description of their union. No efficient nonzero-witness extractor is supplied.
PCC standing remains unverified alongside the local source checks.

#image("flow.svg",width:100%)

Use an actual field F, coordinate maps x,y on primitive records, and pairs
(u,v) in Polynomial F × Polynomial F. A selected support D defines
#raw("K_D = {(u,v): deg u <= dU, deg v <= dV,\n       u(x_j) + y_j*v(x_j) = 0 for each j in D}",block:true)
The exact finite state theorem requires injective x and |D|≤dU. It uses
the existing Mathlib nodal polynomial, with no replacement curve or
arithmetic state-count surrogate. The final assembly applies the laws
unchanged to nonsingular Mathlib coordinates over a prime field.

Lean 4.35.0 / Mathlib cfc55fc. Final canonical gate 003 audits 167 theorems,
0 failures and 0 trusted-native exceptions. All 18 new named laws print only
propext, Classical.choice and Quot.sound. No #raw("--fresh") checker result.

#pagebreak()
#text(size:16pt,weight:"bold")[Why the entire prefix family stays large]

For a clean selected support D, take h_D=Lagrange.nodal D x. Existing
Mathlib laws give its degree, nonzero status and exact selected zeros.
#raw("deg h_D = |D|,  h_D != 0\nh_D(x_j) = 0 iff j belongs to D\n(h_D,0) in K_E iff E is a subset of D",block:true)
If K_D=K_E and both support sizes are within dU, each nodal pair belongs
to both kernels. This gives both support inclusions and hence D=E.
For one choice from each of t colors, the support map is injective and
has size t. The checked finite endpoint is
#raw("Nat.card (range (choice -> K_(D_choice))) = b^t",block:true)

*Scoped cost consequence.* Consider an architecture constructing the entire
prefix level, with one explicit entry per exact kernel, equality-only
memoization and pruning only of kernels with no nonzero pair. Every prefix
has a nonzero nodal pair, so that level contains bᵗ distinct entries.

At the consumer caps dU=floor(m/2), dV=floor((m−3)/2), choose t=floor(m/2).
For b=ceil(r^(1/m)), the hand accounting is
#raw("b^t >= r^(1/2 - 1/(2m))\nlog2(b^t) >= n/2 - O(sqrt(n)), for m ~ sqrt(n)",block:true)
With full input length polynomial in n=ceil(log₂ r), full-level storage
cannot meet the proposed subexponential attempt cap. This is an entry-count
consequence for that architecture, not a formal runtime theorem.

The result gives no visited-state lower bound for lazy/early-stopping
programs, selective searches, future-color pruning or a faithful coarser
representation. Those models need their own cost and yield arguments.

#pagebreak()
#text(size:16pt,weight:"bold")[The union has a compact exact membership test]

For each color, multiply its actual evaluation linear forms:
#raw("P_i(u,v) = product_j (u(x_ij) + y_ij*v(x_ij))",block:true)
The shared law is exact, with both degree caps retained:
#raw("all P_i(u,v)=0  iff  exists choice c with (u,v) in K_(D_c)",block:true)
Product zero means one factor is zero; finite choice combines the colors.
The theorem does not change existential selection into simultaneous
vanishing at every record. It needs neither clean x nor a curve equation.
The companion endpoint carries (u,v)≠0 on both sides, preserving the
nonzero-witness requirement.

At the consumer caps the coefficient vector has m entries. A supplied pair
can be checked by Horner evaluation and t products with b factors each:
O(t·b·(dU+dV+1)) field operations by hand accounting. This is not a measured
implementation or an extraction-time bound. A scan then locates zero
factors; actual group and scalar checks remain in the consumer contract.

*Next algorithmic target:* a capped nonzero-witness search on this factored
homogeneous system under the actual clean random elliptic input law, with
an effective useful-answer floor. Compact membership is the completed
structural component. The extraction algorithm remains open.

*Evidence and dependency.* Prospective plans 1d7dc7c/f7a287b and floor
probes 0e381fb/41218f4 precede their edits. Both source builds and the final
generic/prime-field assemblies pass. The initial gate failed on Lake's
unregistered-source diagnostic; a one-line classification fix preserves
direct compilation and the standard axiom policy. Its 11 existing positive
and negative controls pass. A second audit failure exposed hyphenated
module paths; byte-identical probes now use valid Lean module names.
Gate 003 retains every tracked source and passes 167/0/0. All failures remain.

Sources: exact Mathlib nodal/cardinality/product-zero laws; consumer PR4338;
#link("https://cs.uwaterloo.ca/~eschost/publications/jsc_final.pdf")[Jeannerod–Neiger–Schost–Villard, sections1.1–1.2].
KB unavailable; no novelty claim. No numerical, native, paid or frozen
qualification run. Full proof map and scope: ../interpolation-kernels-20261115.md.
