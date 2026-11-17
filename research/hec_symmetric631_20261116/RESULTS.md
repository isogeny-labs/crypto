# Exact quadratic symmetric inverse algebras

Both registered inputs pass on one execution, exit0/stderrempty. Freeze84e9a4f90dee/checkpoint20261116T025433Z precedes execution. Full degree12parametrizations and original equation residuals are checked before symmetric reconstruction.

D1: u0 satisfies u0²+607u0+445=0, roots43and612. B: u0²+570u0+460=0, roots259and433. In each complete degree12algebra, all six divisor support/interpolation coefficients are exactly affine in u0, with coefficients in RUN.txt. Each quadratic is squarefree. For each of its roots r, gcd(e,u0-r) has degree6. A corrupted quadratic relation fails in the full algebra. The reconstructed rational divisors agree with the preceding recovery record.

Thus the coefficient subalgebra has dimension exactly2, and the ordered-point parametrization covers each symmetric divisor with degree6. Nonconstant u0 and its squarefree quadratic establish the dimension; explicit affine expressions establish generation of every required coefficient. This is a finite exact algebra certificate for two inputs, not a generic inverse-degree theorem or formal Lean proof.

This exposes a concrete mathematical simplification: formulate inversion directly in the six graph-divisor coefficients and eliminate ordered-point permutations before solving. The current checker still reads a solved degree12parametrization, so it is postprocessing evidence and supplies no solver speedup. A future direct formulation must construct symmetric section constraints independently, handle saturation/base cancellations, recover the same classes with adjacent-sign checks and undergo charged comparison. Construction/other costs remain material; a sixfold multiplicity reduction alone does not establish10xoverall.

The earlier failed full cold cost gate remains unchanged. Global correspondence, genus2/4and the complete10xobjective remain open.
