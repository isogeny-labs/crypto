# Explicit coset quotient: native development correctness contract

This component extends the exact P-192 d7 fixture. The user's full end-to-end attack request remains open. No performance or statistical experiment is conducted here.

Inputs: previous six actual J(H)[3] generators, original field/model witness, eight RREF kernel bases. Preserve the source files pinned by earlier PCC records; generate build-only arithmetic headers as in check.sh.

Implement a quotient element as the exact coset D+K in J(H)(Fp8), canonically represented by the lexicographically least full Mumford encoding among its27 classes. Cache orbit labels without changing the equivalence relation. This represents the image of J(H)(Fp8) in the algebraic quotient, not all A(Fp8) or A(Fp). Do not call it a plane-quartic model.

For all eight kernels, check:

- Actual27-element subgroup closure/order, kernel coset collapse and an outside point rejection.
- All729 supplied 3-torsion classes partition into27 cosets, each of size27.
- Quotient addition, negation, inverse, associativity fixtures, Frobenius-fixed transferred points and actual representative independence of [3]D.
- Polarized dual candidate psi([D])=[3]D is well-defined; psi(phi(D))=[3]D and phi(psi(A))=[3]A on the implemented image.
- F=phi*pi^*, T=pi_*psi satisfy TF=[6] for public fixed P-192 source points, including origin and hash-derived full-width scalar correctness fixtures. These are known-answer map checks, not a DLP recovery test.
- Explicit inverse6 recovers P or Q from T(F(P)) or T(F(Q)); the unknown scalar is not recovered by this operation.
- For rational source divisor fixtures, projected transported-base relations are exactly scaled by3: T(phi(sum n_i D_i))=[3]pi_*(sum n_i D_i). Verify representative choices do not change these outputs. This does not compare a new target-curve factor base.
- Corrupted kernel, source point and claimed target fail. Keep failures and pre/post manifests; no clocks, rho ratio or scaling inference.

Success is an executed quotient/dual transfer component on the actual subgroup. Target-curve reconstruction, arbitrary quotient rational-point coverage, a restricted-base relation algorithm, blind-target solving and full cost measurements remain separate requirements.
