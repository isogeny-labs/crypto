# Continuation: rationality of the shared-norm system on random Kummer cosets

2026-10-30 UTC. Derived mathematical note for parent review. No new solver, scientific run, asymptotic attack, formal proof certificate, or novelty claim is made. This supplies a scoped input-family lemma and removes one potential field-of-definition obstacle on a high-probability event; it does not supply the missing efficient relation solver.

## 1. Setting and exact statement

Let p>3 be prime and E/F_p: y^2=f(x)=x^3+ax+b nonsingular. Assume #E(F_p)=r is odd (the original target case has r prime). Let d=2^t divide p-1 and let H=mu_d in F_p^*. Draw g uniformly from F_p^*, set V_g=gH, and let F_g be the signed rational point base with x in V_g. Write B(g)=|F_g|, counting both signs, not abscissas. Let m>=5 be odd.

Define the short-relation bad event Z_g: there are s distinct abscissas in V_g, for some 3<=s<=m-2, and points over the algebraic closure above them whose sum is O. The points need not be F_p-rational; existential signs are included in this event.

A conservative exact upper bound is

    Pr_g[Z_g] <= min(1, Delta(p,d,m)),
    Delta(p,d,m) = (1/(p-1)) * sum_{s=3}^{m-2} binom(d,s) s 2^(s-2).

The sharper standard total-degree value (s-1)2^(s-2) can replace s2^(s-2) if independently verified. The conservative bound used here follows only from the standard per-variable Semaev degree and avoids relying on that sharpening.

The signed base size has the bound

    Pr_g[B(g)<d/2] <= min(1, 4/d + 4(5 sqrt(p)+1)/(p-1)).

On every g outside Z_g, for every nonzero rational target R, every geometric solution of the selected-chart norm system

    A monic degree h, C degree<=h-2, m=2h-1,
    A^2-f C^2=(X-x_R)U,
    A(x_R)-y_R C(x_R)=0,
    U monic degree m, U | X^d-g^d,
    U squarefree, U(x_R)!=0, gcd(C,U)=1,

has U,A,C in F_p[X]. Hence its residual points are F_p-rational. This is simultaneous over all such targets R, not a statement averaged over targets.

This statement concerns the core norm and divisor coefficients. Rationality of arbitrary unconstrained slack variables is not claimed. Quotient/remainder layer coefficients are uniquely determined by U; canonical polynomial Bezout witnesses with strict Euclidean degree bounds are also rational. Exporting redundant free slack coordinates would require separate treatment.

## 2. Branch independence is automatic for odd-order curves and a 2-power base

Odd #E(F_p) implies there is no nonzero rational two-torsion. The roots of f are the three nonzero two-torsion abscissas, so f has no F_p root and is irreducible cubic. In particular b!=0. Frobenius permutes its roots alpha_0,alpha_1,alpha_2 in one 3-cycle.

Suppose two distinct roots have ratio zeta in H. Reindex the cycle, possibly replacing zeta by its inverse, so alpha_1=zeta alpha_0. Because zeta is in F_p, applying Frobenius gives alpha_2=zeta alpha_1 and alpha_0=zeta alpha_2. Therefore zeta^3=1. But zeta^d=1 and gcd(3,d)=1, so zeta=1, contradicting distinct roots.

Thus alpha_i/alpha_j is never in H for i!=j. For distinct w_i,w_j in H the polynomials f(w_i G),f(w_j G) have disjoint roots over F_pbar: a common root would give a forbidden ratio alpha_i/alpha_j=w_i/w_j. Each is squarefree of degree three, with nonzero roots. Their squareclasses in F_pbar(G)^*/squares are independent, since each polynomial has roots at which its own valuation is odd and all the others have valuation zero.

No generic-j assumption is needed. In particular a=0,j=0 is not excluded by this argument. Even curve order or a non-2-power H requires an additional branch-independence hypothesis and is not covered automatically.

## 3. Nonzero univariate summation polynomial and root count

Fix an unordered s-element subset {w_1,...,w_s} of H, s>=3, and consider

    F_w(G)=S_s(Gw_1,...,Gw_s).

This polynomial is nonzero. Indeed, suppose it were identically zero. Over K=F_pbar(G), let P_i=(Gw_i,sqrt(f(Gw_i))). The multiquadratic extension adjoining those roots has a Galois automorphism that negates P_i and fixes all P_j for j!=i. The defining property of S_s would give one sign choice with sum_i epsilon_i P_i=O in that extension. Applying the i-only sign flip and subtracting forces [2]P_i=O. A two-torsion point on the constant curve has constant x-coordinate, whereas Gw_i is nonconstant. Contradiction.

The standard per-variable degree is deg_{X_i} S_s=2^(s-2), so total degree is at most s2^(s-2), and the same bounds deg_G F_w. For a self-contained conservative degree derivation, start from S_3 of degree two in each variable and use the standard resultant recursion with S_3: each old coordinate degree doubles, while a newly added coordinate has degree at most 2*deg_T(S_(s-1))=2^(s-2). The bound needs no first/last-fall or Groebner assumption. PKM sections2/3 supply the summation-polynomial relation interface; the original Semaev result is the underlying standard construction. This note does not claim a new summation-polynomial theorem or upgrade an unread external source to full-text verification.

A nonzero polynomial of that degree has at most s2^(s-2) roots in F_p^*. For fixed w-set, the event that points above g*w_i admit some signed zero sum is exactly its vanishing event; the existential signs are already built into S_s. **Do not multiply this bound by another 2^s sign factor.** Union bound over binom(d,s) w-subsets and then over s gives Delta.

The same base V_g is unchanged under multiplying g by H; drawing uniform g merely samples each coset d times. The union bound remains valid for the induced uniform coset distribution.

## 4. Signed usable-base density by a second moment

Let chi be the quadratic character of F_p, with chi(0)=0. Since f has no F_p roots, f(gw) is nonzero for all g,w under consideration and chi(f(gw)) is +/-1. Thus

    B(g)=d+S(g),  S(g)=sum_{w in H} chi(f(gw)).

For each diagonal term, chi(f(gw))^2=1. For distinct w,v, the degree-six polynomial f(wG)f(vG) is squarefree by section2. The standard quadratic-character Weil bound gives

    |sum_{g in F_p^*} chi(f(wg)f(vg))| <= 5 sqrt(p)+1.

One can also get a slightly stronger constant from Hasse-Weil on the smooth genus-two curve Y^2=f(wG)f(vG), tracking its points at infinity; the conservative degree-six bound above suffices. Hasse-Weil/Weil is an explicit standard input theorem, not re-proved here.

Consequently

    E_g[S(g)^2] <= d + d(d-1)(5 sqrt(p)+1)/(p-1).

If B(g)<d/2, then |S(g)|>d/2. Markov's inequality applied to S^2 yields

    Pr[B(g)<d/2]
      <=4/d+4(d-1)/d*(5 sqrt(p)+1)/(p-1)
      <=4/d+4(5 sqrt(p)+1)/(p-1).

The same expression bounds Pr[|B(g)-d|>d/2]. Mean estimates are not needed for this argument. It bounds actual signed rational point count, not the nominal d roots or an unfiltered polynomial base.

## 5. Why excluding short zero sums forces rational core coefficients

Let a geometric norm solution satisfy the selected chart for nonzero R in E(F_p). All roots of U belong to V_g in F_p, since X^d-g^d splits separably over F_p. Each residual point has y satisfying y^2 in F_p, so it belongs to E(F_(p^2)). Frobenius fixes the rational points and negates the nonrational ones.

Let T be the sum of the nonrational residual points. Applying Frobenius to the full residual relation sum P_i=R and subtracting gives [2]T=O. There is no two-torsion in E(F_(p^2)): f is irreducible cubic, and degree three does not divide two. Equivalently #E(F_(p^2))=r(2p+2-r) is odd. Therefore T=O.

Let a be the number of nonrational residual points. If a=1, a nonzero finite point cannot sum to O. If a=2, the two points are inverses and have the same abscissa, excluded by squarefree U. If a=m, their full sum is O rather than nonzero R. If a=m-1, the sole rational residual equals R, excluded by U(x_R)!=0. Thus a>0 would imply 3<=a<=m-2, a short zero-sum subset at distinct abscissas. Outside Z_g this is impossible.

All residual points, and -R, are therefore rational. The defining function A(x)+C(x)y has precisely those zeros and pole divisor (m+1)O. Frobenius gives another function with the same divisor. Their ratio is a constant; normalization of monic A fixes that constant to one. Hence Frobenius fixes A,C, proving A,C in F_p[X]. U is already a rational monic product of its roots.

For m=3 the range 3<=s<=m-2 is empty, so the same reasoning gives deterministic rationality on this chart for nonzero R without any probabilistic short-relation condition. This explains why the first mixed rational/twist counterexamples occur at larger arity. The retained F17,m5,d16 example is not contradicted: Delta there is at least binom(16,3)*3*2/16=210, so the bound is vacuous.

## 6. Growing-arity regime, constants and complete construction cost

Assume the input prime admits the needed 2-power divisor. Let n=log_2 r, choose an odd m=c sqrt(n)+O(1), and choose d as the least power of two satisfying

    d >= 2 (m! r)^(1/m).

Then d<4(m!r)^(1/m). This step is available only if that d divides p-1. No claim that every prime admits it is made. The usable-base density event B>=d/2 puts the nominal distinct-point representation mean near or above one, up to the usual distinct-abscissa correction; it does **not** prove target coverage or useful relation rank.

For d>=2m the consecutive summands N_s=binom(d,s)s2^(s-2) have N_(s-1)/N_s=(s-1)/(2(d-s+1))<1/2. Hence

    Delta <= [2 binom(d,m-2)(m-2)2^(m-4)]/(p-1)
          <= [m(m-1)(m-2)2^(m-3)/d^2] * [d^m/(m!(p-1))]
          <  [m(m-1)(m-2)8^m/(8d^2)] * [r/(p-1)].

Hasse gives r/(p-1)=1+o(1). Stirling and the lower bound on d therefore give

    log_2 Delta <= 3m -2n/m +O(log m)
                =(3c-2/c)sqrt(n)+O(log n).

Every fixed 0<c<sqrt(2/3) makes this exp(-Omega(sqrt(n))). The conservative choice c=1/2 gives a leading exponent -2.5 sqrt(n). Meanwhile 4/d+O(p^-1/2)=exp(-Omega(sqrt(n))). Therefore a random coset has both substantial usable-base density and automatic core-coefficient rationality with probability 1-exp(-Omega(sqrt(n))) in this admissible family.

Finding zeta of exact order d needs only random nonresidue search and exponentiation, since d is a power of two dividing p-1; full factorization of p-1 is unnecessary. Enumerating V_g, checking character values and extracting rational y coordinates costs d*poly(log p) field/bit work using standard finite-field algorithms. Here log d=n/m+O(log m)=O(sqrt(n)), so base construction is sub-exponential. This is not target-independent advice counted for free; its full cost is included.

**Recognition versus probability.** The theorem does not provide a cheap certificate that a particular g has no short zero sum. Testing that property by exhaustive subset search would reintroduce an exponential cost. Instead it is a probabilistic input-family guarantee. A future algorithm can verify every returned rational relation and resample after bounded failures, but any claimed expected runtime must still include the solver's good-instance bound, failure handling and resampling. This note supplies no such solver bound.

**Applicability.** This proof uses scalar Kummer cosets and automatic branch independence for 2-power H. It does not automatically transfer to Dickson/norm-one fibres or auxiliary isogeny towers. P-256's p+1 construction therefore needs a separate analogue. No arbitrary-prime or deployed-curve attack follows.

## 7. Dedicated-solver strategies: exact limitations and a useful oracle

A separate read-only derivation is in continuation-code-lens.md. It reformulates selected rational relation search as singular-support search in an elliptic algebraic-geometry evaluation matrix. For S_R a rational base excluding -R, W_R=L((m+1)O-(-R)) has dimension m. Every m-1 evaluation columns are independent; an m-column submatrix is singular iff the corresponding distinct points sum to R.

Thus a native tiny matrix-rank oracle can independently validate relation-model sets without using the norm exporter's polynomial engine. This is a useful correctness interface, not a fast solver. Enumerating the supports still costs binom(B,m), exponential at the balanced parameters.

Choosing m-1 residual zeros determines the remaining point exactly as R-sum(P_i), which is generic group completion. Choosing m-2 zeros leaves a degree-two Riemann-Roch pencil, equivalently a two-point decomposition under P->S-P. Uniformly guessing g root blocks has success at most V_R binom(m,g)/binom(B,g) for V_R valid full supports. At balanced B^m/m!~r, V_R=exp(o(n)), and g a constant fraction of m, this retains exp(Omega(n)) branch cost even if the residual oracle were free. A shallow, efficiently discovered structural backdoor remains open; guessing a positive fraction of roots is not a sub-exponential recurrence.

The full membership algebra has dimension binom(d,m). Materializing it is also exponential in n. A method must intersect the curve/norm condition before enumerating all membership states; merely replacing ordered roots by divisor coefficients does not supply that pruning.

## 8. What has and has not advanced

The scoped probabilistic rationality lemma is a concrete supporting derivation worth independent checking: it may remove coefficient field equations from the intended sparse Kummer input regime, while preserving the dense F17 regression as a negative control. It is not a solver and does not establish its own novelty.

No constructive complete solver with exp(o(n)) total cost has been obtained. Neither interpolation completion, pencil search, full membership-algebra expansion, nor constant-fraction root guessing meets that requirement under the stated balanced regime. These are precise closures of those strategies, not a universal lower bound for implicit algebraic solving.

## 9. Standard mixed-target nullspace argument: full factor-base rank is unnecessary

This section formalizes a useful standard consequence of the direct mixed-target index-calculus setup in PKM Algorithm1. It is not claimed novel and does not supply a relation-finding oracle. It clarifies which rank hypothesis the complete-cost argument actually needs.

Fix P of prime order r and Q=[k]P, with k unknown. On every attempt draw a,b independently uniformly in F_r, form R=[a]P+[b]Q, and submit only R plus independently drawn solver randomness to a fixed relation oracle. The oracle returns failure or verified coefficients c=(c_1,...,c_B) satisfying

    sum_j c_j F_j=R.

The solver's acceptance, coefficient choice, any label-free adaptive policy, and the decision when enough successes have been collected must depend only on R, independent solver seeds, and past label-free oracle data. Current and past scalar labels a,b must not feed back into those choices. Retain those labels separately for final recovery.

Write R=[u]P. The map (a,b)->(u=a+kb,b) is an invertible linear map of F_r^2. Hence u,b are independent uniform random variables. Conditional on R, b remains uniform. Across fresh attempts the b labels remain independent, and conditioning on the complete label-free oracle transcript, acceptance positions and its stopping time preserves that property.

After B+1 accepted rows, let C be the (B+1)-by-B factor-base coefficient matrix. There is a nonzero vector w in its left nullspace over F_r, irrespective of rank(C). Choose w using only C and independent label-free randomness. Conditional on the complete oracle transcript, w is fixed and the accepted b_i are independent uniform values. Therefore

    beta=sum_i w_i b_i

is uniform in F_r, because w is nonzero. In particular beta!=0 with probability 1-1/r. Let alpha=sum_i w_i a_i. The verified group relations and wC=0 imply

    [alpha]P+[beta]Q=O,
    alpha+k beta=0 mod r.

When beta!=0, recover k=-alpha/beta mod r and independently verify Q=[k]P. If beta=0, retain the failure and collect another independent block or extra rows under the same isolation contract; the expected number of such blocks is 1/(1-1/r).

Repeated factor-base rows and low rank do not invalidate this argument. They can actually produce a dependency earlier. A pathological oracle that accepts only one fixed R has tiny acceptance probability, so it does not contradict generic lower bounds: relation supply cost remains the bottleneck. The lemma removes a separate full-column-rank or independent-FBlog-supply assumption, not the need for a sufficiently cheap complete verified relation source.

This argument is different from target-independent precomputation of every factor-base logarithm, and it does not automatically cover b fixed to1, walk-correlated scalar labels, or selection/solver policies allowed to inspect the a,b labels. Such protocols need their own dependency analysis.

For a fixed memoryless oracle, if B=exp(O(sqrt(n))), verified acceptance probability rho>=exp(-o(n)), expected cost per attempt C=exp(o(n)), and all failed attempts are charged, collecting B+1 rows costs (B+1)C/rho=exp(o(n)). With label-free adaptivity, replace these by conditional bounds: acceptance probability at least rho and conditional expected attempt cost at most C after every possible history before the stopping time. A one-time marginal acceptance rate does not exclude later deadlock. Dense nullspace computation O(B^3) field operations is also exp(o(n)); polynomial bit cost per F_r operation is harmless. Thus the complete conditional bound can replace “independent useful relation rank” by this explicit fresh mixed-target sampling/nullspace protocol. A sharp exp(O(sqrt(n)log n)) total still requires corresponding sharp bounds on C,1/rho,construction and all other phases.


## 10. Optional tighter density threshold (same proof, not needed for section6)

For every eta>0, the second-moment proof gives

    Pr[|B-d|>eta*d] <= eta^(-2) [1/d+(5 sqrt(p)+1)/(p-1)].

Taking eta=1/m proves B=(1+O(1/m))d with failure probability at most m^2/d+O(m^2/sqrt(p)), still negligible in the growing-arity regime. This avoids losing a factor2 in usable-base density. If d is the least power of two above (1-1/m)^(-1)(m!r)^(1/m), then d<2(1-1/m)^(-1)(m!r)^(1/m), and d^m/(m!r) is at most2^m times a bounded constant. The same short-relation estimate then has log2 Delta<=2m-2n/m+O(log m), permitting any fixed c<1 in m=c sqrt(n). The conservative c=1/2 theorem in section6 already suffices; no sharper constant is needed for the qualitative lemma.

This improvement still leaves arbitrary-prime applicability, actual target coverage, and the complete relation solver unresolved.

## 11. Subsequent coverage result supersedes the earlier coverage gap

The parent proposed and this agent re-derived the supporting coverage proof now saved in continuation-coverage.md. Its weighted multiquadratic-cover map-degree argument bounds the full second moment and selected-chart loss, and proves selected-chart coverage at least1/(192m16^m) for a constant fraction of admissible bases (under the explicit finite conditions there). Independent theory review checked the count, degree identity and constants.

Thus the earlier statements above that actual target coverage remains unresolved refer to the state before this subsequent proof. Within the specific admissible Kummer family and sampling protocol, a sub-exponentially small coverage lower bound is now supplied. Full factor-base rank is addressed by section9, and core-coefficient rationality by sections1-6. These three supporting obligations no longer need to be imported as untested distributional assumptions in that restricted family.

The complete uniform implicit solver remains unconstructed and unbounded. If its complete cost were C(n)=exp(o(n)), the other proved overheads fit exp(o(n)). With m≈sqrt(n)/2, B=2^(n/m+O(log m)), and rho_0^(-1)=192m16^m, the non-solver query-count overhead is

    (B+1)/rho_0 =2^(n/m+4m+O(log n))
               =2^(4sqrt(n)+O(log n)).

This is a count of relation-oracle calls, not an ECDLP runtime: each call's cost is precisely the missing breakthrough. No result here turns a slow oracle into a fast one.
