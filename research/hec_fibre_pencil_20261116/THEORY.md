# Root-free fibres of a divisor pencil

Class: classical Riemann–Roch decomposition, proposed algorithmic organization; no novelty or speed claim. This successor addresses the complete incumbent boundary after the direct-profile gate failed. It does not inherit any historical timing ratio.

Let C: y²=f(x), f monic squarefree of degree 2g+1 over odd Fp. Let D=(u,v) be valid, deg u=g, u squarefree, gcd(u,v)=1. Regard D as its degree-g effective divisor minus g infinity. For t in Fp put

    h_t=v+t u,
    Q_t=(f-h_t²)/u=(f-v²)/u -2t v -t²u.

Q_t is monic of degree g+1. On affine points with u(x) nonzero define phi(P)=(y-v(x))/u(x). Then P belongs to the fibre t iff Q_t(x)=0 and y=h_t(x). At the original effective support, phi has removable singularities; at its conjugate support it has simple poles. Infinity is a simple pole. Thus phi has degree g+1, is separable (simple pole), and its finite fibres encode residual effective divisors E_t with

    div(y-h_t)=D_eff+E_t-(2g+1)infinity,
    [D]+[E_t-(g+1)infinity]=0.

The exact group identity remains true with intersection multiplicities. The first control below deliberately restricts to gcd(Q_t,u)=1 and squarefree Q_t, so ordinary bucket cardinality is sufficient; exceptional parameters are counted and retained, not treated as nonsplit failures of the unrestricted theorem. Singular input divisors/repeated support require a separate local expansion and are outside this gate.

## Exact equivalence to point-add descent

For any point P in a regular full rational fibre, let E=E_t-P. Then [D]+[P-infinity]=-[E-g infinity]. E has degree g, so this is precisely the reduced output of Cantor add_place. Each of the g+1 points of a full fibre gives a successful point-add transition. But every such transition yields the SAME decomposition of D after subtracting the added point. A sieve must deduplicate the entire fibre; counting its points as independent decompositions is an error.

For two distinct full fibres t,s their difference is principal. Their finite point supports are disjoint since phi is a function. Consequently these difference rows have rank exactly F-1 over any field when F complete fibres are retained in the full oriented rational-point catalogue: subtract fibre0 from each other fibre; a column in each nonzero fibre is a unique pivot. A pencil alone cannot constrain other point columns. This is exact full-catalogue row rank, not the dimension after eliminating off-anchor columns or determining a target log.

## Complete-cost screen before any timing

One sieve scans every oriented affine rational point with u(x) nonzero, evaluates u/v, forms one field quotient, and buckets by t. Catalogue discovery, inverses, exceptional handling, and memory must be charged. With M scanned points and F regular full rational fibres, (g+1)F<=M, and F independent decompositions give only F-1 homogeneous rows. A naive g+1-per-fibre gain is duplicate accounting. Against independent point-add attempts a fibre sweep may amortize arithmetic, but this has no tenfold premise at genus 2 from multiplicity alone (g+1=3), nor at g3/4 (4/5). This is not a bound on wall-time gains from a different arithmetic implementation.

Uniform fibre heuristics would predict F about p/(g+1)!, hence about (g+1)! scanned points per retained fibre when M≈p. That is a heuristic, not guaranteed by degree or separability; the frozen finite gate will report actual F. A whole-catalogue matrix costs more columns than Compact's anchor system. A useful successor must avoid that rank/cost tradeoff with a constructive anchor-elimination argument. No timing campaign is authorized by counting raw rational points alone.

## Sources checked

KB ready true, revision 9141e386254340a9bdba3036f2bb706a, refreshed 2026-11-14T21:44:48Z, lexical only. Snapshot predates these studies; current code is authoritative. Queries: hyperelliptic compact anchor principal relations; genus-2 constant factor/theta divisor; hyperelliptic pencil decomposition; Nagao/Riemann decomposition; large-prime tree hyperelliptic. Opened metadata/context source:b3c90d39a347ef661ccf10680a742186 (crypto:research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md), which warns against undercharging matrix and oracle work; related results did not establish this implementation. Nagao pointer source:6b2208c18d3ecd6afd0494c0c44c1a36 and GTTD pointer source:a2b0d45868e0cf479f3ce1e23a626be5 identify established decomposition/LP prior art, not evidence of novelty. Current local sources: ../hec_anchor_relations_20261114/anchor.rs (random point-add descent), compact.rs (signed norm kernel), ../hec_direct_profile_20261116/RESULTS.md (complete-cost failure).

Primary reference: Joux–Vitse, Cover and Decomposition Index Calculus on Elliptic Curves made Practical, EUROCRYPT 2012, https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf . The publisher/author fetch availability is recorded in SESSION.md. Its extension-field/subfield sieve differs from this prime-field full-point sweep. The identities above are derived explicitly and tested, not attributed as new results.

## Signed-catalogue correction and exact rank law

The oriented-catalogue rank above must NOT be substituted for signed group rank: the two sheets at one x are opposite columns in an odd-prime projection, and branch points have zero projected log. After identifying sheets and dropping branch columns, each rational x occurs in at most two complete fibres. A column occurring twice has opposite signs. Thus the transpose is a grounded graph incidence matrix. Vertices are the F retained full fibres; two-sheet occurrences give edges; one-sheet occurrences ground a vertex (other sheet lies outside the retained fibres).

Let c be the number of connected components without any ground. Over an odd prime ell, the left kernel of the signed fibre matrix is spanned by the indicator vectors of these c components. Fibre differences restrict coefficient vectors to sum zero. For F>0, their rank is exactly

    F-1-c+delta,
    delta=1 if at least one closed component size is nonzero mod ell, else 0.

Proof: ker(B^T) has dimension c, and the sum functional on it has rank delta; its intersection with the (F-1)-dimensional sum-zero space has dimension c-delta. The signed rank follows by rank-nullity. For F=0 both ranks are zero. The control independently compares this graph prediction to dense modular elimination. With all component sizes less than ell, delta is 1 iff c>0. This is standard incidence/rank algebra, not a novel claim.

If D is known nonzero of prime order ell>F, any closed component would imply size(component)*D=0 by summing its fibre equations, a contradiction. Hence c=0 and rank F-1 in this special subgroup setting. The control does not assume that its [n]P inputs satisfy this order condition. This detects the exact missing hypothesis in transferring the oriented rank argument into the actual signed catalogue.

## Constructive elimination to an arbitrary anchor subset (post-control derivation)

The graph law also gives the anchor-supported intersection without a full catalogue matrix. Let B be the signed F-by-m fibre matrix and I its zero-sum row-coefficient space. Let O be off-anchor columns. Build the same grounded graph using ONLY O; denote its ungrounded components by C_1..C_k, their sizes by s_j, and let delta_O be the rank (zero or one) of the size functional over the coefficient field. Then

    dim( row(I B) intersect anchor-supported vectors )
      = (k-delta_O) - (c-delta),

where c,delta are the full-graph quantities above. Proof: admissible coefficients are I intersect ker(B_O^T), dimension k-delta_O. The full-map kernel is I intersect ker(B^T), dimension c-delta, and lies inside the admissible space. Rank-nullity gives the result. This is an intersection of row spaces, not a claim that restricting a row to anchor coordinates preserves a principal relation.

A constructive basis requires no modular elimination on all m columns. For each off-graph ungrounded component form w_j=sum_{i in C_j} B_i restricted to anchors. If some s_r!=0 modulo ell, emit w_j-(s_j/s_r)w_r for every j!=r. If every s_j=0, emit all w_j. This spans every anchor-supported difference relation; dependencies of dimension c-delta may remain and can be eliminated in the small anchor space. Columns on grounded components force their coefficient to zero. The formula holds with branch columns omitted and any odd coefficient prime, including component sizes divisible by that prime.

Work includes enumerating every fibre, graph construction and component reduction, and writing the anchor rows. It does not make the catalogue scan free or guarantee k-delta_O positive. If k=0 then no anchor relation survives at all. This directly supplies the structural test the successor needs; it has NOT been independently exercised by the frozen curve gate, which predates this post-control derivation. A new frozen gate must compare its outputs to full/off Gaussian ranks before using it in a supplied-target solver or drawing a performance conclusion.

## Post-anchor-control affine consequence (not a homogeneous rank claim)

For each off-anchor ungrounded component C, sum the original fibre equations and apply an odd-prime cyclic projection pi (which kills omitted branch-place 2-torsion). This gives

    |C| pi([D]) + sum_anchor w_C(P) pi([P-infinity]) = 0.

Thus, under a valid odd-prime cyclic projection with ell not dividing |C|, a SINGLE such component gives a decomposition of D over anchors. It need not give a nonzero homogeneous relation; the zero-sum restriction is unnecessary when D itself is retained as an unknown column. Two size-normalized component equations can be differenced to recover the homogeneous construction. Accordingly zero anchor-intersection rank alone does not rule out target descent. This is an exact consequence of the original divisor identity; no complete DLP solve, cost advantage, or original-group division without a projection/order hypothesis is inferred. Curve diagnostics already record k, the count of off-anchor ungrounded components. Since their counts are below the algebra-control prime1000003, k>0 indicates an available component equation for that control field; it is not a verified subgroup scalar solve.

The unprojected component sum may additionally contain a branch-supported 2-torsion term. The displayed affine equation is in the projected group; dropping that term in the original Jacobian without applying pi would be invalid.
