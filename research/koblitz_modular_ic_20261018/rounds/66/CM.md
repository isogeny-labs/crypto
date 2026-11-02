# R66 second descending Frobenius lattice

The accepted `second-frobenius66-01` tests all 80 classes of `V1/O` using
the saved R65 rank-32 basis and the full R52 multiplication table. The
polynomial order inclusion has quotient length 80. Clearing every
denominator gives 229 binary rows of rank 48 and a 32-dimensional
square-survivor kernel. Every remainder is reconstructed from its binary
coefficients, all kernel products vanish, and a changed kernel is rejected.

The complete second PID lattice `V2` has rank 32. Its quotient lengths
are `dim_F2(V1/V2)=48` and `dim_F2(V2/O)=32`. Both module inclusions and
basis inverse products pass, the old order is included polynomially, and
the square of every one of the 32 `V2` basis columns lies in `V1`.
A changed inverse is rejected. The terminal result line reports 2,542
nonzero rational square-image entries. Exit 0, the exact marker, and
zero Singular question-mark error lines jointly pass.

The saved quotient, full binary map and kernel, and polynomial/field
second-lattice bundles are source-bound outputs. This is the second
descending Frobenius step, not a stabilization result. The next complete
calculation tests all 32 classes of `V2/O` against `x^2 in V2`, rebuilds
`V3`, and repeats until equality. Proper exceptional branches, divisors,
and the level-77 order-11 map still require the resulting normalized
model.
