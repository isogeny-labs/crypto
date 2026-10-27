# Reduction by the inverse constraint

Frozen before exporting the implicit variants. Reduce the relation polynomial
by the explicit inverse equation before handing it to a solver. In the phi
arm replace U^2 W by 1. In the doubling arm replace U^12 W by 1+C W, where
C=B^4. Apply the replacement repeatedly until no monomial is divisible by
its left side. Retain the inverse equation in the final system, so these
substitutions preserve its variety. Record both unreduced and reduced
polynomial statistics and verify evaluations on every actual source.

Both mixed arms receive this normalization. It is a polynomial
representation experiment, with the same finite-domain constraints and
complete physical signed-set checks as the parent supplement. No solver
gain follows from a term or degree count.
