# Joint neighboring-mode coefficients

T-1874. Producer assertion; independent signed verdict pending. [pcc:unverified]

## Identity and alignment

Let r≥7 be a caller-certified odd prime, h=(r−1)/2, and c_N=(-1)^(h−N) binom(h,N), the coefficients of (X−1)^h. The factorial definition gives

c_(N+1) = −c_N (h−N)/(N+1).

All divisions occur in F_r. The paired modes are F1(Z)=sum_j c_(jm+1)Z^j and F2(Z)=sum_j c_(jm+2)Z^j. For m≥3 dividing r−1, their lengths agree. Indeed m divides2h, so h mod m is either0 or m/2. When m is odd it is0; when m is even and at least4, m/2≥2. Thus h mod m is never1 and floor((h−1)/m)=floor((h−2)/m). This covers the reciprocal adapter's narrower4m|(r−1) domain and also other admissible stream parameters.

At every paired position N=jm+1, 1≤N and N+1≤h<r. Therefore every denominator N+1 is nonzero. Top endpoints reaching N+1=h are valid, with derived coefficient c_h=1; the bottom endpoint is N=1. Singleton streams, including maximal order m=r−1, obey the same identity. These conditions are checked before/while constructing the reader; no division at N=r−1 is permitted.

## Implementation

NeighborReader owns only the existing F1 SharedCoeffStream. Each chunk captures the starting coefficient index, takes at most the existing capB anchor coefficients, forms prefix products of their N+1 denominators, performs one modular inversion, and recovers each denominator inverse backward. It emits bounded pairs[c_N,c_(N+1)]. It does not initialize or advance an F2 long-stride stream. Exhaustion and invalid index/field/stride inputs are rejected.

neighbor_block reads one coefficient pair per leaf and evaluates both modes across at most four branch rings. At an internal node, it obtains each branch's right-block power once and combines the two mode values independently. This keeps the same per-branch arithmetic as two shared_mode calls with their common power cache. The image adapter replaces only the F1/F2 stage; C0, canonical filtering, zero handling, caps, phase extraction and final point verification retain their previous paths.

## Charged work

For L paired coefficients, the two old streams have two initializations and2(L−1) long-stride transitions; the new reader has one initialization and L−1 long-stride transitions. This removes the second stream's initial binomial construction and its interval-product transitions. It adds ceil(L/B) adjacent-denominator inversions and, outside those inverse exponentiations, exactly5L+ceil(L/B) modular multiplications in adjacent_coefficients: L prefix products,2L backward inverse products,2L coefficient-ratio products and one inverse verification per chunk. Comparisons, additions/subtractions, index work, allocation and all inverse exponentiation costs remain charged. Existing F1 stream inversions and final advancement remain charged.

The pair buffer stores at mostB pairs. Refill also retains the old buffer until replacement and allocates anchor coefficients, prefix products and new output pairs; actual capacities and overlap are not a hard byte ceiling. Joint mode recursion retains two value arrays at once per subtree plus per-branch power caches. No O(L) full coefficient table is stored, but simultaneous value/copy/cache memory remains charged.

Successful-pair counters report anchor coefficients/transitions, extra adjacent inversions/multiplications and maximum buffered pairs. Removed stream/transition counts follow from the same L; they are not total field-operation or timing ratios. Counts cover the paired stage only, excluding C0/support construction and later phase-extraction streams. Source grids, exact order checks, all other source work and final public-point checks remain charged. Original4,563cohort and prior evidence remain unchanged. No cloud or timed workload is run.

## Controls

The coefficient checker enumerates admissible orders m≥3 in ten supplied prime fields and buffer caps1,2,7,16,64. It compares every pair with the separately compiled dense modes, checks endpoint inequalities, exact transition/batch counts and exhaustion, and includes singleton/maximal-order/top-h endpoints. Invalid index spans, coefficient range, h and stride are rejected.

The value checker compares joint mode outputs with separate shared-mode calls and asserts identical scalar/pair product counters. The state checker compares canonical triples, branch counts/supports, zero flags and materialized candidates through four radii. Public recovery compares the shared predecessor and neighbor adapter at caps64/16. See RESULTS.md and validation01 for observed counts, source pins and failures. The cumulative_counter_observation_sum debug field in stdout adds counter observations at multiple checkpoints; it is not a total product count and is not used as a result.

## Sources and next direction

Exact predecessor: feature-native-shared-branches/{shared.rs,shared-image.rs,context.rs,streamed.rs}. Primary source: NIST DLMF26.3.E1 and26.3.E6, https://dlmf.nist.gov/26.3 (factorial definition and adjacent binomial recurrence). The signed ratio is obtained by including (-1)^(h−N). This uses an established recurrence; no novelty claim. KB MCP unavailable and scripts/research-kb status reports no executable; local source and primary reference provide the fallback.

Next combine the adjacent denominators with the anchor stride denominators in one bounded inversion batch, rather than pay a separate adjacent inversion per chunk. Preserve accounting for every prefix product and retained denominator. Any elapsed-time or end-to-end advantage requires a separately admitted frozen workload and complete source/setup accounting.
