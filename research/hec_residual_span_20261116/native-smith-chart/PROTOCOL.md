# Rational Smith chart reconstruction [pcc:unverified]

Fixed inputs: both original p=61 maps and delta polynomials in native-smith-affine/CONTROL.txt. No substitutions, no other fields. Git custody under owner authorization while S3 is unavailable: push this protocol, source and INPUT hashes before the sole invocation. Bound 600 CPU seconds; overflow checks enabled. Retain all failures, no unchanged rerun.

For every t in F61, exclude any zero delta0, delta1, delta2 or delta4, and exclude quartic projection polynomials with repeated roots. This is deliberately a restricted chart, not the complete normalized curve. Enumerate every nonzero rho in F61 and every sigma in F61. Derive tau uniquely from the x-squared coefficient equation of b squared = F modulo G, b=tau+sigma*x+rho*x². Verify the full congruence and the delta sign equation. Require exactly one b for each root of delta4*rho^4+delta2*rho²-delta1*rho+delta0, and none for nonroots.

Retain every reconstructed point, split/nonsplit trigonal classification, excluded-parameter counts and rational source incidence checks. Check inverse Veronese reconstruction and that changing b's constant coefficient by one fails the congruence. Original split-fibre coverage was only 8 points per map; broader chart coverage may include nonsplit fibres, but no minimum gain in coverage is a success condition. Zero new coverage remains evidence.

The finite bijection is only for F61 points on this restricted chart. It does not prove birationality over the function field, global smooth normalization, genus, nonhyperellipticity, a Jacobian operation, complete divisor transport or speed. Canonical adjoints and exceptional places remain open. Source: Smith https://arxiv.org/abs/0806.2995, equations 8-11 and section 7.
