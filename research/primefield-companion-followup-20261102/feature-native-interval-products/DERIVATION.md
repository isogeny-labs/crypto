# Capped polynomial interval products for mode recurrences

T-1855, 2026-11-07. Producer derivation and deterministic native checks; unsigned evidence [pcc:unverified].

## Construction and scope

The streamed-mode recurrence previously formed each numerator and denominator with m individual field multiplications. This adapter replaces those interval-product loops with capped polynomial blocks and remainder-tree evaluation, preserving the signed-binomial ratios and batch inversion. This is arithmetic product evaluation over the prime scalar field, not a group collision search. Existing public-point source, recovery, trace protocol and original cohort remain unchanged.

For a target stride m and positive cap B, choose w=min(ceil(sqrt(m)),B) with exact integer binary search. IntervalProducts computes

    R(a,l) = product_(i=0..l-1)(a+i) mod r,   0<=a,l<r.

It retains direct multiplication for l<32, w<2 or l<w. Otherwise it lazily builds P_w(X)=product_(i=0..w-1)(X+i), using the existing scalar polynomial product-tree engine. This polynomial is cached per coefficient stream; no full length-m polynomial or list of m points is created.

Write l=t w+u, 0<=u<w. Evaluate P_w at a+jw, j=0..t-1, in batches of at most w points. Each batch has its own product/remainder tree and is discarded before the next. Multiply its values into the output and finish the u residual factors directly.

## Equality proof and edge cases

Partition the interval into disjoint consecutive blocks. Each P_w(a+jw) equals product_(i=0..w-1)(a+jw+i). Multiplying the t values and the tail gives R(a,l), as a polynomial identity over F_r. It remains valid when a factor is zero or offsets wrap modulo r. Evaluation uses monic divisors and does not invert interval values. An empty interval gives1. Noncanonical a, l>=r, zero cap and unsupported modulus range fail.

Offsets satisfy a+jw<2r and r<=u64::MAX/2, so the u64 additions do not overflow; multiplication uses u128. Width is always at least1 and at most the supplied cap. The helper is initialized using m to select width; shorter initialization products may use the same width or the direct route.

For c_N=(-1)^(h-N)binom(h,N), h=(r-1)/2, the descending ratio is unchanged:

    c_(N-m)/c_N = (-1)^m R(N-m+1,m) / R(h-N+1,m).

Every factor in these recurrence intervals lies in1..h, hence each denominator is nonzero. The existing bounded prefix/backward inversion remains valid. Initialization now uses q=min(N,h-N), evaluates R(h-q+1,q)/R(1,q), and applies the original sign (-1)^(h-N). This symmetry avoids long initialization loops when the selected mode is constant (m=r-1 gives N=e<=2). Coefficient equality plus the earlier quotient-ring proof yields identical retained states and public recovery outputs.

## Conditional arithmetic and storage bound

Let M(w) be the cost of multiplying degree-O(w) polynomials. The cached block polynomial costs O(M(w) log w) to build. For an interval of length l, let t=floor(l/w). There are exactly ceil(t/w) evaluation batches on the blocked path. Each costs O(M(w) log w) for tree construction and evaluation. Value combination uses t scalar multiplications, and the tail uses u. Thus a conservative bound, charging block construction, is

    O((1 + ceil(t/w)) M(w) log w + t + w).

A stream reuses its block polynomial across all numerator/denominator products, so setup is paid once per stream, not per product. Scalar inversions, coefficient updates, chunk Horner, gcd and source operations remain separate costs.

If B>=ceil(sqrt(m)) and l=m, then w=ceil(sqrt(m)), t<=w and one evaluation batch suffices. The per-product arithmetic bound becomes O(M(sqrt(m)) log m + sqrt(m)). Under the actual native Karatsuba multiplication model M(n)=O(n^(log2 3)), this is O(m^0.79249 log m), versus m direct factors asymptotically. This is a derived bound, not an observed crossover or timing result. The implementation does not use the more specialized progression-shift algorithm or inherit its sharper bounds.

If B is held fixed as m grows, the number of evaluation batches grows proportionally to m/B^2; the square-root-width bound does not apply. The threshold32 is a deterministic implementation choice, not a measured fastest crossover. Small inputs may cost more than direct multiplication, and no universal performance dominance is claimed.

Persistent product data is one polynomial of w+1 coefficients and constant metadata. Each tree contains O(w log w) coefficients; polynomial-kernel workspace is additional. Coefficient batches remain capped by B. This supersedes the previous stream's constant between-chunk state: it now retains a short cached polynomial, while StreamModes itself remains a parameter-only32-byte object in the native build. Product workspace depends on w even when the retained image degree is smaller. First C0 remains dense; exact-order validation in the mode constructor still takes O(m) scalar steps. These costs are not removed by the interval helper.

## Deterministic software checks

`validation01/receipt.json` pins rustc and46transitive sources; overflow checks enabled. `validation01/check.stdout` records:

- 4,820 interval equalities against direct multiplication across primes7,17,97,257,65537, caps1,2,7,16,64, lengths including0,31/32 boundaries and4096, starts0,1,r-1,(r-1)/2.
- 1,808 blocked-path calls,2,485zero products,1,305wrapped intervals; width, point-batch and polynomial-length assertions pass. These are coverage counts, not performance measurements.
- Three constant-mode initializers at r=65537,m=r-1 produce exact e=0,1,2 coefficients without constructing a block polynomial.
- Additional public panel: curve over F257, subgroup order281, mode56, all281targets;14,421source calls, zero unresolved. Exact recovered scalar, route and call/replay/lift/image statistics agree with compiled/reference recovery. A matching coefficient-stream probe asserts the blocked route is exercised.
- Inherited476coefficient sequences,4,176residues,1,392scalar evaluations,42,324state comparisons and617public recoveries/5,929calls/zero unresolved all pass.

Reference scalars and zeta enumeration occur only in checks. Runtime target access remains public-point-only. No native compile/assertion failures, no cloud/timing experiment, no full Cargo rerun. Historical unrelated missing-fixture/deadline limitation remains. Original4,563-case cohort and prospective trace artifacts are immutable.

Replay into a fresh directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-interval-products/validate-recovery.py /tmp/interval-products-replay-unique
```

## Sources and next construction

KB unavailable: MCP absent and repository wrapper reports no executable. Inspected exact streamed-mode source and `feature-native-hgcd-roots/polynomial.rs`. Read Bostan, Gaudry and Schost, Linear Recurrences with Polynomial Coefficients and Computation of the Cartier-Manin Operator on Hyperelliptic Curves, https://mathexp.eu/bostan/publications/BoGaSc04.pdf, especially the recurrence discussion and standard multipoint-evaluation bound on pages1-4. The current construction uses ordinary product/remainder trees, not the paper's specialized progression-shift machinery. Product decomposition, binomial symmetry and multipoint evaluation are established methods; no novelty claim.

Next reduce repeated evaluation-tree construction or share product evaluation across adjacent recurrence ratios, while retaining explicit caps and exact denominator checks. Constructor order-validation and first-constraint costs remain separately charged. This report's SVG/PDF describes the new construction only; historical and canonical graphs are unchanged.
