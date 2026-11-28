# Exact local preclosure for the M31 projective Semaev chain

Producer-only [pcc:unverified]. Continuation of VERONESE_FINDINGS.md at e4fd6931657bfc0fc1768072a69a9447448b65b5. New tests exclusively use p=2147483647, a=p-3, b=21, reported prime group order 2147487637, cofactor one (PR #5097). No scalar logarithms or generator-dependent inputs occur in this Semaev-only study. Fp² is used solely for geometric witnesses; source coordinate inputs and projected root sets remain over Fp. No factor-base map change.

## Exact local ideal and column bound

For each internal Kummer node use w=u+z-1, s=r+z-1. The existing normalized chart has z²=z, (1-z)(u-1)=0, r=u². Preinsert the redundant degree-three consequence (1-z)(r-1)=0. Substitution gives the exactly equivalent ideal J=(z²-z, zw-w, zs-s, w²-s). Conversely u=w+1-z, r=s+1-z recover the original chart equations. The Veronese vector is (1-z+s,w,z). The quotient is the direct product of the infinity component (z=w=s=0) and affine component (z=1,s=w²).

For any degree-compatible order the leading monomials are z², zw, zs, w². Reduction terminates and standard monomials 1,z,s^i (i>=1), w s^j (j>=0) are independent: infinity evaluation forces the constant coefficient zero, then affine evaluation separates constant, positive even and odd powers of w. Thus these four quadratics are a Gröbner basis and normal-form reduction never increases total degree. This removes the local chart-closure trap; it does not exclude global degree falls after cubic tensors are coupled.

Each node has filtered Hilbert series H_N=(1+2t-t²)/(1-t) for its associated graded normal-form vector space. Each affine leaf square lift x²=q has H_L=(1+t)/(1-t). With m+1 affine inputs and m-2 internal nodes, N=5m-4 variables, the exact number of normal monomials of degree <=D modulo local equations is [t^D] H_L^(m+1) H_N^(m-2)/(1-t). Dense unreduced count is binom(N+D,D). The disjoint local ideals have a union Gröbner basis because leading monomials in distinct blocks are coprime. Applying local normal form to products preserves their residue classes and does not increase degree. A conventional F4 engine may already exploit these leading monomials.

| m | D | Unreduced columns | Local quotient columns |
|---:|---:|---:|---:|
|3|8|75582|15314|
|4|8|735471|151848|
|5|8|4292145|987180|
|6|8|18156204|4729080|

These are exact ambient column dimensions before global S3 constraints, target specialization or slicing, NOT measured F4 matrix sizes or solver timings.

Negative asymptotic control: the fraction removed at fixed D is at most (5m-7) D(D-1)/((N+D)(N+D-1)), by a union bound over m+1 leaf and 4(m-2) internal forbidden quadratic monomials. It is O(D²/m) for fixed D, so local compression alone is not a widening gain with m. No bound on global solving degree follows. See Caminata–Gorla, https://arxiv.org/abs/2112.05579 and Kosters–Yeo, https://arxiv.org/abs/1503.08001.

## Native local correctness receipt

C++20 fallback, seed 20261124; 50 unconditioned pseudorandom x-tuples plus 10 repeated-x controls per m=3,4,5,6. Compare complete fixed-degree Sylvester and Horner resultant coefficients and monic gcd with X^p-X to the product of distinct rational abscissae from independently enumerated signed point sums over Fp². Check all local charts and cubic tensors on intermediate sums, including identity. All 240 cases passed (200 random, 40 exceptional). Aggregate 350 rational roots, 46,080 local chart evaluations, 15,360 tensor evaluations, 600 infinity chart evaluations. These are specialized projected-root gates, not an unspecialized GB extraction or independent PCC.

Local artifact m31_chart_preclosure.tar.gz retains PROTOCOL.md, FINDINGS.md, original reference source, new gate source, counts source, gate.csv, columns.csv, run.log and SHA256SUMS. SHA256 chart_gate.cpp a1343eff0b24eb3f117e55c0fa6fc5e79a1b9f6b2ef32aa98154445610bdc948; horner_reference.cpp d3ad43cc2326f60f64ace9d9354312c420eff64bdec05d16331da33d4d93ddcb; gate.csv 5698277b9b6b586c3f991547abc0f26c48d6c2bcb4719b78373f20f99a6d2313; columns.csv 389a0193c3677178b0001a8f91258483ecdbe38c288124364cc2717682b7ca22. Native source and raw records are delivered in the conversation artifact, not yet checked into this PR by this receipt.

## Next / blocker

Sage, Singular, msolve, FGb and Rust/Cargo absent locally; C++ fallback used for correctness only. No paid compute or merge. Next implement quotient-aware F4 symbolic preprocessing on matched two-parameter zero-dimensional slices, compare against conventional F4 and the monolithic ideal, preserve actual maximal Macaulay degree, matrix dimensions, time, memory and 50 full projected-set gates per cell. No new d_F4 or GB speedup established. [pcc:unverified]
