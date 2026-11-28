# Nonconstant typed generator control

2026-11-16, before this control's data. The explicit-constructor v1 leaves G(1) symbolic, so its rational-looking answer does not qualify finite-field arithmetic. The separate type diagnostic shows the documented GF(p,2) constructor creates a field and assigns a generator g. Use that constructor in a fresh CLI process, then solve x+y-g-1=0,xy-g=0 with actual nonconstant field coefficients.

Require both prescribed roots(g,1),(1,g), reject the nonroot(0,0), and retain the complete coefficients and their type/decomposition. The expected reduced lex basis is {y^2-(g+1)y+g,x+y-g-1}, up to order. Do not infer independent qualification from internal substitutions; native verification of the coefficient representation is still required before inverse-system use. Keep v1 and type diagnostics unchanged. No timing claim.
