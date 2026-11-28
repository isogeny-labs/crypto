# Powered modes in canonical signed recovery

## Section algebra in the actual recovery quotient

The active canonical ring is Q=F_r[W,V]/(g(W), V^2-WV+1). A value is represented as a(W)+V*b(W), with both polynomials reduced modulo monic g. For any v in Q, form Q[X]/(X^m-v). As in the preceding residue-section construction,

    (X-1)^h mod(X^m-v) = sum_(e=0)^(m-1) F_e(v) X^e,
    h=(r-1)/2.

This holds in arbitrary commutative quotients, including zero divisors. `QuadRing.sections` implements the same binary power directly with Quad coefficients, not through a field extension or an inferred scalar root. It retains all m sections, with one wrap by v whenever i+j>=m. It never reads the binomial coefficient streams.

Before allocating those arrays it checks the explicit logical capacity 2*m*deg(g). Overflow or excess returns Error::Limit. The capacity bounds the maximum represented output coefficients, not peak allocation or process RSS; acc, base, output and polynomial-product temporaries coexist. Invalid values return Domain. All prime-field and exact-order assumptions of RecoveryModes remain inherited.

## Direct signed product

For normalized hidden scalar x=k/b and image v=x^m, public weighted observations satisfy

    f_plus = A*x*F1(v),
    f_minus = A*x^(m-1)*F_(m-1)(v), A=m*b^h.

Multiplying gives the direct equation

    A^2*v*F1(v)*F_(m-1)(v) - f_plus*f_minus = 0.

The implementation does not use x: it evaluates v=scale*V in the anchored quotient, where scale is computed from public radii. Once all sections are available, this equation uses two further Quad products and scalar scaling. It does not require reciprocal evaluation, a scale inverse, or the phase factor epsilon. The original phase restrictions remain part of the canonical state and its lift.

For later radii, sections are computed in the old quotient before mode0 filtering. Apply A*F0(v)-f0 first. If g_new divides g_old, every old representative maps correctly into the new quotient. Thus surviving branches can reuse F1 and F_(m-1); products are reduced by the updated ring. This preserves the same roots as the streamed signed-product constraint. It may compute unnecessary sections if mode0 removes a branch; that work is part of the cost.

The zero candidate uses exactly the preceding condition f0=A and f_plus=f_minus=0. Endpoint handling, phase maintenance, canonical selection of orientations and CRT operations remain unchanged.

## Capacity fallback and terminal phase

`PoweredImage` stores an explicit caller-supplied slot cap. If sections do not fit, that branch executes the existing scaled mode0 and reciprocal signed-product code. This is a capacity rule, not a measured performance selector. Cap0 deliberately selects the streaming path everywhere. No default adapter is changed.

The terminal phase helper also avoids the F1 scalar coefficient scan when capacity permits. It embeds the known scalar v as the constant Quad(v,0) in the degree-one W quotient and evaluates sections. The returned F1 component is verified to be scalar. It retains the original denominator-zero behavior, normalization and x^m=v check, followed by the adapter's public-point verification. It falls back to the original phase helper on capacity exhaustion. No inverse of v is needed.

The generated optional adapter changes the state type, passes power_slots and calls phase_powered. Its source queries, call cap, eligibility fallback, history replay, pullback, extra-sign schedule and final verification are otherwise inherited. Since constraints are equivalent, fixtures compare routes and full recorded source stats as well as recovered values.

## Costs retained

Let B(h)=bit_length(h)-1+popcount(h). Dense section powering uses B(h) algebra products, each performing m² Quad products and m(m-1)/2 wrap products. A Quad product itself includes three polynomial products and reductions; coefficient copies, allocation, additions and all setup count. The signed equation then uses two Quad products. Scalar phase currently reuses this general implementation and can be specialized later; no such saving is assumed.

Initial ReciprocalMode::build still constructs the mode0 candidate representation from its coefficient stream. Public source extraction, initial phase construction, canonical filtering/lifting, fallback branches, scalar inverses for public-radius normalization and final verification remain. This patch removes scans from the eligible powered evaluation and terminal phase paths, not from every operation in recovery. The prior large-prime scalar section test is not an end-to-end large-prime recovery test.

## Proof and qualification scope

Product.lean proves the conditional commutative-ring product identity assuming x*y=v. Lean4.34.1, bundled Std, commit5045d0056413266e57c625dcd7c365b10e377c52; axioms propext, Classical.choice, Quot.sound. The substitutions y=x^(m-1), weighted projection, quotient-module construction and runtime refinement remain written/finite-check obligations. No separate lean4checker replay.

Sources checked: ../feature-recurrence-compression-audit/DERIVATION.md and power.rs; ../feature-native-signed-image/{signed-image.rs,signed-adapter.rs,canonical.rs,quadratic.rs}. KB unavailable (MCP absent, wrapper no executable); exact source fallback used. This is integration of the established section identity, not a novelty claim.
