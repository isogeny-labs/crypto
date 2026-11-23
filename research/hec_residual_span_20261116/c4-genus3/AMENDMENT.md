# Prospective v2 after retained fixed-field implementation failure

Original frozen92bd5d0ea64 exits101at p13,a5=0,i5. Diagnostic a1eeaf8593b exposes twistcount2. Error: y^p=-i*y implies y^(p²)=-y, so nonzero fixedpoints need Fp4, not the Fp2domain searched by v1. This is not a determinant counterexample. Original source/failure remain intact.

Newv2 uses x=zeta*t in Fp2 and h=f(x)=y². If h=0 there is one fixed y; otherwise there are two precisely when h^((p-1)/2)=-i. Indeed either algebraic square root y satisfies y^(p-1)=h^((p-1)/2). This tests the geometric fixedpoint condition without restricting y to the wrong field. It shares the elementary exponent criterion with previous Gaussian work; do not claim an independent Fp4enumeration. Ordinary Fp/Fp²/Fp³ counts and gcd/model/candidate/fullpolynomial code unchanged.

Freeze/push v2source/binary/hash before once-only gate under600CPU seconds; same fixed6models,bothorientations,repetitions,budgets. No model replacement or outcome selection. Retain all newoutput and original failedgate/diagnostic. No performance or scalar-action claim. This is an explicitly amended correctness experiment, not overwriting/relabeling the failed run. S3/cloud unused.
