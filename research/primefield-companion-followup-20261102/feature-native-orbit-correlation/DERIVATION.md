# Cyclic orbit correlations as compressed image constraints

T-1808, 2026-11-07. Derived construction and producer software checks. [pcc:unverified]

## Construction

Use the existing prime-field fold parameters: prime r>=5, m>=2 dividing r-1, d=(r-1)/m, zeta of exact order m, center c, nonzero radius b, unknown y=k+c, and image z=y^m. Let s_j=chi_r(y-b*zeta^j) and fix a lag ell in0..m-1. All indices below are cyclic modulo m. Define

    A_ell = sum_j s_j*s_(j+ell) in F_r.
    a=zeta^ell, h=(r-1)/2,
    f_a(X)=((X-1)(X-a))^h=sum_n c_n X^n.

Then

    A_ell = C_ell(z/b^m),
    C_ell(V)=m*sum_(t=0)^d c_(tm)*V^t.                 (1)

Proof: each product of signs equals f_a(y/(b*zeta^j)). The omitted factor b²*zeta^(2j) is a nonzero square, so contributes character1. Sum the expansion over j. The root-of-unity sum of zeta^(-jn) is m when m divides n and zero otherwise. This proves (1), including zero sign entries. The constraints use the residue A_ell in F_r, not an unproved integer-count decoding. They depend only on the power image.

After collecting an orbit, the adapter first applies its existing folded-product constraint, then gcd-intersects the image state with C_ell(Z/b^m)-A_ell for every configured lag. Polynomial reduction occurs before the capped gcd, so a prior small state does not cause an oversized-input rejection. No additional source sign is requested to compute these correlations. The retained individual signs still refine the degree-m scalar fiber when the image becomes a singleton. The final public-point verification remains unchanged.

## Stride coefficient compiler

Differentiating f_a=q^h, q=X²-(1+a)X+a, gives the order-two recurrence

    a*(j+1)*c_(j+1) = (j-h)*(1+a)*c_j + (2h-j+1)*c_(j-1),
    c_0=a^h, c_(-1)=0.

For 0<=j<=r-2, every denominator is nonzero. Let

    T(j) = [[(j-h)(1+a), 2h-j+1], [a(j+1),0]].

The next coefficient pair is T(j)/(a(j+1)) times the current pair. Build the polynomial matrix

    B(X)=T(X+m-1)...T(X),
    D(X)=product_(i=0)^(m-1) a(X+i+1).

A balanced tree multiplies the right block on the left of the earlier block. Its polynomials have degree at most m. Evaluate the four entries and denominator at 0,m,...,(d-1)m with the existing monic product/remainder tree. Batch-invert the d denominator values with one explicitly exponentiated inverse, checking the product is invertible. Advance the coefficient pair through these d evaluated blocks, retaining only c_0,c_m,...,c_(dm). The final coefficient index is dm=r-1. No recurrence denominator crosses characteristic r.

This applies established polynomial-matrix recurrence acceleration to the orbit coefficient selector; it is not a new generic recurrence algorithm. Production does not expand f_a or enumerate field elements. It constructs degree-m block polynomials, a degree-d evaluation tree and d+1 output coefficients. For an unbalanced divisor, m or d can still be large; caps are checked before compilation. Caller-certified primality and the checked exact-order certificate remain explicit.

## Complexity and source accounting

For one lag, a safe field-operation bound is O(M(m+d)*log(m+d)+log r), using fixed2x2 matrix multiplication and ordinary product/remainder trees. This includes the balanced block product, multipoint evaluation, d state transitions, denominator batch inversion and the visible power computations. Installed multiplication is Karatsuba, with exponent log_2(3); no FFT implementation or tuned timing gain is claimed. If m and d are both of order sqrt(r), this gives the derived O(r^(log_2(3)/2)*log r) setup bound. A suitable divisor is required; this is not a uniform prime-order guarantee.

Conservative compiler workspace is O(m log(m+1)+d log(d+1)); each retained lag costs d+1 coefficients. L configured lags multiply setup work and stored coefficients by L. Compiling all m lags is not free and can erase the favorable exponent. The public checks configure one lag; the broader small-field tests deliberately compile all lags for correctness coverage. The max_lags, max_block and max_points caps bound these allocations/work dimensions.

Each orbit still costs m separate existing source evaluations and their group shifts. Source calibration/validation, history storage, residual replay and extra queries remain charged. Computing each observed cyclic correlation uses O(m) sign products/additions; scaling its coefficients costs O(d+log r); reducing and gcd-intersecting its polynomial has the usual degree-d polynomial cost plus scalar inversions. No source speedup or uniform image-separation theorem follows from having more constraints.

IMPORTANT ACCOUNTING CORRECTION: general Poly::inverse in the inherited backend is extended Euclid, not exponentiation. Earlier reports used an incorrect exponentiation description for that path. The native arithmetic and correctness outputs are unchanged. INVERSION_CORRECTION.md records the exact distinction while preserving historical attested artifacts. This compiler's monic remainder tree skips unit inversions, and its explicit denominator batch inverse and powers do use exponentiation. Generic gcd inversions must be charged separately as Euclidean integer operations, not silently converted into modular-product counts.

## Reciprocal symmetry and limitations

The selected coefficients satisfy

    C_ell[t] = chi_r(a)*C_ell[d-t].                     (2)

Indeed X^(r-1)f_a(1/X)=a^h f_(1/a)(X), and f_(1/a)(X)=f_a(aX), since a^(r-1)=1. At indices divisible by m, a^n=1, proving (2). For nonzero normalized images v with v^d=1, it follows that

    C_ell(1/v)=chi_r(a)*C_ell(v).

In particular, if m and d are both even, chi_r(a)=1 for every lag, r=1 mod4 and every image v is a square. Both all pair correlations and the folded product agree under v ->1/v. In original variables, z and b^(2m)/z are indistinguishable by these aggregated constraints at that radius. Tests retain nontrivial examples. The complete sign word can distinguish orientations; the adapter keeps it for later fiber refinement. Another radius changes the reciprocal pairing, but no universal number-of-radii bound is supplied.

For m=2 the nontrivial correlation is simply twice the folded product, and lag0 only records the number of nonzero signs. The resulting constraints add no image restriction beyond the product in the tested exhaustive domain. This is an essential control, not a favorable case to omit. Correlations are not a complete invariant of a sign word, and equality of them is never treated as equality of the scalar.

## Source and proof map

Sources checked (KB): tool absent; repository wrapper has no executable. Read local blocked-character recurrence, inert-seed scalar product tree, pair/triple moments, power-fold and retained-fiber derivations. Read Bostan, Gaudry, Schost, Linear Recurrences with Polynomial Coefficients and Application to Integer Factorization and Cartier-Manin Operator, SIAM J.Comput.36(6),2007, introduction/main algorithm and coefficient recurrence application: https://specfun.inria.fr/bostan/publications/BoGaSc07.pdf (DOI10.1137/S0097539704443793). Only its established recurrence-block mechanism is used; its stronger optimized bounds are not claimed for this implementation. An inaccessible guessed Texmacs URL supplied no evidence. No novelty claim is made.

Blocks.lean proves OrbitCorrelation.run_append: sequential block execution is right-after-left function composition. Lean4.34.1 checks it with no axioms. This is the block-order invariant only. Root-of-unity projection, the finite-field recurrence, matrix interpretation, reciprocal symmetry, source correctness and complexity remain written proofs, not Lean-certified theorems. The available formal project still lacks built Mathlib. No fresh lean4checker executable is available. No custom axiom or bypass is used.

Generation pins the unchanged original production prefix and derives a second recovery method by adding orbit collection and correlation updates before the singleton test. The original method remains present. All arithmetic is native Rust; reference enumeration and the dense polynomial power occur only in tests. Validation-01 passing source is preserved before adding reciprocal-symmetry controls in validation-02. No cloud work, timing panel or rerun of the original4,563-case cohort.

Next: exploit reciprocal structure or higher orbit invariants without paying for every lag; evaluate image separation across radii while preserving the source-query cost. A compact feature alone does not establish complete extraction.

## Final producer check output

```text
running 4 tests
test tests::dense_coefficients_orbit_values_and_sets ... dense_compilers=98 orbit_values=12048 exact_image_sets=2691 strict_refinements=1434 quadratic_redundancy=618
ok
test tests::large_stride_stream_reference_and_caps ... large_stride_cases=15 largest_prime=65537 block_widths=64,128,256,512,1024
ok
test tests::public_correlated_recovery ... public_cases=46 recovered=46 unresolved=0 retained_routes=28 source_calls=453
ok
test tests::reciprocal_image_ambiguity ... reciprocal_controls=792 distinct_image_pairs=660
ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 157 filtered out; finished in 0.73s
```

The98 small compilers match independently expanded schoolbook polynomial powers over primes7,13,17,31,41,97 and all divisor m from2 through8, with all lags.12,048 direct orbit evaluations cover those image constraints;2,691 complete image sets agree with polynomial intersections, with1,434 strict refinements and618 quadratic redundancy controls. Fifteen larger stride cases at r65537 use widths64,128,256,512,1024 and lags0,1,m/2, comparing to a separately streamed scalar recurrence. That larger reference shares the recurrence identity, unlike the independent small dense reference.

Public recovery uses lag1 on the frozen20 r479 points and all13 r13 points for m3 and m6, totaling46. All46 recover, with28 retained-sign routes and453 source calls. These are software path counts, not a timing or speed comparison. Reciprocal controls count ordered radius/point cases; distinct_image_pairs denotes cases with unequal reciprocal images, not unique unordered pairs. Caps, invalid lags/signs/lengths/radii and source boundaries are preserved. All32 source/input hashes and generation hashes match.
