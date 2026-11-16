# Independent review: auxiliary elliptic-base cover

2026-10-30 UTC. Mathematical review only. No relation solver, scientific computation or performance experiment was developed or run. I read the proposed argument, the earlier coverage proof, PKM2016 section3.4 in the retained primary text, and the separate modular-curve counting note. I independently checked the geometric and group-theoretic transfer below; this is not an independent rereading of all of that note's modular-curve sources.

## Verdict and scope

The proposed extension is valid under the stated assumptions. Let the main curve be E/Fp, y²=f(x), with odd whole-curve order and p>3. Let A/Fp be any auxiliary elliptic curve having an Fp-rational cyclic subgroup H of order d=2^t>=8. For a uniform G in A(Fp), use abscissas x_A(G+h), h in H, rejecting 2G in H. The branch independence, rationality exception bound and second-moment coverage proof transfer with the doubled projection degrees and the stated constants. No genericity of A, ordinary/supersingular restriction, trace-distribution assumption or independence of exceptional events is required.

The factor base and isogeny-tower construction are already in PKM2016 section3.4. This review does not establish an efficient positive-relation oracle and does not turn the supporting geometric bounds into a subexponential ECDLP algorithm.

## 1. Branch disjointness

Odd #E(Fp) implies that f is an irreducible cubic: a rational root would give rational nonzero 2-torsion. Since A has rational nonzero 2-torsion, its short-Weierstrass cubic is reducible. It therefore shares no root with f. The three roots alpha of f are unramified values of x_A, giving exactly six distinct geometric points z with x_A(z)=alpha.

These points lie in A(Fp6): alpha lies in Fp3 and its auxiliary y-coordinate needs at most a quadratic extension. Put sigma=Frob². Then sigma³z=z, whereas sigma cycles their three abscissas in length3. Frobenius fixes one nonzero element of A[2], so its permutation of the other two has order at most2. Thus sigma acts trivially on all of A[2]. It also fixes H pointwise.

Suppose z'-z=h in H for two of these branch points.

* If their abscissas agree, z'=-z and 2z belongs to H. Thus delta=sigma(z)-z lies in A[2]. It is fixed by sigma, so sigma³z=z+3delta. Since sigma³z=z and delta has order dividing2, delta=0. This contradicts the abscissa's orbit of length3.
* If their abscissas differ, interchange z,z' if necessary so x_A(z')=x_A(sigma z). Then z'=sigma z or z'=-sigma z. In the first case sigma z=z+h, so sigma³z=z+3h; hence h=0 because H is a 2-group, again a contradiction. In the second case sigma z=-z-h, implying sigma²z=z, also a contradiction.

Consequently distinct translates f(x_A(G+h)) have disjoint sets of six simple zeros. Their poles occur at G=-h with order6, hence even valuation. Every nonempty product has a private simple zero, proving geometric squareclass independence. This proof works for special j and for every auxiliary group structure satisfying the stated rational-subgroup hypothesis.

For any k distinct indices, the smooth multiquadratic cover C of A has degree2^k. The projection

    P_i:C -> E,  G |-> (x_A(G+h_i),y_i)

has degree2^k: x_A has degree2 and the intermediate quadratic extension has degree2 over A. The deck-flip/parallelogram proof therefore gives

    deg(sum_i c_i P_i)=2^k sum_i c_i²

for a nonzero integer coefficient vector. These are actual morphism degrees; inseparability does not invalidate the resulting upper bound on the number of points in a fibre.

## 2. Poles, trace collisions and density

If 2G is not in H, every x_A(G+h) is finite and the d abscissas are distinct. A pole would mean G=-h; a collision at different h,h' would mean G+h=-(G+h'), and either forces 2G in H. The geometric preimage [2]^-1(H) has exactly4d points because p is odd, so at most4d rational samples are excluded. Write M=#A(Fp); the exclusion probability is at most4d/M.

For the density calculation define chi_h(G)=chi(f(x_A(G+h))) away from its pole, and extend the individual character by +1 at G=-h. This is the correct unramified character there: in a standard rational local parameter, x_A has leading term u^-2, so monic f(x_A) has leading term u^-6. For a pair h!=h', its product character at a pole is the product of the two extended individual values; it is not necessarily +1.

The double cover associated to f(x_A(G+h))f(x_A(G+h')) has twelve simple geometric branch points and no ramification at the two poles. Riemann-Hurwitz over genus1 gives genus7. At each rational G its fibre has 1+chi_h(G)chi_h'(G) rational points, including the unramified poles under this extension. Hence

    |sum_G chi_h(G)chi_h'(G)|
      = |#C(Fp)-#A(Fp)| <= (14+2)sqrt(p)=16sqrt(p).

The diagonal characters have square1 everywhere. Thus for S=sum_h chi_h,

    E[S²] <= d+d(d-1)16sqrt(p)/M,
    Pr[S<-d/2] <= 4/d+64sqrt(p)/M.

On an accepted sample, the signed main-curve factor-base size is exactly d+S. On rejected samples that formula only describes an indexed auxiliary quantity; it must not be interpreted as a genuine size. Adding the separate 4d/M exception resolves this distinction.

## 3. Short zero-sums and rationality

For distinct h_1,...,h_s, the rational function S_s(x_A(G+h_1),...,x_A(G+h_s)) is nonzero. Otherwise the connected multiquadratic cover would have a signed identity sum_i +/-P_i=O; flipping one independent y-coordinate would force [2]P_i=O, contradicting its nonconstant abscissa.

The standard per-variable degree bound deg_(X_i) S_s<=2^(s-2) gives a pole of order at most2*2^(s-2) at each of the s distinct points -h_i. Thus its total pole degree, and consequently its number of geometric zeros counted with multiplicity, is at most

    2s2^(s-2).

The union bound is therefore exactly

    Delta_A <= (1/M) sum_(s=3)^(m-2) binom(d,s) 2s2^(s-2).

For accepted G outside this event, the earlier rationality proof applies without modification. All residual abscissas are rational and distinct; Frobenius negates precisely the nonrational residuals; #E(Fp2) is odd; and the selected-chart conditions exclude nonrational subset sizes1,2,m-1,m. The remaining sizes would contradict the short-zero-sum exclusion. Consequently every selected-chart geometric norm solution for a rational nonzero target has rational residuals and rational core coefficients. This is a uniform statement over targets on that good base, not merely an average over targets.

## 4. Exact coverage constants

Use the tuple convention of the retained coverage note: unordered m-subsets of indices with an independent rational sign at each index, and zero tuple counts on rejected G. Set L=binom(d,m) and kappa=d^m/(m!r), where r=#E(Fp). The sign encoding of two tuples is unchanged. Since the projection degrees doubled, the off-diagonal collision sum is at most2m4^m per ordered pair of index subsets. The identical-tuple diagonal is the only zero-map case. Thus

    sum_(G,R) V_G(R)^2 <= sum_G T_G+2m4^m L².

For one m-index set the selected-chart exclusions R=O and R=+/-P_j have total degree at most m2^m(2m+3). With d>=8m² and kappa>=2^m, the earlier argument gives the chart exception probability

    epsilon_chart,A <= 4*4^m(2m²+3m)/M.

For p>=37, Hasse gives r/M<=2. The average target second moment is bounded by

    K_A=2^mL/r+2m4^mL²/(Mr)
       <= 5m4^m kappa².

Outside a further set of bases of probability at most1/4, the individual second moment is at most4K_A. Density and chart-good bases have target mean at least kappa/2^(m+2). Cauchy-Schwarz therefore gives selected-chart coverage

    rho_A >= 1/(320m16^m).

The simultaneous good fraction is at least

    3/4 -4d/M -(4/d+64sqrt(p)/M)
        -epsilon_chart,A -Delta_A.

This is a union bound. The density, trace, chart, moment and rationality events need not be independent. If sampling is explicitly conditioned on passing the trace filter, either divide the remaining unconditional error bounds by its acceptance probability or retain rejection as a charged failed block; silently preserving the original unconditioned probabilities is not justified.

## 5. Exact filter and isogeny membership

Let phi:A->A/H be the separable isogeny with kernel H. The proposed cheap filter is exact:

    [2]phi(G)=O iff phi([2]G)=O iff [2]G is in H.

By contrast, [2d]G=O may reject points outside [2]^-1(H); its geometric kernel can have4d² points. The exact quotient test avoids that unnecessary loss.

For accepted G, phi(G) is neither O nor nonzero 2-torsion. Write the reduced x-map as N_phi(X)/D_phi(X), with degree of N_phi equal to d. Then

    F_G(X)=N_phi(X)-x(phi(G))D_phi(X)

has exactly the d distinct Fp roots x_A(G+h). Indeed the two point fibres phi^-1(+/-phi(G)) are disjoint, rational and exchanged by negation; their x-images coincide. No point in them is auxiliary 2-torsion, so the x-map introduces no ramification. This proves split separable membership; it is not only a cardinality heuristic.

The cyclic rational 2-power kernel permits a chain of rational 2-isogenies. A denominator cannot vanish at a valid abscissa at any intermediate stage: its corresponding auxiliary point would map to O there, and hence to O under the final quotient, contradicting phi(G)!=O. Thus denominator inverses modulo a valid U exist. A shared remainder presentation can impose at each stage the quadratic congruences D_j(R_j)R_(j+1)-N_j(R_j)=0 mod U and D_j(R_j)Z_j=1 mod U, retaining their uniquely bounded monic-division quotients. For degree-2 numerator and degree-1 denominator maps these have O(m) coordinates per stage and degree at most2. This supports the O(mt) quadratic-encoding count, without implying an efficient solving bound.

## 6. Auxiliary sampling and charging

The separate modular-curve note supplies an exact-order-d existence/probability claim under p>=256d^4. Conditional on that claim, the proposed extraction step is correct. Compute N=#A(Fp) by Schoof, let u be its odd part, sample T uniformly in A(Fp), and put S=[u]T. This is uniform in the rational 2-primary subgroup. If that subgroup contains a point of order d, at least half its elements have order at least d: in Z/(2^a) times Z/(2^b), b>=t, the fraction killed by 2^(t-1) is at most1/2. Repeated doubling determines ord(S)=2^e, and [2^(e-t)]S has exact order d when e>=t. It is unnecessary to certify that failed curves have no such point.

Thus a curve-success probability at least1/(24d²) yields an extraction-success probability at least1/(48d²) per independent curve/point trial, and expected at most48d² trials. This uses exact torsion, not just divisibility of N. Uniform rational-point sampling is available with expected polynomial overhead: give each affine point and O one equally likely accepted encoding in a uniform set of size2p+1 and reject invalid encodings. Schoof, scalar arithmetic, exact point sampling, group enumeration and the isogeny chain must all be charged. Their stated classical bounds give expected d² poly(log p) setup, with no factorization of the odd part of N required.

For m=c sqrt(log2 r), c<sqrt(2/3), and d the next power of2 above2(m!r)^(1/m), one has d=exp(O(sqrt(log r)+log log r)). Eventually p>=256d^4, all displayed exceptional probabilities tend to0, and the setup is subexponential. Accordingly this supporting construction can remove the earlier p+/-1 divisibility restriction asymptotically, once the separate modular-curve existence/counting proof is included and charged. It does not remove the unresolved positive-relation oracle obligation. No all-prime-field ECDLP speed claim follows from geometry and setup alone.
