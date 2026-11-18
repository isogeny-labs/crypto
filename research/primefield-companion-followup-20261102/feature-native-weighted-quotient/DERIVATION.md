# Weighted-support quotient recurrence

2026-11-07; T-1841. Derived identity and deterministic native software validation. This extends the exact public-point source polynomial kernel; it does not change input curves, runtime target access or the original 4,563-case cohort.

## Sources and selected construction

KB unavailable: no research-kb MCP tools; `scripts/research-kb status` reports no executable. Read exact predecessor `../feature-native-sparse-quotient/polynomial.rs`, its generation and validation wrappers, and primary [FLINT series division documentation](https://flintlib.org/doc/nmod_poly/index.html#power-series-inversion-and-division), specifically the coefficient recurrence for division and the binomial inverse specialization. The recurrence is standard. No novelty claim. The documentation notes ongoing revision; the proof below is self-contained and checked against native quotient/remainder references.

A support cardinality cap discards some cheap bands: a term at lag j only contributes n-j times. Replace the one-term cap by a derived work certificate, not an empirical crossover: W=sum over j in J of (n-j) <= n-1. This contains every predecessor accepted band and admits many-term late bands. Actual workload prevalence remains unmeasured; support tracing remains a separate next task.

## Quotient identity and proof

Over F_p, with prime p in the existing native kernel domain, let canonical nonzero a,b have degrees m>=d and n=m-d+1. Write A=rev_m(a), B=rev_d(b), alpha=B_0 nonzero, and J={j:1<=j<=min(d,n-1), B_j!=0}. Reversed quotient C has n coefficients and satisfies BC=A modulo X^n. Define C_i=0 for negative i and

    C_i = alpha^-1 (A_i - sum_{j in J, j<=i} B_j C_(i-j)).

Induction on i shows that coefficient i of BC equals A_i; lower indices have already been fixed and alpha is invertible. Thus C is the unique truncated quotient and reversing it returns the unique polynomial quotient. C_0=A_0/alpha is nonzero, so the returned quotient has exactly n coefficients. Terms of B at degree >=n cannot affect C. Their original divisor coefficients remain present in low remainder construction. Existing direct n=1/2 cases, zero dividend and error handling remain unchanged.

## Work certificate, exact arithmetic and storage

Scan j increasingly. For each nonzero term charge cost=n-j. Test cost > n-1-weight before addition; weight<=n-1 is invariant, so neither the subtraction nor accepted addition overflows usize. Reject immediately on exceeding the cap. The scan inspects at most min(d,n-1) coefficients. On rejection, its temporary support vector drops before unchanged Newton arithmetic starts.

For acceptance, store sorted (j,B_j) pairs. For each i traverse until the first j>i, then break. Exactly W entries pass that test, and at most one additional entry per i is examined. Hence support visits <=W+n<=2n-1, including visits for zero quotient coefficients. This is essential: a loop scanning the entire support for every i would not establish this linear traversal bound.

Let u=[alpha!=1], Z(C)=number of nonzero C_i, and Z_j(C)=number of nonzero C_0,...,C_(n-j-1). Skipping zero cross products and zero scaling yields the exact field-product count

    M = u Z(C) + sum_{j in J} Z_j(C) <= u n + W <= 2n-1.
    I = u.

The pre-scaling numerator is nonzero iff C_i is nonzero, because alpha is a unit. One inversion is performed for a nonmonic divisor, independent of the number of coefficients. The scan, at most n outer iterations, modular subtraction per nonzero cross product, and in-place reversal are separately charged O(n) word operations. These are deterministic arithmetic bounds, not elapsed-time comparisons with Newton for every input.

For s=|J|, the positive distinct integers n-j have sum at least s(s+1)/2. Thus s(s+1)/2<=n-1 and s=O(sqrt(n)). Storage is one n-coefficient output vector plus O(sqrt(n)) support pairs, with amortized Vec growth/capacity charged; input normalization and remainder/recursive allocations remain additional. Unlike the predecessor's single optional pair, support storage is not constant. Rejected prefixes also satisfy this support bound. With n=129, lags114..128 have weights15..1, total120<=128, admitting15 terms.

## Validation contract and results

These are deterministic software correctness/cost-identity checks, with no statistical inference, timing measurement, tuning, cloud job or broader benchmark. Frozen check loops in weighted-checks.rs use p=5,17,97,257; n=3,4,5,8,13,32,65,129; leading coefficient1/2; seeds0..3; empty/single/two-term and maximal accepted late-tail supports, plus every support mask for n<=8. Counts include deliberate overlap between named controls and exhaustive masks. Reference quotient uses the older inverse-series route; full quotient/remainder pairs are also checked.

validation-01 passes 2,080 accepted and 4,192 rejected cases. Accepted cases include896 multi-term cases,640 exact-cap cases and maximum support15. Every accepted case checks exact M/I, W productive visits, total visits<=W+n, scan bound, no Newton step and s(s+1)/2<=n-1. Every rejected case checks prior field counts, zero recurrence visits and rejection flags. Quotient and remainder agree in all6,272 cases.

Inherited1,536 single-term controls and384 dense controls pass unchanged, as do224 constant/4,744 linear quotient cases,640 wider cases,256 tree cases,2,592 division cases,288 matrix cases,432 resultant cases and8,320 singleton cases. Wider count equality is required only when the new weighted predicate rejects. Original historical checks/receipts remain immutable.

Commands, compiler, input hashes and stdout digests are in validation-01/receipt.json and recovery-01/receipt.json. Run `python3 validate.py NEW_OUTPUT_DIRECTORY` and `python3 validate-recovery.py ANOTHER_NEW_DIRECTORY` from this directory. Both wrappers use rustc with optimized overflow-checked native arithmetic and a60-second execution timeout. normal-build/receipt.json pins the non-audited metadata build. No full Cargo rerun: the historical missing fixture and deadline failure remain in prior records.

## Next useful work and boundaries

Trace the actual division support/length distribution and all source phases before selecting further kernel changes or claiming an aggregate cost reduction. General-resultant and point-source costs remain distinct. This component broadens the provably linear quotient class without changing public-input semantics. No canonical research graph or official hypothesis status changes; this directory contains a new scoped derivation/report, with unsigned producer PCC evidence.
