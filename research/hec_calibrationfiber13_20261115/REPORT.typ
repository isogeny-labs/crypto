#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Degree-18 calibration fiber
F127 · genus 3 · 15 November 2026
#image("flow.svg", width:100%)

A fixed source-curve family supplies two nonzero source/image calibration pairs without a generator-multiple scan or their relative discrete logarithms. Both doubled pairs pass ell-torsion and final existing-matrix projective checks. Quotient kernel/normalization are reused setup inputs; this is a correctness and structural-cost result, without a timing ratio.

== Construction and bound
Vary P=(x,y) on the source curve, with E=P+(0,44)+(3,21)-3infinity. Restrict all27Weil functions exactly in the quadratic source function field. Divided differences give common denominator d³, d=x(x-3). Exact coefficientwise division by each kernel u³ leaves numerator degrees A≤7,B≤4. Quadratic section combination gives A≤15,B≤11; raw norm A²-fB² has degree≤30. Its fixed-anchor factor d⁶ of degree12 divides exactly, leaving degree≤18. Frobenius gcd and bounded splitting find all rational x-roots. The full regular zero set is independently verified at124source points.

The saturated polynomial has roots54,112, yielding source points(54,56),(112,65). Chord factoring produces target pairs(9,13)-(89,44) and(10,1)-(99,112). Both chords have only two rational intersections. Unique Hessian pair, endpoint differential scales and negation controls pass. Double source and image to clear cofactor2. Any nonzero order-ell calibration base suffices; its log relative to original D1 is unnecessary.

== Frozen attempts and controls
V1: wrong B≤3bound, exit101 before root selection. V2: corrected bound plus exact saturation passes, but the old decoder rejects partially split chords. V3: partial chord decodes, then the erased-Hessian control wrongly expects six rational pairs. V4: control count corrected to h(h-1)/2; both calibration pairs pass final checks, exit0/empty stderr. At h=2, Hessian erasure is non-discriminating; no corruption-rejection claim follows from it. Every version ran once after its separate source/checkpoint gate. Same anchors, input curve and polynomial throughout the amendments.

V4 source d8d3ca3d599e, checkpoint20261115T224740Z. Source curve y²=f(x), f ascending(31,89,73,48,91,41,123,1), subgroup order1028893. Full quotient/input identity, arrays, raw outputs and corrections are in RESULTS.md and RUN-V4.txt.

== Proof and next integration
DegreeBound.lean checks the conditional degree18bound with pinned Lean4.19/mathlib. Exact restriction/divisibility are explicit hypotheses, supported here by coefficient checks and independent values; the theorem does not formalize the geometry or decoder. Printed axioms contain no sorry or custom axiom.

Next refit the8×8matrix from a new calibration pair, map both original D1andB by known-calibrator translations, then join inverse/sign and fresh point-log/conic descent with cold quotient construction. Generic fiber existence, genus2/4 and the full10×gain remain open. Existing performance graphs stay unchanged. Tian2020§4.2.2 already has a two-torsion alignment route; the cyclic scan is specific to this implementation. No mathematical novelty is established. Source: https://arxiv.org/html/2007.03172.

Evidence: PROTOCOL.md, amendmentsV2–V4, RUN records and stderr, fiber.rs, source_jet.rs, PROOF-MAP.md, LEAN.txt, locked dependencies and binary hashes, in research/hec_calibrationfiber13_20261115.
