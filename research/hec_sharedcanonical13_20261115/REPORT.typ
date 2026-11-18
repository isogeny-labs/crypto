#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Shared canonical-divisor variable
Genus 3 · F127 · 15 November 2026
#image("flow.svg",width:100%)

One separately frozen standalone gate passed. Source02af681212a6; pre-run checkpoint20261116T003824Z. The candidate introduces one shared canonical-log variable instead of repeating the same canonical divisor in every line relation. This is a known linear representation, without a novelty or timing claim.

== Exact equivalence
Original equations (Hi−h)x=0 become Hi x−z=0 plus hx−z=0. Lifting x↦(x,hx) and projecting back give inverse maps on kernels. Subtracting the defining row gives the original equations plus one independent auxiliary pivot: rank rises1and nullity is preserved. The auxiliary column remains in the small factor base.

The candidate checks all augmented and defining equations, then every original unlifted sparse relation. All129point logs match the independent dense reference up to common nonzero scale; all314original residuals pass. Twelve planted kernel/corruption controls pass. Exit0, empty stderr, no scientific rerun or omitted case.

== Structural outcome
#table(columns:3, inset:6pt,
 [Count],[Sparse V2],[Shared variable],
 [Input rows],[314],[315],
 [Input coefficients],[2378],[1558],
 [Maximum row support],[8],[5],
 [Reduced shape],[232 × 47],[234 × 48],
 [Reduced coefficients],[2413],[2155],
 [LP operation counter],[14455],[13801],
 [LP entry counter],[9480],[7703])

Input coefficients fall34.5%, reduced nonzeros10.7% and LPoperations4.5%, while dimensions grow. These are structural counters, not measured CPU, wall time, RSS or total-solver work. Original rows are retained alongside augmented rows for verification; reduced input coefficients do not prove a total-memory gain.

== Proof and decision boundary
SharedCanonical.lean checks equation equivalence and unique lift in the pinned Lean4.19/mathlib environment. It does not formalize matrix rank, geometric relations, implementation or performance. The initial file-path proof invocation error is retained separately from scientific data.

Keep as a candidate representation; do not deploy or claim tenfold improvement from these modest counts. The full verified cold path remains sparseV2. Further work must address field/factorbase scaling and construction costs. Genus two/four and the mathematical tenfold objective remain open. The old failed charged coldcost result is unchanged.
