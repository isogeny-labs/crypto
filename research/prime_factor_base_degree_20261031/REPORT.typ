#set page(paper: "a4", margin: 1.8cm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
= Prime-field degree reduction: checked scope and next candidate
2026-10-31 · No positive novel factor-base degree reduction established

This round implements an ordinary-prime polynomial certificate checker and an exact group-reference exporter. Eight release tests pass, including all 625 pairs of univariate linear generators over F5, corrupt-certificate rejection, curve-group arithmetic, exact membership and S3 evaluation on a point sum. The Boolean certificate tool from the earlier binary-field round is not used for prime-field arithmetic.

== A dense-presentation exclusion

For a short-Weierstrass curve over a prime field of characteristic greater than three, let the factor-base abscissae be a set V of B distinct field elements, B≥2. Exact dense membership is $F_V(T)=product_(v in V)(T-v)$. For a fixed target r, use $F_V(x),F_V(y),S_3(x,y,r)$.

Their highest-degree forms are $x^B,y^B,x^2 y^2$. A standard monomial has exponents $i,j < B$ and at least one of i,j at most one. Its degree is at most B; $x^(B-1)y$ survives at degree B. Thus the first zero homogeneous quotient component is degree B+1, regardless of V, curve coefficients or target. This is a statement about this dense two-summand presentation, not every representation of the relation problem.

The monomial-coverage argument is kernel checked in Lean 4.19.0 by four declarations, using only standard Lean axioms. The bridge from the actual polynomial system to the monomial-ideal quotient is given as a written argument; it is not falsely reported as fully formalized.

#figure(image("degree.svg", width: 100%), caption: [Scope of the checked argument. Sources: THEORY.md and proof/TopDegree.lean. No comparative factor-base measurement is shown.])

== Novelty audit

The repo already has `pkm_symmetric_tower.rs` and `pkm_symmetric_complete.rs`; the 2026-10-26 log records lower productive step degrees for that presentation on some cells. Repeating symmetric Kummer membership would not establish a new construction. The older 24-design register also covers exponent intervals, parallel towers, auxiliary isogenies, norm chains and other related ideas.

Read primary sources: Petit–Kosters–Messeng, #link("https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf")[PKC 2016 prime-field paper], and Kosters–Yeo, #link("https://arxiv.org/abs/1503.08001")[Notes on summation polynomials]. Repository sources and exact search scope are in README.md. The candidate below was not found as an explicit experiment in the narrow sources inspected; novelty remains unestablished.

#pagebreak()
== Candidate and exact test

Investigate the rational lifts of the intersection
$V={x : x^d=c_1, (x+h)^d=c_2}$.
This changes the selected abscissae. Use equal-sized low-x and hash-selected controls on the same prime-order curve. No target outcome enters construction. The development exporter enumerates all curve points, checks the full group order is prime, keeps rational lifts, emits both exact membership polynomials plus S3, and records a point-sum existence witness for every target x. It refuses singular curves, unsupported sizes and overwrite.

The first frozen spec is the existing ordinary prime-order unit fixture $E/F_17: y^2=x^3+2x+2$, d=8, h=2, c1=c2=1. It is a smoke control. A degree result on this fixture alone would not establish novelty or scaling. Canonical ICV1/EC1/FB1 registration and replay remain required before promotion.

The dense presentation cannot yield a homogeneous regularity win at fixed B, by the argument on page one. It can serve as an exact reference for affine falls. A genuine structured comparison still needs the complete pair of shifted-power chains, rational-lift conditions, all auxiliary variables and branches, and an iterative F4 trace. That exporter is not implemented here. Equal-point-set encoding controls must separate a presentation effect from a factor-base effect.

== Certificate semantics

For ordinary Fp[x,y] generators, the checker certifies the least bounded-Macaulay refutation degree D by checking two artifacts: a combination of formal-degree-at-most-D generator multiples equal to one, and a lower dual witness annihilating every such row at D−1 while separating the constant. Input bytes are SHA-256 bound. Polynomial identities are replayed separately from elimination and profile metadata is recomputed.

This invariant is not homogeneous degree of regularity or an iterative F4 solving degree. SAT and no-refutation-through-cap remain unresolved by the refutation metric. There are no Boolean substitutions such as $x^2=x$.

== Execution status and remaining result

Local build, eight release tests, strict Clippy and Lean proof checks completed. No factor-base research run was launched. The repository requires an S3 resume checkpoint before experiments; this worker has no upload credential or ready EKS identity, and its GitHub token is invalid. The run remains gated, with no paid or background job.

The user's requested genuinely novel prime-field degree reduction is still open. To establish it, finish the full structured system, verify identical roots against dense and group references, obtain a strict degree drop under a frozen invariant and complete paired targets, measure width/work and natural yield, then complete independent prior-art and mathematical review. The current exclusion and certificate tooling support that work; they do not substitute for the positive result.
