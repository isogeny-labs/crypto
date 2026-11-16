# Apply the next mode before leaving reciprocal coordinates

T-1864, 2026-11-07. Producer derivation and deterministic software checks; unsigned [pcc:unverified].

## Construction

Retain the predecessor's assumptions: caller-certified native prime scalar field F_r, m≥3, 4m dividing r−1, n=(r−1)/(4m), nonzero radius b, A=m b^((r−1)/2), t=b^(−m), and four quartic phases u. First-mode candidates use W=V+V^−1 and G_u(W)=gcd(D_n(W)−u−u^−1,uH(W)−f0/A). Take the squarefree part of G_u, without enumerating or factoring its roots. All its quadratic fibers split over F_r by the predecessor's phase-domain argument.

The new representation is R=(F_r[W]/G_u)[V]/(V²−WV+1). Represent each element as a pair (a,b), meaning a(W)+b(W)V. This is a quadratic quotient algebra, not an extension-field substitution: it contains the reciprocal fibers over the same prime field and can have zero divisors. Each coefficient is reduced modulo G_u.

Multiplication uses

(a+bV)(c+dV)=(ac−bd)+(ad+bc+Wbd)V.

The implementation computes ac,bd,(a+b)(c+d), so three base polynomial products suffice, followed by additions, a shift and modular reductions. It never divides by a pair coefficient or assumes that this algebra is a field.

Modes F1(V),F2(V) are evaluated from the existing coefficient streams by balanced block composition. For two consecutive descending-coefficient blocks, return (value,V^length); combine as value_left*power_right+value_right and power_left*power_right. This avoids dense full-mode expansion and preserves the original stream cap. The next constraint is C=f1²F2(V)−A f2F1(V)², represented as a pair. The phase equation P=V^n−u is computed by pair binary powering.

## Exact compatibility lemma

For a fixed split fiber V²−wV+1=0, write P=a+bV and C=c+dV. Define

N(P)=a²+wab+b²,
N(C)=c²+wcd+d²,
Delta=ad−bc.

There exists a common quadratic root satisfying P=C=0 if and only if both norms and Delta vanish. Necessity follows by evaluation. For sufficiency: if b≠0, Delta makes the second form proportional to the first, and N(P)=0 makes −a/b a quadratic root. If b=0,d≠0, use the symmetric argument and N(P)=a²=0. If b=d=0, the norms force a=c=0, so every fiber root satisfies both forms. The argument also covers the repeated quadratic root w=±2.

Norms alone are insufficient: P and C may vanish on opposite roots. Therefore retain

K=gcd(G_u,N(P),N(C),Delta).

Because G_u is squarefree and split, K is exactly the W-projection of the jointly admissible oriented candidates. No enumeration is used in this step. Scalar norm-zero implications use the prime-field assumption; the proof is pointwise and does not divide by possible zero divisors in the quotient ring.

## Returning to the original image variable

Only after filtering is K lifted, under the unchanged cap 2deg(K)≤B. The predecessor orientation gcd imposes V^n=u. A second filter evaluates C on that lift and takes a gcd: projection guarantees some common orientation, but does not say every orientation is valid when u=±1. This final filter is required and retained.

To evaluate pair polynomials on a monic nonzero-constant lift L(V), obtain V^−1 directly from L(V)=0: its coefficients are −L_{j+1}/L_0. Substitute W=V+V^−1 modulo L, evaluate the pair by Horner, and take gcd with L. There is no polynomial root search. Oriented factors from different u have disjoint roots and are multiplied.

Zero is retained exactly when f0=A and f1² c2=A f2 c1², with h=(r−1)/2, c1=−h and c2=h(h−1)/2. Here h is even by eligibility. Finally substitute V=tZ and normalize. The result is exactly the intersection of the original first two mode constraints with the m-th-power image domain, including multiplicities removed by that squarefree domain.

## Integration, work and storage

RecoveryModes tries this route only while its image domain is implicit and m≥3. On success it intersects the resulting small polynomial and skips the already-applied dense C0/C2 path. On unsupported parameters or a capped lift/union, it retains the predecessor recovery path. The earlier reciprocal adapter is generated independently as ReciprocalControl so its observed10success/265fallback control remains reproducible. No source calls, radii, final public checks or recovery routing rules change.

Let d=deg(G_u), L be a mode's coefficient count. Pair multiplication uses three scalar-polynomial products plus reductions at degree d. Balanced evaluation has at most two pair multiplications per internal block node. A conservative work bound is O(L M(d)) plus coefficient generation; using the actual node degree min(block_length,d) yields the sharper balanced sum sum_j O((L/2^j) M(min(2^j,d))). Charge reductions and additions as well. The implementation retains no full coefficient vector; recursive live pair state has a conservative O(d log(L+1)+B) field-element bound, plus the context and polynomial engine's transient allocations.

Phase powering uses O(log n) pair products. Norm/determinant construction, squarefree normalization and three compatibility gcds are additional charged work per nonempty branch. Full mode streams are presently reevaluated per branch; no unmeasured amortization is assumed. The post-lift pair evaluation is capped but uses Horner in W. Vec capacities, metadata and allocator overhead remain charged, together with the predecessor transform, quartic-root search, source grid and all failed work before fallback. The cap limits lifting/union degree, not all intermediate W-state storage or process bytes.

The observed decrease in projection degree is an exact control result, not an elapsed-time or end-to-end work comparison. The norm/determinant method is exact for this phase plus one additional constraint; arbitrary later constraint histories require further compatibility handling.

## Native checks

validation01 records optimized Rust with overflow checking, compile/check exits0 and57transitive source hashes.

- 7,424 pairs of linear forms over split quadratic fibers in F7,F17,F31; includes3,248zero-leading-coefficient controls and1,536ramified controls.12cases pass both norms but fail common-root compatibility, and are rejected by the determinant.
- 18 eligible mode configurations across prime scalar fields17,41,97,193,281.324pair-mode values agree with independent dense coefficient evaluation.972full output polynomials equal the original dense first-two-mode intersection coefficient-for-coefficient, including zero observations;198zero-cap controls reject nonconstant outputs.
- Public F257 curve y²=x³+x+7, base(1,3), subgroup order281, mode5:281targets match isolated scalar, route and all PullStats;2,881source calls, zero unresolved.275attempts yield40successful compressed intersections and235fallbacks under cap7. Preserved predecessor control has10successes/265fallbacks on the same targets.
- Across branches actually processed before return/fallback in that public panel:890branches, total projection degree2370before and1530after, with410strict reductions. These sums exclude any unprocessed branches after an early cap return; they are not full-workload rates.
- Inherited reciprocal2409dense-image controls, context/cache/arithmetic checks and617targetpanel (5929calls,0unresolved) pass. The separate mode56panel remains281targets/14421calls. Historical simple63-casebaseline still retains17unresolved; later adapters resolve63with511calls.

An initial Python generator SyntaxError occurred before generating source or compiling, caused by quoting an embedded Rust lifetime. The failed generator and error note are retained. The corrected generator and all output hashes are pinned. No native compile/assertion or report failure. No timing/cloud run or fullCargo rerun; historical missing-fixture/deadline limitation remains. Original4563-case cohort and prospective protocol unchanged. Independent signed review is open.

Replay in a fresh directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-quadratic-modes/validate-recovery.py /tmp/quadratic-modes-replay-unique
```

## Sources and next construction

Sources checked (KB): MCP absent; scripts/research-kb status reports no executable. Exact predecessor reciprocal/context code and polynomial engine were inspected. Primary sources: Bhargava-Zieve, reciprocal fibers and Dickson definition, https://websites.umich.edu/~zieve/papers/manjul.pdf; official FLINT modular-polynomial conventions, https://flintlib.org/doc/nmod_poly/index.html. No FLINT performance assertion or novelty claim is made. The compatibility lemma is proved above and independently checked against explicit finite fibers.

Next persist phase-tagged states across additional radii/history constraints, including the changed coordinate scale, instead of lifting after C2. That requires preserving common orientation across the whole constraint set, not independently taking norms. The present report/diagram show only the implemented first-radius C2 path; historical/canonical graphs remain unchanged.
