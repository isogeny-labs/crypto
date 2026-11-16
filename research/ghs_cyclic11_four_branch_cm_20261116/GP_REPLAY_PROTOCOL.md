# Frozen independent elliptic order replay

16 November 2026. Use PARI/GP finite-field and elliptic routines to
independently count the two exact source equations found by the native
Rust search. Fix `K=F5[z]/(z^5+4z+1)` and the models

- `E2: y^2=x^3+x+(1+z^2)`;
- `E24: y^2=x^3+2x+(2+z+2z^3)`.

Record `ellcard` for each and `polisirreducible` for the degree-ten Weil
polynomials `X^10-tX^5+3125`, `t=2,24`. Freeze this script before replay,
retaining stdout, stderr, tool version and source hash. This independently
checks the source curve orders and restriction simplicity. It does not
construct the isogeny or subgroup map to the quotient Jacobian.
