# Continuation: coverage from a multiquadratic-cover second moment

2026-10-30 UTC. Derived proof for independent review, proposed by the parent and re-derived here. No solver or performance experiment was run. This is a supporting coverage bound for an admissible prime-field family, not a sub-exponential ECDLP algorithm or a novelty claim. The remaining algorithmic bottleneck is a complete, sufficiently cheap implicit solver.

## 1. Input family and exact tuple convention

Let E/F_p: y^2=f(x) be nonsingular, p>3, and r=#E(F_p) odd. Let d=2^t divide p-1, H=mu_d in F_p^*, and g uniform in F_p^*. Fix odd m>=3 with d>=m. Let V_g=gH and F_g={P in E(F_p): x(P) in V_g}, including both signs. Since odd r excludes rational two-torsion, f is irreducible cubic and has no F_p root. Every usable abscissa therefore has exactly two rational lifts.

A tuple means an unordered selection of m signed rational points with **distinct abscissas**. For fixed g,R let V_g(R) count such tuples with sum R. Let q_g=|F_g|/2 be the number of usable abscissas and

    T_g=sum_R V_g(R)=2^m binom(q_g,m),
    M=binom(d,m).

Selected-chart tuples additionally require R!=O and x(R) unequal to every residual abscissa. Write V'_g(R),T'_g for their counts, and E_g=T_g-T'_g for excluded tuples. The norm system from the parent report parametrizes these selected-chart tuples; no y=0, opposite pair or target-overlap case is silently included.

## 2. The covers and their projection degrees

For any k-element set K={w_1,...,w_k} subset H, let C_K be the smooth projective model of

    y_i^2=f(w_i G), i=1,...,k.

These squareclasses are geometrically independent. Proof: Frobenius cycles the roots of irreducible f in length3. A ratio of two roots in H subset F_p^* would have cube1 as well as d-th power1, so it would equal1. Thus the root sets of f(w_i G) are disjoint. Every squareclass has an odd valuation at branch points private to it, which proves independence. The resulting cover is geometrically connected of degree2^k, with deck group (C2)^k.

Each morphism

    P_i:C_K -> E,  (G,y_1,...,y_k) |-> (w_i G,y_i)

has degree D=2^(k-1), by the corresponding function-field extension. These rational maps extend to morphisms on the smooth projective model because E is proper. At every g in F_p^*, no f(w_i g) vanishes, so the affine cover is smooth and its F_p-points are exactly the independent rational sign assignments above that g.

## 3. Exact degree identity for weighted sums of projections

For any morphism alpha:C->E define q(alpha)=deg(alpha^* O_E(O)), with value0 for a constant map. For nonconstant maps this is the usual map degree.

The elliptic theorem-of-square identity gives

    q(alpha+beta)+q(alpha-beta)=2q(alpha)+2q(beta).

Here this can be justified directly: on E times E the divisor of x(P)-x(Q) is the diagonal plus antidiagonal minus twice each O-axis, giving the corresponding line-bundle identity. Pulling that identity back along (alpha,beta) and taking degrees gives the formula even when a particular map is constant. The formula also gives q([c]alpha)=c^2 q(alpha) by induction (only c=0,+/-1,2 is needed below).

Let S be a sum of projection maps other than P_i. The deck involution tau_i negates P_i and fixes S. Composition with tau_i preserves degrees, so

    q(S+cP_i)=q(S-cP_i).

The parallelogram identity therefore implies

    q(S+cP_i)=q(S)+c^2 q(P_i).

Induction over i proves the exact identity

    deg(sum_i c_i P_i)=2^(k-1) sum_i c_i^2

whenever at least one c_i is nonzero. In particular every such sum map is nonconstant. This avoids any unproved cancellation assumption about the maps, CM, or generic j. It is not a heuristic numerical degree estimate.

A nonconstant morphism between smooth projective curves is finite. Consequently its fibre above O has at most its degree many geometric points, counting fewer points if multiplicities occur. This bound also bounds the rational points with g in F_p^* in that fibre. No enumeration of C_K or high-genus point-count computation is an algorithmic prerequisite; these covers are used only in the proof.

## 4. Exact collision bound

Fix two ordered abscissa-index subsets I,J of H, each of size m. Let h=|I intersection J| and k=|I union J|=2m-h. On C_(I union J), choose the point sign used by the first tuple at shared indices, and introduce relative signs sigma_i for the second tuple at those h indices. At J-only indices use the second tuple's sign as the cover coordinate. This is a one-to-one encoding of signed tuple pairs at rational g, because no y_i is zero.

Their sums agree precisely when

    Phi=sum_(I-only) P_i -sum_(J-only) P_i
        +sum_(shared) (1-sigma_i)P_i = O.

If ell shared relative signs are negative, the coefficient-square sum is

    sum c_i^2=2(m-h)+4ell,

and section3 gives degree 2^(2m-h-1)[2(m-h)+4ell]. Summing over all relative signs yields

    sum_sigma deg Phi
      =2^(2m-h-1) [2^h*2(m-h)+4*h*2^(h-1)]
      =m4^m.

For h=0 the same formula holds with the obvious zero contribution of the ell term. The only zero-map case is I=J and every relative sign positive. It counts the identical-tuple diagonal, whose total over g,R is sum_g T_g. Every other case is bounded by its fibre degree.

Thus the following exact inequality holds:

    sum_(g in F_p^*) sum_(R in E(F_p)) V_g(R)^2
       <= sum_g T_g + m4^m M^2.                 (C1)

This includes all relative signs already. No additional 2^m or 2^h factor should be appended.

Since T_g<=2^m M, let nu_g=(1/r)sum_R V_g(R)^2. Averaging (C1) gives

    E_g[nu_g] <= K,
    K=2^m M/r + m4^m M^2/((p-1)r).             (C2)

## 5. Selected-chart exclusions are negligible at growing arity

For a fixed index set I, work on C_I, D=2^(m-1). A tuple is excluded when its total sum is O or its total equals plus/minus one of its own P_j. The weighted-map degree formula bounds these fibres by

    sum_i P_i=O:                  D*m;
    sum_i P_i=P_j:               D*(m-1), for each j;
    sum_i P_i=-P_j:              D*(m+3), for each j.

Summing these bounds, with overlaps harmlessly overcounted, gives

    sum_g E_g <= M D(2m^2+3m).                 (C3)

The second case has the coefficient of P_j zero; the third has that coefficient two. All three maps are nonconstant for m>=3. This bound includes all sign assignments via rational cover points and needs no extra sign multiplier.

It follows that

    E_g[E_g] <= M2^(m-1)(2m^2+3m)/(p-1).

This is exponentially small relative to the per-base tuple supply T_g~M in the intended n=log_2 r, m=Theta(sqrt(n)) regime, even though the absolute bound can grow exp(O(m)). It is a quantified chart-loss estimate, not a claim that exceptions never occur.

## 6. A finite, explicit good-base/coverage consequence

Assume p>=17, d>=8m^2, and define

    kappa=d^m/(m!r) >=2^m.

The latter is ensured by taking d as the least admissible power of two at least2(m!r)^(1/m). This choice still requires d|p-1.

The previously derived base-density bound gives

    epsilon_den=4/d+4(5sqrt(p)+1)/(p-1),
    Pr[B(g)<d/2] <=epsilon_den.

On the density-good event q_g>=d/4. Since d>=8m^2, a product estimate for binom(q_g,m) yields

    T_g=2^m binom(q_g,m) >=d^m/(2^(m+1)m!)
                             =kappa*r/2^(m+1). (C4)

By (C3) and Markov, the chance that E_g>kappa*r/2^(m+2) is at most

    epsilon_chart=2*4^m(2m^2+3m)/(p-1).         (C5)

Thus, on both good events,

    mu'_g=T'_g/r >=kappa/2^(m+2).              (C6)

By (C2), Pr[nu_g>4K]<=1/4. Intersecting these events and optionally the short-relation rationality good event from continuation-math.md leaves a set of bases of probability at least

    3/4-epsilon_den-epsilon_chart-Delta.

For each such base let rho_g be the fraction of targets R admitting at least one selected-chart rational decomposition. Cauchy-Schwarz on V'_g gives

    rho_g >= (mu'_g)^2/nu_g
          >= kappa^2/(2^(2m+6)K).

Using M<=d^m/m!=kappa*r, r/(p-1)<=2, and kappa>=2^m,

    K/kappa^2 <= 2^m/kappa +m4^m r/(p-1)
               <=1+2m4^m <=3m4^m.

Therefore the explicit conservative bound is

    rho_g >=1/(192 m 16^m).                   (C7)

This is exp(-O(m)), not a constant coverage claim. All finite conditions and bad-base probabilities are stated above. In the admissible growing-arity regime of the prior rationality note with m=c sqrt(n)+O(1), c<sqrt(2/3), d=next power2 above2(m!r)^(1/m), all three error terms tend to zero; at least half of bases eventually satisfy rationality, sufficient density and (C7).

The coverage result itself does not require excluding nonrational geometric models: it counts rational tuples directly. Intersecting with the rationality-good event permits a geometric coefficient solver to extract core rational coefficients without extra field equations, if that solver is complete and sufficiently efficient.

## 7. How to use a good-base existence theorem without pretending to recognize it

No cheap predicate certifying a particular good g has been supplied. A valid randomized construction can instead sample a base, cap each attempt and the total number of oracle queries for that base, and restart with a new base if it cannot collect the required number of accepted relations.

Suppose, as the remaining algorithmic hypothesis, an implicit solver on every relevant good-base instance completely finds one selected-chart relation or correctly proves none within known C(n)=exp(o(n)) work, including all failed attempts, rational extraction and output verification. On a good base, fresh uniform mixed-target queries have success probability at least rho_0=1/(192m16^m). After an explicitly bounded number O((B+1)/rho_0) queries the probability of collecting B+1 relations is bounded below by a constant (and can be amplified using standard binomial tails). Each accepted relation is verified. A timeout or shortage of relations triggers a full-cost restart, not a false negative mathematical claim.

At least a constant fraction of sampled bases are good asymptotically, so the expected number of such blocks is bounded. Base construction costs d*poly(n)=exp(O(sqrt(n))); B<=2d; the mixed-target nullspace argument in continuation-math.md section9 obtains a nontrivial target dependency with probability1-1/r from any B+1 accepted rows. Dense linear algebra is exp(O(sqrt(n))) as well.

Thus the only genuinely unresolved large algorithmic step in this specific admissible family is now the complete uniform solver C(n)=exp(o(n)), with a known bound usable for a cutoff. The prime-family applicability conditions remain explicit. The proof does not construct that solver and therefore does not establish a sub-exponential algorithm.

## 8. Scope and provenance

- The cover degree, Galois flip orthogonality, collision count and chart-loss proof are mathematical derivations in this session; novelty has not been checked against the full literature and is not asserted.
- The only analytic input used for base density is the standard quadratic-character Weil bound already identified in continuation-math.md. The off-diagonal collision and exclusion counts need only map degrees, not an unproved high-dimensional equidistribution statement.
- The required tower divisor and odd-order input family are assumptions. The argument is not automatically valid for arbitrary primes, arbitrary isogeny/Dickson towers, even curve order, or repeated-abscissa charts.
- No point enumeration on these covers, solver run, cost measurement, asymptotic fit or experimental success is claimed here.
