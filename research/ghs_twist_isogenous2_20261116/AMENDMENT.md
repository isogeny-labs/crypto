# Dual-isogeny construction amendment

The frozen protocol asks for the degree-two dual. A direct Sage 10.10.rc0 call to `phi.dual()` on the characteristic-three isogeny raised `NotImplementedError: computation of dual isogenies not yet implemented in characteristics 2 and 3`. The failed call did return the forward degree-two codomain before that exception. The implementation below constructs the unique rational reverse degree-two isogeny and an explicit codomain isomorphism back to E, then checks its composition equals multiplication by `+2` on the saved order-r point. If it equals `-2`, it negates the reverse map. This is a routine implementation substitution; the exact subgroup identities in the protocol are unchanged.

## Orbit-link discovery and frozen follow-up

The first successful construction returned `E2: y²=x³+x²+(2a³+2a²+1)x+(a⁴+a³+1)`. Translating x to remove the linear term gives the isomorphic model `y²=x³+x²+(a⁴+2a+1)`, the negative quadratic twist of the previously screened positive mask **167** curve `y²=x³+2x²+(2a⁴+a+2)`. The positive mask-30 source has a rational degree-two isogeny to the mask-167 isomorphism class. This links the two previously distinct Frobenius orbits in `../ghs_odd5_carrier_20261115/DEGREE7_FINDING.md`.

Before the new `orbit_bridge.sage` run, predict the five pairings by cubing coefficients:

`(30,167), (194,182), (103,230), (139,3), (233,27)`.

For each pair, test both a2=2 positive and a2=1 negative twists over the fixed K, confirm respective orders 218 and 270, enumerate rational degree-two isogenies, and require a codomain K-isomorphic to the paired target. Record all rows, j-invariants, kernel polynomials, source hash and exact pass/fail status. A mismatch falsifies the orbit-wide extension. Even if all pass, this establishes a newly certified isogeny-assisted eligible orbit in this project, not a new method: [Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) already includes isogeny-assisted cover attacks. The one mask-167 negative s=7 target has a transported cost using the saved exact suffix; the other four are not separately benchmarked.

The first orbit-screen invocation completed its isogeny checks but failed while serializing the source hash because the script contained `..read_bytes()`; Sage parsed this as an ellipsis expression and raised `NameError`. The one-character typo was corrected before the next run. No JSON receipt was produced by the failed invocation.

The second invocation reached serialization but failed because Sage integer literals in the pair list are not directly JSON serializable. The output-only call now passes `default=int`. The pair list, isogeny tests and assertions are unchanged; both failures are preserved in this amendment rather than counted as scientific nulls.

The first saved point pair was on Sage's unnormalized two-isogeny codomain E2. For the exact prior mask-167 curve, the construction now also saves `P167,Q167` on `E167^-: y²=x³+x²+(a⁴+2a+1)` via the explicit coordinate shift `x167=x2-(2a³+2a²+1)`. Attacker-facing preprocessing begins with those normalized points, reverses the shift, and evaluates the same checked reverse isogeny. The original E2 point pair remains in the receipt as a bridge check; this is a target-presentation amendment, not a change of subgroup or scalar.

## Native matched-rho follow-up, frozen before execution

The stitched cost in `cost.json` reuses the earlier E30 negative rho suffix after the evaluated reverse isogeny. To measure a native same-source comparator as well, adapt the earlier order-seven-Frobenius rho implementation to the **exact normalized** E167 negative curve and saved `P167,Q167`, preserving its 16 jumps, seed `20261116`, seven-Frobenius orbit canonicalization, seven-bit distinguished predicate and 100,000-step checkpoints. Confirm the Frobenius eigenvalue from P167 and `Q167=12345P167`; stop at 12,000,000 steps if no collision and preserve the censored receipt. Store a separate checkpoint and result in this directory. Do not overwrite the earlier rho receipt or silently pool timings. A completed native run gives an additional measured comparator; a capped run remains censored and the stitched transport cost remains the only complete comparison for mask 167.

## Direct degree-fourteen composite cover, frozen before execution

The established negative genus-five cover `Y^-→E30^-` has degree seven and exact formula `u=2A/B, v=yC/B²`, with A,B,C in the source-hashed mask-30 receipt. The forward degree-two isogeny `E30^-→E167^-` should compose to a separable degree-fourteen map from the same carrier to the normalized mask-167 curve. `compose_cover.sage` will reconstruct the old cover and use the explicit fixed rational formulas for the two-isogeny, verify the full function-field identity over K(x), check the reduced x-map degree 14, and save all coefficient vectors. A failure falsifies the claimed direct composite cover, even if the pointwise isogeny tests pass. This adds exact cover equations for the second orbit; the composition technique itself is established prior art.
