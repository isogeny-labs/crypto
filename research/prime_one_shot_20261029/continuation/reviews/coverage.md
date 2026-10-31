# Independent review: second-moment coverage from multiquadratic covers

Reviewer: `/root/theory_review`, 2026-10-30 UTC. This independently examines the proposed degree/collision argument sent by `/root`; it is a mathematical review, not a computational experiment, formal proof certificate, solver runtime theorem, or novelty claim.

## Verdict

The proposed argument is mathematically coherent and supplies a **scoped high-probability lower bound on actual rational selected-chart target coverage**, not merely a representation-count mean. The crucial degree quadratic form and the collision classification survive scrutiny. Total morphism degree suffices for the finite-fiber count even when a sum map is inseparable.

An explicit conservative bound is given below. It is exponential only in m. Thus when m=Theta(sqrt(log r)) in the admissible Kummer field family, its inverse is sub-exponential in bit length. This closes the separate coverage-probability assumption for that family, subject to retaining all stated hypotheses. It does not produce an efficient decomposition solver or an efficiently recognizable good-base certificate.

## 1. Assumptions and counted objects

Let p>3, E/Fp be nonsingular, r=#E(Fp) odd, d=2^t divide p-1, H=mu_d, and g vary uniformly in Fp*. The branch-independence argument from `continuation-rationality-review.md` applies: distinct f(wG), w in H, have disjoint finite branch sets and independent squareclasses over Fpbar(G).

For each g, the signed rational base F_g consists of both points above each usable x in gH. Write B(g)=|F_g|. For an unordered m-subset I of H, choose independently one sign of the rational y coordinate at every usable g*w, w in I. This gives an unordered point support with **distinct abscissas**. Let V_g(R) count those supports with sum R in E(Fp). Let

    T_g=sum_R V_g(R)=2^m binomial(B(g)/2,m).

There is no m! in V_g, because I is already an unordered root subset. Signed-point and canonical-abscissa conventions are not interchangeable.

For the norm chart, let W_g(R) retain only supports with R!=O and x(R) outside their abscissa set. At odd arity, every such relation has the desired unique monic Riemann-Roch norm representative; distinct abscissas and target separation make its norm squarefree and gcd(C,U)=1. Hence W is the appropriate actual rational selected-chart count. It satisfies W_g(R)<=V_g(R).

## 2. Degree quadratic form: why the cross terms vanish

Fix k distinct roots w_1,...,w_k in H and let C be the smooth projective normalization of the connected multiquadratic cover

    y_i^2=f(w_i G), 1<=i<=k.

Over Fpbar, its function field has degree 2^k over Fpbar(G). The projection

    P_i:C -> E, (G,y_1,...,y_k) |-> (w_i G,y_i)

is a morphism of degree 2^(k-1), by the corresponding function-field index. Rational maps from the smooth projective source to the projective elliptic curve extend across all missing points.

For integer coefficients c_i, set F_c=sum_i [c_i]P_i. The function

    q(c)=deg(F_c^* O_E(O))

is a nonnegative quadratic form. One way to state this precisely is to pass from Mor(C,E) modulo constant maps to Hom(Jac(C),E); alternatively the theorem of the square gives the parallelogram identity directly. Constants have q=0 and do not affect degree. Multiplication by an integer has total degree c_i^2, including in positive characteristic.

For i!=j, the Galois automorphism flipping only y_i carries P_i to -P_i while fixing P_j. Precomposition by this source automorphism preserves degree, so q(P_i+P_j)=q(P_i-P_j). The parallelogram identity forces the cross term to vanish. Consequently

    deg(F_c)=2^(k-1) sum_i c_i^2

whenever c is nonzero. In particular F_c is nonconstant. The only zero-coefficient map is the identically O map in the collision application.

This is a degree of morphisms, not a claim about the cost of evaluating or solving F_c. A nonconstant finite morphism between projective curves has at most deg(F_c) geometric points in any fiber, because each point contributes a positive local multiplicity. **Separability is not required for this upper bound**; inseparable degree only adds multiplicity. Points over G=0, infinity or branch values may be included in the total degree but only make the bound more generous when counting the desired nonzero finite g values.

## 3. Collision classification and exact sign summation

Fix two m-subsets I,J of H. Let h=|I intersection J| and k=|I union J|=2m-h. A pair of sign choices can be represented uniquely by:

- One choice of y_w for every w in I union J, represented by a point on C.
- A relative sign sigma_w in {+1,-1} for every shared w, specifying the second support's sign relative to the first.

If j of the h relative signs are negative, equality of the two support sums is the O fiber of a map with coefficients:

- +1 on I minus J;
- -1 on J minus I;
- 0 on the shared roots with sigma=+1;
- 2 on the shared roots with sigma=-1.

Thus its degree, unless all coefficients vanish, is

    2^(k-1) [2(m-h)+4j].

Summing this degree over all 2^h relative-sign choices gives exactly

    2^(k-1) [2^h*2(m-h)+4h*2^(h-1)] = m*4^m.

The expression also works when h=0, interpreting the second contribution as zero. Only I=J and every relative sign +1 gives the identically O map. That case is exactly the diagonal where the two point supports are identical. It must be counted as T_g rather than bounded by a degree-zero fiber. No other collision class is a hidden identity, because any nonzero coefficient vector has positive degree by section2.

There are L=binomial(d,m) choices of I. Therefore

    sum_(g in Fp*) sum_(R in E(Fp)) V_g(R)^2
      <= sum_g T_g + m*4^m*L^2.

The fiber argument counts geometric points and can include assignments not rational over Fp, but the actual rational collisions form a subset. That direction is safe. At nonzero rational g, the cover is unramified because f has no rational root, and the sign-pair parametrization is exact for the assignments being counted.

Since T_g<=2^m L, define

    M=2^m L + m*4^m*L^2/(p-1).

Then E_g[sum_R V_g(R)^2]<=M. This is an actual second-moment estimate; it does not replace coverage with the first moment.

## 4. Loss from the norm chart

For one m-subset I, use the same full cover with k=m and all its sign assignments. Supports excluded by the chart fall into the following events:

1. R=sum_i P_i=O. The map has degree m*2^(m-1).
2. R=P_j for some j. The map sum_i P_i-P_j has degree (m-1)*2^(m-1).
3. R=-P_j for some j. The map sum_i P_i+P_j has degree (m+3)*2^(m-1).

Every map is nonconstant for m>=3. The last two events cover x(R)=x(P_j) for finite R. Double-counting overlaps only enlarges the upper bound. Thus across all nonzero g the total number of excluded rational supports for one I is at most

    A_m=m*2^(m-1)+m(m+1)*2^m
       =m*2^(m-1)(2m+3).

Let E_g be the number of excluded supports at g. Summing over I gives

    E_g[E_g] <= A_m L/(p-1).

This avoids an unjustified claim that target overlap is independent of the support. It bounds that loss by a separate weighted sum-map calculation.

## 5. Explicit finite-parameter coverage statement

Assume d is divisible by four and d/4>=m. Let

    L0=binomial(d/4,m),  T0=2^m L0.

On the density event B(g)>=d/2, T_g>=T0. Markov gives

    Pr_g[E_g>T0/2] <= 2 A_m L/((p-1)T0).

For any epsilon in (0,1), the second-moment bound gives

    Pr_g[sum_R V_g(R)^2>M/epsilon] <= epsilon.

On the intersection of the density event and the complements of those two bad events, the total chart-support count is at least T0/2 and its second moment is at most M/epsilon. Cauchy-Schwarz therefore gives actual uniform-target coverage

    rho_g = |{R in E(Fp): W_g(R)>0}|/r
          >= epsilon*T0^2/(4 r M).

This is simultaneous over the realized base g; it is not a claim that every target is decomposable. The failure probability for this statement is at most

    [4/d + 4(5 sqrt(p)+1)/(p-1)]
      + [2 A_m L/((p-1)T0)] + epsilon,

capped at one. If the rationality lemma is also desired, add its Delta_short to the failure probability. All four terms concern one random base g and may be combined by a union bound; no independence is needed.

## 6. Sub-exponential inverse coverage in the stated family

Suppose the continuation note's admissible choice is used:

    d >= 2 (m! r)^(1/m),

with d the least such 2-power dividing p-1. For sufficiently large parameters d>=4m^2. Then

    L0/L >= (1/2)*4^(-m),
    L/r >= 2^(m-1).

The first follows by writing the binomial ratio as a product and using the small m^2/d correction; the second follows from the lower bound on d^m/(m!r) and the same falling-factorial correction. Hasse gives the elementary uniform bound r/(p-1)<3 for p>3. Substituting in the exact finite-parameter coverage formula yields a convenient conservative estimate

    rho_g >= epsilon / [16(1+3m)*16^m].

The chart-loss bad probability is at most

    2m(2m+3)*4^m/(p-1),

which is much smaller than exp(-Omega(m)) when m=Theta(sqrt(log p)). Choosing epsilon=exp(-kappa m) for a fixed positive kappa makes the moment failure probability exp(-kappa m) while keeping inverse coverage exp(O(m)). Density failure and the earlier short-relation failure also decay exponentially in sqrt(n) under the earlier conservative constant restriction c<sqrt(2/3), m=c sqrt(n)+O(1).

Therefore, in that **restricted admissible field family**, a random coset has rational-core behavior and selected-chart rational target coverage at least exp(-O(m)) with probability 1-exp(-Omega(m)). With n=log2 r and m=Theta(sqrt(n)), the inverse coverage is sub-exponential. This conclusion does not assume a Poisson law, uniform representation counts, independent supports, or full factor-base matrix rank.

## 7. What remains unproved or operationally required

- No fast coefficient solver follows from a degree bound on the morphism C->E. The cover used in the proof has exponentially many sheets; constructing or enumerating it would be a different and potentially expensive algorithm.
- The prime must admit the required 2-power divisor of p-1. The proof does not automatically apply to P-256's norm-one/Dickson base, auxiliary isogeny towers, arbitrary primes, or even whole-curve order.
- High-probability goodness is not an efficient recognition procedure for a realized g. A complete randomized algorithm still needs finite per-attempt budgets, independent output verification, a good-instance solver bound, and a retry analysis controlling the complement.
- The collision proof is over uniform g and uniform rational targets. A protocol using biased or label-dependent target selection requires another argument. The fresh mixed-target protocol supplies uniform R when P generates the whole prime-order group and a,b are independently uniform.
- A formal proof artifact and a novelty search are not supplied by this review. The proof uses standard facts about summation polynomials, elliptic-curve morphism degrees, and character sums, with the new combination checked here only on paper.

The coverage argument is a concrete supporting derivation in the proposed input-family analysis. It removes one former hypothesis in that scope, subject to the stated on-paper review limits. The uniform complete relation-solving cost remains the missing breakthrough needed for the user's algorithm request.
