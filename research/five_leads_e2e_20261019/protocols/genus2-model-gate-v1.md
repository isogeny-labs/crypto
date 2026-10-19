# Genus-2 formula-model applicability gate

Read the 21 field-size and 21 exact-order Scholten curve records from the
original three-leads fixture specs. Do not replace curve identities. For
each quintic `f(x)=L*x^5+a4*x^4+...` over `F_p`, require `p>5` and `L!=0`,
then transform `x=L*u-a4/(5L)`, `y=L^3*v`. Check the resulting equation is a
monic quintic with zero `u^4` coefficient, which is the equation form of the
[complete projective group law](https://link.springer.com/article/10.1007/s13389-024-00351-7).
For the first 100 nonzero source-generator multiples on each model, transfer
into the original Jacobian, transform the Mumford divisor and validate it on
the new curve. Check that adding the transferred generator before and after
the coordinate change gives the same normalized divisor for each sample.
Record the exact coefficients, scaling data and sample counts.

Passing only certifies equation-form applicability and sampled affine
divisor conversion/group-law compatibility. It does not implement projective addition, normalization,
root extraction, optimized source rho or the 1,260-target replay.
