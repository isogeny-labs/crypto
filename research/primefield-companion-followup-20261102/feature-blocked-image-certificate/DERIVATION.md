# Shared polynomial blocks for adaptive image recovery

2026-11-07 · T-1821 · Written derivation and exact native producer checks. [pcc:unverified]

## Domain and target

Use the predecessor's caller-certified prime scalar field F_r, squarefree monic G of degree n<r, and m>=2 dividing r-1. Candidate G splits over F_r in recovery. Let B=X^m mod G. If t distinct images survive, the certified adaptive recurrence requires fewer than4t moments. It previously generated these moments sequentially and checked each provisional polynomial by coefficientwise Horner evaluation. Both used O(t) multiplications modulo G.

This successor changes those two arithmetic stages. It retains the exact shortest recurrence, doubling checkpoints, zero-index moment, quotient-annihilation acceptance, and capped pullback. It introduces no scalar or group search table. Every table entry is a polynomial residue B^i modulo the current candidate polynomial.

## Shared representation

At prefix length L, take q=ceil(sqrt L). Construct S_i=B^i mod G for 0<=i<q and H=B^q mod G. Store at most q*n coefficient words in the short table; reject the supplied table cap before construction. This cap does not include all polynomial backend workspace.

Trace weights s_k=Tr(X^k) for0<=k<=2n-2 are obtained once from -T Q'/Q, where Q=T^n G(1/T), Q(0)=1 and s_0=n. For giant residue F_u=H^u mod G, let ell_u(i)=Tr(F_u X^i). Then ell_u(i)=sum_a F_(u,a) s_(a+i). Padding F_u to length n, reversing it and multiplying by the weight polynomial gives ell_u(i) in coefficient n-1+i. Missing high coefficients of a canonical polynomial are zero.

The requested moments are mu_(uq+v)=sum_i coeff_i(S_v) ell_u(i). Only indices below L are written; incomplete terminal blocks are clipped. This is the same quotient trace as sequential powering by linearity and reduction modulo G. The proof does not require L<=n; prefixes through2n are valid.

## Exact certificate evaluation

Write C(Y)=sum_j C_j(Y)Y^(jq), each block C_j of degree below q. Form C_j(B)=sum_i coeff_i(C_j) S_i and evaluate these blocks by Horner in H. This yields exactly C(B) mod G. Coefficient blocks can extend above degree n; there is no restriction deg C<n in this handwritten evaluator. The same short table serves projection and the certificate at that checkpoint.

Consequently the certificate accepts exactly the previous Horner certificate's polynomial, including all provisional rejections. The predecessor proof that an accepted shortest recurrence equals the distinct-image polynomial applies unchanged. Zero remains a valid image because the sequence starts at mu_0.

## Derived cost and tradeoff

For one prefix L, building short powers and giant powers, and the trace convolutions, costs O(sqrt(L) M(n)). Projection scalar pairings cost O(nL). Recurrence degree is at most L at these checkpoints, so certificate block construction costs O(nL), with O(sqrt(L)) further quotient products. Scalar matrix combinations are ordinary loops; no fast matrix multiplication is claimed.

Prefixes are geometric and end below4t, with a possible final truncation at2n. Summing their square roots and lengths gives O(sqrt(t) M(n)+nt) for all projections and certificates, despite recomputing prefixes. The trace-weight setup is shared; B costs O(M(n)log m). Restarted Berlekamp-Massey costs O(t²+t log r), including explicit Fermat discrepancy inversions. Thus the conservative core bound is O(M(n)(log m+sqrt(t))+nt+t²+t log r). Squarefree input validation, inflated output of degree N, original-parent validation and parent divisibility remain additional.

The price is O(n sqrt(t)+n+t) stored coefficients plus backend temporaries, replacing the sequential path's O(n+t) storage. A table-word cap is explicit. This algebraic bound does not assert lower elapsed time or total recovery cost on the checked fixtures. The public adapter uses a finite table allowance of2*cap² words, sufficient for the existing degree and moment caps. Original source setup and sign evaluation remain charged independently.

## Checks and reproducibility

checks.rs compares every trace prefix of lengths1..2n with direct scalar power sums, for scalar primes7,11,101, n=1..min(16,r-1), exponents2,3,5. It checks the complete certificate residue against coefficientwise Horner at degrees0,1,n,2n+1. Every prefix tests the exact q*n table allowance and rejection one word below it. These are deterministic algebra/software tests, not a statistical or timing experiment. [pcc:unverified]

The inherited6,522 subset checks and three larger cases also require equality of adaptive image polynomial, envelope, full accepted moment vector and rejected-prefix count. Thus they check the adaptive schedule as well as the output. The63 public fixtures preserve scalar, route and query fields against the existing lcm recovery path. Known scalars are test-only; original4,563-case cohort unchanged. [pcc:unverified]

Raw compilation and assertion outputs, compiler identity and transitive source hashes are retained in validation directories. generator input/output hashes bind all copied controls. This is unsigned producer evidence, not an independent signed review or formal proof.

## Sources and next construction

Primary source: [FLINT modular composition](https://flintlib.org/doc/nmod_poly/compose_mod.html), which describes polynomial power blocks and composition. The Brent-Kung1978 PDF request timed out; the official FLINT page was readable. KB tools unavailable. Polynomial blocking and transposed power projection are established ideas; no novelty claim for these methods. The implementation uses scalar combinations rather than FLINT's matrix multiplication backend.

Next derive an accounted selector using n, available table words and observed prefix growth, retaining sequential and lcm fallbacks. A faster recurrence backend or rectangular matrix multiplication is separate future work; source evaluation remains a material cost.

## Recorded outcome and retained failure

validation-01 failed with an out-of-bounds convolution access after the backend trimmed trailing zero coefficients. The source snapshot and exact panic are preserved. validation-02 uses zero for absent high coefficients and passes1,272 prefix equalities,5,088 whole certificate remainders and1,272 exact table-cap boundaries. All6,522 small adaptive cases and3 larger cases preserve image/envelope/moment vectors and rejected-prefix counts; all63 public checks pass with511 source calls. The inherited9,185 rejected-prefix count and3,233 below-n+1 moment cases remain unchanged. [pcc:unverified]
