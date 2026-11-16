# Triple orbit correlations without characteristic-crossing division

T-1809, 2026-11-07. Derived construction and producer checks. [pcc:unverified]

## Feature and its required normalization

Retain the prime-field fold contract: r prime, m dividing r-1, zeta of exact order m, d=(r-1)/m, y=k+c and z=y^m. This triple compiler requires d even. Then chi_r(zeta)=1, so all kernel multipliers are squares. Choose distinct nonzero lags u,v<m, put a=zeta^u, e=zeta^v, and let

    q(X)=(X-1)(X-a)(X-e), h=(r-1)/2,
    q(X)^h=sum_n c_n X^n,
    s_j=chi_r(y-b*zeta^j), b!=0.

The triple observation is

    B_(u,v)=sum_j s_j*s_(j+u)*s_(j+v)
           =chi_r(b)*m*sum_t c_(tm)*(z/b^m)^t.          (1)

Proof: each product is chi_r((b*zeta^j)^3)*q(y/(b*zeta^j))^h. The first factor is chi_r(b), since zeta is a square. The root-of-unity sum retains exactly indices divisible by m. The radius character is essential; it cannot be omitted as it was for an even product. Tests include nonsquare radii and count rejection of that omission. Zero sign entries and y=0 satisfy the same identity.

Here the projected degree is 3d/2. Reduce it modulo V(V^d-1): a term V^t with t>d becomes V^(t-d), while the constant is left unchanged. This preserves both zero and all nonzero power images. The stored feature has degree at most d. Scaling by b^m and multiplying by chi_r(b) gives the actual image constraint. The observe primitive assumes the current roots lie in the power image; the public adapter establishes that invariant. No field extension, scalar enumeration or supplied auxiliary group power is used in production.

## Cubic coefficient recurrence

Write q=q0+q1 X+q2 X²+q3 X³. From q f'=h q' f one gets

    q0*(j+1)*c_(j+1)
      =sum_(i=1)^3 ((h+1)*i-j-1)*q_i*c_(j+1-i),
    c0=q0^h, c_(-1)=c_(-2)=0.

Use a3x3 numerator matrix whose first row contains these three coefficients and whose two subdiagonal entries are q0*(j+1). Divide by q0*(j+1) after each step. A balanced degree-m polynomial-matrix block and monic remainder tree retrieve c0,c_m,...,c_(dm), exactly as in the previous stride compiler but with a3-component state. All executed forward indices satisfy j+1<=dm=r-1, so every denominator is nonzero. One batch inverse normalizes all evaluated block denominators; the initial power and inverse are explicitly exponentiated.

The full cubic power has degree N=3h>r-1. Continuing the forward recurrence would meet a zero denominator at r. This implementation does not cross it.

## Reciprocal coefficient reconstruction (derived)

Define q_inv(X)=(X-1)(X-a^-1)(X-e^-1), with q0=-a*e. Direct multiplication gives

    X³*q(1/X)=q0*q_inv(X),
    X^N*q(1/X)^h=q0^h*q_inv(X)^h.

Thus c_n=q0^h*c_inv_(N-n). Since d is even, N=(3d/2)*m is divisible by m. Every needed high index n=t*m with t>d reflects to (3d/2-t)*m, below h and therefore below r. A second recurrence prefix for q_inv supplies those values without characteristic lifting or crossing. The overlapping range t in[d/2,d] is checked against both computations. The last coefficient reconstructs as1, consistent with monicity.

Both prefixes are charged. The implementation reconstructs d/2 high stride coefficients and folds them into the degree-d image polynomial. It stores degree-m block polynomials and O(d) prefix/output values, rather than the complete degree-N coefficient array. Primality and exact-root checks remain the earlier Fold contract. Caps bound m, d and the number of selected features before compilation. Unbalanced divisors can still require large work; no favorable divisor is assumed for every prime.

## Symmetry addressed and symmetry retained

For nonzero y, let y'=b²/y. With d even, the sign words obey

    s_j(y')=chi_r(-b/y)*s_(-j)(y).

Pair correlations lose the common sign factor and, in the even-m/even-d family, the prior folded product and all pair correlations can agree for distinct reciprocal images. A triple observation transforms into the reflected-lag triple observation multiplied by chi_r(-b/y). Therefore a triple can distinguish some of these images. It need not distinguish all: reflected lag patterns, vanishing sums and coincident invariants can preserve equality. The exact controls retain both outcomes. Lags1,3 give an asymmetric pattern for appropriate orbit lengths; small lengths can make that pattern equivalent to its reflection. No universal injectivity or query bound is claimed.

The adapter adds configured triple constraints after configured pair constraints and before the image-singleton gate. It uses only the already collected orbit signs. Retained-sign fiber refinement and final [candidate]P=Q remain unchanged. d odd is rejected for this triple feature, not silently evaluated with the wrong kernel character; pair-only recovery remains available for those parameters.

## Cost and source boundary

Per selected triple, two fixed3x3 matrix compilers give the safe field-operation bound O(M(m+d) log(m+d)+log r), with a larger fixed constant than the pair compiler. O(d) reconstruction/reduction is included. Conservative workspace is O(m log(m+1)+d log(d+1)); a stored feature has d+1 coefficients. Each selected pair/triple is separately charged; compiling all lag pairs is quadratic in m and is not performed by the public adapter. Its fixture configuration has one pair and at most one triple.

Each orbit still costs m source calls and the existing group shifts. Observed triple sums cost O(m) sign products/additions. Scaling and reduction cost O(d+log r), followed by the existing image gcd; history storage, fiber replay, input validation, source setup and candidate verification remain charged. General gcd scalar inverses use extended Euclid, as corrected in the preceding component. The explicitly coded batch inverse/powers use exponentiation. No timing, fastest-tool comparison or source-count ratio is asserted.

The source domain is unchanged: caller-certified source/scalar primes, r>=7, r!=p, gcd(r,p-1)=1,2^v2(p-1)<=4 and the geometry cap. This is a destination feature extension within that existing public source contract, not a new source for every curve.

## Formal map, sources and records

Reflection.lean, TripleOrbit.reflected_index_below, proves the integer bound: if r>0,2N=3(r-1) and n>=r, then N-n<r. Lean4.34.1 accepts it with standard propext and Quot.sound only. This is the characteristic-boundary index fact; the polynomial reversal identity, finite-field recurrence, projection and public source interpretation remain written proofs. No sorry, custom axiom, native_decide or bypass is used. The available project still lacks built Mathlib; no full arithmetic formalization is claimed.

Sources checked (KB): no tool loaded, repository wrapper unavailable. Local pair compiler, retained-fiber code and CONTINUATION notes on lifted coefficient recurrence; the referenced historical lifted-sign REPORT is absent in this sparse checkout. Read Bostan-Gaudry-Schost2007, coefficient-recurrence application and main algorithm, https://specfun.inria.fr/bostan/publications/BoGaSc07.pdf, DOI10.1137/S0097539704443793. Established matrix recurrence acceleration is reused; its more optimized bounds are not borrowed. Reciprocal reconstruction is proved directly above. No novelty claim is made.

Validation-01 retained a failed public fixture setup: extra offsets0..16 were noncanonical in F13. The API rejected Domain. Exact source and failure snapshots remain. Validation-02 changes only that test schedule to canonical offsets below min(17,r); no production arithmetic or domain guard was changed. Overflow checks, normal-build reference exclusions, generation hashes and source/input hashes are retained. No cloud/timing work or rerun of the original4,563-case cohort.

Next: seek a smaller set of orbit invariants that breaks the remaining equalities without compiling a large collection of high-order features, while accounting separately for image acquisition and source work.

## Final producer check output

```text
running 4 tests
test tests::dense_projection_values_and_image_sets ... dense_compilers=22 orbit_values=3150 exact_image_sets=1797 strict_over_all_pairs=648 missing_radius_factor_rejections=306
ok
test tests::large_prefixes_and_domain ... large_cases=3 scalar_prefix_comparisons=6 reflected_high_coefficients=448
ok
test tests::public_higher_recovery ... public_cases=63 recovered=63 unresolved=0 retained_routes=38 source_calls=778
ok
test tests::reciprocal_pairs_split_or_retained ... distinct_reciprocal_cases=624 triple_split=360 still_equal=264
ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 157 filtered out; finished in 0.76s
```

Small dense references multiply the cubic repeatedly by schoolbook convolution, independent of the coefficient recurrence. They cover primes7,13,17,31,41,97 and eligible m3..8 with lags1,2 and (when distinct)1,3. Larger r65537 checks compare both prefixes to scalar recurrences at widths128,256,512; those references share the recurrence identity. The624 reciprocal controls are ordered radius/point cases with distinct images;360 are distinguished by the triple and264 remain equal.

Public cases are the prior46 (frozen20 r479, all13 r13 with m3 and m6) plus17 fixed known-answer inputs on E/F701: y²=x³+2x+8, P=(2,106), r683,m11. The latter source curve was an existing fixture; this is a bounded new adapter check, not the full cohort. Scalar labels generate references only; production receives public points. All63 recover,38 by retained-sign replay.778 source calls are path accounting, not a comparative timing claim.33 source/input and generation hashes match.
