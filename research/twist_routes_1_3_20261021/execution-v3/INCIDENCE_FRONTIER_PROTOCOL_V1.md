# Incidence extension and redundant-curve-mask hypothesis

The earlier 270-cell result remains unchanged. Extend its finite geometry
grid to H,K in {2,4,8,16} and distinct lambda in {1,2,3,H}: 600 cases over
the same ten original curves and original target label 0. The original 270
must reproduce their exact mask coefficients. Use independent Newton finite
differences instead of the earlier Lagrange construction.

Two questions are distinct. First, lambda=H makes the integer map r-Hu
injective when HK<=p, avoiding the collisions of small fixed lambdas. Verify
the cardinality and retain all wraparound cases. Second, on a finite box
with a nonzero rational target y, classical F3 discriminant algebra implies
that arity-two rational-coordinate solutions lift on the source curve.
Test whether the separate curve-residue part of the mask is redundant once
the relation equation is present. The proposed reduced system uses only
r in {0,...,H-1}, u in the previously selected twist-abscissa subset U,
x=r-lambda*u, and F3(x1,x2,target_x)=0.

For every case enumerate every pair from the full unfiltered image R-lambda U.
Check that every F3 pair is in the original rational base, construct both
signed y-coordinates by the explicit discriminant formula and verify their
sum. Compare the full canonical pair set against independent signed source-
group addition on the rational base. Recheck every interpolation grid value
and a corrupted constant control. Save exact sets, witnesses and hashes.

Report maximum generator degrees for the explicit product-membership,
full-mask and reduced-U systems, including F3's degree four. Degree reduction
is not a solver-cost or fastest-reference result. The generic ordinary
encoding using the same supplied U has identical equations: no intrinsic
twist-specific advantage is inferred.

Also search for a concrete arity-three mixed tuple T+A-A with anti-rational
A in the unfiltered image, and verify that its first affine prefix has
nonrational x. Such a witness records why arity-three algebraic-closure
ideal equality cannot be assumed from arity-two reasoning. Its twist sum
cancels separately; it is not a new mixed relation for the original DLP.
Keep finite-Fp and algebraic-closure tasks distinct.

Generate independent PARI replay of every retained arity-two sign lift and
every selected Fp2 mixed-prefix witness. Check captured exits, hashes,
completion marker and clean-error gate; the explicit field-square identity
has its separate Lean proof and does not complete a formal group-law bridge.

Commit source/protocol before execution. One native worker, 4 GiB address
space, 1,200-second native cap and 1 GiB raw-output cap; PARI replay 300 seconds.
No new E28 label, paid host, asymptotic/novelty or calibrated-IC claim. This
finite construction test does not replace the original E30 solver-cost run.
