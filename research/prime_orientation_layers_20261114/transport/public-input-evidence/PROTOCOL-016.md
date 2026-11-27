# Actual growing point-jet and residual construction

## Question and prior sources

Can the actual coordinate function, rather than a supplied kernel or successful divisor witness, satisfy a growing family of point-jet constraints? Existing sources checked: NormGroupBridge.lean pointIdealsCoprime, pointQuotientEquiv, quotientFinite; public Mathlib Polynomial.OfFn, power-series Trunc and ideal quotient CRT; committed CryptoLean.PowerSeriesPolynomialJet and InvertibleIdealResidual/InvertibleIdealCodimension. These are standard interpolation, local algebra, CRT and ideal codimension methods, without novelty claims. KB unavailable in this runtime; source fallback only.

## Domain and prediction

Work over a field, short Weierstrass curve, nonzero two and nonzero ordinates at nonsingular input points. For d>=0 use actual A coefficients of length d+1 and B coefficients of length d, mapped to A+B*Y. Jet levels n_i are nonnegative. Strict sum n_i<2d+1 should derive a nonzero actual constrained function with norm degree<=2d+1. For the global ideal product require injective coordinate pairs; repeated pairs are not treated as coprime. Predict integral residual J and sum n_i+dim(R/J)<=2d+1.

## Validation and falsification

Run one pinned read-only candidate at a time with default Lean limits and public imported APIs. Require exit0, zero warnings and standard printed foundations for all new endpoints; inspect real logs. Elaboration failure rejects the whole candidate; preserve source/log/exit and revise only demonstrated interface failures. Verify exact source and dependency bindings. No favorable kernel, full-rank, residual or factorization premise may substitute for a construction. Include empty families, zero levels, d0 in theorem domains.

## Scope and resources

Proof validation only; no solver, timing, scaling, native cryptanalysis or paid campaign. A noncomputable branch/kernel/residual existence proof does not establish an executable public finder, useful scalar labels, residual splitting, retry probability or expected bit cost. The original weighted W-jet/nonscalar-origin interface is separate. All records unsigned local, outside historical shared Transport53/57 audit/fresh. Preserve prior immutable archives and protected canonical PCC history.
