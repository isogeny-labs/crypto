# Exact Scholten-to-weighted-model normalization

T-1130, 2026-10-31. Producer proof packet; independent source-fidelity review pending. No scientific trial or cost claim.

## Derived and kernel-checked model

The literal Gaussian constants in Scholten::new expand to

F(t)=1+(25/3)t+(299/12)t^2+(95/3)t^3+(187/12)t^4+(5/2)t^5.

The historical genus2_model_gate uses alpha=F[5]=5/2, beta=-F[4]/(5alpha)=-187/150, t=alpha*x+beta and y_old=alpha^3*y_new. Hence alpha^3=125/8. Its normalized polynomial is

N(x)=x^5-(12938/28125)x^3+(907312/10546875)x^2+(44207039/1318359375)x-(57913058672/7415771484375).

Lean checks F((5/2)x-187/150)=(125/8)^2*N(x) in any field where2,3,5 are nonzero, and checks the source expansion when i^2=-1. Those conditions apply to the source's declared primes p>13; this packet does not run primality checks. Every denominator is a product of2,3,5, and the proof explicitly constructs the needed nonzero denominator facts. The cleared source identity is also checked separately.

The normalized polynomial is defined using the existing WeightedDoubling.curve, with its actual argument order (a0,a1,a2,a3). The normalized_eval theorem proves evaluation agrees with N; fourth_zero and fifth_one prove coefficients4 and5 are0 and1. This creates a precise algebraic link to the existing formula model.

## Point maps and boundaries

The four generic coordinate inverse theorems establish both x and y round trips under alpha!=0. The point_forward and point_backward theorems transport the curve equations through the scale alpha^3 and polynomial scale alpha^6. Together with normalized_equation, these describe a bijective change of affine curve coordinates. The stored target coordinates and runtime binaries are not changed.

The historical helper also transforms degree-d Mumford u by substitution divided by alpha^d, and v divided by alpha^3. This packet does not yet prove that helper's polynomial loops implement substitution, preserve Mumford congruence, or induce the required Picard-group map. It also does not instantiate a global projective scheme isomorphism or transport the squarefreeness hypothesis of WeightedProjSmoothFinal. Those are the next bindings.

The pre-existing WeightedPic0Bridge constructs an isomorphism of the actual weighted-Proj infinity stalk with a chart localization. WeightedProjInfinityDVR supplies its DVR structure and local parameter facts. Their own module comments explicitly do not assert a Picard group or global divisor-class equality. This packet reuses their model boundary rather than claiming to replace those results.

## Exact checks

Twelve main theorem declarations: cleared_source, source_expansion, normalized_equation, inverse_x, forward_x, inverse_y, forward_y, point_forward, point_backward, normalized_eval, fourth_zero, fifth_one. Eight fixed controls: F(0)=1, F(-1/2)=0, coordinate values for t=3/y=7, rejection of omitted ordinate scaling, coordinate round trip, and the historical gate's shift and ordinate scale. The t=3/y=7 pair is a coordinate arithmetic fixture, not asserted to be a point on the curve.

The independent stdlib derivation script uses Fraction pairs for Gaussian polynomial arithmetic, not floating point or finite-field sampling. Its expansion.json is descriptive exact arithmetic; the field identities are then separately checked by Lean. This is producer cross-checking, not independent reviewer evidence. A separate axiom readback names every main/control declaration. First proof attempt failed on power-rewrite/tactic sequencing; attempt2 repairs this and preserves the initial receipt. Nonfatal linter warnings are retained.

## Sources and scope

Before implementation: read literal three_leads.rs:1094-1171, historical genus2_model_gate.rs:14-56 and75-88, and committed WeightedPic0Bridge/WeightedProjInfinityDVR/WeightedProjSmoothFinal sources. Consulted official Mathlib Polynomial Eval/Defs documentation for composition/evaluation semantics: https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Eval/Defs.html . No new normalization technique is claimed.

No ecdb curve fact was recomputed. All work is symbolic source algebra and proof checking. No scientific campaign, target generation, timing or recovery was run; cost_ratio remains null and the expired budget is unchanged. No independent binary/kernel rechecker or independent source review is claimed. Bind global model/Picard transport next before using the projective geometry to certify the actual transfer.
