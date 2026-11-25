# Frobenius-sign decomposition of prime-field Semaev root sets

**2026-11-23 — producer-only, [pcc:unverified].** Follow-up to merged PR #5023. **Scope: the Semaev summation-polynomial equation only**; no factor-base map, no complete ECDLP solver, and no claim of Gröbner speedup.

## Exact set-theoretic invariant

Let p>3 be prime, E/Fp: y²=x³+ax+b nonsingular, d a fixed nonsquare, and x_1,...,x_m,X in Fp. Each x-coordinate admits two geometric lifts on E(Fp²). The lifts with square f(x) lie in E(Fp) and obey Frobenius sigma(P)=P. Those with nonsquare f(x) have ordinates omega*v, omega²=d, and obey sigma(P)=-P. Points with y=0 are rational two-torsion; choose their '+' label without loss of generality.

Let A be the signed sum of all '+' lifts and B the signed sum of '-' lifts, represented on the rational twist chart d*v²=f(x). If the full Semaev relation vanishes, A+B=O. Frobenius yields A-B=O, so **A=B=T and 2T=O**. Conversely, when the two sums match in shared rational two-torsion T, A+B=2T=O. The exact identity is

    S_(m+1)(x_1,...,x_m,X)=0 over Fp
    iff some signed '+' sum and signed '-' sum equal T in E(Fp)[2].

The right side includes the identity T=O and every rational y=0 point. This is a statement about **Fp-valued projected roots**, not equality of ideals or geometric schemes over the algebraic closure.

The root extractor enumerates at most 2^(#plus) plus-sector states and 2^(#minus) twist-sector states, plus a bounded set E(Fp)[2] (at most 4 points), and obtains candidate X from T - partial_sum. Cold two-torsion setup is O(p) by scan for tiny p and uses finite-field cubic factorization at p>=1000; it is cached per curve. The state sets remain exponential in m. A root may be at infinity in an intermediate elliptic group addition, so point identity branches are explicit.

## Low-degree polynomial charts

For a *rational* point/leaf, introduce selector s in {0,1}, y in Fp, and nonsquare d. The constraints

    s(s-1)=0
    [1+(d-1)s] y² - (x³+a*x+b)=0

have total degree <=3. For two finite points on the same sector with distinct x-coordinates, use slope lambda and the three degree-2 equations

    lambda*(x2-x1) = y2-y1
    x3 = scale*lambda²-x1-x2
    y3 = lambda*(x1-x3)-y1,

with scale=1 or d. The tangent chart uses lambda*(2*scale*y)=3*x²+a (degree 2). Every numeric affine chart is checked over F5/F7/F11. A *globally chart-complete polynomial system is not yet implemented*: inverse, equality, identity, two-torsion disjunctions and Fp membership are required. Degree <=3 for these exhibited charts is **not** a proof of bounded d_F4 or first fall degree; case splitting and field-membership costs may dominate.

## Correctness evidence

The gate compares an independent point/group-law extractor against the exact **fixed Sylvester-degree** Semaev resultant recurrence, followed by exhaustive root scanning for small p or exact Fp polynomial factorization for p=2^31-1. Every parameter had 50 deterministic random x-tuples (seed 12042026 + 10007*p + 733*m).

- 14 parameter cells, **700/700 random root-set matches**. For p=2^31-1 both (a,b)=(1,1), with three rational nonidentity 2-torsion points, and (1,4), with none, passed m=3,4,5 at 50 cases each.
- **16,440/16,440 exhaustive x-tuples** matched on the four tiny cells (p,a,b,m)=(5,0,1,3), (7,1,1,3), (11,1,1,3), (11,1,1,4). These are exact root sets, not multiplicity or elimination-ideal equalities.
- The exhibited leaf equations have total degree <=3, and the finite generic/double group-law chart equations degree <=2; numeric substitutions were tested for all enumerated finite pairs on p=5,7,11 base and twist charts.

**Smallest zero-only counterexample:** over F5, E:y²=x³+1, x-inputs (0,1,2) and target X=3 satisfy S4(0,1,2,3)=0. An incorrect split allowing only T=O returns no target. The correct common torsion point is (4,0), so the full torsion-aware solver returns {3}. This is *different* from the original PR #5023 counterexamples to Fp-only affine S3 chaining (which require infinity or Fp² witnesses).

## Timing diagnostic: specialized roots, not Gröbner

See sector_bench_medians.csv and regenerate exact raw paired timings with:

    PYTHONPATH=. python -m unittest -q summation_split.test_sector
    PYTHONPATH=. python -m summation_split.sector_experiment --out summation_split --samples 50 --repeats 3

The timing subset used 10 matched examples per tiny cell and only **two per p=2^31-1 cell**, after 50 correctness checks per cell. Python 3.13.5/SymPy, preliminary single-machine timings, producer-owned, curve torsion cache warm. Peak RSS is the process high-water mark, not solver incremental allocation. Method 1 generates the fixed-degree univariate resultant and scans/factors its X roots; method 2 enumerates signed Frobenius-sector point sums. Neither is a Gröbner solver, and the 2- or 10-case medians do not justify a statistical speedup claim.

On p=2^31-1, (a,b)=(1,1), m=3/4/5, matched median per root extraction was approximately 2.30/7.52/23.75 ms for resultant-plus-factoring and 0.060/0.088/0.141 ms for warm sector enumeration. With (a,b)=(1,4), m=3/4/5, it was 1.97/8.04/25.61 ms versus 0.050/0.097/0.113 ms. These **root-oracle** gaps widen with m on this very small m window. They say nothing about F4 performance on unspecialized Eq.(4) constraints.

## Negative results and next falsifier

1. Naive Fp-only affine S3 chaining loses Fp²/infinity witnesses (already established in merged #5023). This sector representation avoids mixed-coordinate Fp² witnesses for rational inputs but does not establish elimination-ideal equality.
2. Dropping nonidentity rational 2-torsion fails already at p=5, m=3. It is not safe to use the zero-only decomposition merely because most target curves have odd group order.
3. Generic full root enumeration remains exponential in m; the partition and twist-cubic setup introduce costs. A low-degree point-arithmetic circuit can still have exponentially growing Gröbner solving degree.
4. **Priority:** implement *all* identity and torsion branches as guarded exact finite-field polynomial systems, apply the same public slicing to the unspecialized S_(m+1) control, then compare actual F4/Singular/MSolve Macaulay degree and full wall/memory on at least two (p,m) configurations. Only thereafter claim success. Keep the p=2^31-1 root oracle as a correctness oracle, not as a substitute for GB.

## Prior work and limitations

- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves* (2004), https://eprint.iacr.org/2004/031 — resultant definition and geometric zero-sum semantics.
- Kousidis–Wiemers, *On the first fall degree of summation polynomials* (2019), https://arxiv.org/abs/1906.05594 — characteristic-two Weil descent, **not** a bound for these odd-prime Frobenius-sector equations.
- Kosters–Yeo, *Notes on summation polynomials* (2015), https://arxiv.org/abs/1503.08001 — warns against conflating first fall with regularity/solving degree and against overbroad complexity heuristics.
- Internal prior work: summation_split/FINDINGS.md (merged #5023), research/pdp_split_dreg_20261011/README.md, research/notes/index-calculus/RESEARCH_SYMMETRIZED_SEMAEV.md. Existing S3 chaining is classical. The Frobenius-sign derivation is elementary and **no literature novelty claim** is made without a further independent search.

Research-KB MCP tools and its local wrapper were not available in this runtime; repository/literature fallback used. Sage, Singular, FGb and msolve absent. No measured d_F4, true first fall degree, independent validator, PCC receipt, or successful unspecialized Gröbner cost comparison is claimed.
