# Polynomial inverse accounting correction

The authoritative polynomial backend at ../feature-native-hgcd-roots/polynomial.rs defines Poly::inverse using extended Euclid on signed integer residues. It does not implement that inverse by exponentiation. Poly::inv_series skips inversion when its constant term is1; the remainder-tree divisors used by this compiler are monic.

Some preceding component derivations and reports (including the half-GCD roots, power-fold and fold-fiber reports) described general polynomial-backend scalar inversions as exponentiation-based and charged a modular-product bound on that basis. That mechanism description is incorrect. Their native arithmetic and retained correctness outputs are unchanged. Historical attested artifacts are preserved, not rewritten.

For general root updates, state field-operation counts with scalar inversions explicit. Each actual backend inversion has O(log r) Euclidean iterations under the usual integer-step accounting, with quotient/division and growing integer coefficients; this is not a claim of O(log r) modular multiplications. Bit complexity and implementation costs require their own model.

The new stride compiler uses monic remainder trees (unit reciprocal constants) and one explicitly exponentiated batch inverse for its nonzero recurrence denominators. It also uses exponentiation for a^h and root-of-unity powers. Those visible exponentiation costs are O(log r) modular products. Correlation constraint scaling similarly computes one inverse by exponentiation. Later generic gcd updates still use the Euclidean scalar inverse backend. Keep these paths separate.
