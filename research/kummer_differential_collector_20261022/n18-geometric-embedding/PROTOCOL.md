# N18 geometric projection certificate protocol

This packet tests whether the declared N16 C1/C2 projection is the
geometric genus-2 Kummer map on the 21 frozen Rosenhain models. It does not
run a target collector. Use the classical Kummer map from Müller and the
corrected `F0` printed independently by Arnth-Jensen–Flynn; map the result
through the exact fast-Kummer linear transformation of Chung–Costello–Smith
(CCS), §7.4. Check CCS's model/denominator assumptions on the 21 selected
theta records. State the printed Müller `f5` discrepancy explicitly.

The native producer proves each of six independent coefficient-basis
identities in `Z[s,t]`, checks the published transformation matrix and
one valid branch-point divisor on every frozen model, and emits a 21-row
CSV. A separate validator checks the universal polynomial residual using
integer interpolation on a degree-bounded grid and recomputes the finite
CSV values. Algebraic limits to degree one and the identity are written
derivations and are labeled as such, not machine-verified samples.

Do not infer all-state totality of the theta xADD implementation from the
geometric map. A zero output remains undefined until a signed-label
fallback is integrated and independently checked. Do not launch a new
1,260-target collector before that and a complete cold-cost protocol.
