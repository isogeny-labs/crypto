# Riemann candidate reconstruction contract

Input is the completed actual P-192 d7 kernel1 quotient theta-square vector from QUOTIENT_THETA_PROTOCOL, not a published destination curve. Proceed only if all28 odd values vanish and all36 even values are nonzero.

Apply Tian section4.1 equations4.3 and Theorem4.2: reconstruct three additional Aronhold bitangents from theta-square products and checked square roots, solve the two3-by3 systems, and form the homogeneous Riemann quartic. Check every derived squared bitangent coefficient against its input theta ratios. Do not choose a new theta vector if an identity fails.

Check all seven advertised bitangents by exact restriction to a square binary quartic. For each line, at most five fixed basis shifts avoid a zero leading coefficient. Check geometric smoothness by the rank of the degree7 homogeneous Jacobian-ideal matrix: the three cubic partials times all15 degree4 monomials give a45-by36 matrix; rank36 certifies no projective singularity. The singular x^4 form and a corrupted bitangent must reject.

Emit exact coefficients in the declared Fp24 tower and test whether the normalized coefficients already lie in Fp. A quartic over the auxiliary tower is only a candidate geometric reconstruction; same-field descent/twist, agreement with the exact polarized quotient, executable divisor map, kernel proof, and destination restricted-base DLP remain open until checked. No paid trial, runtime claim or source-curve substitution.
