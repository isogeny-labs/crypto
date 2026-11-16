# Minimum-depth coordinate propagation

Frozen after the retained lean-check trial and before this scheduling change.
All polynomial, proof and reduction caps remain unchanged. No runtime,
regularity or larger-field performance conclusion is asserted `[pcc:unverified]`.

Removing unused input-check provenance moves the six-variable failure to
change of order: 63952 proof terms, 8 polynomial terms, degree 4. The code
currently reuses a coordinate as soon as it is constructed in an edge scan.
On a cycle this may follow a long certificate chain before reaching a later
edge from a closer known vertex. The failed trial is preserved
`[pcc:unverified]`.

Prediction: propagate in breadth-first waves using only coordinates known at
the start of each wave. Keep the input-order route available as an explicit
ablation. Record each coordinate's parent, derivation depth and policy in
the result, and include those depths in a later budget failure. On the fixed
six-cycle rooted at vertex 5, the maximal dependency depth should become 3
rather than the input-order route's 4. This is a graph dependency depth, not
polynomial solving degree.

Test the same seed-7 six-variable inputs on both primes. A successful basis
must retain original-input certificates, input vanishing, monicity/reducedness
and every S-pair check, including the native F4 representation checks. Compare
the input-order budget outcome on the already diagnosed 31-bit case. Also
retain the three-/four-variable basis controls. Do not replace failed seeds,
raise caps or accept a partial basis. If breadth-first propagation still fails,
retain that failure before considering the separate certificate-DAG lead.

This tests certificate construction paths for the same equations and ideal.
It does not claim a lower degree of regularity, different reduced basis,
timed advantage over optimized F4, factor-base yield or logarithm recovery.
Any later runtime comparison must charge all construction and checking and
use its separate frozen root-codebase protocol `[pcc:unverified]`.
