# Special-chart evaluation development contract

Continuation of the unchanged WEIL_PROTOCOL and fixed actual P-192 d7 kernel1. Source: Tian section5, formal-parameter regularization of determinant Weil functions. No paid job or performance trial.

The regular evaluator divides by Res(U,u_P), which vanishes when the test divisor shares support with the kernel divisor. This does not imply a pole of the Weil function: a generic degree3 class is outside the theta divisor. Evaluate the cancelled rational expression on a formal curve arc instead.

Use the algebra K[x]/U, with squarefree cubic U. Simultaneously deform the three points by x'=2y, y'=f'(x). Recover U(t) as the characteristic polynomial of multiplication by x(t), and V(t) by interpolation. Check the deformed Mumford equation to precision24. This vector field also moves branch points through y, avoiding an invalid assumption that V is invertible modulo U.

Write the determinant ratio by Cramer's rule as C(t)/D(t). The function is C(t)^3*Res(U(t),A+B*V(t))/D(t)^3; cancel valuations and take the constant coefficient. The denominator's valuation is at most18 for three squarefree support points moving with this vector field, so precision24 is a fixed bound, not a retry-dependent accuracy. Reject an unresolved valuation or negative valuation explicitly.

Qualify at all26 nonidentity kernel classes: evaluate the matching and opposite indexed functions through the singular support chart and compare symmetry after normalization. Check the regular evaluator against arc evaluation on the three original regular classes for all27 functions. Include a zero-denominator/no-motion arc as a rejection control. Do not claim a plane quartic or target divisor map from these tests.
