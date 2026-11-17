#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
#text(size: 23pt, weight: "bold")[N83 first-fall degree sweep]

#text(fill: rgb("#44566c"))[Balanced candidate domains · all m = 2–9 · 27 October 2026]

#block(fill: rgb("#edf2f7"), inset: 11pt, radius: 4pt)[
*Standard first-fall values cover all 48 original inputs.*
Koblitz and general binary: 2 throughout, from direct elimination and explicit linear witnesses. Prime: 24, 17, 14, 12, 10, 9, 9, 8, using the written lower-bound derivation and checked factored witnesses. Prime values are not full n83 matrix-rank measurements.
]

#image("degree-grid.svg", width: 100%)

*Scope.* One planted and one natural target for each of three curve families and eight arities. ell = floor(83/m): 41, 27, 20, 16, 13, 11, 10, 9; candidate x-domain size 2^ell. All targets and system hashes are unchanged from the original n83 sweep. Candidate abscissas still need curve lifting; this is not an end-to-end index-calculus run.

*Definition.* The standard first fall is the least degree of a nontrivial syzygy of the highest homogeneous parts, modulo trivial syzygies. For binary inputs the measured value2 also certifies the earliest new affine degree fall. For prime inputs, novelty of the resulting affine polynomial in a lower multiplication-closed space has not been established. These quantities must remain separate.

*What remains unresolved.* Last fall, solving degree and degree of regularity. The original96 regularity attempts remain resource-limited. Full Lean formalization and independent research review of the prime lower-bound argument also remain open. The first-fall grid does not change those statuses.

#pagebreak()
= Why the prime values are proof-assisted

Put h = floor(ell/2), and quotient the leading Boolean-selector equations to obtain the square-zero algebra. Normalize each bit to y_j = 2^j b_j. The field characteristic exceeds83, so every scaling is invertible and the strong Lefschetz theorem applies. Multiplication by the square of the block sum has rank min(binomial(ell,r), binomial(ell,r+2)); it is injective below r=h.

A finite filtration of the quartic Koszul complex has leading operators L0²L1² and U_(k−1)²L_(k+1)², which act on disjoint tensor factors. Their first homology vanishes below h+4. Lifting through that finite filtration excludes nontrivial original syzygies below h+4. This is the written lower-bound argument in `ARGUMENT.md`; the n83 rank figures used here are theorem-derived.

For the matching upper bound, choose g = product of (y_even−y_odd) over the h pairs in the final selector block. Each pair annihilates its sum in the square-zero algebra, so g annihilates the square of the full block sum. Hence g annihilates the endpoint leading quartic. The multiplier is nonzero and cannot be a Koszul boundary: specializing earlier blocks and all intermediates to zero kills the other quartics but preserves g. Thus the standard first fall is h+4.

The native checker also finds an explicit Boolean assignment where the affine product is nonzero. Its degree is at most h+3 after selector reduction. This certifies a degree-lowering combination; it does not prove the affine result was absent from the lower closed space V(F,h+3).

= Controls, replay and provenance

Five exact small-matrix controls over65537, at (ell,m) = (2,4), (3,3), (4,3), (5,2), (6,2), detect first kernels at5,5,6,6,7. All preceding degree matrices have zero kernel. These degrees are below8, so quartic Koszul boundaries cannot explain the detected kernels. Three R4 native tests pass; R3 had seven focused tests and positive/negative controls.

R4 rechecked all16 prime presentations against the original saved equations, validated all16 factored certificates and replayed all16 in fresh processes. R3 previously recorded32 binary witnesses and48 certificate replays. Raw manifests and unified exports pass file verification. Same-code replay is not independent scientific review.

The Lean4.34.1 lemma `square_zero_remainder_annihilates` checks a structural upper-bound dependency and uses no axioms. It does not formalize the selector quotient, nontriviality, Lefschetz decomposition or filtered Koszul lower bound. `PROOF_MAP.md` records these missing bridges; the complete formal theorem remains open.

*R4 identity.* Job `n83-prime-first-fall-m2-m9-20261027-r4`, pod `8504c912-0848-46f3-a1d7-7ee0029ebdf5`, exit0. Two CPUs,4GiB,600-second cap, no retries. Protocol hash begins `6e12ffdda6e16107`; full hash is in `protocol.json` and `experiment.export.json`. Source, executable and immutable image are separately pinned. All clocks are diagnostic.

*Evidence.* `evidence/results.json` (R4), `evidence/r3-results.json` (retained R3), `evidence/combined-grid.json`, `evidence/replay.jsonl`, `terminal.json`, and `experiment.export.json`. Diagram source: `../../src/bin/prime_report.rs`. No statistical inference or ECDLP performance claim.

*Primary references.* #link("https://arxiv.org/html/2112.05579")[Caminata–Gorla, Definition1.3]; #link("https://arxiv.org/html/2211.13548")[Phuong–Tran, Theorem2.7]; #link("https://arxiv.org/html/1503.08001")[Kosters–Yeo, binary trace identities].
