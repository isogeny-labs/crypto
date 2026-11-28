# Exact rank-preserving contraction of signed four-leaf output

2026-11-23. Continuation of draft PR #5064, frozen input head `c428c527d39a55f11b14ad57d8dbc614db69959c`. Pure mathematics/validation preparation, **[pcc:unverified]**. No original-curve execution or independent signature.

## 1. Two different output contracts

Let G have odd prime order n>32, A={±P_i:1<=i<=B}, with distinct nonzero x-classes, and let e_i denote the formal coefficient basis of F_n^B. Each signed leaf is labelled ±e_i. In the existing rational pair chart, write L_s for the set of signed two-leaf coefficient vectors u whose group sum is s; include the identity bucket and repeated leaves. Store an original labelled witness with each vector. Distinct ordered witnesses can have the same vector; this is a rank-only interface, not full witness reporting. Use oriented point keys (not only x). Source construction and exact signed replay are charged.

For a fixed query R and fixed public augmentation tail w (e.g. the two generator/target coefficients, with signs chosen consistently), all rows from matching pair buckets s,t=R-s are

    M_{s,t} = {(u+v,w) : u in L_s, v in L_t}.

With anchors u0 in L_s, v0 in L_t, the following actual rows suffice for **exact equality of row spaces**:

    {(u+v0,w): u in L_s} union {(u0+v,w): v in L_t}.

Indeed (u+v,w)=(u+v0,w)+(u0+v,w)-(u0+v0,w). The retained rows are a subset of the full rows, so both inclusions follow over any field. The shared tail is essential; if the tail differs, group by its exact value. This covers the identity subtotal, signs, cancellation and repeated leaves without assigning a discrete-log label. It preserves the span of the entire augmented matrix, not only the factor-base rank; the retained rows still admit authentic four-leaf witnesses. It cannot substitute for all-output tuple enumeration or a certificate demanding every decomposition.

Diagram of one matching block:

    (u0,v0) ---- (u0,v1) ---- ... ---- (u0,vb)
       |
    (u1,v0)       interior omitted
       |
      ...
       |
    (ua,v0)

All interior rows equal the row anchor + its row difference + its column difference. From a*b rows the star needs at most a+b-1. When a=b=k, this is 2k-1 rather than k^2. If either bucket is singleton there is no benefit. Union across every matched ordered key yields the exact full matrix span; duplicate anchors may be discarded. Since s->R-s is bijective, the total number of emitted star rows is <=2 sum_s |L_s| <= 2(2B)^2, irrespective of full Cartesian multiplicity. Charge O(D_g) matching probes (D_g is the oriented group support size, including O), O(sum matched bucket sizes) label visits and retained row verification, plus the actual linear algebra and any affine chain reconstruction. Output row count is O(B^2) worst case, not a faster empty-target predicate. Dense RREF need not be cheap. No claim that this changes the natural uniform-query dominant workload.

## 2. Global pair-collision subspace, conditional extension

Let W be the span of all u-u_s over all buckets and fixed representative u_s. Each difference is a genuine homogeneous four-leaf group relation (take the signs of the second pair reversed); no target coefficients. Generators of W can be enumerated in at most sum_s(|L_s|-1)=O(B^2) rows, with full linear algebra charged. The unknown but nonzero vector of true factor-base logs annihilates W, hence dim W<=B-1. Modulo W x {0}, every matched pair block contributes only one representative (u_s+u_t,w). Thus if W is separately constructed and its witness basis included, a query needs at most one augmented row per matched ordered pair key for **exact rank/row-space equivalence modulo the preloaded homogeneous relations**. W may cost more than the savings; rank compression does not establish rank B, cheap setup or sublinear search. No such preprocessing has been measured. In a collision-free signed pair chart W can be zero after coefficient deduplication. A rank-only protocol must price initial W separately and not count its rows again at every target. In the standard target rank test, preserve the full augmented columns and the nonzero target coefficient, not just the factor-base projection.

## 3. Canonical coefficient count tightens all-output rank upper bound

For B signed abscissae, all integer coefficient vectors from four leaves have L1 norm 0,2 or 4. Conversely, any vector of these norms has a four-leaf witness by padding with opposite leaves, since B>=1. A norm-0 row cannot represent a nonidentity target. For n>32 these bounded vectors inject into F_n^B; a group sum may collide with another vector or with O, but each vector has exactly one oriented target.

    K2 = 2B + 4 binom(B,2) = 2B^2
    K4 = 2B + 4B(B-1) + 4 binom(B,2)
         + 8B binom(B-1,2) + 16 binom(B,4)
       = (2B^4+4B^2)/3
    K = K2+K4 = (2B^4+10B^2)/3.

The five terms enumerate magnitude partitions 4,3+1,2+2,2+1+1,1+1+1+1. Equivalently, use the coefficients of ((1+t)/(1-t))^B. This is the oriented-vector counterpart of the previous cancellation-reduced nonidentity x-support cap; unlike ordered tuple counts it bounds distinct useful coefficient rows even if the solver reports **all** witnesses.

For T targets each marginally uniform on G minus O, E[number of distinct available nonzero coefficient vectors] <= TK/(n-1), by counting each vector once per query and dropping any vectors summing to O. Starting with factor-base rank d (deterministic or conditioned on the source and initial rows), the rank gain is at most the count. Thus for full factor-base rank with probability >=eta,

    T >= eta (B-d)(n-1)/K.

This holds even when retaining every output, unlike the one-retained-row bound of the preceding note. It is a necessary rank-coverage bound only, not a target-log-recovery lower bound; different public (a,b) tails can carry distinct information even with identical factor-base coefficients. No independence of target samples is needed for this first moment, only the stated uniform marginals. Adaptive nonuniform sampling, source preloaded rank and a solver needing fewer than B factor-base columns are outside the zero-initial-rank claim.

For independent uniform nonidentity targets with replacement, further merge scalar-collinear coefficient vectors. A nonzero 2-norm vector v and ±2v,±v are all the same projective direction; all remaining norm-4 vectors have an odd entry and gcd of coordinates 1 (gcd divides 4). Integer nonproportional vectors stay nonproportional mod n>32: a nonzero 2x2 minor has absolute value at most 32. There are K2/2 four-vector classes and (K4-K2)/2 two-vector classes, disjoint. For a nonzero class target, the four group points ±R,±2R are distinct at prime n>32; for the other classes there are two. Any class that maps to O contributes nothing. Each direction contributes at most one to the factor-base span. Consequently, writing N=n-1,

    E[rank gain] <= ((K4-K2)/2)*(1-(1-2/N)^T)
                   + (K2/2)*(1-(1-4/N)^T).

Divide by B-d to bound the probability of achieving full rank using Markov's inequality. This is still a necessary condition, not a success guarantee. For B=31 and n=2147446831, K2=1922, K4=616962, K=618884. There are 307520 two-vector and 961 four-vector directions. With d=0 and eta=0.9, the first-moment necessary integer count is 96810 nonidentity target queries. The independent-target projective occupancy bound raises this to 96814 (numerical evaluation, with margins about 0.000218 and 0.000070 around 27.9 at 96813 and 96814 respectively; receipt checks arithmetic). By contrast iteration8's ordered-tuple first moment was about 4055 using uniform group targets. The sharper count arises from quotienting actual coefficient duplicates, not from any new relation-rate measurement.

## 4. Validation and decision

Freeze this mathematical source before post-publication abstract checks. Test the star identity by exact row-space checks on finite coefficient matrices, including empty buckets, singleton, repeated labels, unequal cardinalities and nonzero common tails. Cross-check K2,K4 via coefficient-generating-function recurrence and small signed multiset enumeration. On a later authorized native test, compare full signed output and rank-only outputs using original Rust point arithmetic, test every target on small eligible odd-order curves, retain zero subtotals and rational affine-chain witness checks, verify augmented row spaces (not just equal counts), and separately price all-output enumeration versus compressed rank-only verification, source table and LA. No Rust solver step selected until the exact source/reservations are confirmed. Do not conflate producer abstract linear-algebra checks with independently validated cryptographic execution.

The rank-star is a component contraction for positive-query row traffic. The existing D_g complement scan and its empty-target cost remain. The sublinear exact Dickson-specific index, complete charged setup, actual source useful rank and independent native testing remain open.

## Source review

- Iteration8 group-semiring note, Git blob 20f9428434bf228c75de404bdb8c4fde4f7b3675; exact Cartesian product and older ordered-row rank bound.
- Iteration10 pencil note da12c7a459405f4fdea44118cc03ade721d5eb90; rational oriented support and full output contract.
- Iteration13 explicit frontier f039100b13acf888dc33c0214c02b71612c06706 and receipt c96913bf724af5f85e3c3b0a1ca15a868f431d98; one-retained-row model kept distinct.
- Huang, Petit, Shinohara, Takagi, *Improvement of FPPR method to solve ECDLP* (2015), section 2.1, https://doi.org/10.1186/s40736-015-0012-6 ; describes multiple decompositions and augmented relation matrix. No novelty claim for linear span/star identities or general matrix row reduction.
- Shared KB/Conductor and native worker slots unavailable; the source and literature review is scoped, not exhaustive. No signed claim promotion.
