# Actual dense identity-origin matrix instance

1. Structure: existing square-matrix RingHom.mapMatrix over integralRationals, Q and Q[X]; actual dense kernel and already constructed integral inverse.
2. Laws AS STATED: RingHom map_mul/map_one; Polynomial eval_C/coe_evalRingHom; Subring.subtype_apply; actual NonscalarOrigin.kernel_origin/kernel_nonzero_node; integral_two_sided_inverse157 and integralOrigin_coe154. No missing generic matrix map/product law.
3. Object: coefficientMap=Polynomial.C.comp integralRationals.subtype, constantInverse=coefficientMap.mapMatrix actual integralInverse; normalizedKernel=actual K*constantInverse. Old evaluation matrix explicitly actual completed W evaluated at t.
4. Chain: pointwise constant inverse evaluation -> subtype.mapMatrix inverse; pointwise K0 -> subtype.mapMatrix origin; matrix map_mul -> actual inverse product -> map_one gives J0=I. For t!=0 in source grid actual Kt=oldWt, so same map_mul gives changed node Jt=Wt*inverseCoefficients. Expand only actual new aliases and named public component equations.
5. Missing instance/evaluation/product endpoints only. No favorable inverse/origin/node premise or unrestricted Boolean kernel composition. Actual new coefficient matrix is constant in T; no new scalar/ring extraction claimed.
6. Inheritance: actual integral unit157, genuine inverse support171 remains separate; generic RingHom coherence supplies product equations. This is physical originI but not Boolean target/child-cost, compact weighted modes or expected bit complexity. [pcc:unverified]
