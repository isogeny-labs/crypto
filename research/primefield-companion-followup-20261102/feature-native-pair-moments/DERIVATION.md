# Closed pair moments and bounded root initialization

T-1794, 2026-11-07. Producer derivation and native checks; unsigned and not independently reviewed.

## Exact count

Let r>=5 be a caller-certified odd prime, h=(r-1)/2, epsilon=chi_r(-1), and s,t in {+1,-1}. Put S={x:chi(x)=s,chi(x+1)=t}. The exact integer count is

    m = (r-2-s*t-t-s*epsilon)/4.

The uncorrected product (1+s*chi(x))(1+t*chi(x+1)) equals4 on S,0 at other nonendpoints. Its values at0 and-1 are1+t and1+s*epsilon. The individual character sums vanish and sum chi(x(x+1))=-1, proving the formula. The latter identity also follows by counting y²=x(x+1): (2x+1-2y)(2x+1+2y)=1 has r-1 solutions, so r+sum chi=r-1. Thus the count needs no field enumeration or dense polynomial.

## Low-order moments

For 0<=j<h, let mu_j=sum_(x in S)x^j, with x^0=1 even at0. Then in F_r,

    mu_j = (-s*t*binom(h,j)
            -(-1)^j*(1+s*epsilon)
            -[j=0]*(1+t))/4.

To prove it, weight the previous uncorrected indicator product by x^j. The constant and single-character sums vanish modulo r in this range: their polynomial degrees are below r-1. For the double-character term,

    sum_x x^j*chi(x)*chi(x+1)
      = sum_x x^(h+j)*(x+1)^h
      = -binom(h,j).

Only the coefficient of x^(r-1)=x^(2h) contributes; the total degree is below2(r-1). Finally subtract the two endpoint values, with0^0 handled by the explicit indicator. For j=0 the formula agrees with the canonical residue of the exact integer count.

The binomials update by B_0=1, B_j=B_(j-1)*(h-j+1)/j. Requested order d<h uses only d+1 binomial/moment slots and d+1 inverse slots. The inverse recurrence builds denominators1..d in O(d) field work; the current implementation computes1/4 by one O(log r) exponentiation. Total prefix work O(d+log r), storage O(d). Bounds on d are checked before allocations. This is normalized offsets0,1; arbitrary-offset affine transport is not implemented here.

## Root initialization and public-point recovery

Every pair class has m<h, so the prefix through m is in range. If m exceeds max_degree, roots() returns Limit before requesting that prefix. Otherwise the previous Newton identities reconstruct the monic degree-m root polynomial in O(m²) field work. roots.rs is extracted byte-for-byte from the preceding root-state module; its subsequent modular-power/gcd updates are unchanged.

Recovery obtains signs at Q and Q+P from the standalone source. A first zero supplies candidate0; a second zero supplies candidate r-1. Both require the same final [candidate]P=Q check. For nonzero signs the closed pair count/moments initialize Roots directly, without constructing a degree-r indicator, an O(r) inverse table, or an enumerating compiler. Later signs use the shrinking root quotient; query exhaustion is Unresolved and a degree cap is Limit. The initial source check precedes point addition. No scalar label enters runtime.

The full root state still has m+1 coefficients with m approximately r/4. Eliminating the degree-r indicator does not make full root initialization polylogarithmic: the Newton conversion is quadratic in m. A low-order moment prefix alone does not determine this large candidate set. The earlier Chebyshev pair-gcd work already supplies a succinct pair polynomial evaluator; this component is a moment-prefix interface and native recovery composition, not a claim of new pair structure or measured superiority.

## Software evidence and limits

validation-02 passes three selected methods,157 imported filtered. All36 nonzero-sign classes across primes5,7,11,13,17,31,43,101,479 match exact test-only candidate enumeration for count and all moments through m; reconstructed root polynomial coefficients match products of the corresponding linear factors. All20 frozen p509/r479 public-point recoveries pass final verification, including2 zero-sign transcripts. Initial degrees are bounded by120; the degree16 control rejects before large root allocation. One-query exhaustion and noncanonical input controls pass.

At r=2^61-1, the test constructs only17 moments for each of four sign classes. Their summed moments satisfy the partition of F_r minus{0,-1}, and degree128 root requests reject. This is a bounded scalar-field prefix/consistency check, not an independently enumerated large-field reference or a large-curve recovery. The proof supplies the general formula; these controls check its implementation only in the stated ways.

Initial validation-01 failed compilation from a missing impl delimiter; exact source and errors retained. Corrected validation-02 has no failed selected tests. Test/normal builds retain16/26 warnings.27 dependency hashes are pinned. Normal dep-info excludes the dense aggregate, previous hybrid, quartic compiler and bitplane modules, plus legacy source/transport paths. Source prime/word/projection domain restrictions remain unchanged. The original4563-case cohort was not rerun. No cloud experiment or timing comparison.

## Sources and next task

KB wrapper unavailable. Read current moment-root and aggregate derivations, native RootSet implementation, and CONTINUATION.md's Chebyshev-pair and triple-constraint sections. Those summaries are prior-art context; the older full reports were not available in this checkout. Keith Conrad, Gauss and Jacobi Sums, Theorem2.6 and quadratic specialization: https://kconrad.math.uconn.edu/blurbs/gradnumthy/Gauss-Jacobi-sums.pdf . Elementary power-sum/coefficient reasoning is written out above. No novelty claim.

The next scaling dependency is a compressed update of the moment representation after a third and later sign, avoiding conversion to approximately r/4 roots. Existing triple-constraint work supplies a count via an elliptic order but does not yet supply this bounded-order moment stream. Any such extension must retain endpoint corrections, exact count and source-query costs.
