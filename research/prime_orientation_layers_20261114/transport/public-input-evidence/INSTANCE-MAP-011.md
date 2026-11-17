# Actual curve equation and remainder filter

1. Structure: Field K, actual f:K[X], outer Y polynomial over K[X]; no nonsingularity or cubic claim needed for these identities.
2. Existing laws AS STATED: monic_X_pow_sub_C, modByMonic_eq_zero_iff_dvd, eval₂_modByMonic_eq_self_of_root, evaluation ring laws. No generic division/evaluation theorem re-proved.
3. Objects: curveEquation f=Y²-Cf; curveRemainder f p=p%ₘcurveEquation f; public evaluation is eval₂(evalRingHom x)y.
4. Chain: monic/nonzero actual curve equation; remainder zero iff curve divides p; remainder nonzero iff not divisible; actual curve evaluation zero iff y²=f(x); remainder agrees at every such point.
5. Does not establish nonzero remainder for the auxiliary, preserve full ambient jet multiplicity, prove norm/scalar extraction or compute runtime. Any such bridge stays open.
6. Binds the known-equation rejection filter to actual constructor and public points, not a supplied favorable reduction map. Literature and library review in journal; local default-limit proof only. [pcc:unverified]
