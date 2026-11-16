# Coordinate-exact extension-field metric plan

UNEXECUTED extension of the lattice representation, not an extension of the measured prime-field interval algorithm. No old source or evidence is edited, no extra experiment is run, and no novelty or speed claim is made. Linear representations of field multiplication and exact lattice basis changes are classical.

## Field and coordinate contract

Let K=Fp[theta]/g(theta), where g is a specified monic irreducible polynomial of degree r. Fix ordered polynomial basis(1,theta,...,theta^(r-1)) and coefficient order. For a in K, define the r-by-r matrix M(a) over Fp by coord(a*x)=M(a)coord(x). Every matrix column is the independently reduced product of a with one basis element. The actual field definition, irreducibility proof/status, basis order and coordinate convention belong to every cache/input key; equal cardinality does not establish identical representations.

Suppose a guarded field chart has T=sum_i a_i F_i+a0 and U=sum_i b_i F_i+b0 for FIVE field-valued features. This premise must first be proved for that chart/family. In odd characteristic, the formal W=u²−S encoding can be transported as a FIELD polynomial identity if the original distinct-anchor and2A!=0 guards hold. Its integer envelopes do not transport.

Stack ordinary Fp coordinate vectors x_i=coord(F_i), t=coord(T), u=coord(U). The two field equations become2r scalar congruences:

t−sum_i M(a_i)x_i=coord(a0),
u−sum_i M(b_i)x_i=coord(b0) modp.

The affine primal coordinate lattice lives in Z^(7r). Its homogeneous congruence map is onto Fp^(2r), because the t and u blocks are identity blocks. Hence its kernel index/determinant is p^(2r), not p^(2r²), and constants give an affine translate rather than extra homogeneous constraints. An explicit basis can use5r free feature coordinates and2r p-multiple output coordinates, with output blocks M(a_i),M(b_i); coordinate offset has feature blocks zero and output blocks coord(a0),coord(b0).

## Exact dual certificate lattice and transpose requirement

Let gamma,delta be r-coordinate integer LINEAR FUNCTIONALS under ordinary coordinate dot product. A dual vector consists of five feature-functional blocks lambda_i plus gamma,delta, hence7r integer coordinates. Its5r scalar constraints are

lambda_i = M(a_i)^T gamma + M(b_i)^T delta modp.

The transposes are mandatory. Treating ordinary coordinate functionals as field coefficients and writing lambda_i=gamma*a_i is generally incorrect. A trace-pairing formulation may identify functionals with field elements only after fixing the corresponding dual basis and proving the identification.

These5r constraints are independent/onto since each lambda block is free, so the dual lattice has index/determinant p^(5r). Its explicit generator rows are p times the5r feature unit vectors plus2r gamma/delta unit rows with the above transpose-generated feature entries. For any actual chart tuple and centered integer lift c of gamma·coord(a0)+delta·coord(b0),

J=c+sum_i lambda_i·x_i−gamma·t−delta·u is a multiple of p.

An independently established INTEGER box/range containing every allowed coordinate tuple gives a sound exclusion when ceil(L/p)>floor(R/p). Failure to find such a certificate is UNKNOWN. Repeated roots are not removed by this necessary-condition test, but a support/root-completeness proof remains separate.

Membership in all5r dual congruences plus absolute determinant p^(5r) proves equality of a proposed full7r-row basis with the original dual lattice. Positive diagonal metric changes and exact unimodular row operations therefore permit the same unscale/reweight/repair argument as the prime-field method. Every metric's coordinate ranges and masks must be freshly checked; no old bound is reused. Cache identity includes p,r,g,basis/encoding, all coefficient matrices, target/anchor data, chart guards/domain and offset. A changed representation forces a cold basis. Larger7r dimension can increase reduction cost substantially; no performance conclusion follows.

## Why scalar integer encodings cannot be substituted

Concrete counterexample: GF4=F2[alpha]/(alpha²+alpha+1), encoded by binary polynomial coordinates0,1,alpha,alpha+1 ->0,1,2,3. Field alpha²=alpha+1 has encoding3, whereas integer2² mod4 is0. Also alpha+alpha=0 in the field, whereas2+2 mod4=0 happens only accidentally; multiplicative failure already disproves a ring encoding. More generally there is no unital ring homomorphism from a characteristic-p field into Z/(p^r) for r>1: p*1 is0 in the source and nonzero in that target. For K to Fp, a unital homomorphism would be injective and cannot map a larger finite field into Fp.

Consequently do NOT reuse integer S in[0,3M], integer elementary symmetric coefficient intervals, ordinary integer cubic isolation, or source-size estimates M^9 H^4 for extension-field roots. A polynomial-basis encoding is a coordinate label, not an ordered integer ring. Even though raw coordinates can be bounded by0..p−1, such boxes may have no useful pruning. Product-span/domain-dependent coordinate bounds require fresh proofs.

Characteristic two is outside this guarded odd-characteristic norm chart because2A=0 identically. It requires its existing binary-curve formulation, not an inverse of2A. The GF4 example is a counterexample to scalar encoding, not a claim of characteristic-two chart support.

## Concrete bounded next experiment (not executed)

Use GF9=F3[theta]/(theta²+1) and GF25=F5[theta]/(theta²+2), with explicit irreducibility checks by exhaustive prime-field evaluation. First independently implement multiplication as polynomial convolution/reduction and build M(a) from basis products; compare every a,x product against a distinct direct small-table oracle. Verify matrix multiplication, transpose dot identities, affine primal congruences and dual congruences for every tiny tuple under a declared finite tuple cap.

Then test independently enumerated tiny field-root/support cases, including repeated-root cubics and polynomials with no domain-supported roots. Use D={0,1,theta} as a fixed field set and compare polynomial products/factor multiplicities with direct evaluation and exhaustive ordered triples over that SAME set. Demonstrate that an integer-cubic support routine on scalar encodings disagrees on the GF4 witness; it must never be used as the extension oracle. Keep chart-correctness tests separate from generic root/support controls; no elliptic chart is admitted until its pole/anchor/rationality contract is supplied.

For the metric component, generate bounded exact unimodular transformations of7r coordinate bases, verify every congruence and determinant p^(5r), compare increasing/decreasing metric repairs, and reject changed-g/basis cache keys and corrupted-index bases. Compare certificates against exhaustive tiny allowed coordinate tuples; all unknown/capped branches stay explicit. No generic-rank assumption or sample membership test proves completeness.

Proposed worker budget:90seconds,4096MiB,one core; predeclare at most200000 scalar tuple/oracle checks and at most128 full basis transformations before allocation. Run only after independent review, frozen source/field/domain/input hashes and a NEW experiment ID via lab.runner. Any cap or field/representation mismatch returns incomplete; never a false empty root set. This coordinate-equivalence and tiny-support experiment establishes no hard-field solve, global all-roots completeness, novelty, or matched F4/F5/F6 speed ratio.
