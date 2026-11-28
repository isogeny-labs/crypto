# Independent coordinate-lattice derivation review

Reviewer checked directory absence before creating this fresh review. This records a static derivation; no scientific code was run.

In a fixed polynomial basis of K=Fp^r, multiplication by a is the linear map M(a). Expanding the two field equations into coordinates gives 2r independent congruences because the t and u output coordinate blocks contain identity matrices. The primal kernel in Z^(7r) consequently has index p^(2r). The affine constant gives an offset, not an additional homogeneous equation.

A coordinate functional gamma pairs with M(a)x as gamma^T M(a)x=(M(a)^T gamma)^T x. Thus the dual constraints are lambda_i=M(a_i)^T gamma+M(b_i)^T delta. The 5r lambda coordinates prove surjectivity of the dual constraint map and determinant p^(5r). Membership of every row plus that exact absolute determinant proves a full proposed basis equals the kernel. Determinant alone or membership alone is insufficient.

For r=2 the primal and dual determinants are p^4 and p^10. A concrete wrong-transpose witness is GF9 with theta²=-1, a=theta, gamma=(1,0), x=theta: correct pairing is -1=2 while multiplying the scalar-encoded gamma as a field element gives 1. In GF25 with theta²=-2 the corresponding values are 3 and 1. These witnesses prevent an accidentally symmetric test from accepting the wrong representation.

An exact unimodular row operation preserves the kernel; positive diagonal metric changes do not alter the unweighted lattice. Every metric still requires fresh sound coordinate bounds and cache keys must include field modulus and basis. This does not transport ordered integer interval envelopes or integer/Sturm cubic isolation into extension fields. Root/support checks must use actual finite-field arithmetic, explicit domains and full multiplicity-preserving polynomial products.

The proposed GF9/GF25 tiny arithmetic, basis and cubic-domain checks establish coordinate representation and support-oracle consistency only. They are not an admitted elliptic chart, characteristic-two guarded chart, GF(2^83) solve or performance comparison. The multiplication-matrix method is classical; no primary-source novelty conclusion is asserted.
