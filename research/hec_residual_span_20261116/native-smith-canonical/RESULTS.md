# Native Smith canonical quartics [pcc:unverified]

Freeze727541cc8d3. INPUT hashes pass; sole600CPU-bounded invocation exits0. No source substitutions: H/F61 is y²=x⁷+x³+5x+1, the original mixed-term genus3 source. All native checks below pass; independent review remains open.

## Finite singularities and canonical sections

The degree11 residual discriminant factor has irreducible residue degrees2,2,1,6 for map0 and1,3,2,1,4 for map1. Every factor is independently Frobenius-certified and their product exactly reconstructs the original residual. Over each residue field, gcd(P,P_rho) is linear. The recovered point satisfies P=P_t=P_rho=0 and nonzero Hessian determinant, giving one geometric ordinary node per conjugate. Both maps therefore have11geometric ordinary finite nodes.

Evaluating the14 verified infinity adjoints at these nodes gives rank11, leaving3sections on each model. All original finite constraints vanish on every resulting section. Combined with infinity delta22 and the prior normalized genus3 deduction, the11nodes exhaust the remaining delta budget33; no further singularity conditions are missing under those dependencies. The degree7 adjoint system is therefore canonical. This conclusion depends on the prior exact infinity resolution and generic branch/connectedness deductions, not on rational point counts.

## Exact canonical relations and geometric smoothness

For each map, degree2 relations have kernel dimension0, degree3 dimension0, and degree4 dimension1. Relations are computed by exact uniform pseudo-reduction modulo the original quartic in rho over F61(t), not point fitting. No quadratic canonical relation means the genus3 normalized curve is nonhyperelliptic. The unique quartic is its canonical image under the standard canonical-map theorem.

Native Buchberger arithmetic verifies the unit ideal (F,F_x,F_y) on all3projective affine patches of each quartic. This excludes singularities over the algebraic closure, not just F61-rational singularities. All6patches pass within40-41processedpairs and19-20basis elements. The separate smooth Fermat control and factorization control pass.

For map0 an explicit equation is

    Z^4 +16YZ^3 +37Y²Z² +35Y³Z +50Y^4
    +53XZ³ +21XYZ² +39XY²Z +46XY³
    +19X²Z² +14X²YZ +11X²Y²
    +10X³Z +56X³Y +2X^4 = 0  (mod61).

Both quartic equations and all canonical sections are retained in CONTROL.txt and machine-readable MODELS.json. The two maps arise from the same source kernel and must not be counted as two independent isogeny escape routes; no projective equivalence computation is claimed here.

## Corroborating checks and scope

Both smooth quartics have66Fp-rational points, equal to the original H point count. All55previous map0 and54previous map1 chart points have nonzero canonical coordinates, satisfy their quartic equation and map injectively. Equality of a single field point count is not a zeta-function or isogeny certificate; the transport interpretation still rests on Smith's correspondence and the earlier generic identification.

This completes a native curve-model construction for the two admitted F61 maps, with exact algebraic smoothness checks. It does not implement the full Jacobian map on original public source/target divisors, exceptional-support handling, quartic class arithmetic, target solving or matched cost comparison. It does not establish a10x gain or a new mathematical attack. The construction follows the known Smith route. Original p211/p401 twist obstructions, genus2/genus4 obligations and full10xgenus2+goal remain open.

Source: Smith section7 https://arxiv.org/abs/0806.2995 ; canonical adjoints background https://magma.maths.usyd.edu.au/magma/handbook/text/1466 . No Magma/Python/service execution was used. Git custody while S3 unavailable;noS3polling/cloud/merge. Prefreeze duplicate helper/derivative fixes recorded in SESSION; unused copied helper warnings only.
