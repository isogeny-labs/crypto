#set page(paper: "a4", margin: 18mm)
#set text(size: 10.5pt)
= Closed moments for the first two signs
T-1794 • 2026-11-07 • PCC-20261107-40916c • unsigned producer evidence

The pair of nonzero signs at offsets 0 and 1 now supplies an exact candidate count and a requested prefix of power sums directly. The implementation uses a binomial recurrence, with no dense indicator polynomial or enumerated coefficient compiler. It can initialize the existing shrinking-root recovery path under an explicit degree cap.

#image("flow.svg", width: 100%)

== Formula and boundary
For h=(r−1)/2 and epsilon=chi(−1), the integer count is (r−2−st−t−s epsilon)/4. For 0≤j≤h−1, the jth moment modulo r is one quarter of −st binom(h,j)−(−1)^j(1+s epsilon)−[j=0](1+t). The last terms remove the zero endpoints. The proof uses finite-field power sums and the quadratic correlation identity; DERIVATION.md writes out both.

== Native checks
Validation-02 passes three selected methods, with 157 imported methods filtered. All 36 sign classes across nine primes through 479 match enumerated counts, moments through the candidate count and exact root-polynomial coefficients. All 20 frozen public-point cases recover and pass forward verification, including two zero-sign transcripts. The initial root degree is at most 120; a degree16 cap rejects. One-query and coordinate controls pass.

At scalar prime 2^61−1, four 17-moment prefixes satisfy the partition identity for the field excluding 0 and −1, and degree128 root requests reject before large allocation. This is a bounded consistency control, not an independently enumerated large-field reference or a large-curve recovery.

== Costs and remaining construction
A prefix through d costs O(d+log r) field operations and O(d) storage. Full root initialization needs d=m approximately r/4 and O(m²) Newton conversion, so its inverse array and root state remain Theta(r). COST_CLARIFICATION.md corrects overly broad inverse-table wording in the preserved attested derivation. The source oracle and later gcd costs remain. No timing gain is asserted.

Next extend the moment representation through a third and later sign without materializing approximately r/4 roots. Prior triple-constraint work supplies a count through an elliptic order, but not this bounded-order moment stream. The source's prime/word/projection restrictions and original 4,563-case cohort remain unchanged.

== Evidence and sources
Twenty-seven dependency hashes pinned; normal dep-info excludes dense aggregate/hybrid and quartic compiler paths. Initial missing-delimiter failure preserved with exact source; corrected test/normal builds retain 16/26 warnings. Replay script and logs included. Existing pair/Chebyshev summaries and Keith Conrad's Gauss and Jacobi Sums, Theorem2.6, informed the derivation. KB unavailable. No independent review, cloud run or timing panel.
