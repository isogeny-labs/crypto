#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Conic descent with source-verified logarithms
F127 · genus 3 · 15 November 2026

#image("flow.svg", width: 100%)

The two closed divisors from the algebraic inverse admit conic residual descent. Of twenty fixed anchor pairs, seven split: three for E0 and four for E1. These give five distinct conics; the three E0 acceptances reuse one conic. All seven divisor identities pass independent Jacobian arithmetic, and all seven logarithm recoveries yield k=183049, verified by scalar multiplication on the source. No expected scalar is supplied.

== The algebraic simplification
Write L(P)=log(P-O), K0=log(K-4O), D0=log(Dinf-3O). Centering gives log(beta)=D0-K0/2. For X=(E-Dinf)+beta, log X=log(E-3O)-K0/2. A conic through E and anchors A,B leaves a degree-three residual R, with E+A+B+R linearly equivalent to 2K. Consequently:

#align(center)[*log X = 3 K0/2 − L(A) − L(B) − sum of residual point logs.*]

This formula uses the inverse of two in the odd subgroup. Dinf and beta group calculations cancel from logarithmic descent. The known tangent section gives K0 by two point lookups. Apply the previously verified public sign, subtract log G=47376, and divide by log G. The resulting log B is 619220 for all seven pairs. Public sign and generator calibration remain setup dependencies.

== Frozen execution and controls
Gate1: source333061c10519, checkpoint20261115T222341Z; one exit0 execution. Gate2: source442d2bb8fc0e, checkpoint20261115T222548Z; one exit0 execution. Both stderr files are empty. Exact graph constraints, squarefreeness, known-factor division and residual support are checked. Each resultant is verified against 127 direct Sylvester evaluations. The independent group law verifies every accepted conic identity. Both corrupted graph controls are rejected. Thirteen nonsplit exclusions and a corrected pre-freeze compilation error remain recorded.

== Scope and evidence
Source y²=f(x), f ascending (31,89,73,48,91,41,123,1), subgroup order1028893, D1=2(0,44), B=2(3,21). Full target coefficients, closed divisors, anchors and proof are in RESULTS.md. Candidate descent uses polynomial/matrix arithmetic and point-log lookups, with zero Jacobian additions. Setup and verifiers retain their own costs. The point-log kernel and public sign are reused; this is a joined correctness gate, not a complete cold performance study. Existing performance graphs stay unchanged. Integrated cold construction, map/sign and scalable sparse point-log solving remain next; genus2/4 and the full10×goal remain open.

Classification: engineering, with a derived centering identity; no mathematical novelty claimed. Primary sources: Diem2012, https://jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/ ; Tian §4.2.3, https://arxiv.org/html/2007.03172. Evidence: RUN.txt, LOGS.txt, PROTOCOL.md, PROTOCOL-LOGS.md, main.rs, log_api.rs and binary hashes, in research/hec_conicdescent13_20261115.

Formalization: Centering.lean and PROOF-MAP.md cover the conditional scalar identity over a field with 2 nonzero, using pinned Lean4.19.0/mathlib. The geometric hypotheses and Rust execution are outside that formal theorem; see final LEAN.txt and EXIT-LEAN.txt.
