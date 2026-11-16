# Actual kernel3 source Kummer evaluation

Fixed before source image data,2026-11-16. Continue Tian arXiv:2007.03172 equations4.1--4.2 and Propositions3.2/3.3, and the unchanged native source-normalization/determinant helpers. KB unavailable in this runtime. This local fixed-input functional unit adds no paid/performance run.

For a source divisor class D and representative k in the recorded kernel, put Z=D+delta+k. For labels S=0..7 use the source code obtained by swapping the two three-bit halves. Compute f_S(D)=(alpha_S*eta2(S,Z))^3*(theta(Z+S)/theta(Z))^2, then use the signed Fourier sums divided by the theta-null coordinates, exactly as in EVEN_KUMMER_PROTOCOL.md. Recompute the64 source normalizing scalars with384 heldout cocycle equations; use the existing deterministic square-root choices.

Try recorded representatives k1 through k26 in order, accepting the first two for which all eight shifted supports have degree3, all existing theta determinant denominators are nonzero, and the base theta value is nonzero. This is a fixed chart rule; retain rejected indices. Unsupported arithmetic at an accepted chart rejects, with no silent replacement. Require the two computed function vectors to agree exactly. Require f0=1.

Use public fixtures origin, pullback of the fixed P-192 generator G, its negation, and pullback of Q=37G. The evaluator consumes divisor classes and never the scalar37. The fixture generator constructs Q outside it. Require origin coordinates projectively equal the pinned theta-null; G and minus-G coordinates projectively equal. Separately evaluate pull(G)+k1 and require invariance under the actual nonzero recorded kernel class. Require pull(Q)=37*pull(G) by source arithmetic. Retain each source divisor, both representatives, rejected charts, eight function values and coordinates. Perturb one nonzero projective coordinate and require rejection.

This component computes source Kummer images, not destination divisors. The finite inverse equations, sign/orientation, divisor-class homomorphism, Frobenius/kernel/dual checks, destination relations and public scalar recovery remain required. All output is producer-unverified.
