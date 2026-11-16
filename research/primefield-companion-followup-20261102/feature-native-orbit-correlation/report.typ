#set page(paper: "a4", margin: 15mm)
#set text(size: 9.2pt)
= Cyclic correlations on power images
T-1808 • 2026-11-07 • PCC-20261107-c6c777 • unsigned producer evidence

The image phase now uses cyclic correlations of individual orbit signs, in addition to their product. These provide polynomial constraints on z=(k+c)^m before the image becomes a singleton. They require no extra source queries once the orbit has been collected.

#image("flow.svg", width: 90%)

== Feature and coefficient compiler
Let s_j=χ(k+c−bζ^j), a=ζ^ell, h=(r−1)/2 and d=(r−1)/m. If ((X−1)(X−a))^h has coefficients c_n, then the cyclic correlation sum of s_j s_(j+ell) equals C(z/b^m), where C(V)=m sum_t c_(tm)V^t. The root-of-unity sum cancels all other exponents; the factored source multiplier is a square. The identity includes zero sign entries.

The coefficient recurrence is a(j+1)c_(j+1)=(j−h)(1+a)c_j+(2h−j+1)c_(j−1). A balanced2×2 polynomial-matrix block advances m indices. Evaluating it at0,m,...,(d−1)m retrieves only the needed coefficients. All denominators lie between indices1 and r−1 and remain invertible. A single explicit batch inverse normalizes the evaluated blocks. Production does not expand the degree-r−1 coefficient polynomial.

== Exact native checks
Final validation-02 passes four selected methods,157 imported filtered.98 compilers match independently expanded schoolbook powers across primes7,13,17,31,41,97, divisors m from2 through8 and all lags.12,048 direct orbit values agree.2,691 complete image sets match the intersections;1,434 are strictly refined by correlations.618 quadratic cases retain exactly the product-only image set.

Fifteen larger cases at r65537 use block widths64,128,256,512,1024 and lags0,1,m/2. They match a streamed scalar recurrence, exercising Karatsuba and Newton arithmetic. This larger reference shares the recurrence identity; the small dense references do not.

Public recovery with lag1 passes all46 fixtures: frozen20 on E/F509, y²=x³+2x+10, P=(3,68), r479,m2; all13 on E/F7, y²=x³+3, P=(1,2), r13 with m3 and m6.28 use retained-sign replay.453 source calls are recorded as software coverage, not a speed result.32 source/input hashes and generation hashes match. Original4,563 cohort unchanged; no cloud or timing panel.

== Symmetry that the extractor must retain
The coefficients obey C[t]=χ(a)C[d−t]. For nonzero normalized images v with v^d=1, C(1/v)=χ(a)C(v). When m and d are both even, all correlations and the folded product agree for reciprocal images at a fixed radius.792 ordered controls include660 cases with distinct reciprocal images. These are retained collisions, not silently accepted recoveries. Another radius changes the reciprocal pairing; no universal separation bound is supplied. Pair correlation at m2 is redundant.

== Cost and accounting correction
One lag has a safe setup bound O(M(m+d) log(m+d)+log r), using installed Karatsuba multiplication and ordinary product/remainder trees. Balanced m,d of order sqrt(r) give the derived exponent log_2(3)/2, with a suitable divisor required. L lags multiply setup and stored coefficients; compiling all m lags is not free. Conservative compiler workspace is O(m log(m+1)+d log(d+1)). Each orbit still uses m separate source evaluations and their group shifts.

General polynomial-backend inverses use extended Euclid. Earlier reports incorrectly described them as exponentiation-based; historical evidence is preserved and INVERSION_CORRECTION.md fixes the accounting. This compiler's explicit batch inverse and scalar powers do use exponentiation, while monic tree reciprocals skip unit inversions. Later gcd inversions remain separately charged.

== Proof map, sources and next construction
Lean4.34.1 checks the sequential block-composition invariant with no axioms. The finite-field projection, recurrence interpretation, symmetry and complexity remain written proofs. Validation-01 source snapshots are retained before symmetry controls. The report is producer evidence without an independent signed verdict.

Sources: existing blocked recurrence, inert seed and fold/fiber modules; Bostan–Gaudry–Schost2007, DOI10.1137/S0097539704443793, introduction/main algorithm and coefficient application. Established recurrence acceleration is reused; no novelty claim. KB unavailable. Next: exploit reciprocal structure or higher orbit invariants without compiling every lag, while resolving image separation across radii and retaining source cost.
